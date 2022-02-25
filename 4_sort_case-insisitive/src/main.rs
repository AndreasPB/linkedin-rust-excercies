fn main() {
    let users = vec!["Todd", "amy"];
    println!(
        "Sorted users case-insensitively: {:?}",
        sort_usernames(users)
    );
}

fn sort_usernames(mut users: Vec<&str>) -> Vec<&str> {
    users.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    return users;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let users = vec!["Todd", "amy"];
        assert_eq!(sort_usernames(users), vec!["amy", "Todd"]);
    }
}
