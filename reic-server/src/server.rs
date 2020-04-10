use crate::pb::{
    reic_server::{Reic, ReicServer},
    FileChanges, FileList, FileListRequest, OpenFileRequest, SampleCommand, SampleCommandResponse,
};
use futures_core::Stream;
use std::pin::Pin;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug)]
struct ReicService;

#[tonic::async_trait]
impl Reic for ReicService {
    type GetFileListStream =
        Pin<Box<dyn Stream<Item = Result<FileList, Status>> + Send + Sync + 'static>>;

    async fn get_file_list(
        &self,
        request: Request<FileListRequest>,
    ) -> Result<Response<Self::GetFileListStream>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    type OpenFileStream =
        Pin<Box<dyn Stream<Item = Result<FileChanges, Status>> + Send + Sync + 'static>>;

    async fn open_file(
        &self,
        request: Request<OpenFileRequest>,
    ) -> Result<Response<Self::OpenFileStream>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn sample_command(
        &self,
        request: Request<SampleCommand>,
    ) -> Result<Response<SampleCommandResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}
