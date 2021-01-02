use std::convert::TryInto;

use pest::Parser;

// #[macro_use]
// extern crate guid_proc;
#[macro_use]
extern crate pest_derive;
extern crate pest;
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
pub(crate) mod guid_pest {
    #[derive(Parser)]
    #[grammar = "guid.pest"]
    pub(crate) struct Guid;
}

impl std::convert::From<String> for Guid {
    fn from(v: String) -> Self {
        let mut token_str = v.replace(' ', "");
        token_str = token_str.replace('"', "");
        let guid_parsed = guid_pest::Guid::parse(guid_pest::Rule::guid, &token_str)
            .unwrap()
            .next()
            .unwrap();

        let mut data1: u32 = 0;
        let mut data2: u16 = 0;
        let mut data3: u16 = 0;
        let mut data4: Vec<u8> = Vec::new();
        for part in guid_parsed.into_inner() {
            match part.as_rule() {
                guid_pest::Rule::part1_u32 => {
                    data1 = u32::from_str_radix(part.as_str(), 16).unwrap();
                }
                guid_pest::Rule::part2_u16 => {
                    data2 = u16::from_str_radix(part.as_str(), 16).unwrap();
                }
                guid_pest::Rule::part3_u16 => {
                    data3 = u16::from_str_radix(part.as_str(), 16).unwrap();
                }
                guid_pest::Rule::part4_u8_8 => {
                    for byte in part.into_inner() {
                        data4.push(u8::from_str_radix(byte.as_str(), 16).unwrap());
                    }
                }
                _ => {}
            }
        }
        Guid {
            data1,
            data2,
            data3,
            data4: data4.as_slice().try_into().unwrap(),
        }
    }
}

impl syn::parse::Parse for Guid {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(input.to_string().into())
    }
}
