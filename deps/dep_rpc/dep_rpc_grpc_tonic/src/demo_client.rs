use demo::UserRequest;
use demo::user_service_client::UserServiceClient;

pub mod demo {
    tonic::include_proto!("demo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserServiceClient::connect("http://localhost:50051").await?;

    let req = tonic::Request::new(UserRequest {
        user_id: 42,
        filter: None,
    });

    let resp = client.get_user(req).await?.into_inner();

    println!("Status: {:?}", resp.status);
    let info = resp.info.unwrap();
    println!(
        "Info: name: {},created_at: {:?},active_duration: {:?}",
        info.name, info.created_at, info.active_duration
    );

    Ok(())
}
