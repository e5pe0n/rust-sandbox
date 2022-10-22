pub fn split_into_string_iter(s: &'static str) -> impl Iterator<Item = String> {
    s.split(' ').map(String::from)
}
