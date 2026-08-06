use crate::app::{Node, Connection};

pub fn network() -> Vec<Node> {
    vec![
        Node{
            name: "User",
            position: egui::Pos2 { x: 50.0, y: 100.0 },
            connections: vec![
                Connection {
                    target: 1,
                    total_travel_time_ms: 1000,
                    packet_sent_times_ms: std::collections::VecDeque::default(),
                }
            ],
        }, 
        Node{
            name: "Leader",
            position: egui::Pos2 { x: 200.0, y: 100.0 },
            connections: vec![
                Connection {
                    target: 0,
                    total_travel_time_ms: 1000,
                    packet_sent_times_ms: std::collections::VecDeque::default(),
                }
            ],
        },
        // Node{
        //     id: egui::Id::new("node3"),
        //     name: "Replica 1",
        //     position: egui::Pos2 { x: 400.0, y: 50.0 },
        //     connections: vec![],
        // },
        // Node{
        //     id: egui::Id::new("node4"),
        //     name: "Replica 2",
        //     position: egui::Pos2 { x: 400.0, y: 150.0 },
        //     connections: vec![],
        // }
    ]
}
