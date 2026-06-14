import os
from pathlib import Path
from datetime import datetime

ROOT_DIR = "./"


def get_date_prefix(path):
    ts = path.stat().st_mtime
    dt = datetime.fromtimestamp(ts)
    return dt.strftime("%B%d")  # Example: June10


# Delete .out files
for path in Path(ROOT_DIR).rglob("*.out"):
    print(f"Deleting: {path}")
    path.unlink()


# Rename files first (deepest paths first)
all_paths = sorted(
    Path(ROOT_DIR).rglob("*"),
    key=lambda p: len(p.parts),
    reverse=True
)

for path in all_paths:
    if not path.exists():
        continue

    prefix = get_date_prefix(path)

    # Skip if already renamed
    if path.name.startswith(prefix + "_"):
        continue

    new_name = f"{prefix}_{path.name}"
    new_path = path.parent / new_name

    try:
        path.rename(new_path)
        print(f"Renamed: {path} -> {new_path}")
    except Exception as e:
        print(f"Failed: {path} ({e})")
