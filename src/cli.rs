use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "mock - A Command-line Tool for a Mock Mina Network to Test the Abstract Engine"
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage local network
    #[clap(subcommand)]
    Network(NetworkCommand),

    /// Manage a single node
    #[clap(subcommand)]
    Node(NodeCommand),
}

#[derive(Subcommand)]
pub enum NetworkCommand {
    /// Create a local network
    Create(CreateNetworkArgs),
    /// Delete a local network
    Delete(NetworkId),
    /// Start a local network
    Start(NetworkId),
    /// Stop a local network
    Stop(NetworkId),
    /// Status of a local network
    Status(NetworkId),
}

#[derive(Args, Debug)]
pub struct NetworkId {
    #[clap(short, long)]
    pub network_id: String,
}

#[derive(Args)]
pub struct CreateNetworkArgs {
    /// Network identifier
    #[clap(short = 'n', long)]
    pub network_id: String,

    /// Runtime config path (JSON)
    #[clap(short = 'r', long)]
    pub genesis_ledger: std::path::PathBuf,

    /// Topology file path (JSON)
    #[clap(short = 't', long)]
    pub topology: std::path::PathBuf,
}

#[derive(Subcommand)]
pub enum NodeCommand {
    /// Start a node
    Start(NodeCommandStartArgs),
    /// Stop a node
    Stop(NodeCommandArgs),
    /// Get data from an archive node
    DumpArchiveData(NodeCommandArgs),
    /// Get logs from a node
    DumpMinaLogs(NodeCommandArgs),
    /// Get precomputed blocks from a node
    DumpPrecomputedBlocks(NodeCommandArgs),
    /// Get logs by replaying an archive node's database
    RunReplayer(ReplayerCommandArgs),
}

#[derive(Args, Debug)]
pub struct NodeId {
    #[clap(short = 'i', long)]
    pub node_id: String,
}

#[derive(Args, Debug)]
pub struct FreshState {
    #[clap(short = 'f', long)]
    pub fresh_state: bool,
}

#[derive(Args, Debug)]
pub struct Slot {
    #[clap(short = 's', long)]
    pub start_slot_since_genesis: u32,
}

#[derive(Args, Debug)]
pub struct NodeCommandArgs {
    #[clap(flatten)]
    pub network_id: NetworkId,

    #[clap(flatten)]
    pub node_id: NodeId,
}

#[derive(Args, Debug)]
pub struct NodeCommandStartArgs {
    #[clap(flatten)]
    pub fresh_state: FreshState,

    #[clap(flatten)]
    pub network_id: NetworkId,

    #[clap(flatten)]
    pub node_id: NodeId,
}

#[derive(Args, Debug)]
pub struct ReplayerCommandArgs {
    #[clap(flatten)]
    pub network_id: NetworkId,

    #[clap(flatten)]
    pub node_id: NodeId,

    #[clap(flatten)]
    pub start_slot_since_genesis: Slot,
}

impl NodeCommandArgs {
    pub fn node_id(&self) -> &str {
        &self.node_id.node_id
    }

    pub fn network_id(&self) -> &str {
        &self.network_id.network_id
    }
}

impl NodeCommandStartArgs {
    pub fn node_id(&self) -> &str {
        &self.node_id.node_id
    }

    pub fn network_id(&self) -> &str {
        &self.network_id.network_id
    }
}

impl ReplayerCommandArgs {
    pub fn node_id(&self) -> &str {
        &self.node_id.node_id
    }

    pub fn network_id(&self) -> &str {
        &self.network_id.network_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_network_create_command() {
        let network_id = "network0";
        let genesis_ledger_path = "/path/to/genesis_ledger.json";
        let topology_path = "/path/to/topology.json";
        let args = vec![
            "mock",
            "network",
            "create",
            "--network-id",
            network_id,
            "--genesis-ledger",
            genesis_ledger_path,
            "--topology",
            topology_path,
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Network(NetworkCommand::Create(args)) => {
                assert_eq!(args.network_id, network_id);
                assert_eq!(args.genesis_ledger, PathBuf::from(genesis_ledger_path));
                assert_eq!(args.topology, PathBuf::from(topology_path));
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_network_delete_command() {
        let network_id = "test";
        let args = vec!["mock", "network", "delete", "--network-id", network_id];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Network(NetworkCommand::Delete(args)) => {
                assert_eq!(args.network_id, network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_network_start_command() {
        let network_id = "test";
        let args = vec!["mock", "network", "start", "--network-id", network_id];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Network(NetworkCommand::Start(args)) => {
                assert_eq!(args.network_id, network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_network_stop_command() {
        let network_id = "test";
        let args = vec!["mock", "network", "stop", "--network-id", network_id];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Network(NetworkCommand::Stop(args)) => {
                assert_eq!(args.network_id, network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_node_start_command_fresh_state() {
        let network_id = "network0";
        let node_id = "node0";
        let args = vec![
            "mock",
            "node",
            "start",
            "--network-id",
            network_id,
            "--node-id",
            node_id,
            "--fresh-state",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Node(NodeCommand::Start(args)) => {
                assert!(args.fresh_state.fresh_state);
                assert_eq!(args.node_id(), node_id);
                assert_eq!(args.network_id(), network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_node_start_command_non_fresh_state() {
        let network_id = "network0";
        let node_id = "node0";
        let args = vec![
            "mock",
            "node",
            "start",
            "--network-id",
            network_id,
            "--node-id",
            node_id,
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Node(NodeCommand::Start(args)) => {
                assert!(!args.fresh_state.fresh_state);
                assert_eq!(args.node_id(), node_id);
                assert_eq!(args.network_id(), network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_node_stop_command() {
        let node_id = "test";
        let network_id = "banana";
        let args = vec![
            "mock",
            "node",
            "stop",
            "--node-id",
            node_id,
            "--network-id",
            network_id,
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Node(NodeCommand::Stop(args)) => {
                assert_eq!(args.node_id(), node_id);
                assert_eq!(args.network_id(), network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }

    #[test]
    fn test_node_logs_command() {
        let network_id = "network0";
        let node_id = "test";
        let args = vec![
            "mock",
            "node",
            "dump-mina-logs",
            "--network-id",
            network_id,
            "--node-id",
            node_id,
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Node(NodeCommand::DumpMinaLogs(args)) => {
                assert_eq!(args.node_id(), node_id);
                assert_eq!(args.network_id(), network_id);
            }
            _ => panic!("Unexpected command parsed"),
        }
    }
}
