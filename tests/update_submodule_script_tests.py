#!/usr/bin/env python3
"""Integration tests for the repository submodule update script."""

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
SCRIPT_SOURCE = REPOSITORY_ROOT / "update-submodule.sh"


class UpdateSubmoduleScriptTests(unittest.TestCase):
    """Exercises submodule updates against isolated local Git repositories."""

    def setUp(self) -> None:
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary_directory.name)
        self.remote = self.root / "rs-ci.git"
        self.seed = self.root / "seed"
        self.other_remote = self.root / "other.git"
        self.other_seed = self.root / "other-seed"
        self.parent = self.root / "parent"
        self.run_git("init", "--bare", "--initial-branch=main", self.remote)
        self.run_git("init", "--initial-branch=main", self.seed)
        self.configure_identity(self.seed)
        (self.seed / "README.md").write_text("initial\n", encoding="utf-8")
        self.run_git("add", "README.md", cwd=self.seed)
        self.run_git("commit", "-m", "initial", cwd=self.seed)
        self.run_git("remote", "add", "origin", str(self.remote), cwd=self.seed)
        self.run_git("push", "-u", "origin", "main", cwd=self.seed)

        self.run_git("init", "--bare", "--initial-branch=dev", self.other_remote)
        self.run_git("init", "--initial-branch=dev", self.other_seed)
        self.configure_identity(self.other_seed)
        (self.other_seed / "README.md").write_text("initial\n", encoding="utf-8")
        self.run_git("add", "README.md", cwd=self.other_seed)
        self.run_git("commit", "-m", "initial", cwd=self.other_seed)
        self.run_git("remote", "add", "origin", str(self.other_remote), cwd=self.other_seed)
        self.run_git("push", "-u", "origin", "dev", cwd=self.other_seed)

        self.run_git("init", "--initial-branch=main", self.parent)
        self.configure_identity(self.parent)
        shutil.copy2(SCRIPT_SOURCE, self.parent / "update-submodule.sh")
        self.run_git(
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "add",
            "-b",
            "main",
            str(self.remote),
            ".rs-ci",
            cwd=self.parent,
        )
        self.run_git(
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "add",
            "-b",
            "dev",
            str(self.other_remote),
            ".other",
            cwd=self.parent,
        )
        self.run_git("add", ".gitmodules", ".rs-ci", ".other", "update-submodule.sh", cwd=self.parent)
        self.run_git("commit", "-m", "add rs-ci submodule", cwd=self.parent)

    def tearDown(self) -> None:
        self.temporary_directory.cleanup()

    def run_git(
        self,
        *arguments: object,
        cwd: Path | None = None,
        check: bool = True,
    ) -> subprocess.CompletedProcess[str]:
        return self.run_command("git", *arguments, cwd=cwd, check=check)

    def run_command(
        self,
        *arguments: object,
        cwd: Path | None = None,
        check: bool = True,
    ) -> subprocess.CompletedProcess[str]:
        environment = os.environ.copy()
        environment["GIT_ALLOW_PROTOCOL"] = "file"
        return subprocess.run(
            [str(argument) for argument in arguments],
            cwd=cwd,
            text=True,
            capture_output=True,
            check=check,
            env=environment,
        )

    def configure_identity(self, repository: Path) -> None:
        self.run_git("config", "user.name", "Test User", cwd=repository)
        self.run_git("config", "user.email", "test@example.com", cwd=repository)

    def add_remote_commit(self, seed: Path, contents: str) -> str:
        (seed / "README.md").write_text(contents, encoding="utf-8")
        self.run_git("add", "README.md", cwd=seed)
        self.run_git("commit", "-m", "remote update", cwd=seed)
        self.run_git("push", cwd=seed)
        return self.run_git("rev-parse", "HEAD", cwd=seed).stdout.strip()

    def initialize_submodule(self, path: str) -> None:
        self.run_git(
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "update",
            "--init",
            path,
            cwd=self.parent,
        )

    def run_script(self) -> subprocess.CompletedProcess[str]:
        return self.run_command("bash", "./update-submodule.sh", cwd=self.parent, check=False)

    def switch_to_tracking_branch(self, submodule: Path, branch: str) -> None:
        main_exists = self.run_git(
            "show-ref",
            "--verify",
            "--quiet",
            "refs/heads/main",
            cwd=submodule,
            check=False,
        ).returncode == 0
        if main_exists:
            self.run_git("switch", branch, cwd=submodule)
        else:
            self.run_git("switch", "-c", branch, "--track", f"origin/{branch}", cwd=submodule)
        self.run_git("branch", "--set-upstream-to=origin/" + branch, branch, cwd=submodule)

    def assert_submodule_tracks_remote_branch(self, path: str, branch: str) -> None:
        submodule = self.parent / path
        actual_branch = self.run_git("branch", "--show-current", cwd=submodule).stdout.strip()
        upstream = self.run_git(
            "rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}", cwd=submodule
        ).stdout.strip()
        head = self.run_git("rev-parse", "HEAD", cwd=submodule).stdout.strip()
        remote_main = self.run_git("rev-parse", "origin/" + branch, cwd=submodule).stdout.strip()
        self.assertEqual(branch, actual_branch)
        self.assertEqual("origin/" + branch, upstream)
        self.assertEqual(remote_main, head)

    def assert_all_submodules_track_configured_branches(self) -> None:
        self.assert_submodule_tracks_remote_branch(".rs-ci", "main")
        self.assert_submodule_tracks_remote_branch(".other", "dev")

    def test_initializes_uninitialized_submodule_on_main(self) -> None:
        self.run_git("submodule", "deinit", "--force", "--all", cwd=self.parent)
        shutil.rmtree(self.parent / ".rs-ci", ignore_errors=True)
        shutil.rmtree(self.parent / ".other", ignore_errors=True)

        result = self.run_script()

        self.assertEqual(0, result.returncode, result.stdout + result.stderr)
        self.assert_all_submodules_track_configured_branches()

    def test_fast_forwards_existing_submodule_main(self) -> None:
        self.initialize_submodule(".rs-ci")
        self.initialize_submodule(".other")
        latest_main_commit = self.add_remote_commit(self.seed, "updated main\n")
        latest_dev_commit = self.add_remote_commit(self.other_seed, "updated dev\n")

        result = self.run_script()

        self.assertEqual(0, result.returncode, result.stdout + result.stderr)
        self.assert_all_submodules_track_configured_branches()
        self.assertEqual(
            latest_main_commit,
            self.run_git("rev-parse", "HEAD", cwd=self.parent / ".rs-ci").stdout.strip(),
        )
        self.assertEqual(
            latest_dev_commit,
            self.run_git("rev-parse", "HEAD", cwd=self.parent / ".other").stdout.strip(),
        )

    def test_rejects_dirty_submodule(self) -> None:
        self.initialize_submodule(".rs-ci")
        self.initialize_submodule(".other")
        submodule = self.parent / ".rs-ci"
        self.switch_to_tracking_branch(submodule, "main")
        (submodule / "README.md").write_text("local change\n", encoding="utf-8")
        original_head = self.run_git("rev-parse", "HEAD", cwd=submodule).stdout.strip()

        result = self.run_script()

        self.assertNotEqual(0, result.returncode)
        self.assertIn("uncommitted", result.stderr)
        self.assertEqual(original_head, self.run_git("rev-parse", "HEAD", cwd=submodule).stdout.strip())
        self.assertEqual("local change\n", (submodule / "README.md").read_text(encoding="utf-8"))

    def test_rejects_diverged_local_main(self) -> None:
        self.initialize_submodule(".rs-ci")
        self.initialize_submodule(".other")
        submodule = self.parent / ".rs-ci"
        self.configure_identity(submodule)
        self.switch_to_tracking_branch(submodule, "main")
        (submodule / "LOCAL.md").write_text("local\n", encoding="utf-8")
        self.run_git("add", "LOCAL.md", cwd=submodule)
        self.run_git("commit", "-m", "local commit", cwd=submodule)
        local_head = self.run_git("rev-parse", "HEAD", cwd=submodule).stdout.strip()
        self.add_remote_commit(self.seed, "remote change\n")

        result = self.run_script()

        self.assertNotEqual(0, result.returncode)
        self.assertIn("diverged", result.stderr)
        self.assertEqual(local_head, self.run_git("rev-parse", "HEAD", cwd=submodule).stdout.strip())

    def test_rejects_missing_branch_configuration(self) -> None:
        self.run_git("submodule", "deinit", "--force", "--all", cwd=self.parent)
        shutil.rmtree(self.parent / ".rs-ci", ignore_errors=True)
        shutil.rmtree(self.parent / ".other", ignore_errors=True)
        self.run_git(
            "config",
            "--file",
            ".gitmodules",
            "--unset",
            "submodule..other.branch",
            cwd=self.parent,
        )

        result = self.run_script()

        self.assertNotEqual(0, result.returncode)
        self.assertIn("branch", result.stderr)
        self.assertFalse((self.parent / ".rs-ci" / ".git").exists())
        self.assertFalse((self.parent / ".other" / ".git").exists())


if __name__ == "__main__":
    unittest.main()
