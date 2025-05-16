#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DataType {
    #[default]
    Unknown = 0,
    ClassicTransaction = 1,
}

impl DataType {
    pub fn from_u8(t: u8) -> Self {
        match t {
            1 => DataType::ClassicTransaction,
            _ => DataType::Unknown,
        }
    }
}
