use {
    crate::app::app::MessagingInterface,
    async_trait::async_trait,
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

pub struct Messaging<A: MessagingInterface>(A);

impl<A: MessagingInterface> Messaging<A> {
    pub fn new(a: A) -> Self {
        Self(a)
    }
}

#[async_trait]
impl<A: MessagingInterface + Send + Sync + 'static> AccountService
    for Messaging<A>
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
