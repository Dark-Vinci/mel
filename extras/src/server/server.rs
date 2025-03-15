use {
    crate::app::app::Operations,
    async_trait::async_trait,
    sdk::generated_proto_rs::{
        mel_extras::{
            extras_service_server::ExtrasService, PingResponse,
            SayHelloRequest, SayHelloResponse,
        },
        mel_utils::Empty,
    },
    tonic::{Request, Response, Status},
    uuid::Uuid,
};

pub struct Extras<E: Operations>(E);

impl<E: Operations> Extras<E> {
    pub fn new(a: E) -> Self {
        Self(a)
    }
}

#[async_trait]
impl<T: Operations + Send + Sync + 'static> ExtrasService for Extras<T> {
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
