// @generated
/// Generated client implementations.
pub mod auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuthServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AuthServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AuthServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn confirm_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmUserRequest>,
        ) -> Result<tonic::Response<super::ConfirmUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/ConfirmUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn confirm_forgot_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmForgotPasswordRequest>,
        ) -> Result<
            tonic::Response<super::ConfirmForgotPasswordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/ConfirmForgotPassword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn forgot_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ForgotPasswordRequest>,
        ) -> Result<tonic::Response<super::ForgotPasswordResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/ForgotPassword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/Login",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn logout(
            &mut self,
            request: impl tonic::IntoRequest<super::LogoutRequest>,
        ) -> Result<tonic::Response<super::LogoutResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/Logout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn respond_to_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::RespondToChallengeRequest>,
        ) -> Result<tonic::Response<super::RespondToChallengeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/RespondToChallenge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn register_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/RegisterUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resend_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendVerificationCodeRequest>,
        ) -> Result<
            tonic::Response<super::ResendVerificationCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/ResendVerificationCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_device_code(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeviceCodeRequest>,
        ) -> Result<tonic::Response<super::GetDeviceCodeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/GetDeviceCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccessTokenRequest>,
        ) -> Result<tonic::Response<super::GetAccessTokenResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.AuthService/GetAccessToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod auth_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AuthServiceServer.
    #[async_trait]
    pub trait AuthService: Send + Sync + 'static {
        async fn confirm_user(
            &self,
            request: tonic::Request<super::ConfirmUserRequest>,
        ) -> Result<tonic::Response<super::ConfirmUserResponse>, tonic::Status>;
        async fn confirm_forgot_password(
            &self,
            request: tonic::Request<super::ConfirmForgotPasswordRequest>,
        ) -> Result<
            tonic::Response<super::ConfirmForgotPasswordResponse>,
            tonic::Status,
        >;
        async fn forgot_password(
            &self,
            request: tonic::Request<super::ForgotPasswordRequest>,
        ) -> Result<tonic::Response<super::ForgotPasswordResponse>, tonic::Status>;
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status>;
        async fn logout(
            &self,
            request: tonic::Request<super::LogoutRequest>,
        ) -> Result<tonic::Response<super::LogoutResponse>, tonic::Status>;
        async fn respond_to_challenge(
            &self,
            request: tonic::Request<super::RespondToChallengeRequest>,
        ) -> Result<tonic::Response<super::RespondToChallengeResponse>, tonic::Status>;
        async fn register_user(
            &self,
            request: tonic::Request<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status>;
        async fn resend_verification_code(
            &self,
            request: tonic::Request<super::ResendVerificationCodeRequest>,
        ) -> Result<
            tonic::Response<super::ResendVerificationCodeResponse>,
            tonic::Status,
        >;
        async fn get_device_code(
            &self,
            request: tonic::Request<super::GetDeviceCodeRequest>,
        ) -> Result<tonic::Response<super::GetDeviceCodeResponse>, tonic::Status>;
        async fn get_access_token(
            &self,
            request: tonic::Request<super::GetAccessTokenRequest>,
        ) -> Result<tonic::Response<super::GetAccessTokenResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AuthServiceServer<T: AuthService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthService> AuthServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthServiceServer<T>
    where
        T: AuthService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/lekko.bff.v1beta1.AuthService/ConfirmUser" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmUserSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ConfirmUserRequest>
                    for ConfirmUserSvc<T> {
                        type Response = super::ConfirmUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).confirm_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfirmUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/ConfirmForgotPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmForgotPasswordSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ConfirmForgotPasswordRequest>
                    for ConfirmForgotPasswordSvc<T> {
                        type Response = super::ConfirmForgotPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmForgotPasswordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).confirm_forgot_password(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfirmForgotPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/ForgotPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ForgotPasswordSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ForgotPasswordRequest>
                    for ForgotPasswordSvc<T> {
                        type Response = super::ForgotPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForgotPasswordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).forgot_password(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ForgotPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::LoginRequest>
                    for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/Logout" => {
                    #[allow(non_camel_case_types)]
                    struct LogoutSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::LogoutRequest>
                    for LogoutSvc<T> {
                        type Response = super::LogoutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).logout(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LogoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/RespondToChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct RespondToChallengeSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::RespondToChallengeRequest>
                    for RespondToChallengeSvc<T> {
                        type Response = super::RespondToChallengeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RespondToChallengeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).respond_to_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RespondToChallengeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/RegisterUser" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterUserSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::RegisterUserRequest>
                    for RegisterUserSvc<T> {
                        type Response = super::RegisterUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/ResendVerificationCode" => {
                    #[allow(non_camel_case_types)]
                    struct ResendVerificationCodeSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ResendVerificationCodeRequest>
                    for ResendVerificationCodeSvc<T> {
                        type Response = super::ResendVerificationCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResendVerificationCodeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resend_verification_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResendVerificationCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/GetDeviceCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetDeviceCodeSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetDeviceCodeRequest>
                    for GetDeviceCodeSvc<T> {
                        type Response = super::GetDeviceCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDeviceCodeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_device_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDeviceCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.AuthService/GetAccessToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccessTokenSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetAccessTokenRequest>
                    for GetAccessTokenSvc<T> {
                        type Response = super::GetAccessTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccessTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_access_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccessTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AuthService> Clone for AuthServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AuthService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthService> tonic::server::NamedService for AuthServiceServer<T> {
        const NAME: &'static str = "lekko.bff.v1beta1.AuthService";
    }
}
/// Generated client implementations.
pub mod bff_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BffServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BffServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BffServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BffServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BffServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_user_git_hub_repos(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserGitHubReposRequest>,
        ) -> Result<tonic::Response<super::GetUserGitHubReposResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetUserGitHubRepos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user_git_hub_installations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserGitHubInstallationsRequest>,
        ) -> Result<
            tonic::Response<super::GetUserGitHubInstallationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetUserGitHubInstallations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user_logged_in_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserLoggedInInfoRequest>,
        ) -> Result<tonic::Response<super::GetUserLoggedInInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetUserLoggedInInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangePasswordRequest>,
        ) -> Result<tonic::Response<super::ChangePasswordResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ChangePassword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn o_auth_user(
            &mut self,
            request: impl tonic::IntoRequest<super::OAuthUserRequest>,
        ) -> Result<tonic::Response<super::OAuthUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/OAuthUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user_o_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserOAuthRequest>,
        ) -> Result<tonic::Response<super::GetUserOAuthResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetUserOAuth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_user_o_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserOAuthRequest>,
        ) -> Result<tonic::Response<super::DeleteUserOAuthResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/DeleteUserOAuth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn authorize_device(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthorizeDeviceRequest>,
        ) -> Result<tonic::Response<super::AuthorizeDeviceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/AuthorizeDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn generate_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateApiKeyRequest>,
        ) -> Result<tonic::Response<super::GenerateApiKeyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GenerateAPIKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_api_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiKeysRequest>,
        ) -> Result<tonic::Response<super::ListApiKeysResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ListAPIKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiKeyRequest>,
        ) -> Result<tonic::Response<super::DeleteApiKeyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/DeleteAPIKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckApiKeyRequest>,
        ) -> Result<tonic::Response<super::CheckApiKeyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/CheckAPIKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_team(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTeamRequest>,
        ) -> Result<tonic::Response<super::CreateTeamResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/CreateTeam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_team(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTeamRequest>,
        ) -> Result<tonic::Response<super::DeleteTeamResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/DeleteTeam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn use_team(
            &mut self,
            request: impl tonic::IntoRequest<super::UseTeamRequest>,
        ) -> Result<tonic::Response<super::UseTeamResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/UseTeam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_team_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTeamMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListTeamMembershipsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ListTeamMemberships",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_user_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListUserMembershipsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ListUserMemberships",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn upsert_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::UpsertMembershipRequest>,
        ) -> Result<tonic::Response<super::UpsertMembershipResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/UpsertMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMembershipRequest>,
        ) -> Result<tonic::Response<super::RemoveMembershipResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/RemoveMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepositoryRequest>,
        ) -> Result<tonic::Response<super::CreateRepositoryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/CreateRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepositoryRequest>,
        ) -> Result<tonic::Response<super::DeleteRepositoryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/DeleteRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> Result<tonic::Response<super::ListRepositoriesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ListRepositories",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeatureRequest>,
        ) -> Result<tonic::Response<super::GetFeatureResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_repository_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryContentsRequest>,
        ) -> Result<
            tonic::Response<super::GetRepositoryContentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetRepositoryContents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::AddNamespaceRequest>,
        ) -> Result<tonic::Response<super::AddNamespaceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/AddNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveNamespaceRequest>,
        ) -> Result<tonic::Response<super::RemoveNamespaceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/RemoveNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::AddFeatureRequest>,
        ) -> Result<tonic::Response<super::AddFeatureResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/AddFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFeatureRequest>,
        ) -> Result<tonic::Response<super::RemoveFeatureResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/RemoveFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveRequest>,
        ) -> Result<tonic::Response<super::SaveResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/Save",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_starlark(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveStarlarkRequest>,
        ) -> Result<tonic::Response<super::SaveStarlarkResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/SaveStarlark",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn convert_rule_to_string(
            &mut self,
            request: impl tonic::IntoRequest<super::ConvertRuleToStringRequest>,
        ) -> Result<tonic::Response<super::ConvertRuleToStringResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ConvertRuleToString",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pr(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrRequest>,
        ) -> Result<tonic::Response<super::GetPrResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetPR",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_p_rs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPRsRequest>,
        ) -> Result<tonic::Response<super::ListPRsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ListPRs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_branch(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBranchRequest>,
        ) -> Result<tonic::Response<super::CreateBranchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/CreateBranch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_branches(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBranchesRequest>,
        ) -> Result<tonic::Response<super::ListBranchesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/ListBranches",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_branch(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBranchRequest>,
        ) -> Result<tonic::Response<super::DeleteBranchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/DeleteBranch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn review(
            &mut self,
            request: impl tonic::IntoRequest<super::ReviewRequest>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/Review",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn merge(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeRequest>,
        ) -> Result<tonic::Response<super::MergeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/Merge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn eval(
            &mut self,
            request: impl tonic::IntoRequest<super::EvalRequest>,
        ) -> Result<tonic::Response<super::EvalResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/Eval",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_flag_evaluation_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlagEvaluationMetricsRequest>,
        ) -> Result<
            tonic::Response<super::GetFlagEvaluationMetricsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetFlagEvaluationMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn restore(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreRequest>,
        ) -> Result<tonic::Response<super::RestoreResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/Restore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_repository_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryLogsRequest>,
        ) -> Result<tonic::Response<super::GetRepositoryLogsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetRepositoryLogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRolloutRequest>,
        ) -> Result<tonic::Response<super::GetRolloutResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lekko.bff.v1beta1.BFFService/GetRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod bff_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with BffServiceServer.
    #[async_trait]
    pub trait BffService: Send + Sync + 'static {
        async fn get_user_git_hub_repos(
            &self,
            request: tonic::Request<super::GetUserGitHubReposRequest>,
        ) -> Result<tonic::Response<super::GetUserGitHubReposResponse>, tonic::Status>;
        async fn get_user_git_hub_installations(
            &self,
            request: tonic::Request<super::GetUserGitHubInstallationsRequest>,
        ) -> Result<
            tonic::Response<super::GetUserGitHubInstallationsResponse>,
            tonic::Status,
        >;
        async fn get_user_logged_in_info(
            &self,
            request: tonic::Request<super::GetUserLoggedInInfoRequest>,
        ) -> Result<tonic::Response<super::GetUserLoggedInInfoResponse>, tonic::Status>;
        async fn change_password(
            &self,
            request: tonic::Request<super::ChangePasswordRequest>,
        ) -> Result<tonic::Response<super::ChangePasswordResponse>, tonic::Status>;
        async fn o_auth_user(
            &self,
            request: tonic::Request<super::OAuthUserRequest>,
        ) -> Result<tonic::Response<super::OAuthUserResponse>, tonic::Status>;
        async fn get_user_o_auth(
            &self,
            request: tonic::Request<super::GetUserOAuthRequest>,
        ) -> Result<tonic::Response<super::GetUserOAuthResponse>, tonic::Status>;
        async fn delete_user_o_auth(
            &self,
            request: tonic::Request<super::DeleteUserOAuthRequest>,
        ) -> Result<tonic::Response<super::DeleteUserOAuthResponse>, tonic::Status>;
        async fn authorize_device(
            &self,
            request: tonic::Request<super::AuthorizeDeviceRequest>,
        ) -> Result<tonic::Response<super::AuthorizeDeviceResponse>, tonic::Status>;
        async fn generate_api_key(
            &self,
            request: tonic::Request<super::GenerateApiKeyRequest>,
        ) -> Result<tonic::Response<super::GenerateApiKeyResponse>, tonic::Status>;
        async fn list_api_keys(
            &self,
            request: tonic::Request<super::ListApiKeysRequest>,
        ) -> Result<tonic::Response<super::ListApiKeysResponse>, tonic::Status>;
        async fn delete_api_key(
            &self,
            request: tonic::Request<super::DeleteApiKeyRequest>,
        ) -> Result<tonic::Response<super::DeleteApiKeyResponse>, tonic::Status>;
        async fn check_api_key(
            &self,
            request: tonic::Request<super::CheckApiKeyRequest>,
        ) -> Result<tonic::Response<super::CheckApiKeyResponse>, tonic::Status>;
        async fn create_team(
            &self,
            request: tonic::Request<super::CreateTeamRequest>,
        ) -> Result<tonic::Response<super::CreateTeamResponse>, tonic::Status>;
        async fn delete_team(
            &self,
            request: tonic::Request<super::DeleteTeamRequest>,
        ) -> Result<tonic::Response<super::DeleteTeamResponse>, tonic::Status>;
        async fn use_team(
            &self,
            request: tonic::Request<super::UseTeamRequest>,
        ) -> Result<tonic::Response<super::UseTeamResponse>, tonic::Status>;
        async fn list_team_memberships(
            &self,
            request: tonic::Request<super::ListTeamMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListTeamMembershipsResponse>, tonic::Status>;
        async fn list_user_memberships(
            &self,
            request: tonic::Request<super::ListUserMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListUserMembershipsResponse>, tonic::Status>;
        async fn upsert_membership(
            &self,
            request: tonic::Request<super::UpsertMembershipRequest>,
        ) -> Result<tonic::Response<super::UpsertMembershipResponse>, tonic::Status>;
        async fn remove_membership(
            &self,
            request: tonic::Request<super::RemoveMembershipRequest>,
        ) -> Result<tonic::Response<super::RemoveMembershipResponse>, tonic::Status>;
        async fn create_repository(
            &self,
            request: tonic::Request<super::CreateRepositoryRequest>,
        ) -> Result<tonic::Response<super::CreateRepositoryResponse>, tonic::Status>;
        async fn delete_repository(
            &self,
            request: tonic::Request<super::DeleteRepositoryRequest>,
        ) -> Result<tonic::Response<super::DeleteRepositoryResponse>, tonic::Status>;
        async fn list_repositories(
            &self,
            request: tonic::Request<super::ListRepositoriesRequest>,
        ) -> Result<tonic::Response<super::ListRepositoriesResponse>, tonic::Status>;
        async fn get_feature(
            &self,
            request: tonic::Request<super::GetFeatureRequest>,
        ) -> Result<tonic::Response<super::GetFeatureResponse>, tonic::Status>;
        async fn get_repository_contents(
            &self,
            request: tonic::Request<super::GetRepositoryContentsRequest>,
        ) -> Result<
            tonic::Response<super::GetRepositoryContentsResponse>,
            tonic::Status,
        >;
        async fn add_namespace(
            &self,
            request: tonic::Request<super::AddNamespaceRequest>,
        ) -> Result<tonic::Response<super::AddNamespaceResponse>, tonic::Status>;
        async fn remove_namespace(
            &self,
            request: tonic::Request<super::RemoveNamespaceRequest>,
        ) -> Result<tonic::Response<super::RemoveNamespaceResponse>, tonic::Status>;
        async fn add_feature(
            &self,
            request: tonic::Request<super::AddFeatureRequest>,
        ) -> Result<tonic::Response<super::AddFeatureResponse>, tonic::Status>;
        async fn remove_feature(
            &self,
            request: tonic::Request<super::RemoveFeatureRequest>,
        ) -> Result<tonic::Response<super::RemoveFeatureResponse>, tonic::Status>;
        async fn save(
            &self,
            request: tonic::Request<super::SaveRequest>,
        ) -> Result<tonic::Response<super::SaveResponse>, tonic::Status>;
        async fn save_starlark(
            &self,
            request: tonic::Request<super::SaveStarlarkRequest>,
        ) -> Result<tonic::Response<super::SaveStarlarkResponse>, tonic::Status>;
        async fn convert_rule_to_string(
            &self,
            request: tonic::Request<super::ConvertRuleToStringRequest>,
        ) -> Result<tonic::Response<super::ConvertRuleToStringResponse>, tonic::Status>;
        async fn get_pr(
            &self,
            request: tonic::Request<super::GetPrRequest>,
        ) -> Result<tonic::Response<super::GetPrResponse>, tonic::Status>;
        async fn list_p_rs(
            &self,
            request: tonic::Request<super::ListPRsRequest>,
        ) -> Result<tonic::Response<super::ListPRsResponse>, tonic::Status>;
        async fn create_branch(
            &self,
            request: tonic::Request<super::CreateBranchRequest>,
        ) -> Result<tonic::Response<super::CreateBranchResponse>, tonic::Status>;
        async fn list_branches(
            &self,
            request: tonic::Request<super::ListBranchesRequest>,
        ) -> Result<tonic::Response<super::ListBranchesResponse>, tonic::Status>;
        async fn delete_branch(
            &self,
            request: tonic::Request<super::DeleteBranchRequest>,
        ) -> Result<tonic::Response<super::DeleteBranchResponse>, tonic::Status>;
        async fn review(
            &self,
            request: tonic::Request<super::ReviewRequest>,
        ) -> Result<tonic::Response<super::ReviewResponse>, tonic::Status>;
        async fn merge(
            &self,
            request: tonic::Request<super::MergeRequest>,
        ) -> Result<tonic::Response<super::MergeResponse>, tonic::Status>;
        async fn eval(
            &self,
            request: tonic::Request<super::EvalRequest>,
        ) -> Result<tonic::Response<super::EvalResponse>, tonic::Status>;
        async fn get_flag_evaluation_metrics(
            &self,
            request: tonic::Request<super::GetFlagEvaluationMetricsRequest>,
        ) -> Result<
            tonic::Response<super::GetFlagEvaluationMetricsResponse>,
            tonic::Status,
        >;
        async fn restore(
            &self,
            request: tonic::Request<super::RestoreRequest>,
        ) -> Result<tonic::Response<super::RestoreResponse>, tonic::Status>;
        async fn get_repository_logs(
            &self,
            request: tonic::Request<super::GetRepositoryLogsRequest>,
        ) -> Result<tonic::Response<super::GetRepositoryLogsResponse>, tonic::Status>;
        async fn get_rollout(
            &self,
            request: tonic::Request<super::GetRolloutRequest>,
        ) -> Result<tonic::Response<super::GetRolloutResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BffServiceServer<T: BffService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BffService> BffServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BffServiceServer<T>
    where
        T: BffService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/lekko.bff.v1beta1.BFFService/GetUserGitHubRepos" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserGitHubReposSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetUserGitHubReposRequest>
                    for GetUserGitHubReposSvc<T> {
                        type Response = super::GetUserGitHubReposResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserGitHubReposRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_git_hub_repos(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserGitHubReposSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetUserGitHubInstallations" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserGitHubInstallationsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<
                        super::GetUserGitHubInstallationsRequest,
                    > for GetUserGitHubInstallationsSvc<T> {
                        type Response = super::GetUserGitHubInstallationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUserGitHubInstallationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_git_hub_installations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserGitHubInstallationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetUserLoggedInInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserLoggedInInfoSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetUserLoggedInInfoRequest>
                    for GetUserLoggedInInfoSvc<T> {
                        type Response = super::GetUserLoggedInInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserLoggedInInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_logged_in_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserLoggedInInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ChangePassword" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePasswordSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ChangePasswordRequest>
                    for ChangePasswordSvc<T> {
                        type Response = super::ChangePasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangePasswordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_password(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangePasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/OAuthUser" => {
                    #[allow(non_camel_case_types)]
                    struct OAuthUserSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::OAuthUserRequest>
                    for OAuthUserSvc<T> {
                        type Response = super::OAuthUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OAuthUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).o_auth_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OAuthUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetUserOAuth" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserOAuthSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetUserOAuthRequest>
                    for GetUserOAuthSvc<T> {
                        type Response = super::GetUserOAuthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserOAuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_o_auth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserOAuthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/DeleteUserOAuth" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserOAuthSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::DeleteUserOAuthRequest>
                    for DeleteUserOAuthSvc<T> {
                        type Response = super::DeleteUserOAuthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserOAuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_user_o_auth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteUserOAuthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/AuthorizeDevice" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeDeviceSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::AuthorizeDeviceRequest>
                    for AuthorizeDeviceSvc<T> {
                        type Response = super::AuthorizeDeviceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthorizeDeviceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).authorize_device(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthorizeDeviceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GenerateAPIKey" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateAPIKeySvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GenerateApiKeyRequest>
                    for GenerateAPIKeySvc<T> {
                        type Response = super::GenerateApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).generate_api_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateAPIKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ListAPIKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListAPIKeysSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ListApiKeysRequest>
                    for ListAPIKeysSvc<T> {
                        type Response = super::ListApiKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListApiKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_api_keys(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAPIKeysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/DeleteAPIKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAPIKeySvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::DeleteApiKeyRequest>
                    for DeleteAPIKeySvc<T> {
                        type Response = super::DeleteApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_api_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAPIKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/CheckAPIKey" => {
                    #[allow(non_camel_case_types)]
                    struct CheckAPIKeySvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::CheckApiKeyRequest>
                    for CheckAPIKeySvc<T> {
                        type Response = super::CheckApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).check_api_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckAPIKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/CreateTeam" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTeamSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::CreateTeamRequest>
                    for CreateTeamSvc<T> {
                        type Response = super::CreateTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTeamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_team(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTeamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/DeleteTeam" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTeamSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::DeleteTeamRequest>
                    for DeleteTeamSvc<T> {
                        type Response = super::DeleteTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTeamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_team(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteTeamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/UseTeam" => {
                    #[allow(non_camel_case_types)]
                    struct UseTeamSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::UseTeamRequest>
                    for UseTeamSvc<T> {
                        type Response = super::UseTeamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UseTeamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).use_team(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UseTeamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ListTeamMemberships" => {
                    #[allow(non_camel_case_types)]
                    struct ListTeamMembershipsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ListTeamMembershipsRequest>
                    for ListTeamMembershipsSvc<T> {
                        type Response = super::ListTeamMembershipsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTeamMembershipsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_team_memberships(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTeamMembershipsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ListUserMemberships" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserMembershipsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ListUserMembershipsRequest>
                    for ListUserMembershipsSvc<T> {
                        type Response = super::ListUserMembershipsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserMembershipsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_user_memberships(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListUserMembershipsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/UpsertMembership" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertMembershipSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::UpsertMembershipRequest>
                    for UpsertMembershipSvc<T> {
                        type Response = super::UpsertMembershipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpsertMembershipRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).upsert_membership(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertMembershipSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/RemoveMembership" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMembershipSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::RemoveMembershipRequest>
                    for RemoveMembershipSvc<T> {
                        type Response = super::RemoveMembershipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveMembershipRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_membership(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveMembershipSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/CreateRepository" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRepositorySvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::CreateRepositoryRequest>
                    for CreateRepositorySvc<T> {
                        type Response = super::CreateRepositoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRepositoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_repository(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRepositorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/DeleteRepository" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRepositorySvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::DeleteRepositoryRequest>
                    for DeleteRepositorySvc<T> {
                        type Response = super::DeleteRepositoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRepositoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_repository(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRepositorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ListRepositories" => {
                    #[allow(non_camel_case_types)]
                    struct ListRepositoriesSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ListRepositoriesRequest>
                    for ListRepositoriesSvc<T> {
                        type Response = super::ListRepositoriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRepositoriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_repositories(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListRepositoriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetFeature" => {
                    #[allow(non_camel_case_types)]
                    struct GetFeatureSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetFeatureRequest>
                    for GetFeatureSvc<T> {
                        type Response = super::GetFeatureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFeatureRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_feature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFeatureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetRepositoryContents" => {
                    #[allow(non_camel_case_types)]
                    struct GetRepositoryContentsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetRepositoryContentsRequest>
                    for GetRepositoryContentsSvc<T> {
                        type Response = super::GetRepositoryContentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRepositoryContentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_repository_contents(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRepositoryContentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/AddNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct AddNamespaceSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::AddNamespaceRequest>
                    for AddNamespaceSvc<T> {
                        type Response = super::AddNamespaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_namespace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddNamespaceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/RemoveNamespace" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveNamespaceSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::RemoveNamespaceRequest>
                    for RemoveNamespaceSvc<T> {
                        type Response = super::RemoveNamespaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveNamespaceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_namespace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveNamespaceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/AddFeature" => {
                    #[allow(non_camel_case_types)]
                    struct AddFeatureSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::AddFeatureRequest>
                    for AddFeatureSvc<T> {
                        type Response = super::AddFeatureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddFeatureRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_feature(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddFeatureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/RemoveFeature" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFeatureSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::RemoveFeatureRequest>
                    for RemoveFeatureSvc<T> {
                        type Response = super::RemoveFeatureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveFeatureRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_feature(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFeatureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/Save" => {
                    #[allow(non_camel_case_types)]
                    struct SaveSvc<T: BffService>(pub Arc<T>);
                    impl<T: BffService> tonic::server::UnaryService<super::SaveRequest>
                    for SaveSvc<T> {
                        type Response = super::SaveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/SaveStarlark" => {
                    #[allow(non_camel_case_types)]
                    struct SaveStarlarkSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::SaveStarlarkRequest>
                    for SaveStarlarkSvc<T> {
                        type Response = super::SaveStarlarkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveStarlarkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).save_starlark(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveStarlarkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ConvertRuleToString" => {
                    #[allow(non_camel_case_types)]
                    struct ConvertRuleToStringSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ConvertRuleToStringRequest>
                    for ConvertRuleToStringSvc<T> {
                        type Response = super::ConvertRuleToStringResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConvertRuleToStringRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).convert_rule_to_string(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConvertRuleToStringSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetPR" => {
                    #[allow(non_camel_case_types)]
                    struct GetPRSvc<T: BffService>(pub Arc<T>);
                    impl<T: BffService> tonic::server::UnaryService<super::GetPrRequest>
                    for GetPRSvc<T> {
                        type Response = super::GetPrResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPrRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_pr(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPRSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ListPRs" => {
                    #[allow(non_camel_case_types)]
                    struct ListPRsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ListPRsRequest>
                    for ListPRsSvc<T> {
                        type Response = super::ListPRsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPRsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_p_rs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPRsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/CreateBranch" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBranchSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::CreateBranchRequest>
                    for CreateBranchSvc<T> {
                        type Response = super::CreateBranchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBranchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_branch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateBranchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/ListBranches" => {
                    #[allow(non_camel_case_types)]
                    struct ListBranchesSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::ListBranchesRequest>
                    for ListBranchesSvc<T> {
                        type Response = super::ListBranchesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBranchesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_branches(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListBranchesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/DeleteBranch" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBranchSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::DeleteBranchRequest>
                    for DeleteBranchSvc<T> {
                        type Response = super::DeleteBranchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBranchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_branch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBranchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/Review" => {
                    #[allow(non_camel_case_types)]
                    struct ReviewSvc<T: BffService>(pub Arc<T>);
                    impl<T: BffService> tonic::server::UnaryService<super::ReviewRequest>
                    for ReviewSvc<T> {
                        type Response = super::ReviewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReviewRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).review(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReviewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/Merge" => {
                    #[allow(non_camel_case_types)]
                    struct MergeSvc<T: BffService>(pub Arc<T>);
                    impl<T: BffService> tonic::server::UnaryService<super::MergeRequest>
                    for MergeSvc<T> {
                        type Response = super::MergeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).merge(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MergeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/Eval" => {
                    #[allow(non_camel_case_types)]
                    struct EvalSvc<T: BffService>(pub Arc<T>);
                    impl<T: BffService> tonic::server::UnaryService<super::EvalRequest>
                    for EvalSvc<T> {
                        type Response = super::EvalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).eval(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EvalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetFlagEvaluationMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct GetFlagEvaluationMetricsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetFlagEvaluationMetricsRequest>
                    for GetFlagEvaluationMetricsSvc<T> {
                        type Response = super::GetFlagEvaluationMetricsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetFlagEvaluationMetricsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_flag_evaluation_metrics(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFlagEvaluationMetricsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/Restore" => {
                    #[allow(non_camel_case_types)]
                    struct RestoreSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::RestoreRequest>
                    for RestoreSvc<T> {
                        type Response = super::RestoreResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).restore(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RestoreSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetRepositoryLogs" => {
                    #[allow(non_camel_case_types)]
                    struct GetRepositoryLogsSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetRepositoryLogsRequest>
                    for GetRepositoryLogsSvc<T> {
                        type Response = super::GetRepositoryLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRepositoryLogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_repository_logs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRepositoryLogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lekko.bff.v1beta1.BFFService/GetRollout" => {
                    #[allow(non_camel_case_types)]
                    struct GetRolloutSvc<T: BffService>(pub Arc<T>);
                    impl<
                        T: BffService,
                    > tonic::server::UnaryService<super::GetRolloutRequest>
                    for GetRolloutSvc<T> {
                        type Response = super::GetRolloutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRolloutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_rollout(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRolloutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BffService> Clone for BffServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BffService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BffService> tonic::server::NamedService for BffServiceServer<T> {
        const NAME: &'static str = "lekko.bff.v1beta1.BFFService";
    }
}
