use std::collections::HashMap;
use std::fmt::Error;
use std::fs;

pub fn read_hosts(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Error reading {}", path))
}

pub fn parse_hosts(hosts: &str) -> Result<HashMap<&str, Vec<&str>>, Error> {
    let split = hosts.split("\n");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut vec_ref = None;
    for s in split {
        vec_ref = None;
        //ignore comments
        if s.starts_with("#") || s.is_empty() {
            continue;
        }
        let mut prev: usize = 0;

        for (i, c) in s.chars().enumerate() {
            if c == ' ' || i == s.len() - 1 {
                //we dont need empty words
                if i - prev > 1 {
                    match &mut vec_ref {
                        None => {
                            let spl = &s[0..i];
                            map.insert(spl, vec![]);
                            vec_ref = Some(map.get_mut(spl).unwrap());
                        }
                        Some(vec_) => {
                            vec_.push(&s[prev + 1..(if i == s.len() - 1 { i + 1 } else { i })]);
                        }
                    };
                }
                prev = i;
            }
        }
        if vec_ref.unwrap().len() == 0 {
            return Err(Error::default());
        }
    };
    Ok(map)
}