pub fn get_size_string(position: usize) -> &'static str {
    match position {
        0 => "toy",
        1 => "small",
        2 => "medium",
        _ => "unknown",
    }
}