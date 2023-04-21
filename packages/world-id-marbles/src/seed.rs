pub use ruint::aliases::U256;

pub trait Seedable {
    fn into(self) -> U256;
}

impl Seedable for U256 {
    fn into(self) -> U256 {
        self
    }
}

impl Seedable for u32 {
    fn into(self) -> U256 {
        U256::try_from(self).unwrap()
    }
}

impl Seedable for i32 {
    fn into(self) -> U256 {
        U256::try_from(self).unwrap()
    }
}

impl Seedable for usize {
    fn into(self) -> U256 {
        U256::try_from(self).unwrap()
    }
}

impl Seedable for String {
    fn into(self) -> U256 {
        U256::from_str_radix(&self, 10).unwrap()
    }
}
impl Seedable for &str {
    fn into(self) -> U256 {
        U256::from_str_radix(self, 10).unwrap()
    }
}
