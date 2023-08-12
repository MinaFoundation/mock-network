use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NetworkCreate {
    pub network_id: String,
    pub node_map: std::collections::HashMap<String, NodeInfo>,
}

#[derive(Debug, Serialize)]
pub struct NetworkDelete {
    pub network_id: String,
}

#[derive(Debug, Serialize)]
pub struct NetworkStart {
    pub network_id: String,
}

#[derive(Debug, Serialize)]
pub struct NetworkStop {
    pub network_id: String,
}

#[derive(Debug, Serialize)]
pub struct NetworkStatus {
    pub network_id: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct NodeInfo {
    pub graphql_uri: Option<String>,
    pub network_id: String,
    pub network_keypair: Option<String>,
    pub node_id: String,
    pub node_type: NodeType,
}

#[derive(Debug, Serialize)]
pub enum NodeType {
    Archive,
    BlockProducer,
    Seed,
    SnarkCoordinator,
    SnarkWorker,
}

#[derive(Debug, Serialize)]
pub struct NodeStart {
    pub fresh_state: bool,
    pub git_commit: String,
    pub network_id: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct NodeStop {
    pub network_id: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct ArchiveData {
    pub data: String,
    pub network_id: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct MinaLogs {
    pub logs: String,
    pub network_id: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct PrecomputedBlocks {
    pub blocks: String,
    pub network_id: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct ReplayerLogs {
    pub logs: String,
    pub network_id: String,
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub network_id: String,
}

macro_rules! impl_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
                Ok(())
            }
        }
    };
}

impl_display!(NetworkCreate);
impl_display!(NetworkStart);
impl_display!(NetworkStop);
impl_display!(NetworkStatus);
impl_display!(NodeStart);
impl_display!(NodeStop);
impl_display!(ArchiveData);
impl_display!(MinaLogs);
impl_display!(PrecomputedBlocks);
impl_display!(ReplayerLogs);
