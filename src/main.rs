use sidecar::gen::backend_beta::configuration_service_server::{ConfigurationService, ConfigurationServiceServer};
use sidecar::gen::backend_beta::configuration_service_client::{ConfigurationServiceClient};
use sidecar::gen::backend_beta::{GetBoolValueRequest, GetBoolValueResponse, GetProtoValueRequest, GetProtoValueResponse, GetJsonValueRequest, GetJsonValueResponse};
use tonic::{transport::{Channel, Server}};

pub struct Passthrough {
    client_channel: Channel,
}

#[tonic::async_trait]
impl ConfigurationService for Passthrough {
        async fn get_bool_value(
            &self,
            request: tonic::Request<GetBoolValueRequest>,
        ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
	    println!("Got request: {:?}", request);
	    let mut client = ConfigurationServiceClient::new(self.client_channel.clone());
	    let resp = client.get_bool_value(request).await;
	    if resp.is_err() {
		println!("error in proxying {:?}", resp)
	    }
	    return resp
	}
        async fn get_proto_value(
            &self,
            request: tonic::Request<GetProtoValueRequest>,
        ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
	    let mut client = ConfigurationServiceClient::new(self.client_channel.clone());
	    client.get_proto_value(request).await
	}
        async fn get_json_value(
            &self,
            request: tonic::Request<GetJsonValueRequest>,
        ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
	    let mut client = ConfigurationServiceClient::new(self.client_channel.clone());
	    client.get_json_value(request).await
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("listening on port: {:?}", addr);
    let proxy_addr = "http://localhost:8080";
    println!("proxying to: {}", proxy_addr);
    let passthrough = Passthrough{
	client_channel: Channel::from_static(proxy_addr).connect().await?,
    };
    
    Server::builder()
        .add_service(ConfigurationServiceServer::new(passthrough))
        .serve(addr)
        .await?;
	
    Ok(())
}
