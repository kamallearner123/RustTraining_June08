#!/usr/bin/env python3

import os
import subprocess
from pathlib import Path

root_dir = Path.cwd()

for cargo_toml in root_dir.rglob("Cargo.toml"):
    project_dir = cargo_toml.parent

    print(f"\nCleaning: {project_dir}")

    try:
        result = subprocess.run(
            ["cargo", "clean"],
            cwd=project_dir,
            check=True,
            capture_output=True,
            text=True
        )

        print("SUCCESS")

    except subprocess.CalledProcessError as e:
        print("FAILED")
        print(e.stderr)
