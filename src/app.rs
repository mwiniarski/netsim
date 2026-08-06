use std::collections::VecDeque;

use eframe::CreationContext;
use egui::Shape;

pub struct App {
    nodes: Vec<Node>,
}

pub struct Node {
    // Text inside the node (must be unique)
    pub name: &'static str,

    // Top-left coordinate of a node relative to the panel
    pub position: egui::Pos2,

    // Indexes of nodes this node connects to
    pub connections: Vec<Connection>,
}

pub struct Connection {
    // Index of the target in Node vector
    pub target: usize,

    // Time of packet travel animation
    pub total_travel_time_ms: u128,

    // Distances traveled by packets
    pub packet_sent_times_ms: VecDeque<u128>,
}

impl eframe::App for App {
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
        for node in &mut self.nodes {
            let res = Self::paint_node(&node, ui);
            let node_rect = res.rect;
            rects.push(node_rect);
            
            // The first paint returns bad rect, so need to check if it's dragged
            if res.dragged() && node_rect.min != node.position {
                node.position = node_rect.min - ui.max_rect().min.to_vec2();
            }

            // Send a message when button clicked
            if res.clicked() {
               node.connections[0].packet_sent_times_ms.push_back(now);
            }
        }

        // Draw connections
        for (i, node) in self.nodes.iter_mut().enumerate() {
            for connection in node.connections.iter_mut() {

                let node1_center = rects[i].center();
                let node2_center = rects[connection.target].center();

                // Draw lines
                ui.painter().add(Shape::dashed_line(
                    &vec![node1_center, node2_center],
                    egui::Stroke::new(1.0_f32, egui::Color32::LIGHT_GREEN),
                    5.0f32, 
                    5.0f32
                ));

                // Draw text below a line
                let line_vec = node2_center.to_vec2() - node1_center.to_vec2();
                let mut offset_dir = line_vec.normalized().rot90();
                if offset_dir.y > 0.0 {
                    offset_dir *= -1.0;
                }

                let text_pos = node1_center + line_vec / 2.0 - offset_dir * 20.0;

                ui.painter().text(
                    text_pos, 
                    egui::Align2::CENTER_CENTER, 
                    "2s", 
                    egui::FontId::default(),
                    egui::Color32::WHITE
                );

                // Remove packets that already arrived
                while let Some(_) = connection.packet_sent_times_ms.pop_front_if(|sent_time| {
                    now > *sent_time + connection.total_travel_time_ms
                }) {
                    // Here logic for packets arriving at destination
                }
                
                // Draw packets sent
                for packet_sent_ms in &connection.packet_sent_times_ms {
                    let part_travelled = (now - *packet_sent_ms) as f32 / connection.total_travel_time_ms as f32;
                    let packet_pos = node1_center + line_vec * part_travelled + offset_dir * 10.0;
                    ui.painter().circle_filled(
                        packet_pos,
                        10.0,
                        egui::Color32::WHITE
                    );
                }
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

    pub fn new(cc: &CreationContext<'_>, network: Vec<Node>) -> Self {

        // Dark mode
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        
        // todo: do some checks (e.g. if connections are valid)
        Self { nodes: network }
    }
}