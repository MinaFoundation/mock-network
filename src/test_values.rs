use crate::output::*;

pub fn node_map() -> std::collections::HashMap<String, node::Info> {
    let node0 = node::Info {
        graphql_uri: None,
        network_keypair: None,
        node_type: node::Type::SnarkCoordinator,
    };
    let node1 = node::Info {
        graphql_uri: Some("node1/graphql".to_string()),
        network_keypair: Some("EKEigSUES3iGHmKJNMMHVVhQmoiV5gWGXwvkMzQtE6rVnAauex4Y".to_string()),
        node_type: node::Type::Archive,
    };
    let node2 = node::Info {
        graphql_uri: Some("node2/graphql".to_string()),
        network_keypair: Some("EKEU3eCHwY5sj2VVH5RXSNQddCpK26aUgCqxXEg9JkkqVeE3XGLu".to_string()),
        node_type: node::Type::BlockProducer,
    };
    let node3 = node::Info {
        graphql_uri: Some("node3/graphql".to_string()),
        network_keypair: Some("EKEXjtu5y3N3VoBtDF8iuaYrrcB7arjsVEjnbcBRH7ZEL5k3odK9".to_string()),
        node_type: node::Type::Seed,
    };
    let node4 = node::Info {
        graphql_uri: Some("node4/graphql".to_string()),
        network_keypair: Some("EKEiAqExT3sNGYA4Lp6ZPfqfACYJSkTNAVzsHj7Qkzw59A5BsZGH".to_string()),
        node_type: node::Type::SnarkWorker,
    };

    std::collections::HashMap::from([
        ("node0".to_string(), node0),
        ("node1".to_string(), node1),
        ("node2".to_string(), node2),
        ("node3".to_string(), node3),
        ("node4".to_string(), node4),
    ])
}
