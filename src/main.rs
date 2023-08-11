mod cli;
mod output;
mod test_values;

use clap::Parser;
use cli::{Cli, Command, NetworkCommand, NodeCommand};
use output::*;
use rand::random;
use serde::Serialize;
use test_values::*;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Command::Network(net_cmd) => match net_cmd {
            NetworkCommand::Create(cmd) => {
                let network_id = cmd.network_id().to_string();
                let res = NetworkCreate {
                    network_id: network_id.clone(),
                    node_map: node_map(network_id),
                };
                pretty_print(&res);
            }
            NetworkCommand::Delete(cmd) => {
                let res = NetworkDelete {
                    network_id: cmd.network_id,
                };
                pretty_print(&res);
            }
            NetworkCommand::Start(cmd) => {
                let success = rand::random::<bool>();
                if success {
                    let res = NetworkStart {
                        network_id: cmd.network_id,
                    };
                    pretty_print(&res);
                } else {
                    let error = Error {
                        network_id: cmd.network_id,
                    };
                    pretty_print(&error);
                }
            }
            NetworkCommand::Stop(cmd) => {
                let success = rand::random::<bool>();
                if success {
                    let res = NetworkStop {
                        network_id: cmd.network_id,
                    };
                    pretty_print(&res);
                } else {
                    let error = Error {
                        network_id: cmd.network_id,
                    };
                    pretty_print(&error);
                }
            }
            NetworkCommand::Status(cmd) => {
                let success = rand::random::<bool>();
                if success {
                    let res = NetworkStatus {
                        network_id: cmd.network_id,
                        status: "status".to_string(),
                    };
                    pretty_print(&res);
                } else {
                    let error = Error {
                        network_id: cmd.network_id,
                    };
                    pretty_print(&error);
                }
            }
        },
        Command::Node(node_cmd) => match node_cmd {
            NodeCommand::Start(cmd) => {
                let res = NodeStart {
                    fresh_state: random::<bool>(),
                    git_commit: "abcdef012345".to_string(),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                };
                pretty_print(&res);
            }
            NodeCommand::Stop(cmd) => {
                let res = NodeStop {
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                };
                pretty_print(&res);
            }
            NodeCommand::Logs(cmd) => {
                let res = NodeLogs {
                    logs: String::from("log0\nlog1\nlog2"),
                    network_id: cmd.network_id().to_string(),
                    node_id: cmd.node_id().to_string(),
                };
                pretty_print(&res);
            }
        },
    }
}

fn pretty_print<T: Serialize>(res: &T) {
    println!("{}", serde_json::to_string_pretty(res).unwrap());
}
