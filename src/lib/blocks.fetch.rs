use tokio::time::{self, Duration}; 
use std::env;
use colored::Colorize;
use reqwest;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::{thread};
use throbber::Throbber;

use simplelog::*;

// #[macro_use] extern crate log;
extern crate simplelog;
#[path = "optimize.rs"]
mod optimize;


pub async fn run(rpc: String, block_number: i32, webhook: String, keep_on: bool) {
let mut interval = time::interval(Duration::from_millis(1));
let rpc = Arc::new(rpc);
let hblock = format!("{:X}", block_number);
let hexblock_number =&format!("0x{hblock}");
let home_dir = dirs::home_dir().unwrap();
let home_dir = home_dir.display();

if !Path::new(&format!("{home_dir}/.fiddi")).exists() {
    fs::create_dir(&format!("{home_dir}/.fiddi")).unwrap();
}
if !Path::new(&format!("{home_dir}/.fiddi/debug.log")).exists() {
    File::create(&format!("{home_dir}/.fiddi/debug.log")).unwrap();
}


env::set_var("CURRENT_BLOCK", hexblock_number);
let home_dir = dirs::home_dir().unwrap();
let home_dir = home_dir.display();
CombinedLogger::init(
    vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, Config::default(), 
        OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{home_dir}/.fiddi/debug.log"))
        .unwrap()
    ),
    ]
).unwrap();
if keep_on == true {
loop {
    let current_block:String = env::var("CURRENT_BLOCK").expect("error in here").parse().unwrap();
    interval.tick().await;
    let res = reqwest::Client::new()
    .post(format!("{}",rpc))
    .json(&serde_json::json!({ 
    "jsonrpc":"2.0",
    "method":"eth_getBlockByNumber",
    "params":[
        current_block, 
        true
    ],
    "id":"1"}))
    .send()
    .await
    .expect("failed to get response")
    .json::<serde_json::Value>() 
    .await
    .expect("failed to get payload");

let  json_string = format!("{}",res["result"]["transactions"]);

let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();


let d = json_string.replace("]", "").replace("[", "");

let array: Vec<&str> = d.split("},{").collect();


println!("{} {} {} {}", "Found", format!("{}",array.len()).blue(), "transactions in block".blue(), format!("{:?}",i32::from_str_radix(current_block.trim_start_matches("0x"), 16).unwrap()).blue());
info!("{} {} {} {}", "Found", format!("{}",array.len()), "transactions in block", format!("{:?}",i32::from_str_radix(current_block.trim_start_matches("0x"), 16).unwrap()));
  

let home_dir = dirs::home_dir().unwrap();
let home_dir = home_dir.display();
let tree = sled::open(format!("{home_dir}/.fiddi/wallet-addresses")).expect("Failed to open");

for i in 0..array.len() {
    if data[i]["value"] != "0x0" {
let exists = tree.contains_key(&format!("{}",data[i]["to"]).replace('"', "").as_bytes()).expect("error error");
        if exists {
            if webhook == "None" {
                info!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
                println!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
            }else{
                reqwest::Client::new()
                .post(format!("{}",webhook))
                .body(format!("{}",data[i]))
                .send()
                .await
                .expect("failed to get payload");
    info!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
    println!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
            }
        }
   
    }
}
drop(tree);
if !optimize::this(format!("{}",rpc.clone())).await {
    let next_block = format!("{:X}", latest_block(format!("{}",rpc.clone())).await);
    let hexblock_number =&format!("0x{next_block}");
        env::set_var("CURRENT_BLOCK", hexblock_number.to_string());
let mut throbber = Throbber::new()
        .frames(&throbber::DEFAULT_F)
        .interval(Duration::from_millis(200));

    throbber.start();
        throbber.change_message(format!("{} {:?} {}","optimizing block number to reduce to maximum of 'latest block number'(", latest_block(format!("{}",rpc.clone())).await, ")"));
        thread::sleep(Duration::from_millis(12000));
}


let next_block = i32::from_str_radix(current_block.trim_start_matches("0x"), 16).expect("error while trying!")+1;
let next_block = format!("{:X}", next_block);
let hexblock_number =&format!("0x{next_block}");
    env::set_var("CURRENT_BLOCK", hexblock_number.to_string());
}


}

if keep_on == false{
    let current_block:String = env::var("CURRENT_BLOCK").expect("error in here").parse().unwrap();
    interval.tick().await;
    let res = reqwest::Client::new()
    .post(format!("{}",rpc))
    .json(&serde_json::json!({ 
    "jsonrpc":"2.0",
    "method":"eth_getBlockByNumber",
    "params":[
        current_block, 
        true
    ],
    "id":"1"}))
    .send()
    .await
    .expect("failed to get response")
    .json::<serde_json::Value>() 
    .await
    .expect("failed to get payload");

let  json_string = format!("{}",res["result"]["transactions"]);

let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();


let d = json_string.replace("]", "").replace("[", "");

let array: Vec<&str> = d.split("},{").collect();


println!("{} {} {} {}", "Found", format!("{}",array.len()).blue(), "transactions in block".blue(), format!("{:?}",i32::from_str_radix(current_block.trim_start_matches("0x"), 16).unwrap()).blue());
info!("{} {} {} {}", "Found", format!("{}",array.len()), "transactions in block", format!("{:?}",i32::from_str_radix(current_block.trim_start_matches("0x"), 16).unwrap()));
  
let home_dir = dirs::home_dir().unwrap();
let home_dir = home_dir.display();
let tree = sled::open(format!("{home_dir}/.fiddi/wallet-addresses")).expect("Failed to open");

for i in 0..array.len() {
    if data[i]["value"] != "0x0" {
let exists = tree.contains_key(&format!("{}",data[i]["to"]).replace('"', "").as_bytes()).expect("error error");
        if exists {
            if webhook == "None" {
                info!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
                println!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
            }else{
            reqwest::Client::new()
            .post(format!("{}",webhook))
            .body(format!("{}",data[i]))
            .send()
            .await
            .expect("failed to get payload");
info!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));
println!("{} {}","incoming transaction to", format!("{}",data[i]["to"]));

            }
        }
   
    }
}




}


}


pub async fn latest_block(rpc: String) -> i32 {
    let res = reqwest::Client::new()
      .post(format!("{}",rpc))
      .json(&serde_json::json!({
          "jsonrpc":"2.0",
          "method":"eth_blockNumber",
          "params":[],
          "id":83
      }))
      .send()
      .await
      .expect("failed to get response")
      .json::<serde_json::Value>() 
      .await
      .expect("failed to get payload");
  
  let  json_string: String = format!("{}",res["result"]);
  let latest_block: i32 = i32::from_str_radix(&json_string.to_string().replace("0x", "").replace('"', ""), 16).unwrap(); 
  
  return latest_block;
  
  }
