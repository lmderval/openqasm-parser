pub fn is_alnum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub fn is_alpha(c: char) -> bool {
    is_upper(c) || is_lower(c)
}

pub fn is_bracket(c: char) -> bool {
    match c {
        '[' => true,
        ']' => true,
        '(' => true,
        ')' => true,
        _ => false,
    }
}

pub fn is_digit(c: char) -> bool {
    match c {
        '0'..'9' => true,
        _ => false,
    }
}

pub fn is_id(c: char) -> bool {
    is_alnum(c) || c == '_'
}

pub fn is_lower(c: char) -> bool {
    match c {
        'a'..'z' => true,
        _ => false,
    }
}

pub fn is_operator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '^' => true,
        _ => false,
    }
}

pub fn is_punct(c: char) -> bool {
    match c {
        ',' => true,
        ';' => true,
        _ => false,
    }
}

pub fn is_space(c: char) -> bool {
    match c {
        ' ' => true,
        '\n' => true,
        '\t' => true,
        _ => false,
    }
}

pub fn is_upper(c: char) -> bool {
    match c {
        'A'..'Z' => true,
        _ => false,
    }
}
