use crate::demo::user_service_server::{UserService, UserServiceServer};
use crate::demo::{UserInfo, UserRequest, UserResponse, UserStatus};
use prost_types::{Duration, Timestamp};
use tonic::{Request, Response, Status, transport::Server};

pub mod demo {
    tonic::include_proto!("demo");
}

#[derive(Default)]
pub struct MyUserService;

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let info = UserInfo {
            name: format!("User {}", req.user_id),
            created_at: Some(Timestamp {
                seconds: 1_700_000_000,
                nanos: 0,
            }),
            active_duration: Some(Duration {
                seconds: 3600,
                nanos: 0,
            }),
        };

        let resp = UserResponse {
            status: UserStatus::Active.into(),
            info: Some(info),
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let svc = MyUserService::default();

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
