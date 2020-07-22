use std::collections::HashMap;

const TEXT: &str = "Rust has a focus on safety and speed. It accomplishes these goals through many ‘zero-cost abstractions’, which means that in Rust, abstractions cost as little as possible in order to make them work. The ownership system is a prime example of a zero cost abstraction. All of the analysis we’ll talk about in this guide is done at compile time. You do not pay any run-time cost for any of these features";

fn bubble_sort(char_st: &mut Vec<(char, u8)>, start: usize, end: usize) {
    for i in start..end {
        for j in start..end {
            if (char_st[i].0 as u16) < (char_st[j].0 as u16) {
                let tmp = char_st[i];
                char_st[i] = char_st[j];
                char_st[j] = tmp;
            }
        }
    }
}

fn sort(char_st: &mut Vec<(char, u8)>, start: usize, end: usize) {
    if end - start == 1 {
        return ();
    }
    let bissection_width: usize = (end - start) / 2;
    sort(char_st, start, start + bissection_width);
    sort(char_st, start + bissection_width, end);
    bubble_sort(char_st, start, end);
}

fn main() {
    let mut char_stats: HashMap<char, u8> = HashMap::new(); 
    let characters = TEXT.chars().flat_map(char::to_lowercase).collect::<Vec<char>>();
    for single_char in characters {
        match char_stats.get_mut(&single_char) {
            Some(chr_nmb) => {
                *chr_nmb += 1;
                0
            },
            None => {
                char_stats.insert(single_char, 1);
                0
            }
        };
    }
    let mut vec_order: Vec<(char, u8)> = Vec::with_capacity(char_stats.len());
    for single_char in char_stats {
        vec_order.push((single_char.0, single_char.1));
    }
    let vec_len = vec_order.len();
    sort(&mut vec_order, 0 , vec_len);
    for single_char in vec_order {
        println!("{} occurs {} times", single_char.0, single_char.1);
    }
}
