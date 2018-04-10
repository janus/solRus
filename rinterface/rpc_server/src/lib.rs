extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use jsonrpc_core::*;
use jsonrpc_http_server::*;
use serde::ser::Serialize;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelData {
    pub group: String,
    pub payload: Vec<serde_json::Value>,
}

pub fn transport(msgs: Vec<ChannelData>, sockaddr: &SocketAddr) {
    let mut io = IoHandler::new();
    for msg in msgs {
        io.add_method(&msg.group.clone(), move |_: Params| {
            Ok(Value::Array(msg.payload.clone()))
        });
    }

    let _server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Value("altheamesh.com".into()),
            AccessControlAllowOrigin::Null,
        ]))
        .rest_api(RestApi::Secure)
        .start_http(&sockaddr);

    match _server {
        Ok(server) => server.wait(),
        Err(_) => {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewChannel {
        sign: String,
        amount: String,
    }

    #[test]
    fn test_rpc_server() {
        let dummy_channel_1 = NewChannel {
            sign: "0xf567779ab2355".to_string(),
            amount: "0xffffff".to_string(),
        };

        let payload_1 = serde_json::to_string(&dummy_channel_1).unwrap();
        let payload_2 = serde_json::to_string(&dummy_channel_1).unwrap();
        let payload_3 = serde_json::to_string(&dummy_channel_1).unwrap();
        let payload_4 = serde_json::to_string(&dummy_channel_1).unwrap();

        let data_1 = ChannelData {
            group: "rpc_update".to_string(),
            payload: vec![serde_json::Value::String(payload_1)],
        };

        let data_2 = ChannelData {
            group: "rpc_update_sp".to_string(),
            payload: vec![serde_json::Value::String(payload_2)],
        };

        let data_3 = ChannelData {
            group: "rpc_data".to_string(),
            payload: vec![serde_json::Value::String(payload_3)],
        };

        let data_4 = ChannelData {
            group: "rpc_signs".to_string(),
            payload: vec![serde_json::Value::String(payload_4)],
        };

        println!("Starting RPC_JSON server");
        let vec_data = vec![data_1, data_2, data_3, data_4];
        transport(vec_data, &"127.0.0.1:3030".parse().unwrap());
    }
}
