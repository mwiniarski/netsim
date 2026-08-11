use eframe::CreationContext;
use egui::Shape;

pub struct App {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    packets: Vec<Packet>,
}

struct Packet {
    // Connections on which the packet travels
    connection_id: ConnectionId,
    
    // Contents of the packet
    payload: String,
    
    // Timestamp when packet was sent
    sent_time: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct NodeId(pub usize);

pub struct Node {
    // Text inside the node (must be unique)
    pub name: &'static str,
    
    // Top-left coordinate of a node relative to the panel
    pub position: egui::Pos2,
    
    // Methods defining what happens when iteracted with
    pub behavior: Box<dyn NodeBehavior>,
    
    // Connections this node holds
    pub connections: Vec<ConnectionId>,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ConnectionId(pub usize);

pub struct Connection {
    // Index of the source in Node vector
    pub source: NodeId,

    // Index of the target in Node vector
    pub target: NodeId,

    // Time of packet travel animation
    pub total_travel_time_ms: u128,
}

pub struct NetworkContext {
    local_node: NodeId,
    sendable_nodes: Vec<NodeId>,
    packets_to_send: Vec<(NodeId, String)>,
}

impl NetworkContext {
    pub fn send_packet(&mut self, target: NodeId, payload: String) {
        self.packets_to_send.push((target, payload));
    }
    
    pub fn connections(&self) -> Vec<NodeId> {
        self.sendable_nodes.clone()
    }

    pub fn local_node(&self) -> NodeId {
        self.local_node
    }
}

pub trait NodeBehavior {
    fn on_packet_received(&self, _ctx: &mut NetworkContext, _source: NodeId, _payload: String) {}
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Self::millis_since_epoch();
       
        // Remove packets that already arrived
        let mut new_packets = vec![];
        self.packets.retain(|packet| {
            if now < packet.sent_time + self.connections[packet.connection_id.0].total_travel_time_ms {
                return true;
            }

            // Prepare context
            let connection = &self.connections[packet.connection_id.0];
            let local_node = connection.target;
            
            let sendable_connections = &self
                .nodes[local_node.0]
                .connections;

            let sendable_nodes = sendable_connections.iter()
                .map(|connection_id|{
                    self.connections[connection_id.0].target
                }).collect();

            let mut ctx = NetworkContext {
                local_node,
                sendable_nodes,
                packets_to_send: vec![],
            };

            // Ask node/user what they want to do
            {
                self.nodes[local_node.0].behavior.on_packet_received(&mut ctx, connection.source, packet.payload.clone());
            }

            // Act on result
            for (target_node, payload) in ctx.packets_to_send {
                match sendable_connections.iter().find(|connection_id| {
                    self.connections[connection_id.0].target == target_node
                }) {
                    Some(connection_id) => {
                        new_packets.push(Packet {
                            connection_id: *connection_id,
                            payload,
                            sent_time: now,
                        });
                    },
                    None => {
                        println!("Couldn't send ")
                    }
                }
            }
            
            false
        });

        self.packets.append(&mut new_packets);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Panel with the settings and stats of the thing that was clicked
        egui::Panel::left("left").show_inside(ui, |ui| {
            ui.label("Settings!");
            
            // Here we need to know what was clicked
            self.show_settings(ui);
        });
        
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.paint_network(ui);
        });

        // This is needed to keep the animations animating
        ui.request_repaint();
    }
}

