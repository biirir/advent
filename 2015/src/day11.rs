// https://adventofcode.com/2015/day/11

/* YOLO */
unsafe fn string_next(s: &mut str) {
    let bytes = s.as_bytes_mut();
    let mut i = bytes.len();

    while i > 0 {
        i -= 1;
        let c = &mut bytes[i];
        if *c == b'z' {
            *c = b'a'
        } else {
            *c += 1;
            break;
        }
    }
}

fn valid(s: &str) -> bool {
    let mut b = '\0'; // Previous char.
    let mut inc = (0, 1);
    let mut pairs = (None, None);

    for c in s.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }
        if c == b {
            pairs = match pairs {
                (None, None) => (Some(c), None),
                (Some(t), None) if t != c => (Some(t), Some(c)),
                _ => pairs,
            }
        }
        b = c;
        let c = c as u8;
        inc = match inc {
            (p, 3) => (p, 3),
            (p, n) if c == p + 1 => (c, n + 1),
            _ => (c, 1),
        };
    }

    pairs.0.is_some() && pairs.1.is_some() && inc.1 == 3
}

pub fn day11(filename: Option<&str>) -> (String, String) {
    let mut input = crate::readline(filename.unwrap_or("input/11"));
    let password = &mut input;
    let mut result = Vec::new();

    while result.len() < 2 {
        unsafe {
            string_next(password);
        }
        if valid(password) {
            result.push(password.clone());
        }
    }

    let b = result.pop();
    let a = result.pop();

    (a.unwrap(), b.unwrap())
}
