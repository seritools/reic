#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileChanges {
    #[prost(uint64, tag = "1")]
    pub changes_id: u64,
    #[prost(uint64, tag = "2")]
    pub changes_hash: u64,
    #[prost(message, repeated, tag = "3")]
    pub changes: ::std::vec::Vec<FileChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileChange {}
