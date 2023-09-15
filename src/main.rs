use gdlauncher_challenge::{find_unsecure_numbers, utils::FileUtils};

fn main() {
    let contents: Vec<u128> = FileUtils::parse_text_file("challenge_input.txt").unwrap();

    let result = find_unsecure_numbers(&contents, 100);
    dbg!(result);
}
