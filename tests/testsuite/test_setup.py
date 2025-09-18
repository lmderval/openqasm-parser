from pathlib import Path

from subprocess import Popen, PIPE

from qiskit.circuit import QuantumCircuit, ClassicalRegister, QuantumRegister
from qiskit.primitives import StatevectorSampler

import numpy as np


def test_parser(binary: Path):
    input = b'''
    OPENQASM 2.0;
    qreg q[2];
    creg c[2];
    U (pi/2, 0, pi) q[0];
    CX q[0], q[1];
    measure q -> c;
    reset q;
    '''
    proc = Popen(
        args=[binary],
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE,
    )
    stdout, stderr = proc.communicate(input=input, timeout=5.0)
    assert proc.returncode == 0
    assert stderr == b""


def test_qiskit():
    q = QuantumRegister(size=2, name="q")
    c = ClassicalRegister(size=2, name="c")

    qc = QuantumCircuit(q, c)
    qc.u(np.pi / 2, 0, np.pi, q[0])
    qc.cx(q[0], q[1])
    qc.measure(q, c)

    sampler = StatevectorSampler()
    result = sampler.run([qc], shots=2000).result()
    pub_data = result[0].data
    counts = pub_data.c.get_counts()

    c00 = int(counts.get("00", 0))
    c11 = int(counts.get("11", 0))

    p00 = c00 / (c00 + c11)
    p11 = c11 / (c00 + c11)

    err = abs(p00 - p11)
    assert err < 0.05
