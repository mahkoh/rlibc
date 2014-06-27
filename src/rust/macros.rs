#![macro_escape]

#[macro_export]
macro_rules! cc {
    ($e:expr) => {
        $e as char_t
    }
}

#[macro_export]
macro_rules! cs {
    ($e:expr) => {
        (bytes!($e, 0)).repr().data as *char_t
    }
}
