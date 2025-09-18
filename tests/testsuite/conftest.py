from pathlib import Path


def pytest_addoption(parser):
    parser.addoption("--binary", action="store")


def pytest_generate_tests(metafunc):
    binary = Path(metafunc.config.option.binary)
    if "binary" in metafunc.fixturenames and binary is not None:
        metafunc.parametrize("binary", [binary])
