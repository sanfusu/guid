//! # Example
//! ```
//! use guid::*;
//!
//! #[guid(72631e54-78a4-11d0-bcf7-00aa00b7b32a)]
//! struct Protocol;
//! assert_eq!(Protocol::guid(), Guid!("72631e54-78a4-11d0-bcf7-00aa00b7b32a"));
//! ```

#[macro_use]
extern crate guid_proc;

/// 全局唯一标识符 (RFC 4122)
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
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

#[macro_export]
macro_rules! Guid {
    ($t:literal) => {
        guid_proc! {$t}
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[guid(72631e54-78a4-11d0-bcf7-00aa00b7b32a)]
    struct A;
    #[test]
    fn test() {
        let b = Guid! {"72631e54-78a4-11d0-bcf7-00aa00b7b32a"};
        assert_eq!(A::guid(), b);
    }
}
