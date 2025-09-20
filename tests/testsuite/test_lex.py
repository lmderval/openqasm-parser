import pytest

from pathlib import Path

from subprocess import Popen, PIPE

import utils


@pytest.mark.parametrize("program", utils.load(["good", "measure"]))
def test_valid_programs(binary: Path, program: Path):
    input = program.read_bytes()
    proc = Popen(
        args=[binary],
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE,
    )
    stdout, stderr = proc.communicate(input=input, timeout=5.0)
    assert proc.returncode == 0
    assert stderr == b""


@pytest.mark.parametrize("program", utils.load(["lex"]))
def test_invalid_programs(binary: Path, program: Path):
    input = program.read_bytes()
    proc = Popen(
        args=[binary],
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE,
    )
    stdout, stderr = proc.communicate(input=input, timeout=5.0)
    assert proc.returncode == 2
    assert stderr != b""
