use std::collections::HashMap;
use std::fmt::Error;

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
        let index_of_last_char = s.len() - 1;
        for (i, c) in s.chars().enumerate() {
            if c == ' ' || i == index_of_last_char {
                //we dont need empty words
                if i - prev > 1 {
                    match &mut vec_ref {
                        None => {
                            let spl = &s[0..i];
                            map.insert(spl, vec![]);
                            vec_ref = Some(map.get_mut(spl).unwrap());
                        }
                        Some(vec_) => {
                            vec_.push(&s[prev + 1..(if i == index_of_last_char && c != ' ' { i + 1 } else { i })]);
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

pub fn hosts_map_to_string(hosts: &HashMap<&str, Vec<&str>>) -> Result<String, String> {
    let mut str_buf = String::new();
    for (addr, names) in hosts {
        let addr = addr.trim();
        if addr.is_empty() {
            return Err("Property name is empty".to_string());
        }
        str_buf.push_str(addr);
        str_buf.push(' ');
        for name in names {
            let name = name.trim();
            if name.is_empty() {
                return Err(format!("Value for property {} is empty", addr));
            }
            str_buf.push_str(name);
            str_buf.push(' ');
        }
        str_buf.push('\n');
    }
    Ok(str_buf)
}