// https://leetcode.com/problems/angle-between-hands-of-a-clock

pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let minute_angle = (f64::from(minutes) % 60_f64) / 60_f64;
    let hour_angle = (f64::from(hour) % 12_f64) / 12_f64 + minute_angle / 12_f64;
    let diff = (1_f64 + minute_angle - hour_angle) % 1_f64;
    diff.min(1_f64 - diff) * 360_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official1() {
        assert!((angle_clock(12, 30) - 165.0).abs() < 1e-5);
    }

    #[test]
    fn official2() {
        assert!((angle_clock(3, 30) - 75.0).abs() < 1e-5);
    }

    #[test]
    fn official3() {
        assert!((angle_clock(3, 15) - 7.5).abs() < 1e-5);
    }
}
