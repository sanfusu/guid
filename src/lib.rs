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
