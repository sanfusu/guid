use std::{convert::TryInto, fmt::Display};

#[macro_use]
extern crate layout;

/// 全局唯一标识符 (RFC 4122)
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
#[derive(Layout)]
pub struct Guid {
    /// TimeLow: 时间戳的低字段
    pub data1: u32,
    /// TimeMid: 时间戳的中间字段
    pub data2: u16,
    /// 时间戳的高位字段和版本号（复用）
    pub data3: u16,
    /// 第一个字节为时钟序列的高位字段；
    /// 第二个字节为时钟序列的低位字段；
    /// 剩余 6 个字节为唯一的节点标识，
    /// 可以是 IEEE 802 地址，或者用于加密的随机数。
    pub data4: [u8; 8],
}

impl std::str::FromStr for Guid {
    type Err = ParseGuidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_owned();
        s.retain(|c| c.is_ascii_hexdigit());
        if s.len() != std::mem::size_of::<Guid>() * 2 {
            return Err(ParseGuidError::default());
        }
        // It could not be failed, since we have make sure the len and the asscii_hexdigit
        let integer = u128::from_str_radix(&s, 16).unwrap().to_be_bytes();
        // Slice try_into array could not be failed too. If that, it will be a compiler internal error.
        Ok(Guid {
            data1: u32::from_be_bytes(integer[GuidLayout::data1()].try_into().unwrap()).to_le(),
            data2: u16::from_be_bytes(integer[GuidLayout::data2()].try_into().unwrap()).to_le(),
            data3: u16::from_be_bytes(integer[GuidLayout::data3()].try_into().unwrap()).to_le(),
            data4: integer[GuidLayout::data4()].try_into().unwrap(),
        })
    }
}
impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
    }
}

#[derive(Debug)]
pub enum ParseGuidErrorKind {
    InvalidLenError,
}

impl Display for ParseGuidErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseGuidErrorKind::InvalidLenError => {
                write!(f, "Invalid Length")
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseGuidError {
    source: ParseGuidErrorKind,
}

impl Display for ParseGuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing Guid string: {}", self.source)
    }
}

impl Default for ParseGuidError {
    fn default() -> Self {
        Self {
            source: ParseGuidErrorKind::InvalidLenError,
        }
    }
}

impl std::error::Error for ParseGuidError {}

impl ParseGuidError {
    pub fn kind(&self) -> &ParseGuidErrorKind {
        &self.source
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_str_test() {
        let guid: Guid = "01020304-0506-0708-090a-0b0d0e0f1011".parse().unwrap();
        assert_eq!(
            guid,
            Guid {
                data1: 0x01020304,
                data2: 0x0506,
                data3: 0x0708,
                data4: [0x09, 0x0a, 0x0b, 0x0d, 0x0e, 0x0f, 0x10, 0x11],
            },
        );
        assert_eq!("01020304-0506-0708-090a-0b0d0e0f1011", guid.to_string());
        println!("{}", guid);
        let guid = " 1020304-0506-0708-090a-0b0d0e0f1011"
            .parse::<Guid>()
            .expect_err("It should be error:Invalid length");

        match guid.kind() {
            ParseGuidErrorKind::InvalidLenError => {}
        }
    }
}
