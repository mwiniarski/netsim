use crate::app::*;

struct DebugBehavior;
impl NodeBehavior for DebugBehavior {
    fn on_packet_received(&self, ctx: &mut NetworkContext, source: NodeId, payload: String) {
        println!("Packet received on {:?}! [from]: {:?}, [payload]: {:?}", ctx.local_node(), source, payload);
    }
}

struct PingPong {}
impl NodeBehavior for PingPong {
    fn on_packet_received(&self, ctx: &mut NetworkContext, source: NodeId, payload: String) {
        println!("Packet received on {:?}! [from]: {:?}, [payload]: {:?}", ctx.local_node(), source, payload);
        ctx.send_packet(source, payload);
    }
}

struct Forwarder {}
impl NodeBehavior for Forwarder {
    fn on_packet_received(&self, ctx: &mut NetworkContext, source: NodeId, payload: String) {
        println!("Packet received on {:?}! [from]: {:?}, [payload]: {:?}", ctx.local_node(), source, payload);

        for connection in ctx.connections() {
            if connection == source {
                continue
            }

            println!("Forwarding to: {:?}", connection);
            ctx.send_packet(connection, payload.clone());
        }
    }
}

pub fn network() -> (Vec<Node>, Vec<Connection>) {
    (vec![
        Node{
            name: "User",
            position: egui::Pos2 { x: 50.0, y: 100.0 },
            behavior: Box::new(DebugBehavior{}),
            connections: vec![],
        }, 
        Node{
            name: "Leader",
            position: egui::Pos2 { x: 200.0, y: 100.0 },
            behavior: Box::new(Forwarder{}),
            connections: vec![],
        },
        Node{
            name: "Replica 1",
            position: egui::Pos2 { x: 400.0, y: 50.0 },
            behavior: Box::new(PingPong{}),
            connections: vec![],
        },
        Node{
            name: "Replica 2",
            position: egui::Pos2 { x: 400.0, y: 150.0 },
            behavior: Box::new(PingPong{}),
            connections: vec![],
        }
    ],
    vec![
        Connection {
            source: NodeId(0),
            target: NodeId(1),
            total_travel_time_ms: 1000,
        },
        Connection {
            source: NodeId(1),
            target: NodeId(2),
            total_travel_time_ms: 3000,
        },
        Connection {
            source: NodeId(1),
            target: NodeId(3),
            total_travel_time_ms: 2000,
        }
    ])
}

