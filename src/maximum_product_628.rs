// https://leetcode.com/problems/maximum-product-of-three-numbers
use itertools::Itertools;

pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let largest: Vec<i32> = nums.iter().k_largest_by_key(6, |&x| i32::abs(*x)).copied().collect();
    largest.iter().combinations(3).map(|c| c.iter().copied().product()).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::maximum_product;

    #[test]
    fn official1() {
        assert_eq!(6, maximum_product(vec![1, 2, 3]));
    }

    #[test]
    fn official2() {
        assert_eq!(24, maximum_product(vec![1, 2, 3, 4]));
    }

    #[test]
    fn official3() {
        assert_eq!(-6, maximum_product(vec![-1, -2, -3]));
    }
    #[test]
    fn official90() {
        assert_eq!(864577350, maximum_product(vec![174,-524,-624,903,982,-219,126,876,-875,-617,-495,-621,194,-333,804,-199,-916,-88,-706,562,-293,-876,-697,975,-6,197,544,-919,-487,432,-849,512,619,44,252,-388,-177,-256,-847,-206,114,116,-827,518,-511,-511,257,-630,56,706,675,-705,-211,170,-13,684,836,-708,336,728,511,-229,-403,310,206,539,784,666,506,-252,-34,709,233,-290,633,29,-550,-412,-778,-107,-123,724,-58,-97,71,776,104,207,-381,-132,88,312,-39,478,-817,-484,-929,651,434,-911]));
    }


}
