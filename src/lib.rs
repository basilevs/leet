#[derive(Clone, Copy)]
struct Bits {
    most_significant: u8,
    count: u8,
    has_any: u8,
    has_none: u8,
}

const DEFAULT_BITS: Bits = Bits {
    most_significant: 0,
    count: 0,
    has_any: 0,
    has_none: 1,
};

const fn compute_bit(mut idx: usize) -> Bits {
    let mut result: Bits = DEFAULT_BITS;
    while idx > 0 {
        result.count += (idx % 2) as u8;
        result.most_significant += 1;
        result.has_any = 1;
        idx >>= 1;
    }
    result.has_none = (result.has_any == 0) as u8;
    result
}

const fn generate_table() -> [Bits; 256] {
    let mut table = [DEFAULT_BITS; 256];
    let mut i = 0;

    while i < 256 {
        table[i] = compute_bit(i);
        i += 1;
    }

    table
}

const LOOKUP_TABLE: [Bits; 256] = generate_table();

pub fn number_of_steps(num: i32) -> i32 {
    let num: usize = num.try_into().expect("Non-negative number expected");
    let b1 = &LOOKUP_TABLE[num & 0xFF];
    let b2 = &LOOKUP_TABLE[(num >> 8) & 0xFF];
    let b3 = &LOOKUP_TABLE[(num >> 16) & 0xFF];
    let b4 = &LOOKUP_TABLE[(num >> 24) & 0xFF];

    let mut acc = 0;
    acc += b1.count + b1.most_significant * b2.has_none * b3.has_none * b4.has_none;
    acc += b2.count + (b2.most_significant + 8 * b2.has_any) * b3.has_none * b4.has_none;
    acc += b3.count + (b3.most_significant + 16 * b3.has_any) * b4.has_none;
    acc += b4.count + (b4.most_significant + 24 * b4.has_any);

    acc as i32
}

pub fn number_of_steps_naive(num: i32) -> i32 {
    let bits = compute_bit(num.try_into().expect("Non-negative number expected"));
    (bits.count + bits.most_significant) as i32
}

#[cfg(test)]
mod tests {
    use super::{number_of_steps, number_of_steps_naive};

    #[test]
    fn matches_reference_on_sample_range() {
        for i in 0..=100_000 {
            assert_eq!(number_of_steps_naive(i), number_of_steps(i), "testing {i}");
        }
    }

    #[test]
    fn handles_edges() {
        for i in [0, 1, 2, 3, 257, 65_535, i32::MAX] {
            assert_eq!(number_of_steps_naive(i), number_of_steps(i), "testing {i}");
        }
    }
}
