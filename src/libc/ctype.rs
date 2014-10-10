use types::{int_t};

#[no_mangle]
pub extern fn isalnum(c: int_t) -> int_t {
    match c as u8 as char {
        'a'...'z' => 1,
        'A'...'Z' => 1,
        '0'...'9' => 1,
        _        => 0,
    }
}

#[no_mangle]
pub extern fn isalpha(c: int_t) -> int_t {
    match c as u8 as char {
        'a'...'z' => 1,
        'A'...'Z' => 1,
        _        => 0,
    }
}

#[no_mangle]
pub extern fn isblank(c: int_t) -> int_t {
    match c as u8 as char {
        ' ' | '\t' => 1,
        _          => 0,
    }
}

#[no_mangle]
pub extern fn iscntrl(c: int_t) -> int_t {
    match c as u8 as char {
        '\x07'...'\r' => 1,
        _            => 0,
    }
}

#[no_mangle]
pub extern fn isdigit(c: int_t) -> int_t {
    match c as u8 as char {
        '0'...'9' => 1,
        _        => 0,
    }
}

#[no_mangle]
pub extern fn isgraph(c: int_t) -> int_t {
    match c {
        0x21...0x7e => 1,
        _          => 0,
    }
}

#[no_mangle]
pub extern fn islower(c: int_t) -> int_t {
    match c as u8 as char {
        'a'...'z' => 1,
        _        => 0,
    }
}

#[no_mangle]
pub extern fn isprint(c: int_t) -> int_t {
    match c {
        0x20...0x7e => 1,
        _          => 0,
    }
}

#[no_mangle]
pub extern fn ispunct(c: int_t) -> int_t {
    match isspace(c) + isalnum(c) {
        0 => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern fn isspace(c: int_t) -> int_t {
    match c {
        0x09...0x0d => 1,
        0x20       => 1,
        _          => 0,
    }
}

#[no_mangle]
pub extern fn isupper(c: int_t) -> int_t {
    match c as u8 as char {
        'A'...'Z' => 1,
        _        => 0,
    }
}

#[no_mangle]
pub extern fn isxdigit(c: int_t) -> int_t {
    match c as u8 as char {
        '0'...'9' => 1,
        'A'...'F' => 1,
        'a'...'f' => 1,
        _        => 0,
    }
}

#[no_mangle]
pub extern fn tolower(c: int_t) -> int_t {
    match c as u8 as char {
        'A'...'Z' => c + 0x20,
        _        => c,
    }
}

#[no_mangle]
pub extern fn toupper(c: int_t) -> int_t {
    match c as u8 as char {
        'a'...'z' => c - 0x20,
        _        => c,
    }
}
