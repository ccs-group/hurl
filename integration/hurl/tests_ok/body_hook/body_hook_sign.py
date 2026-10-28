# Body hook used by `body_hook.hurl`: reads the request body on standard input and writes it
# back on standard output, with an HMAC-SHA256 signature trailer.
import hashlib
import hmac
import os
import sys

KEY = b"secret"


def sign(method: str, url: str, body: bytes) -> str:
    message = f"{method} {url}\n".encode() + body
    return hmac.new(KEY, message, hashlib.sha256).hexdigest()


if __name__ == "__main__":
    body = sys.stdin.buffer.read()
    signature = sign(os.environ["HURL_REQUEST_METHOD"], os.environ["HURL_REQUEST_URL"], body)
    sys.stdout.buffer.write(body + f"\n<!-- signature={signature} -->".encode())
