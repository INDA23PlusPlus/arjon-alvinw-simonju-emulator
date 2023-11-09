pub fn gen_fibonacci_binary() {
    let bytes = vec![
        // A = 4 = 0100
        // B = 5 = 0101
        // C = 6 = 0110
        // D = 7 = 0111
        // E = 8 = 1000

        // // A = 0
        // 0: IADD A ZERO ZERO 0
        0b0100_0100, 0b0000_0000, 0, 0,

        // // B = 1
        // 1: IADD B ZERO ZERO 1
        0b0100_0101, 0b0000_0000, 0, 1,

        // // C = A + B
        // 2: IADD C A B 0
        0b0100_0110, 0b0100_0101, 0, 0,

        // // D = D + 1
        // 3: IADD D D ZERO 1
        0b0100_0111, 0b0111_0000, 0, 1,

        // // print(C)
        // 4: IOUT C
        0b0010_0110, 0, 0, 0,

        // // A = B
        // 5: IADD A ZERO B 0
        0b0100_0100, 0b0000_0101, 0, 0,

        // // B = C
        // 6: IADD B ZERO C 0
        0b0100_0101, 0b0000_0110, 0, 0,

        // // E = 20
        // 7: IADD E ZERO ZERO 20
        0b0100_1000, 0b0000_0000, 0, 20,

        // // jump to 10 if E == D
        // 8: JUMP ZERO E D 10
        0b0011_0000, 0b1000_0111, 0, 10,

        // // jump to 2
        // 9: JUMP ZERO ZERO ZERO 2
        0b0011_0000, 0b0000_0000, 0, 2,

        // // Halt
        // 10: HALT
        0b0001_0000, 0, 0, 0,
    ];

    std::fs::write("fibonacci.bin", bytes).unwrap();
}