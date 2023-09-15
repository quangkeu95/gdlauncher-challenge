use std::collections::HashSet;

pub mod utils;

pub fn find_unsecure_numbers(vector: &[u128], secure_range: usize) -> Vec<(usize, u128)> {
    let mut result = vec![];

    if vector.len() < secure_range {
        panic!("vector length is less than number of secure range");
    }

    let mut i = 0;
    let mut j = secure_range;

    while j < vector.len() {
        let mut set = HashSet::<u128>::new();
        let mut first_num: Option<u128> = None;
        let mut second_num: Option<u128> = None;

        'inner: while i < j {
            if vector[j] <= vector[i] {
                i += 1;
                continue;
            }
            let diff = vector[j] - vector[i];

            if set.contains(&diff) {
                //
                first_num = Some(vector[i]);
                second_num = Some(diff);
                break 'inner;
            } else {
                set.insert(vector[i]);
            }
            i += 1;
        }

        if first_num.is_none() && second_num.is_none() {
            result.push((j, vector[j]));
        }

        j += 1;
        i = j - secure_range;
    }
    result
}
