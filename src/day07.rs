use std::collections::HashMap;
use std::ops::Add;

const FILESYSTEM_TOTAL_SIZE: u32 = 70000000;
const UPDATE_REQ_SPACE_LIMIT: u32 = 30000000;

#[derive(PartialEq, Debug)]
enum DirentType {
    File,
    Directory,
}

struct Dirent {
    parent_dir: String,

    e_type: DirentType,
    e_name: String,
    e_size: u32,
}

impl Dirent {
    fn from_string(dir: &str, in_str: &str) -> Self {
        let parts: Vec<&str> = in_str.split(" ").collect();
        assert_eq!(parts.len(), 2);

        if parts[0] == "dir" {
            return Self {
                parent_dir: String::from(dir),
                e_type: DirentType::Directory,
                e_name: String::from(parts[1]),
                e_size: 0,
            };
        }

        Self {
            parent_dir: String::from(dir),
            e_type: DirentType::File,
            e_name: String::from(parts[1]),
            e_size: parts[0].parse::<u32>().unwrap(),
        }
    }

    fn full_path(&self) -> String {
        if self.parent_dir == "/" {
            let r = String::from("/").add(&self.e_name);
            return r;
        }

        let r = String::from(self.parent_dir.clone())
            .add("/")
            .add(&self.e_name);
        r
    }
}

fn dirents_in_dir<'a>(path: &'a str, files: &'a HashMap<String, Dirent>) -> Vec<&'a Dirent> {
    let res = files
        .iter()
        .filter_map(|elem| {
            let (_, dirent) = elem;
            if dirent.parent_dir == path {
                Some(dirent)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    return res;
}

fn calculate_size_for_dir(path: &str, files: &HashMap<String, Dirent>) -> u32 {
    let dir_files = dirents_in_dir(path, files);
    let sizes = dir_files
        .iter()
        .map(|d| {
            if d.e_type == DirentType::File {
                d.e_size
            } else {
                let mut new_path = String::from(path).add("/").add(&d.e_name);
                if path == "/" {
                    new_path = String::from("/").add(&d.e_name);
                }
                calculate_size_for_dir(&new_path, files)
            }
        })
        .collect::<Vec<u32>>();

    let ret = sizes.iter().sum();

    return ret;
}

pub fn day07(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut files: HashMap<String, Dirent> = HashMap::new();

    let mut current_dir: String = String::from("/");
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let firstchar = line.chars().nth(0).unwrap();
        if firstchar == '$' {
            // command
            let parts: Vec<&str> = line.split(" ").collect();
            let command = parts[1];
            let arg: &str = if parts.len() == 3 { parts[2] } else { "" };

            if command == "cd" {
                if arg == "/" {
                    current_dir = String::from("/");
                } else if arg == ".." {
                    let cd_clone = current_dir.clone();
                    let path_elems: Vec<&str> = cd_clone.split("/").collect();
                    current_dir = String::from("/");
                    if path_elems.len() - 1 > 0 {
                        let elem_count = path_elems.len() - 1;
                        for i in 0..elem_count {
                            current_dir = current_dir.add(path_elems[i]);
                            if (i > 0 || current_dir != "/") && i != elem_count - 1 {
                                current_dir = current_dir.add("/");
                            }
                        }
                    }
                } else {
                    if current_dir != "/" {
                        current_dir = current_dir.add("/");
                    }
                    let new_dir = current_dir.add(arg.clone());
                    assert!(files.contains_key(&new_dir));
                    current_dir = new_dir;
                }
            }
        } else {
            // must be dirent
            let dirent = Dirent::from_string(&current_dir, line);
            files.insert(dirent.full_path(), dirent);
        }
    }

    let total_size_less_100k: u32 = files
        .iter()
        .filter_map(|item| {
            let (path, entry) = item;
            if entry.e_type == DirentType::Directory {
                Some((path, calculate_size_for_dir(&entry.full_path(), &files)))
            } else {
                None
            }
        })
        .filter(|p| {
            let (_, size) = *p;
            size <= 100000
        })
        .map(|p| p.1)
        .sum();

    let total_size_root: u32 = calculate_size_for_dir("/", &files);
    let unused_space = FILESYSTEM_TOTAL_SIZE - total_size_root;
    let required_space = UPDATE_REQ_SPACE_LIMIT - unused_space;

    assert!(required_space > 0);

    let mut matching_dirs: Vec<(String, u32)> = files
        .iter()
        .filter_map(|item| {
            let (_, entry) = item;
            if entry.e_type == DirentType::Directory {
                let d_siz = calculate_size_for_dir(&entry.full_path(), &files);
                if d_siz >= required_space {
                    return Some((String::from(&entry.full_path()), d_siz));
                }
            }
            None
        })
        .collect();

    assert!(matching_dirs.len() > 0);

    matching_dirs.sort_by(|a, b| a.1.cmp(&b.1));
    let candidate = &matching_dirs[0];

    return format!(
        "Total size of dirs <100000: {}\nRoot size: {}\nRequired space: {}\nDeletion candidate: {} (size {})",
        total_size_less_100k, total_size_root, required_space, candidate.0, candidate.1
    );
}
