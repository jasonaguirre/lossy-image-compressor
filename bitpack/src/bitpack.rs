use std::convert::TryInto;

/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    // check if n is between width
    let max = (1_i32 << width - 1) - 1;
    let min = (1_i32 << (width - 1)) * -1;
    if n >= min.into() && n <= max.into(){
        true
    }
    else{
        false
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {

    let max = (1_u32 << width) - 1;
    if n <= max.try_into().unwrap(){
        true
    }
    else{
        false
    }
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    let first_left = 32 - (lsb + width);
    let first_right = 32 - width;
    (((word << first_left >> first_right << lsb) as i32) << first_left >> first_right) as i64
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    word << 64 - (lsb + width) >> 64 - width
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, _width: u64, lsb: u64, value: u64) -> Option<u64> {
    Some(word | (value << lsb))

}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {

    let conversion = (value & ((1 << width) - 1) as i64) as u64;
    Some(word | (conversion << lsb))

}


#[cfg(test)]
mod tests {

    use crate::bitpack::{newu,news};

    #[test]
    fn it_works() {
        let mut word: u64 = 0;
        let a = 133;
        let b = -1;
        let c = -1;
        let d = 0;
        let pb = 2;
        let pr = 3;
        word = newu(word, 4, 0, pr).unwrap();
        word = newu(word, 4, 4, pb).unwrap();
        word = news(word, 5, 8, d).unwrap();
        word = news(word, 5, 13, c).unwrap();
        word = news(word, 5, 18, b).unwrap();
        word = newu(word, 9, 23, a).unwrap();
        assert_eq!(1124065315, word)
    }
}
