
use tonic::{transport::Server, Request, Response, Status};
use log::info;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::oneshot::Sender;
use tokio::sync::Mutex;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};


lazy_static! {
    /// Channel used to send shutdown signal - wrapped in an Option to allow
    /// it to be taken by value (since oneshot channels consume themselves on
    /// send) and an Arc<Mutex> to allow it to be safely shared between threads
    static ref SHUTDOWN_TX: Arc<Mutex<Option<Sender<()>>>> = <_>::default();
}

use rust_grpc::yao::proto::model_server::{ModelServer,Model};

use rust_grpc::yao::proto::{Request as ModelRequest,Response as ModelResponse};

use rust_grpc::yao::plugin::grpc_controller_server::{GrpcControllerServer,GrpcController};
use rust_grpc::yao::plugin::Empty;

#[derive(Debug, Default)]
pub struct MyModel {}
#[derive(Debug, Default)]
pub struct MyController {}

#[tonic::async_trait]
impl Model for MyModel {
    async fn exec(
        &self,
        request: Request<ModelRequest>,
    ) -> Result<Response<ModelResponse>, Status> {
        info!("Received request from: {:?}", request);
        // JSON data as a string
        let json_data = r#"{
            "name": "John Doe",
            "age": 30,
            "city": "New York"
        }"#;

        // Deserialize the JSON data into a value of type `serde_json::Value`
        let value: serde_json::Value = serde_json::from_str(json_data).unwrap();
        // Serialize the value to a byte vector
        let bytes: Vec<u8> = serde_json::to_vec(&value).unwrap();
        let response = ModelResponse {
            response: bytes,
            r#type: "map".to_owned(),
        };
        Ok(Response::new(response))
    }
}
#[tonic::async_trait]
impl GrpcController for MyController {
    async fn shutdown(
        &self,
        _request: tonic::Request<Empty>,
    ) -> std::result::Result<tonic::Response<Empty>, tonic::Status>{
        info!("Kill Request received");
        if let Some(tx) = SHUTDOWN_TX.lock().await.take() {
            let _ = tx.send(());
        }
        Ok(Response::new(Empty{}))
    }
}


// Use the tokio runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    SHUTDOWN_TX.lock().await.replace(tx);

    let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build("logs/output.log")?;

    let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    let addr = "127.0.0.1:50051".parse()?;
    let mymodel: MyModel = MyModel::default();
    let controller: MyController = MyController::default();

    let handshake = "1|1|tcp|127.0.0.1:50051|grpc";
    println!("{}", handshake);
    info!("Listening on http://{}", addr);

    Server::builder()
        .add_service(ModelServer::new(mymodel))
        .add_service(GrpcControllerServer::new(controller))
        .serve_with_shutdown(addr, async {
            rx.await.ok();
        }).await?;
    info!("Server stop");

    Ok(())
}