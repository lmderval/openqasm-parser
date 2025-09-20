OPENQASM 2.0;

qreg q[2];

U (pi/2, 0, pi) q[0];
CX q[], q[1];
