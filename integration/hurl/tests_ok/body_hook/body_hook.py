import re

from app import app
from flask import request

from tests_ok.body_hook.body_hook_sign import sign


@app.route("/body-hook", methods=["GET", "POST"])
def body_hook():
    if request.method == "GET":
        assert request.data == b""
        return ""
    body, trailer = request.data.rsplit(b"\n", 1)
    match = re.fullmatch(rb"<!-- signature=([0-9a-f]{64}) -->", trailer)
    assert match
    assert match.group(1).decode() == sign("POST", request.url, body)
    text = body.decode("utf-8")
    assert re.search(r"<SaleId>sal_[A-Za-z0-9]{28}</SaleId>", text)
    assert "<Currency>GBP</Currency>" in text
    return ""
