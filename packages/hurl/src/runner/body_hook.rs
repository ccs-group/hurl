/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;

use crate::http::{Body, RequestSpec};

/// Runs the shell `command` with the body of `request` on standard input, and replaces the body
/// with the command standard output.
///
/// The request method and URL are exposed to the command with `HURL_REQUEST_METHOD` and
/// `HURL_REQUEST_URL` environment variables. Returns an error message if the command can't be
/// spawned or exits with a non-zero status.
pub fn run_body_hook(command: &str, request: &mut RequestSpec) -> Result<(), String> {
    let mut child = shell_command(command)
        .env("HURL_REQUEST_METHOD", request.method.to_string())
        .env("HURL_REQUEST_URL", request.url.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    // Write stdin from a dedicated thread so a large body can't deadlock with the command filling
    // its stdout pipe. Write errors are ignored: the command is free not to read its input.
    let input = request.body.bytes();
    let mut stdin = child.stdin.take().expect("stdin is piped");
    let writer = thread::spawn(move || {
        let _ = stdin.write_all(&input);
    });

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let _ = writer.join();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        let mut message = match output.status.code() {
            Some(code) => format!("exit code {code}"),
            None => "terminated by signal".to_string(),
        };
        if !stderr.is_empty() {
            message.push_str(&format!(" ({stderr})"));
        }
        return Err(message);
    }

    request.body = match String::from_utf8(output.stdout) {
        Ok(text) => Body::Text(text),
        Err(e) => Body::Binary(e.into_bytes()),
    };
    Ok(())
}

#[cfg(unix)]
fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd
}

#[cfg(windows)]
fn shell_command(command: &str) -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(command);
    cmd
}

#[cfg(all(test, unix))]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::http::{Method, Url};

    fn request(body: Body) -> RequestSpec {
        RequestSpec {
            method: Method("POST".to_string()),
            url: Url::from_str("http://localhost:8000/sign").unwrap(),
            body,
            ..Default::default()
        }
    }

    #[test]
    fn test_body_hook_appends_trailer() {
        let mut req = request(Body::Text("hello".to_string()));
        run_body_hook(
            "cat; printf '|%s %s' \"$HURL_REQUEST_METHOD\" \"$HURL_REQUEST_URL\"",
            &mut req,
        )
        .unwrap();
        assert_eq!(
            req.body,
            Body::Text("hello|POST http://localhost:8000/sign".to_string())
        );
    }

    #[test]
    fn test_body_hook_file_and_binary_body() {
        let mut req = request(Body::File(b"abc".to_vec(), "data.bin".to_string()));
        run_body_hook("tr a-z A-Z", &mut req).unwrap();
        assert_eq!(req.body, Body::Text("ABC".to_string()));

        let mut req = request(Body::Text(String::new()));
        run_body_hook("printf '\\377'", &mut req).unwrap();
        assert_eq!(req.body, Body::Binary(vec![0xff]));
    }

    #[test]
    fn test_body_hook_ignores_stdin() {
        let mut req = request(Body::Binary(vec![0; 1024 * 1024]));
        run_body_hook("echo replaced", &mut req).unwrap();
        assert_eq!(req.body, Body::Text("replaced\n".to_string()));
    }

    #[test]
    fn test_body_hook_failure() {
        let mut req = request(Body::Text("hello".to_string()));
        let error = run_body_hook("echo boom >&2; exit 3", &mut req).unwrap_err();
        assert_eq!(error, "exit code 3 (boom)");
        assert_eq!(req.body, Body::Text("hello".to_string()));
    }
}
