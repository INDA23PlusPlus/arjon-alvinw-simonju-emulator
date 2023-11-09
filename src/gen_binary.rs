pub fn gen_test_binary() {
// | // A = 0
// | 0: IADD A ZERO ZERO 0
// | 
// | // B = 1
// | 1: IADD B ZERO ZERO 1
// | 
// | // C = A + B
// | 2: IADD C A B
// | 
// | // D = D + 1
// | 3: IADD D D ZERO 1
// | 
// | // print(C)
// | 4: IOUT C
// | 
// | // A = B
// | 5: IADD A ZERO B 0
// | 
// | // B = C
// | 6: IADD B ZERO C 0
// | 
// | // E = 100
// | 7: IADD E ZERO ZERO 100
// | 
// | // jump to 9 if E == D
// | 7: JUMP ZERO E D 9
// | 
// | // jump to 2
// | 8: JUMP ZERO ZERO ZERO 2
// | 
// | // Halt
// | 9: HALT

    let bytes = vec![
        // // A = 0
        // 0: IADD A ZERO ZERO 0
        0b0100_0100, 0b0000_0000, 0, 0,

        // // B = 1
        // 1: IADD B ZERO ZERO 1
        0b0100_0101, 0b0000_0000, 0, 1,

        // // print(A)
        // 2: IOUT A
        0b0010_0100, 0, 0, 0,

        // // print(B)
        // 2: IOUT B
        0b0010_0101, 0, 0, 0,

        // // HALT
        0b0001_0000, 0, 0, 0
    ];

    std::fs::write("test.bin", bytes).unwrap();
}