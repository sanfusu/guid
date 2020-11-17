extern crate guid_proc;
pub use guid_proc::*;

#[derive(Debug)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[cfg(test)]
mod test {
    use super::Guid;
    use guid_proc::guid;
    extern crate guid_proc;

    #[test]
    fn test() {
        let b = guid! {"72631e54-78a4-11d0-bcf7-00aa00b7b32a"};
        println!("{:#?}", b);
    }
}
