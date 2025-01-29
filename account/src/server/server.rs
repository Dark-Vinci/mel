use {
    async_trait::async_trait,
    crate::app::app::AccountInterface,
    sdk::generated_proto_rs::{
        mel_account::{
            account_service_server::AccountService, PingResponse,
            SayHelloRequest, SayHelloResponse,
        },
        mel_utils::Empty,
    },
    tonic::{Request, Response, Status},
    uuid::Uuid,
};

pub struct Account<A: AccountInterface>(A);

impl<A: AccountInterface> Account<A> {
    pub fn new(a: A) -> Self {
        Self(a)
    }
}

#[async_trait]
impl<A: AccountInterface + Send + Sync + 'static> AccountService
    for Account<A>
{
    async fn ping(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<PingResponse>, Status> {
        let id = Uuid::new_v4();

        let res = PingResponse {
            message: format!("{} HELLO", id.to_string()),
        };

        Ok(Response::new(res))
    }

    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        let SayHelloRequest { name, request_id } = request.into_inner();

        let res = SayHelloResponse {
            message: format!("Hello {} from say hell method", name).to_string(),
            request_id,
        };

        Ok(Response::new(res))
    }
}
