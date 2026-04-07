#[derive(Clone, Copy)]
struct ByteStats {
    most_significant: u8,
    count: u8,
    has_any: u8,
    has_none: u8,
}

const DEFAULT_BITS: ByteStats = ByteStats {
    most_significant: 0,
    count: 0,
    has_any: 0,
    has_none: 1,
};

const fn compute_stats(mut input: usize) -> ByteStats {
    let mut result: ByteStats = DEFAULT_BITS;
    while input > 0 {
        if input % 2 == 1 {
            result.count += 1;
        }
        result.most_significant += 1;
        result.has_any = 1;
        input >>= 1;
    }
    result.has_none = (result.has_any == 0) as u8;
    result
}

const fn generate_table() -> [ByteStats; 256] {
    let mut table = [DEFAULT_BITS; 256];
    let mut i = 0;

    while i < 256 {
        table[i] = compute_stats(i);
        i += 1;
    }

    table
}

const LOOKUP_TABLE: [ByteStats; 256] = generate_table();

pub fn number_of_steps(num: i32) -> i32 {
    let num: usize = num.try_into().expect("Non-negative number expected");
    let b1 = &LOOKUP_TABLE[num & 0xFF];
    let b2 = &LOOKUP_TABLE[(num >> 8) & 0xFF];
    let b3 = &LOOKUP_TABLE[(num >> 16) & 0xFF];
    let b4 = &LOOKUP_TABLE[(num >> 24) & 0xFF];

    let mut acc: u8 = 0;
    acc += b1.count + b1.most_significant * b2.has_none * b3.has_none * b4.has_none;
    acc += b2.count + (b2.most_significant + 8 * b2.has_any) * b3.has_none * b4.has_none;
    acc += b3.count + (b3.most_significant + 16 * b3.has_any) * b4.has_none;
    acc += b4.count + (b4.most_significant + 24 * b4.has_any);
    acc = acc.saturating_sub(1);

    i32::from(acc)
}

pub fn number_of_steps_declarative(num: i32) -> i32 {
    let num: u32 = num.try_into().expect("Non-negative number expected");
    if num == 0 {
        return 0;
    }

    (num.count_ones() + (u32::BITS - 1 - num.leading_zeros())) as i32
}

pub fn number_of_steps_from_leet(num: i32) -> i32 {
    ((u32::BITS - 1).saturating_sub(num.leading_zeros()) + num.count_ones()) as i32
}

pub fn number_of_steps_imperative(num: i32) -> i32 {
    let num: usize = num.try_into().expect("Non-negative number expected");
    let b1 = &LOOKUP_TABLE[num & 0xFF];
    let b2 = &LOOKUP_TABLE[(num >> 8) & 0xFF];
    let b3 = &LOOKUP_TABLE[(num >> 16) & 0xFF];
    let b4 = &LOOKUP_TABLE[(num >> 24) & 0xFF];

    let mut acc: u8 = 0;
    acc += b1.count;
    acc += b2.count;
    acc += b3.count;
    acc += b4.count;

    if b4.has_any > 0 {
        acc += b4.most_significant + 24
    } else if b3.has_any > 0 {
        acc += b3.most_significant + 16
    } else if b2.has_any > 0 {
        acc += b2.most_significant + 8
    } else if b1.has_any > 0 {
        acc += b1.most_significant
    }
    acc = acc.saturating_sub(1);

    i32::from(acc)
}

pub fn number_of_steps_naive(mut num: i32) -> i32 {
    let mut result: i32 = 0;
    while num > 0 {
        result += num % 2;
        if num > 1 {
            result += 1;
        }
        num >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{
        number_of_steps, number_of_steps_declarative, number_of_steps_from_leet,
        number_of_steps_imperative, number_of_steps_naive,
    };
    const SAMPLES: [i32; 12] = [
        0,
        1,
        2,
        3,
        8,
        14,
        123,
        257,
        65_535,
        83962,
        i32::MAX - 6,
        i32::MAX,
    ];

    #[test]
    fn naive_matches_official_table() {
        for (input, expected) in [(0, 0), (8, 4), (14, 6), (123, 12), (83962, 27)] {
            assert_eq!(expected, number_of_steps_naive(input), "testing {}", input);
        }
    }

    #[test]
    fn number_of_steps_from_leet_matches_naive() {
        for i in SAMPLES {
            assert_eq!(
                number_of_steps_naive(i),
                number_of_steps_from_leet(i),
                "testing {i}"
            );
        }
    }

    #[test]
    fn number_of_steps3_matches_naive() {
        for i in SAMPLES {
            assert_eq!(
                number_of_steps_naive(i),
                number_of_steps_imperative(i),
                "testing {i}"
            );
        }
    }

    #[test]
    fn number_of_steps_declarative_matches_naive() {
        for i in SAMPLES {
            assert_eq!(
                number_of_steps_naive(i),
                number_of_steps_declarative(i),
                "testing {i}"
            );
        }
    }

    #[test]
    fn matches_reference_on_sample_range() {
        for i in 0..=100_000 {
            assert_eq!(number_of_steps_naive(i), number_of_steps(i), "testing {i}");
        }
    }

    #[test]
    fn number_of_steps_matches_naive() {
        for i in SAMPLES {
            assert_eq!(number_of_steps_naive(i), number_of_steps(i), "testing {i}");
        }
    }
}
