use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, RwLock};
use futures::FutureExt;
use log::info;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tonic::transport::Server;
use tonic::{Response, Status, Request};

use crate::common::Key;
use crate::ecdsa_manager_grpc::{BaseResponse, SetRequest, GetRequest, EmptyRequest};
use crate::ecdsa_manager_grpc::ecdsa_manager_service_server::{EcdsaManagerService, EcdsaManagerServiceServer};
use crate::status::ServerStatus;

pub const SERVER_LOCK_TIME_OUT_DEFAULT: Duration = Duration::from_secs(10);
pub const SERVER_TASK_GET_BACK_TIME_OUT_DEFAULT: Duration = Duration::from_secs(60);
pub const SERVER_EXIT_TIME_OUT_AFTER_TASK_DONE_DEFAULT: Duration = Duration::from_secs(300);

#[derive(Debug)]
pub struct EcdsaManagerServer {
    pub server_info: Arc<Mutex<ServerInfo>>,
    task_run_tx: UnboundedSender<String>,
}

#[derive(Debug)]
pub struct  ServerInfo {
    pub status: ServerStatus,
    pub storage: RwLock<HashMap<Key, String>>,
    pub last_update_time: Instant,
    pub server_lock_time_out: Duration,
    pub server_task_get_back_time_out: Duration,
    pub server_exit_time_out_after_task_done: Duration,
    pub error: String,
}

impl Default for ServerInfo {
    fn default() -> Self {
        ServerInfo {
            status: ServerStatus::default(),
            storage: RwLock::new(HashMap::new()),
            last_update_time: Instant::now(),
            server_lock_time_out: SERVER_LOCK_TIME_OUT_DEFAULT,
            server_task_get_back_time_out: SERVER_TASK_GET_BACK_TIME_OUT_DEFAULT,
            server_exit_time_out_after_task_done: SERVER_EXIT_TIME_OUT_AFTER_TASK_DONE_DEFAULT,
            error: String::default(),
        }
    }
}

impl EcdsaManagerServer {
    pub fn new(task_run_tx: UnboundedSender<String>) -> Self {
        EcdsaManagerServer { 
            server_info: Arc::new(Mutex::new(ServerInfo::default())), 
            task_run_tx,
        }
    }

    pub fn set_time_out(
        &self,
        server_lock_time_out: Duration,
        server_task_get_back_time_out: Duration,
        server_exit_time_out_after_task_done: Duration,
    ) -> anyhow::Result<()> {
        let mut si = match self.server_info.lock() {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };
        si.server_lock_time_out = server_lock_time_out;
        si.server_task_get_back_time_out = server_task_get_back_time_out;
        si.server_exit_time_out_after_task_done = server_exit_time_out_after_task_done;
        Ok(())
    }

    pub fn set_server_lock_time_out(
        &self,
        time_out: Duration
    ) -> anyhow::Result<()> {
        let mut si = match self.server_info.lock() {
            Ok(s) => s,
            Err(e) =>  {
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };
        si.server_lock_time_out = time_out;
        Ok(())
    }

    pub fn set_server_task_get_back_time_out(
        &self,
        time_out: Duration
    ) -> anyhow::Result<()> {
        let mut si = match self.server_info.lock() {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };
        si.server_task_get_back_time_out = time_out;
        Ok(())
    }

    pub fn set_server_exit_time_out_after_task_done(
        &self,
        time_out: Duration
    ) -> anyhow::Result<()> {
        let mut si = match self.server_info.lock() {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };
        si.server_exit_time_out_after_task_done = time_out;
        Ok(())
    }

    // fn do_task()
    // fn lock_server_if_free()
    // fn get_task_result()
    // fn unlock()
}

#[tonic::async_trait]
impl EcdsaManagerService for EcdsaManagerServer {
    async fn set(
        &self,
        request: Request<SetRequest>
    ) -> Result<Response<BaseResponse>, Status> {
        let msg = format!("success");
        Ok(Response::new(BaseResponse { msg: msg.to_string() }))
    }

    async fn get(
        &self,
        request: Request<GetRequest>
    ) -> Result<Response<BaseResponse>, Status> {
        let msg = format!("success");
        Ok(Response::new(BaseResponse { msg: msg.to_string() }))
    }

    async fn keygen(
        &self,
        request: Request<EmptyRequest>
    ) -> Result<Response<BaseResponse>, Status> {
        let msg = format!("success");
        Ok(Response::new(BaseResponse { msg: msg.to_string() }))
    }
}

pub async fn run_server(
    server_exit_rx: oneshot::Receiver<String>,
    srv: EcdsaManagerServer,
    port: String,
) {
    let mut addr_s = "0.0.0.0:".to_string();
    addr_s += &port;
    let addr = addr_s.parse::<SocketAddr>().unwrap();
    info!("Server listening on {}", addr);
    Server::builder()
        .accept_http1(true)
        .add_service(EcdsaManagerServiceServer::new(srv))
        .serve_with_shutdown(addr, server_exit_rx.map(drop))
        .await
        .unwrap();
    info!("server stop listen")
}