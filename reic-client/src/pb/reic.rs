#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenFileRequest {
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(uint64, tag = "1")]
    pub file_id: u64,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileList {
    #[prost(message, repeated, tag = "1")]
    pub files: ::std::vec::Vec<File>,
}
#[doc = r" Generated client implementations."]
pub mod reic_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ReicClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReicClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReicClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn get_file_list(
            &mut self,
            request: impl tonic::IntoRequest<super::FileListRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::FileList>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/reic.Reic/GetFileList");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn open_file(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenFileRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::reic_changes::FileChanges>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/reic.Reic/OpenFile");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn sample_command(
            &mut self,
            request: impl tonic::IntoRequest<super::super::reic_commands::SampleCommand>,
        ) -> Result<
            tonic::Response<super::super::reic_commands::SampleCommandResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/reic.Reic/SampleCommand");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ReicClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ReicClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ReicClient {{ ... }}")
        }
    }
}
