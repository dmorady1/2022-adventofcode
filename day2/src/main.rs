use std::collections::HashMap;
use std::fs;

use std::mem::ManuallyDrop;
use std::ptr;

// https://users.rust-lang.org/t/hashmap-with-tuple-keys/12711/7
fn get(map: HashMap<(String, String), i64>, a: &String, b: &String) -> Option<i64> {
    unsafe {
        // The 24-byte string headers of `a` and `b` may not be adjacent in
        // memory. Copy them (just the headers) so that they are adjacent. This
        // makes a `(String, String)` backed by the same data as `a` and `b`.
        let k = (ptr::read(a), ptr::read(b));

        // Make sure not to drop the strings, even if `get` panics. The caller
        // or whoever owns `a` and `b` will drop them.
        let k = ManuallyDrop::new(k);

        // Deref `k` to get `&(String, String)` and perform lookup.
        let v = map.get(&k);

        // Turn `Option<&i64>` into `Option<i64>`.
        v.cloned()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct TwoStrings {
    a: String,
    b: String,
}

fn part1(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut mapping: HashMap<TwoStrings, i32> = HashMap::new();
    mapping.insert(
        TwoStrings {
            a: "A".to_owned(),
            b: "Y".to_owned(),
        },
        6 + 2,
    );
    mapping.insert(
        TwoStrings {
            a: "A".to_owned(),
            b: "X".to_owned(),
        },
        3 + 1,
    );
    mapping.insert(
        TwoStrings {
            a: "A".to_owned(),
            b: "Z".to_owned(),
        },
        0 + 3,
    );
    mapping.insert(
        TwoStrings {
            a: "B".to_owned(),
            b: "X".to_owned(),
        },
        0 + 1,
    );
    mapping.insert(
        TwoStrings {
            a: "B".to_owned(),
            b: "Y".to_owned(),
        },
        3 + 2,
    );
    mapping.insert(
        TwoStrings {
            a: "B".to_owned(),
            b: "Z".to_owned(),
        },
        6 + 3,
    );
    mapping.insert(
        TwoStrings {
            a: "C".to_owned(),
            b: "X".to_owned(),
        },
        6 + 1,
    );
    mapping.insert(
        TwoStrings {
            a: "C".to_owned(),
            b: "Y".to_owned(),
        },
        0 + 2,
    );
    mapping.insert(
        TwoStrings {
            a: "C".to_owned(),
            b: "Z".to_owned(),
        },
        3 + 3,
    );
    let result: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let result: i32 = result
        .iter()
        .map(|words| {
            mapping
                .get(&TwoStrings {
                    a: words[0].to_owned(),
                    b: words[1].to_owned(),
                })
                .unwrap()
        })
        .sum();

    return result;
}

fn part2(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut mapping: HashMap<TwoStrings, i32> = HashMap::new();
    mapping.insert(
        TwoStrings {
            a: "A".to_owned(),
            b: "Y".to_owned(),
        },
        3 + 1,
    );
    mapping.insert(
        TwoStrings {
            a: "A".to_owned(),
            b: "X".to_owned(),
        },
        0 + 3,
    );
    mapping.insert(
        TwoStrings {
            a: "A".to_owned(),
            b: "Z".to_owned(),
        },
        6 + 2,
    );
    mapping.insert(
        TwoStrings {
            a: "B".to_owned(),
            b: "X".to_owned(),
        },
        0 + 1,
    );
    mapping.insert(
        TwoStrings {
            a: "B".to_owned(),
            b: "Y".to_owned(),
        },
        3 + 2,
    );
    mapping.insert(
        TwoStrings {
            a: "B".to_owned(),
            b: "Z".to_owned(),
        },
        6 + 3,
    );
    mapping.insert(
        TwoStrings {
            a: "C".to_owned(),
            b: "X".to_owned(),
        },
        0 + 2,
    );
    mapping.insert(
        TwoStrings {
            a: "C".to_owned(),
            b: "Y".to_owned(),
        },
        3 + 3,
    );
    mapping.insert(
        TwoStrings {
            a: "C".to_owned(),
            b: "Z".to_owned(),
        },
        6 + 1,
    );
    let result: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let result: i32 = result
        .iter()
        .map(|words| {
            mapping
                .get(&TwoStrings {
                    a: words[0].to_owned(),
                    b: words[1].to_owned(),
                })
                .unwrap()
        })
        .sum();

    return result;
}

fn main() {
    let file_path: &str = "input.txt";
    let result = part1(file_path);
    println!("Part 1 Result: {}", result);

    let result = part2(file_path);
    println!("Part 2 Result: {}", result);
}
