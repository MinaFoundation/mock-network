pub mod network {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Create {
        pub network_id: String,
        pub node_map: std::collections::HashMap<String, super::node::Info>,
    }

    #[derive(Debug, Serialize)]
    pub struct Start {
        pub network_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Stop {
        pub network_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Status {
        pub network_id: String,
        pub status: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Delete {
        pub network_id: String,
    }
}

pub mod node {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Info {
        pub graphql_uri: Option<String>,
        pub node_type: Type,
        pub private_key: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub enum Type {
        Archive,
        BlockProducer,
        Seed,
        SnarkCoordinator,
        SnarkWorker,
    }

    #[derive(Debug, Serialize)]
    pub struct Start {
        pub fresh_state: bool,
        pub git_commit: String,
        pub network_id: String,
        pub node_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Stop {
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
}

#[derive(Debug, serde::Serialize)]
pub struct Error {
    pub network_id: String,
}

macro_rules! impl_display {
    ($name:path) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
                Ok(())
            }
        }
    };
}

impl_display!(network::Create);
impl_display!(network::Start);
impl_display!(network::Stop);
impl_display!(network::Status);
impl_display!(node::Start);
impl_display!(node::Stop);
impl_display!(node::ArchiveData);
impl_display!(node::MinaLogs);
impl_display!(node::PrecomputedBlocks);
impl_display!(node::ReplayerLogs);
