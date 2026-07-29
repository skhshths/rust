fn strip_all<'a>(original: &'a str, target: &'a str) -> &'a str {
  original.strip_prefix(target).unwrap().strip_suffix(target).unwrap()
}