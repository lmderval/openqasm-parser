OPENQASM 2.0;

qreg q[2];
creg c[2];

U (pi/2, 0, pi) q[0];
CX q[0], q[1];

measure q -> c;
