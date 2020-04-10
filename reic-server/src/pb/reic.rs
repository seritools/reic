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
#[doc = r" Generated server implementations."]
pub mod reic_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ReicServer."]
    #[async_trait]
    pub trait Reic: Send + Sync + 'static {
        #[doc = "Server streaming response type for the GetFileList method."]
        type GetFileListStream: Stream<Item = Result<super::FileList, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn get_file_list(
            &self,
            request: tonic::Request<super::FileListRequest>,
        ) -> Result<tonic::Response<Self::GetFileListStream>, tonic::Status>;
        #[doc = "Server streaming response type for the OpenFile method."]
        type OpenFileStream: Stream<Item = Result<super::super::reic_changes::FileChanges, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn open_file(
            &self,
            request: tonic::Request<super::OpenFileRequest>,
        ) -> Result<tonic::Response<Self::OpenFileStream>, tonic::Status>;
        async fn sample_command(
            &self,
            request: tonic::Request<super::super::reic_commands::SampleCommand>,
        ) -> Result<
            tonic::Response<super::super::reic_commands::SampleCommandResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ReicServer<T: Reic> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Reic> ReicServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ReicServer<T>
    where
        T: Reic,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/reic.Reic/GetFileList" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileListSvc<T: Reic>(pub Arc<T>);
                    impl<T: Reic> tonic::server::ServerStreamingService<super::FileListRequest> for GetFileListSvc<T> {
                        type Response = super::FileList;
                        type ResponseStream = T::GetFileListStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FileListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_file_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetFileListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reic.Reic/OpenFile" => {
                    #[allow(non_camel_case_types)]
                    struct OpenFileSvc<T: Reic>(pub Arc<T>);
                    impl<T: Reic> tonic::server::ServerStreamingService<super::OpenFileRequest> for OpenFileSvc<T> {
                        type Response = super::super::reic_changes::FileChanges;
                        type ResponseStream = T::OpenFileStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.open_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = OpenFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reic.Reic/SampleCommand" => {
                    #[allow(non_camel_case_types)]
                    struct SampleCommandSvc<T: Reic>(pub Arc<T>);
                    impl<T: Reic>
                        tonic::server::UnaryService<super::super::reic_commands::SampleCommand>
                        for SampleCommandSvc<T>
                    {
                        type Response = super::super::reic_commands::SampleCommandResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::reic_commands::SampleCommand>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sample_command(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SampleCommandSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Reic> Clone for ReicServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Reic> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Reic> tonic::transport::NamedService for ReicServer<T> {
        const NAME: &'static str = "reic.Reic";
    }
}
