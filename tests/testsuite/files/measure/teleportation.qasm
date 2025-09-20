OPENQASM 2.0;

qreg q[3];
creg c[2];

// Initialize message qubit
U (-0.4, 0.3, 1.2) q[0];

// Prepare Bell state
U (pi/2, 0, pi) q[1];
CX q[1], q[2];

// Encode message
CX q[0], q[1];
U (pi/2, 0, pi) q[0];

// Measure and send
measure q[0] -> c[0];
measure q[1] -> c[1];

// Decode message
CX q[1], q[2];

/* CZ gates can be written using H and CX gates:
 *
 *            --Z-- = --H-X-H--
 *
 * so,
 *
 *            --x--   ----x----
 *              |   =     |
 *            --Z--   --H-X-H-- */
U (pi/2, 0, pi) q[2];
CX q[0], q[2];
U (pi/2, 0, pi) q[2];
