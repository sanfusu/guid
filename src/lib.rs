use std::convert::TryInto;

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

impl quote::ToTokens for Guid {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let data1 = self.data1;
        let data2 = self.data2;
        let data3 = self.data3;
        let data4 = self.data4;
        let out = quote::quote! {
            guid::Guid {
                data1:#data1,
                data2:#data2,
                data3:#data3,
                data4: [#(#data4,)*]
            }
        };
        *tokens = out;
    }
}
