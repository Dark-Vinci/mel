use {
    crate::app::app::ChannelInterface,
    async_trait::async_trait,
    sdk::generated_proto_rs::{
        mel_channel::{
            channel_service_server::ChannelService, PingResponse,
            SayHelloRequest, SayHelloResponse,
        },
        mel_utils::Empty,
    },
    tonic::{Request, Response, Status},
    uuid::Uuid,
};

pub struct Channel<A: ChannelInterface>(A);

impl<A: ChannelInterface> Channel<A> {
    pub fn new(a: A) -> Self {
        Self(a)
    }
}

#[async_trait]
impl<A: ChannelInterface + Send + Sync + 'static> ChannelService
    for Channel<A>
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
