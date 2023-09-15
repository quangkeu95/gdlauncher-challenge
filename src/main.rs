use std::collections::HashSet;

use gdlauncher_challenge::utils::FileUtils;

fn main() {
    let contents: Vec<u128> = FileUtils::parse_text_file("challenge_input.txt").unwrap();

    let secure_range = 100usize;

    if contents.len() < secure_range {
        panic!("need at least a vector of 100 items");
    }

    let mut i = 0;
    let mut j = secure_range;

    while j < contents.len() {
        let mut set = HashSet::<u128>::new();
        let mut first_num: Option<u128> = None;
        let mut second_num: Option<u128> = None;

        'inner: while i < j {
            if contents[j] <= contents[i] {
                i += 1;
                continue;
            }
            let diff = contents[j] - contents[i];

            if set.contains(&diff) {
                //
                first_num = Some(contents[i]);
                second_num = Some(diff);
                break 'inner;
            } else {
                set.insert(contents[i]);
            }
            i += 1;
        }

        if first_num.is_none() && second_num.is_none() {
            dbg!(contents[j], j);
        }

        j += 1;
        i = j - secure_range;
    }
}
