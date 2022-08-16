#[macro_export]
macro_rules! leaky_box_raw {
    ($val:expr) => {
        Box::into_raw(Box::new($val))
    };
}

#[macro_export]
macro_rules! leaky_box_u8 {
    ($val:expr) => {
        leaky_box_raw!($val) as *mut u8
    };
}

#[macro_export]
macro_rules! leaky_box_be_bytes {
    ($val:expr) => {
        leaky_box_u8!($val.to_be_bytes())
    };
}
