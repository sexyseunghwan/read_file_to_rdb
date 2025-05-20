/*
Author      : Seunghwan Shin
Create date : 2025-05-20
Description :

History     : 2025-05-20 Seunghwan Shin       # [v.1.0.0] first create
*/

mod common;
use core::error;

use common::*;

mod utils_module;
use utils_module::io_utils::*;
use utils_module::logger_utils::*;

mod service;
use service::query_service::*;

mod model;

mod controller;
use controller::main_controller::*;

mod entity;

mod configuration;
use configuration::env_config::*;

mod repository;

#[tokio::main]
async fn main() {
    set_global_logger();
    load_env();
    
    info!("File -> MySQL Start");

    let query_service: QueryServicePub = QueryServicePub::new();
    let main_controller: MainController<QueryServicePub> = MainController::new(Arc::new(query_service));

    let start: std::time::Instant = std::time::Instant::now();

    match main_controller.main_task().await {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
        }
    }

    let duration: Duration = start.elapsed();
    println!("⏱ 프로그램 실행 시간: {:?}", duration);
}
