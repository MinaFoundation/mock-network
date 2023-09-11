mod cli;
mod output;
mod test_values;

use clap::Parser;
use cli::{Cli, Command, NetworkCommand, NodeCommand};
use output::*;
use serde::Serialize;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Command::Network(net_cmd) => match net_cmd {
            NetworkCommand::Create(cmd) => {
                pretty_print(network::Create {
                    network_id: cmd.network_id,
                    nodes: test_values::node_map(),
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
                    status: "status0\nstatus1\nstatus2".to_string(),
                });
            }
        },
        Command::Node(node_cmd) => match node_cmd {
            NodeCommand::Start(cmd) => {
                pretty_print(node::Start {
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
                    data: "datum0\ndatum1\ndatum2".to_string(),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::DumpMinaLogs(cmd) => {
                pretty_print(node::MinaLogs {
                    logs: "log0\nlog1\nlog2".to_string(),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::DumpPrecomputedBlocks(cmd) => {
                pretty_print(node::PrecomputedBlocks {
                    blocks: "block0\nblock1\nblock2".to_string(),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                });
            }
            NodeCommand::RunReplayer(cmd) => {
                pretty_print(node::ReplayerLogs {
                    logs: "log0\nlog1\nlog2".to_string(),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                    start_slot_since_genesis: 0,
                });
            }
        },
    }
}

fn pretty_print<T: Serialize>(res: T) {
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
