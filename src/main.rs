use tokio;
use clap::Parser;
use serde::{Deserialize, Serialize}; 
use std::process;
use actix_web::{web, App, HttpServer};
use colored::Colorize;





extern crate serde_json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    block_number: i32,
    #[clap(short, long)]
    port: u16,
    #[clap(short, long)]
    rpc: String,
    #[clap(short, long)]
    ip_address: String,
    #[clap(short, long)]
    webhook: String,
    #[clap(short, long)]
    keep_on: bool,
    #[clap(short, long)]
    http: bool,

}

#[path = "lib/optimize.rs"]
mod optimize;

#[path = "lib/blocks.fetch.rs"]
mod block_fetch;

#[path = "lib/http.rs"]
mod http;

#[derive(Deserialize, Serialize, Clone)]
struct WalletAddresses {
    address: String,
}



#[tokio::main]
async fn main() {

    let args = Args::parse();
    let rpc = args.rpc;
    let block_number = args.block_number;
    let keep_on = args.keep_on;
    let http = args.http;
    let ip_address = args.ip_address;
    let port = args.port;
    let webhook = args.webhook;


    tokio::join!(
        http_init(ip_address, port, http),
        crawler(rpc, block_number, webhook,keep_on)
    );



}

async fn http_init(ip_address: String, port: u16, http: bool) {
    if http {
    HttpServer::new(|| App::new().route("/api/new/address", web::post().to(http::new_address)))
    .bind((ip_address, port)).expect("error")
    .run()
    .await
    .expect("error bosss");
 }
}

async fn crawler(rpc: String, block_number: i32, webhook: String, keep_on: bool){
    if keep_on{
    
        block_fetch::run(format!("{}",rpc), block_number, webhook, keep_on).await;
        if !optimize::this(format!("{}",rpc)).await {
            println!("{}", "The block number that you supplied is not valid".red());
            

        }
    }
    else {
    
        block_fetch::run(format!("{}",rpc),  block_number, webhook, keep_on).await;
        if !optimize::this(format!("{}",rpc)).await {
            println!("{}", "The block number that you supplied is not valid".red());
        }

        process::exit(1);
    
    }
}




