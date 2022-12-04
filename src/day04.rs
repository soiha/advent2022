#[derive(Copy, Clone)]
struct Section {
    pub start: i32,
    pub end: i32,
}

impl Section {
    fn new(start: i32, end: i32) -> Section {
        assert!(start <= end);

        Section { start, end }
    }

    fn from_string(input: &str) -> Section {
        let parts: Vec<&str> = input.split('-').collect();
        assert_eq!(parts.len(), 2);

        let start = parts[0].parse::<i32>().unwrap();
        let end = parts[1].parse::<i32>().unwrap();

        Section::new(start, end)
    }

    fn fully_contains(self, other: Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(self, other: Section) -> bool {
        self.fully_contains(other)
            || (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

pub fn day04(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let section_pairs: Vec<(Section, Section)> = lines
        .iter()
        .map(|l| {
            let pair: Vec<&str> = l.split(',').collect();
            assert_eq!(pair.len(), 2);
            (Section::from_string(pair[0]), Section::from_string(pair[1]))
        })
        .collect();

    let mut fully_contains_count = 0;
    let mut overlaps_count = 0;
    section_pairs.iter().for_each(|pair| {
        let (s1, s2) = pair;
        if s1.fully_contains(*s2) || s2.fully_contains(*s1) {
            fully_contains_count += 1;
        }

        if s1.overlaps(*s2) || s2.overlaps(*s1) {
            overlaps_count += 1;
        }
    });

    return format!(
        "{} fully containing sections, {} overlapping",
        fully_contains_count, overlaps_count
    );
}
