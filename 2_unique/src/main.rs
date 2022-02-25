fn main() {
    let list = vec![1, 1, 3];
    println!("{:?}", unique(list))
}

fn unique(list: Vec<i32>) -> Vec<i32> {
    let mut unique_vec: Vec<i32> = Vec::new();

    for num in list {
        if unique_vec.contains(&num) {
        } else {
            unique_vec.push(num);
        }
    }
    return unique_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_unique() {
        let list = vec![1, 4, 5];
        assert_eq!(unique(list), vec![1, 4, 5]);
    }

    #[test]
    fn test_unique() {
        let list = vec![1, 1, 3];
        assert_eq!(unique(list), vec![1, 3]);
    }
}
