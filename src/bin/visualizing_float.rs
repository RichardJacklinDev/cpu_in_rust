// Similar values exist in std::f32 module
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    // Disect components of a floating point value using tuples
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    // Output components of value as chart
    println!("{} -> {}", n, n_);
    println!("field     |   as bits | as real number");
    println!("sign      |   {:01b}  | {}", sign, sign_);
    println!("exponent  | {:08b}    | {}", exp, exp_);
    println!("mantissa  | {:023b}   | {}", frac, mant);
}

/// Returns the parts of a float value
fn to_parts(n: f32) -> (u32, u32, u32) {
    // Cast float 32 parameter's value as unsigned 32 for bit manipulation
    let bits = n.to_bits();

    // Perform isolation operations on each part of float value
    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    // Return a tuple value containing the parts of float value
    (sign, exponent, fraction)
}

/// Returns the decoded parts of a float value
fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    // Convert sign bit to 1.0 or -1.0
    let signed_1 = (-1.0_f32).powf(sign as f32);

    // Ensure constant is positive when subtracting bias, then cast as f32
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    // Decode mantissa
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    // Return tuple containing parts of decoded float value
    (signed_1, exponent, mantissa)
}

/// Returns a float value from its parts
fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
