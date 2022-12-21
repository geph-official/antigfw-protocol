use std::collections::HashMap;

use async_trait::async_trait;
use nanorpc::{nanorpc_derive, JrpcRequest};

#[nanorpc_derive]
#[async_trait]
pub trait AntigfwProtocol {
    /// Report new binder activity, including HTTP headers.
    async fn binder_report(&self, headers: HashMap<String, String>, body: JrpcRequest);

    /// Report something getting blocked.
    async fn report_blocked(&self, bridge_ip: String);

    /// Check whether a particular *blind token hash* (in hex) is blacklisted.
    async fn blacklisted(&self, token_hash: String);
}
