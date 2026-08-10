use crate::app::{Node, Connection, NodeId, NodeBehavior};

struct NoBehavior;
impl NodeBehavior for NoBehavior {}

struct DebugBehavior;
impl NodeBehavior for DebugBehavior {
    fn on_packet_received(&self) {
        println!("Packet received!");
    }

    fn on_step(&self) {
        println!("Step!");
    }
}

pub fn network() -> Vec<Node> {
    vec![
        Node{
            name: "User",
            position: egui::Pos2 { x: 50.0, y: 100.0 },
            connections: vec![
                Connection {
                    target: NodeId(1),
                    total_travel_time_ms: 1000,
                    packet_sent_times_ms: std::collections::VecDeque::default(),
                }
            ],
            behavior: Box::new(DebugBehavior{}),
        }, 
        Node{
            name: "Leader",
            position: egui::Pos2 { x: 200.0, y: 100.0 },
            connections: vec![
                Connection {
                    target: NodeId(2),
                    total_travel_time_ms: 3000,
                    packet_sent_times_ms: std::collections::VecDeque::default(),
                },
                Connection {
                    target: NodeId(3),
                    total_travel_time_ms: 2000,
                    packet_sent_times_ms: std::collections::VecDeque::default(),
                }
            ],
            behavior: Box::new(DebugBehavior{})
        },
        Node{
            name: "Replica 1",
            position: egui::Pos2 { x: 400.0, y: 50.0 },
            connections: vec![],
            behavior: Box::new(DebugBehavior{})
        },
        Node{
            name: "Replica 2",
            position: egui::Pos2 { x: 400.0, y: 150.0 },
            connections: vec![],
            behavior: Box::new(DebugBehavior{})
        }
    ]
}

