use grin_api::{client, json_rpc::*};
use grin_history_tools_models::block::{BlockPrintable, BlockHeaderPrintable, Tip};

use serde_json::json;
use crate::client::HttpClientError;

const ENDPOINT: &str = "/v2/foreign";

#[derive(Clone)]
pub struct HTTPNodeClient {
	node_url: String,
	node_api_secret: Option<String>,
}

impl HTTPNodeClient {
	/// Create a new client that will communicate with the given grin node
	pub fn new(node_url: &str, node_api_secret: Option<String>) -> HTTPNodeClient {
		HTTPNodeClient {
			node_url: node_url.to_owned(),
			node_api_secret: node_api_secret,
		}
	}
	fn send_json_request<D: serde::de::DeserializeOwned>(
		&self,
		method: &str,
		params: &serde_json::Value,
	) -> Result<D, HttpClientError> {
		let timeout = match method {
			// 6 hours read timeout
			"validate_chain" => client::TimeOut::new(20, 21600, 20),
			_ => client::TimeOut::default(),
		};
		let url = format!("http://{}{}", self.node_url, ENDPOINT);
		let req = build_request(method, params);
		let res = client::post::<Request, Response>(
			url.as_str(),
			self.node_api_secret.clone(),
			&req,
			timeout,
		);

		match res {
			Err(e) => {
				let report = format!("Error calling {}: {}", method, e);
				error!("{}", report);
				Err(HttpClientError::RPCError(report))
			}
			Ok(inner) => match inner.clone().into_result() {
				Ok(r) => Ok(r),
				Err(e) => {
					error!("{:?}", inner);
					let report = format!("Unable to parse response for {}: {}", method, e);
					error!("{}", report);
					Err(HttpClientError::RPCError(report))
				}
			},
		}
	}

    pub fn _get_header(&self, height: u64) -> Result<BlockHeaderPrintable, HttpClientError> {
		let params = json!([height, null, null]);
		self.send_json_request::<BlockHeaderPrintable>("get_header", &params)
	}

    pub fn get_block(&self, height: u64) -> Result<BlockPrintable, HttpClientError> {
		let params = json!([height, null, null]);
		self.send_json_request::<BlockPrintable>("get_block", &params)
	}

    pub fn get_tip(&self) -> Result<Tip, HttpClientError> {
		let params = json!([]);
		self.send_json_request::<Tip>("get_tip", &params)
	}
}