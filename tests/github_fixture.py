"""Loopback-only GitHub fixture for token, pagination and device-flow regressions."""
import json
from http.server import BaseHTTPRequestHandler
from urllib.parse import parse_qs, urlsplit


class GitHubFixture(BaseHTTPRequestHandler):
    polls = 0

    def log_message(self, *args):
        pass

    def reply(self, status, body):
        payload = json.dumps(body).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)

    def do_GET(self):
        if self.headers.get("Authorization") != "Bearer fixture-session":
            return self.reply(401, {"message": "Bad credentials"})
        path = urlsplit(self.path)
        if path.path == "/user":
            return self.reply(200, {"login": "fixture"})
        if path.path == "/user/repos":
            page = int(parse_qs(path.query)["page"][0])
            count = 100 if page == 1 else 1
            return self.reply(200, [
                {"id": i + 100 * page, "name": f"fixture-{i}", "full_name": f"fixture/repo-{i}",
                 "owner": {"login": "fixture"}, "forks_count": i, "stargazers_count": 1,
                 "size": 1, "language": "GDScript", "private": False, "description": "",
                 "updated_at": "2026-01-01", "pushed_at": "2026-01-01"}
                for i in range(count)])
        if path.path in ("/user/orgs", "/users/fixture/events"):
            return self.reply(200, [])
        self.reply(404, {})

    def do_POST(self):
        body = parse_qs(self.rfile.read(int(self.headers["Content-Length"])).decode())
        if body.get("client_id") != ["Ov23liGgLL9AjIdQnRFt"]:
            return self.reply(400, {})
        if self.path == "/login/device/code" and body.get("scope") == ["repo,read:user,read:org"]:
            type(self).polls = 0
            return self.reply(200, {"user_code": "TEST-CODE", "device_code": "fixture-device",
                                    "interval": 1, "expires_in": 15,
                                    "verification_uri": "https://github.com/login/device"})
        if self.path == "/login/oauth/access_token" and body.get("device_code") == ["fixture-device"]:
            type(self).polls += 1
            if self.polls == 1:
                return self.reply(200, {"error": "authorization_pending"})
            return self.reply(200, {"access_token": "fixture-session"})
        self.reply(400, {})
