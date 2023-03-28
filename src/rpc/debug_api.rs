use crate::{
    bundler::Mode,
    types::{reputation::ReputationEntry, user_operation::UserOperation},
};
use ethers::types::Address;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(server, namespace = "debug_bundler")]
pub trait DebugApi {
    #[method(name = "clearState")]
    async fn clear_state(&self) -> RpcResult<()>;

    #[method(name = "dumpMempool")]
    async fn dump_mempool(&self, entry_point: Address) -> RpcResult<Vec<UserOperation>>;

    #[method(name = "setReputation")]
    async fn set_reputation(
        &self,
        reputation_entries: Vec<ReputationEntry>,
        entry_point: Address,
    ) -> RpcResult<()>;

    #[method(name = "dumpReputation")]
    async fn dump_reputation(&self, entry_point: Address) -> RpcResult<Vec<ReputationEntry>>;

    #[method(name = "setBundlingMode")]
    async fn set_bundling_mode(&self, mode: Mode) -> RpcResult<()>;
}
