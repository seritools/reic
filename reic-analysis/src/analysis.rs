use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MemorySlice {
    /// relative virtual address
    rva: u64,
    /// length of the memory slice
    length: u64,
}

type MemoryMap = BTreeMap<MemorySlice, MemorySection>;

enum MemorySection {
    Code(Box<[u8]>),
    InitializedData(Box<[u8]>),
    UninitializedData,
}

enum AnalysisItemType {
    /// Interpret the item as another type than the default for the section
    ReinterpretItem,
    
    DataType,
    Comment,
    Function,
    Struct,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
