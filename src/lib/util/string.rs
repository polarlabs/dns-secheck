pub fn split_on_first(s: &str, c: char) -> (Option<&str>, Option<&str>) {
    match s.split_once(c) {
        None => (None, None),
        Some((s1, s2)) => {
            if s1.len() > 0 && s2.len() > 0 {
                (Some(s1), Some(s2))
            } else {
                (Some(s1), None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_on_first() {
        assert_eq!(split_on_first("foo", ':'), (None, None));
        assert_eq!(split_on_first("foo:", ':'), (Some("foo"), None));
        assert_eq!(split_on_first("foo:bar", ':'), (Some("foo"), Some("bar")));
        assert_eq!(
            split_on_first("foo:bar:bar:bar", ':'),
            (Some("foo"), Some("bar:bar:bar"))
        );
    }
}
