use actix_web::{error, web, Error, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

// extern crate crypto;

// use self::crypto::digest::Digest;
// use self::crypto::sha3::Sha3;



#[derive(Serialize, Deserialize)]
struct Obj {
    address: String
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

// #[post("/api/new/address")]
pub async fn new_address(mut payload: web::Payload) -> Result<HttpResponse, Error> {
let tree = sled::open("./.fiddi/wallet-addresses").expect("Failed to open");
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<Obj>(&body)?;
    let address = Arc::new(obj.address);
    let exists = tree.contains_key(format!("{}", address.clone()).as_bytes()).unwrap();
if !exists{

    tree.insert(format!("{}", address.clone()), format!("{}", address.clone()).to_lowercase().as_bytes()).expect("can't do that");
    let obj = json!({"status" : "0", "address": format!("{}", address.clone()), "msg": "address was successfully added to watchlist"});
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}else{
    return Err(error::ErrorBadRequest(json!({"status" : "0", "msg": "address was already added to the watchlist"})));

}
    

}





// fn validate_eth_address(address: &str) -> bool {
//     let check = eth_checksum_encode(&address);
//     if check == address {
//         return true;
//     }
//     eth_checksum_encode(&check) == check
// }

// fn eth_checksum_encode(address: &str) -> String {
//     let input = String::from(address.to_ascii_lowercase().trim_start_matches("0x"));
//     let mut hasher = Sha3::keccak256();
//     hasher.input_str(&input);
//     let hex = hasher.result_str();
//     let mut ret = String::with_capacity(42);
//     ret.push_str("0x");
//     for i in 0..40 {
//         if u32::from_str_radix(&hex[i..i+1], 16).unwrap() > 7 {
//             ret.push_str(&address[i+2..i+3].to_ascii_uppercase()); 
//         } else {
//             ret.push_str(&address[i+2..i+3]);
//         }
//     }
//     ret
// }
