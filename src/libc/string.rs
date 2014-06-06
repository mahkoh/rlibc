use types::{uchar_t, int_t, size_t};
use internal::{offset, offset_mut, Repr};

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn memcpy(dst: *mut uchar_t, src: *uchar_t, n: size_t) -> *mut uchar_t {
    let mut i = 0;
    while i < n as int {
        *offset_mut(dst, i) = *offset(src, i);
        i += 1;
    }
    dst
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn memmove(dst: *mut uchar_t, src: *uchar_t,
                             n: size_t) -> *mut uchar_t {
    if (dst as uint) < (src as uint) {
        return memcpy(dst, src, n);
    }
    let mut i = n as int;
    while i > 0 {
        *offset_mut(dst, i-1) = *offset(src, i-1);
        i -= 1;
    }
    dst
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strcpy(dst: *mut uchar_t, src: *uchar_t) -> *mut uchar_t {
    let mut i = 0;
    while *offset(src, i) != 0 {
        *offset_mut(dst, i) = *offset(src, i);
        i += 1;
    }
    *offset_mut(dst, i) = 0;
    dst
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strncpy(dst: *mut uchar_t, src: *uchar_t,
                             n: size_t) -> *mut uchar_t {
    let n = n as int;
    let mut i = 0;
    while i < n && *offset(src, i) != 0 {
        *offset_mut(dst, i) = *offset(src, i);
        i += 1;
    }
    while i < n {
        *offset_mut(dst, i) = 0;
        i += 1;
    }
    dst
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strcat(dst: *mut uchar_t, src: *uchar_t) -> *mut uchar_t {
    let base = strlen(dst as *_) as int;
    let mut i = 0;
    while *offset(src, i) != 0 {
        *offset_mut(dst, base+i) = *offset(src, i);
        i += 1;
    }
    *offset_mut(dst, base+i) = 0;
    dst
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strncat(dst: *mut uchar_t, src: *uchar_t,
                             n: size_t) -> *mut uchar_t {
    let base = strlen(dst as *_) as int;
    let mut i = 0;
    while i < n as int && *offset(src, i) != 0 {
        *offset_mut(dst, base+i) = *offset(src, i);
        i += 1;
    }
    *offset_mut(dst, base+i) = 0;
    dst
}


#[no_mangle]
#[no_split_stack]
pub unsafe extern fn memcmp(m1: *uchar_t, m2: *uchar_t, n: size_t) -> int_t {
    let mut i = 0;
    while i < n as int {
        let v1 = *offset(m1, i) as int;
        let v2 = *offset(m2, i) as int;
        match v1 - v2 {
            j if j < 0 => return -1,
            j if j > 0 => return 1,
            _ => { },
        }
        i += 1;
    }
    0
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strcmp(m1: *uchar_t, m2: *uchar_t) -> int_t {
    let mut i = 0;
    loop {
        let v1 = *offset(m1, i) as int;
        let v2 = *offset(m2, i) as int;
        match v1 - v2 {
            j if j < 0 => return -1,
            j if j > 0 => return 1,
            _ => { },
        }
        if v1 == 0 {
            break;
        }
        i += 1;
    }
    0
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strcoll(m1: *uchar_t, m2: *uchar_t) -> int_t {
    strcmp(m1, m2)
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strncmp(m1: *uchar_t, m2: *uchar_t, n: size_t) -> int_t {
    let mut i = 0;
    while i < n as int {
        let v1 = *offset(m1, i) as int;
        let v2 = *offset(m2, i) as int;
        match v1 - v2 {
            j if j < 0 => return -1,
            j if j > 0 => return 1,
            _ => { },
        }
        if v1 == 0 {
            break;
        }
        i += 1;
    }
    0
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strxfrm(dst: *mut uchar_t, src: *uchar_t, n: size_t) -> size_t {
    let len = strlen(src);
    if len < n {
        memcpy(dst, src, len+1);
    }
    len
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn memchr(s: *uchar_t, c: int_t, n: size_t) -> *uchar_t {
    let c = c as uchar_t;
    let mut i = 0;
    while i < n as int {
        if *offset(s, i) == c {
            return offset(s, i);
        }
        i += 1;
    }
    0 as *uchar_t
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strchr(s: *uchar_t, c: int_t) -> *uchar_t {
    if c == 0 {
        return offset(s, strlen(s) as int);
    }
    let c = c as uchar_t;
    let mut i = 0;
    while *offset(s, i) != 0 {
        if *offset(s, i) == c {
            return offset(s, i);
        }
        i += 1;
    }
    0 as *uchar_t
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strcspn(s1: *uchar_t, s2: *uchar_t) -> size_t {
    let len = strlen(s2);
    let mut i = 0;
    while *offset(s1, i) != 0 {
        if memchr(s2, *offset(s1, i) as int_t, len) as uint != 0 {
            break;
        }
        i += 1;
    }
    i as size_t
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strpbrk(s1: *uchar_t, s2: *uchar_t) -> *uchar_t {
    let len = strlen(s2);
    let mut i = 0;
    while *offset(s1, i) != 0 {
        if memchr(s2, *offset(s1, i) as int_t, len) as uint == 0 {
            return offset(s1, i);
        }
        i += 1;
    }
    0 as *uchar_t
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strrchr(s: *uchar_t, c: int_t) -> *uchar_t {
    let mut last = -1;
    let mut i = 0;
    while *offset(s, i) != 0 {
        if *offset(s, i) as int_t == c {
            last = i;
        }
        i += 1;
    }
    match last {
        -1 => 0 as *uchar_t,
        _  => offset(s, last)
    }
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strspn(s1: *uchar_t, s2: *uchar_t) -> size_t {
    let len = strlen(s2);
    let mut i = 0;
    while *offset(s1, i) != 0 {
        if memchr(s2, *offset(s1, i) as int_t, len) as uint == 0 {
            break;
        }
        i += 1;
    }
    i as size_t
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strstr(s1: *uchar_t, s2: *uchar_t) -> *uchar_t {
    let len1 = strlen(s1) as int;
    let len2 = strlen(s2) as int;
    let mut i = 0;
    while i <= len1 - len2 {
        if memcmp(offset(s1, i), s2, len2 as size_t) == 0 {
            return offset(s1, i);
        }
        i += 1;
    }
    0 as *uchar_t
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strtok(s1: *mut uchar_t, s2: *uchar_t) -> *uchar_t {
    static mut ss: *mut uchar_t = 0 as *mut uchar_t;
    static mut len: int = 0;
    if s1 as uint != 0 {
        ss = s1;
        len = strlen(ss as *_) as int;
    }
    if ss as uint == 0 {
        return 0 as *uchar_t;
    }
    let len2 = strlen(s2) as int;
    let mut i = 0;
    while i < len {
        if memchr(s2, *offset_mut(ss, i) as int_t, len2 as size_t) as uint != 0 {
            break;
        }
        i += 1;
    }
    ss = offset_mut(ss, i);
    len -= i;
    if len == 0 {
        ss = 0 as *mut uchar_t;
        return 0 as *uchar_t;
    }
    let mut i = 0;
    while i < len {
        if memchr(s2, *offset_mut(ss, i) as int_t, len2 as size_t) as uint == 0 {
            break;
        }
        i += 1;
    }
    if i == len {
        len = 0;
        let tmp = ss;
        ss = 0 as *mut uchar_t;
        return tmp as *_;
    }
    *offset_mut(ss, i) = 0;
    let tmp = ss;
    ss = offset_mut(ss, i+1);
    len -= i+1;
    tmp as *_
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn memset(dst: *mut uchar_t, c: int_t, n: size_t) -> *mut uchar_t {
    let c = c as uchar_t;
    let mut i = 0;
    while i < n as int {
        *offset_mut(dst, i) = c;
        i += 1;
    }
    dst
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strerror(num: int_t) -> *uchar_t {
    let s = bytes!("u w0t m8?", 0);
    s.repr().data
}

#[no_mangle]
#[no_split_stack]
pub unsafe extern fn strlen(s: *uchar_t) -> size_t {
    let mut len = 0;
    while *offset(s, len) != 0 {
        len += 1;
    }
    len as size_t
}
