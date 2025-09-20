import os

from pathlib import Path


FILES_PATH = Path(os.path.dirname(__file__), "files")


def load(categories: list[str]) -> list[Path]:
    tests = []
    for category in categories:
        tests.extend((FILES_PATH / category).iterdir())
    return tests
