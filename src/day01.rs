pub fn day01(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut calorie_counts: Vec<i64> = vec![];
    let mut elf_index: usize = 0;
    let mut calories_max: i64 = 0;
    let mut calories_max_index: usize = 0;

    calorie_counts.push(0);
    for elem in lines {
        if elem.trim().len() > 0 {
            let calorie_val: u32 = elem.trim().parse().unwrap_or(0);
            calorie_counts[elf_index] += calorie_val as i64;
            if calorie_counts[elf_index] >= calories_max {
                calories_max = calorie_counts[elf_index];
                calories_max_index = elf_index;
            }
        } else {
            elf_index += 1;
            calorie_counts.push(0);
        }
    }

    assert!(calorie_counts.len() >= 3);

    calorie_counts.sort_by(|a, b| b.cmp(a));
    let top3_total = calorie_counts[0] + calorie_counts[1] + calorie_counts[2];

    return format!(
        "Max calories {} on index {}, top three total {}",
        calories_max, calories_max_index, top3_total
    );
}
