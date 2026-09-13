/// Extracts bits from a unsigned 32-bit integer.
///
/// Follows the common source[13:7] notation.
///
/// # Arguments
///
/// * `source` - The value to extract the bits from
/// * `highest_bit` - The highest bit to extract, inclusive
/// * `lowest_bit` - The lowest bit to extract, inclusive
///
/// # Returns
///
/// The extracted bits shifted to the lowest value bit positions
pub(crate) fn extract_bits_32(source: u32, highest_bit: u32, lowest_bit: u32) -> u32 {
    assert!(highest_bit <= 31, "Highest bit cannot be larger than 31");
    assert!(lowest_bit <= 31, "Lowest bit cannot be larger than 31");
    assert!(
        highest_bit >= lowest_bit,
        "Highest bit cannot be smaller than lowest bit"
    );

    if highest_bit == 31 && lowest_bit == 0 {
        return source;
    }

    let bit_count = highest_bit - lowest_bit + 1;
    let value_unshifted = 2_u32.pow(bit_count) - 1;
    let bit_mask = value_unshifted << lowest_bit;

    (source & bit_mask) >> lowest_bit
}
