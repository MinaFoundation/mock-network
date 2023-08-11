use crate::output::*;

pub fn node_map(network_id: String) -> std::collections::HashMap<String, NodeInfo> {
    let node0 = NodeInfo {
        graphql_uri: None,
        network_id: network_id.clone(),
        network_keypair: None,
        node_id: "node0".to_string(),
        node_type: NodeType::SnarkCoordinator,
    };
    let node1 = NodeInfo {
        graphql_uri: Some("node1/graphql".to_string()),
        network_id: network_id.clone(),
        network_keypair: Some("EKEigSUES3iGHmKJNMMHVVhQmoiV5gWGXwvkMzQtE6rVnAauex4Y".to_string()),
        node_id: "node1".to_string(),
        node_type: NodeType::Archive,
    };
    let node2 = NodeInfo {
        graphql_uri: Some("node2/graphql".to_string()),
        network_id: network_id.clone(),
        network_keypair: Some("EKEU3eCHwY5sj2VVH5RXSNQddCpK26aUgCqxXEg9JkkqVeE3XGLu".to_string()),
        node_id: "node2".to_string(),
        node_type: NodeType::BlockProducer,
    };
    let node3 = NodeInfo {
        graphql_uri: Some("node3/graphql".to_string()),
        network_id: network_id.clone(),
        network_keypair: Some("EKEXjtu5y3N3VoBtDF8iuaYrrcB7arjsVEjnbcBRH7ZEL5k3odK9".to_string()),
        node_id: "node3".to_string(),
        node_type: NodeType::Seed,
    };
    let node4 = NodeInfo {
        graphql_uri: Some("node4/graphql".to_string()),
        network_id,
        network_keypair: Some("EKEiAqExT3sNGYA4Lp6ZPfqfACYJSkTNAVzsHj7Qkzw59A5BsZGH".to_string()),
        node_id: "node4".to_string(),
        node_type: NodeType::SnarkWorker,
    };

    std::collections::HashMap::from([
        (node0.node_id.clone(), node0),
        (node1.node_id.clone(), node1),
        (node2.node_id.clone(), node2),
        (node3.node_id.clone(), node3),
        (node4.node_id.clone(), node4),
    ])
}
