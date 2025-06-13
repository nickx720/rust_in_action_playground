use std::collections::VecDeque;

use crate::md5::md5;

#[derive(Debug)]
struct MappedItem {
    original: String,
    computed_hash: String,
}
impl MappedItem {
    pub fn new(original: String, computed_hash: String) -> Self {
        Self {
            original,
            computed_hash,
        }
    }
}

pub fn crack(item: String) -> String {
    let mut queue = VecDeque::new();
    let items = generate_perumates(&mut queue, 4)
        .iter()
        .map(|item| {
            let computed_hash = md5(item.to_owned());
            MappedItem::new(item.to_owned(), computed_hash)
        })
        .collect::<Vec<MappedItem>>();
    let item = items.iter().find(|mapped| {
        if item == mapped.computed_hash {
            return true;
        }
        false
    });
    if let Some(output) = item {
        output.original.to_owned()
    } else {
        "".to_string()
    }
}

//queue = [""]
//while queue not empty:
//    word = queue.pop_front()
//    if word.length == 4:
//        result.append(word)
//    else:
//        for letter in alphabet:
//            if letter not in word: // avoid repetition
//                queue.push(word + letter)
//
pub fn generate_perumates(queue: &mut VecDeque<String>, length: usize) -> Vec<String> {
    let alphabets = "abcdefghijklmnopqrstuvwxyz".to_string();
    let alphabets_vec = alphabets
        .chars()
        .map(|item| item.to_ascii_uppercase().to_string())
        .collect::<Vec<String>>();
    let mut result = Vec::new();
    queue.push_back("".to_string());
    while !queue.is_empty() {
        if let Some(curr) = queue.pop_front() {
            if curr.len() == length {
                result.push(curr);
            } else {
                for letter in alphabets_vec.iter() {
                    let new_word = format!("{}{}", curr, letter);
                    queue.push_back(new_word);
                }
            }
        } else {
            break;
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cracking_pass() {
        let item = "7a95bf926a0333f57705aeac07a362a2".to_string();
        let cracked = crack(item);
        assert_eq!(cracked, "PASS".to_string());
    }
    #[test]
    fn test_cracking_code() {
        let item = "08054846bbc9933fd0395f8be516a9f9".to_string();
        let cracked = crack(item);
        assert_eq!(cracked, "CODE".to_string());
    }
}
