"""Exercise the actual desktop project with an isolated GitHub service and profile."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import threading
from http.server import HTTPServer

from github_fixture import GitHubFixture


def run():
    repo = Path(__file__).resolve().parents[1]
    with tempfile.TemporaryDirectory(prefix="gosh-migration-") as folder:
        root = Path(folder)
        name = root.name
        project = root / "desktop"
        shutil.copytree(repo / "desktop", project, ignore=shutil.ignore_patterns(".godot"))
        # Windows and macOS use platform APIs for user:// and may ignore XDG overrides.
        # Give this test copy a unique profile before starting Godot.
        (project / "override.cfg").write_text(
            f'[application]\nconfig/custom_user_dir_name="{name}"\n', encoding="utf-8"
        )
        env = dict(os.environ, GOSH_TEST_ROOT=root.as_posix(), XDG_DATA_HOME=str(root / "data"),
                   XDG_CACHE_HOME=str(root / "cache"), XDG_CONFIG_HOME=str(root / "config"))
        origin = root / "origin"
        subprocess.run(["git", "init", "-qb", "main", origin], check=True)
        git = ["git", "-C", str(origin)]
        for key, value in [("user.name", "Fixture"), ("user.email", "fixture@example.invalid"),
                           ("core.autocrlf", "false")]:
            subprocess.run(git + ["config", key, value], check=True)
        (origin / "example.txt").write_bytes(b"original\n")
        (origin / "skip.tmp").touch()
        subprocess.run(git + ["add", "."], check=True)
        subprocess.run(git + ["commit", "-qm", "initial"], check=True)
        with HTTPServer(("127.0.0.1", 0), GitHubFixture) as server:
            threading.Thread(target=server.serve_forever, daemon=True).start()
            (root / "port").write_text(str(server.server_port))
            engine = ["godot", "--headless", "--path", str(project)]
            try:
                for command in [engine + ["--import"], engine + ["--script", str(repo / "tests/migration.gd")]]:
                    result = subprocess.run(command, env=env, stdout=subprocess.PIPE,
                                            stderr=subprocess.STDOUT, timeout=120)
                    output = result.stdout.decode(errors="replace")
                    print(output)
                    if result.returncode or "SCRIPT ERROR:" in output or "ERROR:" in output:
                        return 1
                return 0 if "Migration checks complete. Failures: 0" in output else 1
            finally:
                server.shutdown()
                marker = root / "profile-path"
                if marker.exists():
                    profile = Path(marker.read_text()).resolve()
                    if profile.name == name:
                        shutil.rmtree(profile, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(run())
