OPENQASM 2.0;

qreg q[1];
creg c[1];

U (pi/2, 0, pi) q;

measure q -> c;
