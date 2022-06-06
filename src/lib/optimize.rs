use reqwest;
use std::env;


pub async fn this(rpc: String) -> bool{
    let current_block:String = env::var("CURRENT_BLOCK").expect("$CURRENT_BLOCK is not set, run this command to set it `export CURRENT_BLOCK=<block number to start with>`").parse().unwrap();
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
let local_block: i32 = i32::from_str_radix(current_block.trim_start_matches("0x"), 16).unwrap();


return latest_block > local_block;

}





