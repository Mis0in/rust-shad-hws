#![forbid(unsafe_code)]

use std::collections::HashMap;
////////////////////////////////////////////////////////////////////////////////

pub type IniFile = HashMap<String, HashMap<String, String>>;

fn trim_concat(s: &[&str]) -> String {
    s.join("").trim().to_string()
}

pub fn parse(content: &str) -> IniFile {
    let input = content.split('\n');
    let mut section = "".to_string();

    let mut result: IniFile = HashMap::new();

    input.for_each(|line| {
        let binding = line.trim().split("").collect::<Vec<&str>>();
        let prepared = binding.as_slice();
        match prepared {
            ["", ""] => {}
            ["", "[", .., "]", ""] => match prepared[2..prepared.len() - 2].join("") {
                s if s.is_empty() => panic!("Invalid format: empty section"),
                s if s.contains("[") || s.contains("]") => {
                    panic!("Invalid format: section with parenthesis")
                }
                s => {
                    section = s.to_string();
                    if !result.contains_key(&section) {
                        result.insert(section.clone(), HashMap::new());
                    }
                }
            },
            key_value if !section.is_empty() => {
                match key_value.iter().position(|&chr| chr == "=") {
                    Some(index) => {
                        let (key, value) = (&key_value[..index], &key_value[index + 1..]);
                        if value.contains(&"=") {
                            panic!("Invalid symbol in value: =");
                        }
                        result
                            .get_mut(&section)
                            .unwrap()
                            .insert(trim_concat(key), trim_concat(value));
                    }
                    None => {
                        result
                            .get_mut(&section)
                            .unwrap()
                            .insert(trim_concat(key_value), "".to_string());
                    }
                }
            }
            _ => panic!("Invalid format: must be at least one section before values"),
        }
    });

    result
}
