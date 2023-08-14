mod cli;
mod output;
mod test_values;

use clap::Parser;
use cli::{Cli, Command, NetworkCommand, NodeCommand};
use output::*;
use serde::Serialize;
use test_values::*;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Command::Network(net_cmd) => match net_cmd {
            NetworkCommand::Create(cmd) => {
                pretty_print(network::Create {
                    network_id: cmd.network_id,
                    node_map: node_map(),
                });
            }
            NetworkCommand::Start(cmd) => {
                pretty_print(network::Start {
                    network_id: cmd.network_id,
                });
            }
            NetworkCommand::Delete(cmd) => {
                pretty_print(network::Delete {
                    network_id: cmd.network_id,
                });
            }
            NetworkCommand::Stop(cmd) => {
                pretty_print(network::Stop {
                    network_id: cmd.network_id,
                });
            }
            NetworkCommand::Status(cmd) => {
                pretty_print(network::Status {
                    network_id: cmd.network_id,
                    status: "status".to_string(),
                });
            }
        },
        Command::Node(node_cmd) => match node_cmd {
            NodeCommand::Start(cmd) => {
                pretty_print(node::Start {
                    fresh_state: cmd.fresh_state.fresh_state,
                    git_commit: "abcdef012345".to_string(),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::Stop(cmd) => {
                pretty_print(node::Stop {
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::DumpArchiveData(cmd) => {
                pretty_print(node::ArchiveData {
                    data: String::from("datum0\ndatum1\ndatum2"),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::DumpMinaLogs(cmd) => {
                pretty_print(node::MinaLogs {
                    logs: String::from("log0\nlog1\nlog2"),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::DumpPrecomputedBlocks(cmd) => {
                pretty_print(node::PrecomputedBlocks {
                    blocks: String::from("block0\nblock1\nblock2"),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::RunReplayer(cmd) => {
                pretty_print(node::ReplayerLogs {
                    logs: String::from("log0\nlog1\nlog2"),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
        },
    }
}

fn pretty_print<T: Serialize>(res: T) {
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
