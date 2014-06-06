use types::{int_t, short_t};

pub struct pollfd {
    pub fd: int_t,
    pub events: short_t,
    pub revents: short_t,
}