impl App {
    fn paint_node(node: &Node, ui: &mut egui::Ui) -> egui::Response {
        egui::Area::new(node.name.into())

            // Make it clickable and dragable
            .sense(egui::Sense::click_and_drag())

            // Where it should be - mostly used for starting position
            .current_pos(Self::to_panel_pos(node.position, &ui))

            // So that the node cannot be dragged out of the panel
            .constrain_to(ui.max_rect())

            .show(ui, |ui| {

                // Frame is styling the area
                egui::Frame::NONE

                    // Background
                    .fill(egui::Color32::BLACK)

                    // Outline
                    .stroke(egui::Stroke::new(1.0_f32, egui::Color32::LIGHT_GRAY))

                    // Circle
                    .corner_radius(egui::CornerRadius::same(255))
                    
                    // Padding
                    .inner_margin(20.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::widgets::Label::new(node.name)
                                .selectable(false)
                        );
                    });
            }).response
    }

    fn paint_network(&mut self, ui: &mut egui::Ui) {

        let mut rects = vec![];
        let now = Self::millis_since_epoch();

        // Draw nodes
        for (node_id, node) in self.nodes.iter_mut().enumerate() {
            let res = Self::paint_node(&node, ui);
            let node_rect = res.rect;
            rects.push(node_rect);
            
            // The first paint returns bad rect, so need to check if it's dragged
            if res.dragged() && node_rect.min != node.position {
                node.position = node_rect.min - ui.max_rect().min.to_vec2();
            }

            // Send a message when button clicked
            if res.clicked() {
                for (conn_id, connection) in self.connections.iter_mut().enumerate() {
                    if connection.source == NodeId(node_id) {
                        self.packets.push(Packet {
                            sent_time: now,
                            connection_id: ConnectionId(conn_id),
                            payload: now.to_string(),
                        });
                    }
                }
            }
        }

        // Draw connections
        for (conn_id, connection) in self.connections.iter_mut().enumerate() {

            let source_center = rects[connection.source.0].center();
            let target_center = rects[connection.target.0].center();

            // Draw lines
            ui.painter().add(Shape::dashed_line(
                &vec![source_center, target_center],
                egui::Stroke::new(1.0_f32, egui::Color32::LIGHT_GREEN),
                5.0f32, 
                5.0f32
            ));

            // Draw text below a line
            let line_vec = target_center.to_vec2() - source_center.to_vec2();
            let mut offset_dir = line_vec.normalized().rot90();
            if offset_dir.y > 0.0 {
                offset_dir *= -1.0;
            }

            let text_pos = source_center + line_vec / 2.0 - offset_dir * 20.0;

            ui.painter().text(
                text_pos, 
                egui::Align2::CENTER_CENTER, 
                "2s", 
                egui::FontId::default(),
                egui::Color32::WHITE
            );

            // Draw packets in flight
            for packet in &self.packets {
                if packet.connection_id != ConnectionId(conn_id) {
                    continue
                }

                let part_travelled = (now - packet.sent_time) as f32 / connection.total_travel_time_ms as f32;
                let packet_pos = source_center + line_vec * part_travelled + offset_dir * 10.0;
                ui.painter().circle_filled(
                    packet_pos,
                    10.0,
                    egui::Color32::WHITE
                );
            }
        }
    }

    fn to_panel_pos(absolute_pos: egui::Pos2, ui: &egui::Ui) -> egui::Pos2 {
        absolute_pos + ui.max_rect().min.to_vec2()
    }

    fn millis_since_epoch() -> u128 {
        std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis()
    }

    fn show_settings(&self, _ui: &mut egui::Ui) {
        
    }

    pub fn new(cc: &CreationContext<'_>, network: (Vec<Node>, Vec<Connection>)) -> Self {

        // Dark mode
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        
        let mut app = Self { nodes: network.0, connections: network.1, packets: vec![] };

        // Make all connections bi-directional
        let mut reverse_connections = vec![];
        for connection in app.connections.iter() {
            reverse_connections.push(Connection {
                source: connection.target, 
                target: connection.source, 
                total_travel_time_ms: connection.total_travel_time_ms
            });
        }
        app.connections.append(&mut reverse_connections);
        
        // Add outgoing connection to node
        for (conn_id, connection) in app.connections.iter().enumerate() {
            app.nodes[connection.source.0].connections.push(ConnectionId(conn_id));
        }

        app
    }
}