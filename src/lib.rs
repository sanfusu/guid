use std::{array::TryFromSliceError, convert::TryInto, fmt::Display, num::ParseIntError};

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

impl std::convert::TryFrom<String> for Guid {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut value = value.replace(' ', "");
        value = value.replace('"', "");
        value.retain(|c| c != '-');
        if value.len() != std::mem::size_of::<Guid>() * 2 {
            return Err("Invalid format");
        }
        let integer = u128::from_str_radix(&value, 16)
            .or(Err("Invalid format"))?
            .to_be_bytes();
        Ok(Guid {
            data1: u32::from_be_bytes(integer[GuidLayout::data1()].try_into().unwrap()).to_le(),
            data2: u16::from_be_bytes(integer[GuidLayout::data2()].try_into().unwrap()).to_le(),
            data3: u16::from_be_bytes(integer[GuidLayout::data3()].try_into().unwrap()).to_le(),
            data4: integer[GuidLayout::data4()].try_into().unwrap(),
        })
    }
}

impl std::str::FromStr for Guid {
    type Err = ParseGuidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.replace(' ', "");
        s = s.replace('"', "");
        s.retain(|c| c != '-');
        if s.len() != std::mem::size_of::<Guid>() * 2 {
            return Err(ParseGuidError::default());
        }
        let integer = u128::from_str_radix(&s, 16)
            .map_err(ParseGuidError::parse_int_err)?
            .to_be_bytes();
        Ok(Guid {
            data1: u32::from_be_bytes(
                integer[GuidLayout::data1()]
                    .try_into()
                    .map_err(ParseGuidError::try_from_slice_err)?,
            )
            .to_le(),
            data2: u16::from_be_bytes(
                integer[GuidLayout::data2()]
                    .try_into()
                    .map_err(ParseGuidError::try_from_slice_err)?,
            )
            .to_le(),
            data3: u16::from_be_bytes(
                integer[GuidLayout::data3()]
                    .try_into()
                    .map_err(ParseGuidError::try_from_slice_err)?,
            )
            .to_le(),
            data4: integer[GuidLayout::data4()]
                .try_into()
                .map_err(ParseGuidError::try_from_slice_err)?,
        })
    }
}

#[derive(Debug)]
pub enum ParseGuidErrorKind {
    ParseIntError(ParseIntError),
    TryFromSliceError(TryFromSliceError),
    InvalidLenError,
}

impl Display for ParseGuidErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseGuidErrorKind::ParseIntError(x) => {
                write!(f, "{}", x)
            }
            ParseGuidErrorKind::TryFromSliceError(x) => {
                write!(f, "{}", x)
            }
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

impl std::error::Error for ParseGuidError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match (*self).source {
            ParseGuidErrorKind::ParseIntError(ref x) => Some(x),
            ParseGuidErrorKind::TryFromSliceError(ref x) => Some(x),
            ParseGuidErrorKind::InvalidLenError => None,
        }
    }
}

impl ParseGuidError {
    fn parse_int_err(x: ParseIntError) -> ParseGuidError {
        Self {
            source: ParseGuidErrorKind::ParseIntError(x),
        }
    }
    fn try_from_slice_err(x: TryFromSliceError) -> ParseGuidError {
        Self {
            source: ParseGuidErrorKind::TryFromSliceError(x),
        }
    }
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
        let guid = " 1020304-0506-0708-090a-0b0d0e0f1011"
            .parse::<Guid>()
            .expect_err("It should be error:Invalid length");

        match guid.kind() {
            ParseGuidErrorKind::InvalidLenError => {}
            _ => {
                panic!("It should be error:Invalid length")
            }
        }

        let guid = "01020304-0x06-0708-090a-0b0d0e0f1011"
            .parse::<Guid>()
            .expect_err("It should be parse int error");
        match guid.kind() {
            ParseGuidErrorKind::ParseIntError(_) => {}
            _ => {
                panic!("It should be parse int error")
            }
        }
    }
}
