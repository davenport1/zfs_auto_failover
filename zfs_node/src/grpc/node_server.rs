use tonic::transport::Server;
use zfs_protos::calculator::calculator_server::{Calculator, CalculatorServer};

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<zfs_protos::calculator::CalculationRequest>,
    ) -> Result<tonic::Response<zfs_protos::calculator::CalculationResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);
        
        let input = request.get_ref();
        
        let response = zfs_protos::calculator::CalculationResponse {
            result: input.a + input.b,
        };
        
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    
    let calc = CalculatorService::default();
    
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(zfs_protos::calculator::FILE_DESCRIPTOR_SET)
        .build_v1()?;
    
    Server::builder()
        .add_service(reflection_service)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;
    
    Ok(())
}
