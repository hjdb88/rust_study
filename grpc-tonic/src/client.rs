use internal_files::internal_files_client::InternalFilesClient;
use internal_files::ExistsRequest;

pub mod internal_files {
    include!("proto/internalfiles.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = InternalFilesClient::connect("http://[::1]:8888").await?;

    let request = tonic::Request::new(ExistsRequest {
        hash: "Tonic".into(),
    });

    let response = client.exists(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
