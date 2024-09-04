use std::error::Error;

use zfs_protos::calculator::calculator_client::CalculatorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;

    let req = zfs_protos::calculator::CalculationRequest { a: 4, b: 5};
    let request = tonic::Request::new(req);

    let response = client.add(request).await?;
    println!("Response: {:?}", response.get_ref().result);

    Ok(())
}
