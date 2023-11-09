# Instruction Set
NOOP: _ , _ , _ , _ // Do nothing
HALT: _ , _ , _ , _ // Halt program
COUT: r , _ , _ , _ // Print character
IOUT: r , _ , _ , _ // Print integer

JUMP: r1, _ , _ , i // Jump to @(r1 + i)
FORK: r1, r2, r3, i // Jump to @(r3 + i), if r1 = r2
LOAD: r1, r2, _ , i // Load value from @(r2 + i) into r1
POOL: r1, r2, _ , i // Store value r1 at @(r2 + i)

IADD: r1, r2, r3, i // Set r1 to (r2 + r3) + i
ISUB: r1, r2, r3, i // Set r1 to (r2 - r3) - i
IMUL: r1, r2, r3, i // Set r1 to (r2 * r3) * i
IDIV: r1, r2, r3, i // Set r1 to (r2 / r3) / i

# Reserved Registries
ZERO: 0
POSU: 1
NEGU: -1
RAND: ?