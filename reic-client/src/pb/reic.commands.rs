#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleCommand {
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleCommandResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::std::option::Option<BaseCommandResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseCommandResult {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(oneof = "base_command_result::FailReason", tags = "2")]
    pub fail_reason: ::std::option::Option<base_command_result::FailReason>,
}
pub mod base_command_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FailReason {
        /// If set, there are still changes left to be sent back.
        /// Try again when this change was received/processed
        #[prost(uint64, tag = "2")]
        WaitUntilChange(u64),
    }
}
