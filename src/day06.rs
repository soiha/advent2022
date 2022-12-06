const START_MARKER_LEN: usize = 4;
const MESSAGE_MARKER_LEN: usize = 14;

fn find_distinct_index(s: &str, length: usize) -> usize {
    let mut index: i32 = -1;
    for i in 0..s.len() {
        let slice = &s[i..i + length];
        let is_distinct = slice
            .chars()
            .map(|c| slice.matches(c).count() == 1)
            .all(|t| t == true);
        if is_distinct {
            index = length as i32 + i as i32;
            break;
        }
    }

    assert_ne!(index, -1);

    index as usize
}

pub fn day06(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let binding = lines.join("");
    let input_all = binding.trim();
    let start_marker_at = find_distinct_index(input_all, START_MARKER_LEN);
    let message_marker_at = find_distinct_index(input_all, MESSAGE_MARKER_LEN);

    return format!(
        "Start marker start at {}\nMessage marker at {}",
        start_marker_at, message_marker_at
    );
}
