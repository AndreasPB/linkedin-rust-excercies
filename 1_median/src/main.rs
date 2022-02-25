fn main() {}

pub fn median(nums: Vec<f32>) -> Option<f32> {
    println!("Len of list: {}", nums.len());
    let middle_index = nums.len() / 2;

    if nums.len() % 2 == 0 {
        return Some((nums[middle_index] + nums[middle_index - 1]) / 2.0);
    } else {
        return Some(nums[middle_index]);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd() {
        let nums_odd = vec![1.0, 4.0, 5.0];
        assert_eq!(median(nums_odd), Some(4.0));
    }

    #[test]
    fn test_even() {
        let nums_even = vec![1.0, 3.0, 5.0, 8.8];
        assert_eq!(median(nums_even), Some(4.0));
    }

    #[test]
    fn test_empty() {
        let nums_odd = vec![1.0, 4.0, 5.0];
        assert_eq!(median(nums_odd), Some(4.0));
    }
}
