const BASE: u32 = 0b0_01111110_00000000000000000000000; // 126 in base 10

fn mock_rand(n: u8) -> f32 {
    // Align input byte to 32 bits, then increase value by shifting
    // 15 bits to the left
    let large_n = (n as u32) << 15;

    // Merges base with input byte
    let f32_bits = BASE | large_n;

    // Interprets f32_bits as an f32 value
    let m = f32::from_bits(f32_bits);

    // Normalzes the output range
    2.0 * (m - 0.5)
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
