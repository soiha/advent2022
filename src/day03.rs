use itertools::Itertools;

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn prio_for_char(c: char) -> i32 {
    let is_up = c.is_ascii_uppercase();
    let mut set = String::from(CHARS.to_ascii_lowercase());
    if is_up {
        set = String::from(CHARS);
    }
    let pos = set.chars().position(|ch| ch == c).unwrap() as i32;
    (pos + 1) + if is_up { 26 } else { 0 }
}

pub fn day03(input: String) -> String {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut prio_tot_all = 0;

    for items in lines {
        let l = items.len() / 2;
        let compartment1 = &items[0..l];
        let compartment2 = &items[l..];

        prio_tot_all += compartment1
            .chars()
            .filter_map(|c| {
                if compartment2.find(c).is_some() {
                    Some(c)
                } else {
                    None
                }
            })
            .unique()
            .map(|c| prio_for_char(c))
            .sum::<i32>();
    }

    lines = input.lines().collect();
    let mut badge_prio_total = 0;
    for group in lines.chunks(3) {
        assert_eq!(group.len(), 3);
        let s: String = group
            .iter()
            .map(|elem| {
                elem.chars()
                    .unique()
                    .collect::<Vec<char>>()
                    .into_iter()
                    .collect::<String>()
            })
            .join("");
        let res: Vec<char> = s
            .chars()
            .map(|c| if s.matches(c).count() == 3 { c } else { ' ' })
            .filter(|c| *c != ' ')
            .unique()
            .collect();
        assert_eq!(res.len(), 1);

        badge_prio_total += prio_for_char(res[0]);
    }

    return format!(
        "Total {}\nBadge prio sum {}",
        prio_tot_all, badge_prio_total
    );
}
