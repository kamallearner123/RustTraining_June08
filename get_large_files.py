#!/usr/bin/env python3

from pathlib import Path

ROOT_DIR = "."

SIZE_LIMIT_MB = 10
SIZE_LIMIT_BYTES = SIZE_LIMIT_MB * 1024 * 1024

print(f"Files larger than {SIZE_LIMIT_MB} MB:\n")

for file_path in Path(ROOT_DIR).rglob("*"):
    if file_path.is_file():
        size = file_path.stat().st_size
        if size > SIZE_LIMIT_BYTES:
            print(
                f"{size / (1024 * 1024):8.2f} MB  {file_path}"
            )
