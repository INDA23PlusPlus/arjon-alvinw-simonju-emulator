# Instruction Set
NOOP: _ , _ , _ , _ // Do nothing
HALT: _ , _ , _ , _ // Halt program
IOUT: r , _ , _ , _ // Print integer
JUMP: r1, r2, r3, i // Jump to @(r1 + i), if r2 = r3

IADD: r1, r2, r3, i // Set r1 to (r2 + r3) + i
ISUB: r1, r2, r3, i // Set r1 to (r2 - r3) - i
IMUL: r1, r2, r3, i // Set r1 to (r2 * r3) * i
IDIV: r1, r2, r3, i // Set r1 to (r2 / r3) / i

# Reserved Registries
ZERO: 0
POSU: 1
NEGU: -1
RAND: ?