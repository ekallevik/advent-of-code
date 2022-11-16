use anyhow::Result;
use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> Result<String> {
    let _secret = get_input_string(filename);

    //md5::compute();

    Ok("count".to_string())
}

/*
fn find_md5_hash(secret: &str) {
    let k = get_md5_k();

    let mut a = 0xd76aa478;
    let mut b = 0xefcdab89;
    let mut c = 0x98badcfe;
    let mut d = 0x10325476;


    let bytes = secret.bytes();


    let appended = fmt!("{secret}{'1'}");

    // Notice: the two padding steps above are implemented in a simpler way
    //  in implementations that only work with complete bytes: append 0x80
    //  and pad with 0x00 bytes so that the message length in bytes ≡ 56 (mod 64).

    append original length in bits mod 264 to message

// Process the message in successive 512-bit chunks:
    for each 512-bit chunk of padded message do
    break chunk into sixteen 32-bit words M[j], 0 ≤ j ≤ 15
    // Initialize hash value for this chunk:
    var int A := a0
    var int B := b0
    var int C := c0
    var int D := d0
    // Main loop:
    for i from 0 to 63 do
    var int F, g
    if 0 ≤ i ≤ 15 then
    F := (B and C) or ((not B) and D)
    g := i
    else if 16 ≤ i ≤ 31 then
    F := (D and B) or ((not D) and C)
    g := (5×i + 1) mod 16
    else if 32 ≤ i ≤ 47 then
    F := B xor C xor D
    g := (3×i + 5) mod 16
    else if 48 ≤ i ≤ 63 then
    F := C xor (B or (not D))
    g := (7×i) mod 16
    // Be wary of the below definitions of a,b,c,d
    F := F + A + K[i] + M[g]  // M[g] must be a 32-bits block
    A := D
    D := C
    C := B
    B := B + leftrotate(F, s[i])
    end for
    // Add this chunk's hash to result so far:
    a0 := a0 + A
    b0 := b0 + B
    c0 := c0 + C
    d0 := d0 + D
    end for

var char digest[16] := a0 append b0 append c0 append d0 // (Output is in little-endian)

}




const MD5_S: [u32; 64] = [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
];

const MD5_K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1
];



fn get_md5_k() -> Vec<u32> {

    let mut k = Vec::with_capacity(64);

    for i in 0..64 {
        let value = floorf32(232 * sinf32(i+1).abs()) as u32;
        k.push(value)
    };

    k


}


 */

pub fn solve_2(filename: &str) -> Result<String> {
    let _secret = get_input_string(filename);

    Ok("Could not find an answer".to_string())
}