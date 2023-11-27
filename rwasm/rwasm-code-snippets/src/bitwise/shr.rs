use crate::consts::BYTE_MAX_VAL;

#[no_mangle]
fn bitwise_shr(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    b0: u64,
    b1: u64,
    b2: u64,
    b3: u64,
) -> (u64, u64, u64, u64) {
    let mut s0 = 0;
    let mut s1 = 0;
    let mut s2 = 0;
    let mut s3 = 0;

    if a0 != 0 || a1 != 0 || a2 != 0 || a3 > BYTE_MAX_VAL {
        // return (0, 0, 0, 0);
    } else if a3 >= 192 {
        let shift = a3 - 192;
        s3 = b0 >> shift;
        // return (0, 0, 0, s3);
    } else if a3 >= 128 {
        let shift = a3 - 128;
        let shift_inv = 64 - shift;
        s2 = b0 >> shift;
        s3 = b0 << shift_inv | b1 >> shift;
        // return (0, 0, s2, s3);
    } else if a3 >= 64 {
        let shift = a3 - 64;
        let shift_inv = 64 - shift;
        s1 = b0 >> shift;
        s2 = b0 << shift_inv | b1 >> shift;
        s3 = b1 << shift_inv | b2 >> shift;
        // return (0, s1, s2, s3);
    } else {
        let shift = a3;
        let shift_inv = 64 - shift;
        s0 = b0 >> shift;
        s1 = b0 << shift_inv | b1 >> shift;
        s2 = b1 << shift_inv | b2 >> shift;
        s3 = b2 << shift_inv | b3 >> shift;
    }
    return (s0, s1, s2, s3);
}
