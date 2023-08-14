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
    #[clap(short, long, default_value = "default")]
    pub network_id: String,
}

#[derive(Args)]
pub struct CreateNetworkArgs {
    /// Network identifier
    #[clap(short = 'n', long)]
    pub network_id: String,

    /// Genesis ledger, constants, proof config, topology, etc (JSON)
    #[clap(short = 'c', long)]
    pub test_config: std::path::PathBuf,
}

#[derive(Subcommand)]
pub enum NodeCommand {
    /// Start a node
    Start(NodeCommandWithCommitArgs),
    /// Stop a node
    Stop(NodeCommandArgs),
    /// Get data from an archive node
    DumpArchiveData(NodeCommandArgs),
    /// Get logs from a node
    DumpMinaLogs(NodeCommandArgs),
    /// Get precomputed blocks from a node
    DumpPrecomputedBlocks(NodeCommandArgs),
    /// Get logs by replaying an archive node's database
    RunReplayer(NodeCommandArgs),
}

#[derive(Args, Debug)]
pub struct NodeId {
    #[clap(short = 'i', long)]
    pub node_id: String,
}

#[derive(Args, Debug)]
pub struct GitCommit {
    #[clap(short = 'g', long)]
    pub git_commit: String,
}

#[derive(Args, Debug)]
pub struct NodeCommandArgs {
    #[clap(flatten)]
    pub network_id: NetworkId,

    #[clap(flatten)]
    pub node_id: NodeId,
}

#[derive(Args, Debug)]
pub struct NodeCommandWithCommitArgs {
    #[clap(flatten)]
    pub git_commit: GitCommit,

    #[clap(flatten)]
    pub network_id: NetworkId,

    #[clap(flatten)]
    pub node_id: NodeId,
}

impl NodeCommandArgs {
    pub fn node_id(&self) -> &str {
        &self.node_id.node_id
    }

    pub fn network_id(&self) -> &str {
        &self.network_id.network_id
    }
}

impl NodeCommandWithCommitArgs {
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
        let test_config_file = "/path/to/test/config";
        let args = vec![
            "mock",
            "network",
            "create",
            "--network-id",
            network_id,
            "--test-config",
            test_config_file,
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Network(NetworkCommand::Create(args)) => {
                assert_eq!(args.network_id, network_id);
                assert_eq!(args.test_config, PathBuf::from(test_config_file));
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
    fn test_node_start_command() {
        let git_commit = "abcdef012345";
        let node_id = "node0";
        let args = vec![
            "mock",
            "node",
            "start",
            "--node-id",
            node_id,
            "--git-commit",
            git_commit,
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Node(NodeCommand::Start(args)) => {
                assert_eq!(args.node_id(), node_id);
                assert_eq!(args.network_id(), "default");
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
        let node_id = "test";
        let args = vec!["mock", "node", "dump-mina-logs", "--node-id", node_id];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Node(NodeCommand::DumpMinaLogs(args)) => {
                assert_eq!(args.node_id(), node_id);
                assert_eq!(args.network_id(), "default");
            }
            _ => panic!("Unexpected command parsed"),
        }
    }
}
