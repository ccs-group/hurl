import re

from app import app
from flask import request


@app.route("/post-file-replace", methods=["POST"])
def post_file_replace():
    assert request.headers["Content-Type"] == "application/xml"
    body = request.data.decode("utf-8")
    assert re.search(r"<SaleId>sal_[A-Za-z0-9]{28}</SaleId>", body)
    assert re.search(r"<Name>[^<]+</Name>", body)
    assert "<Name>Placeholder</Name>" not in body
    assert "<Currency>GBP</Currency>" in body
    return ""
