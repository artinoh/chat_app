use eframe::{egui, App, Frame};
use tokio::sync::mpsc;
use super::client::connect_to_server;
use crate::common::types::ChatMessage;
use chrono::Local;
use crate::common::config::ServerSettings;
use rand::Rng;
use std::collections::HashMap;


pub struct ChatApp {
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub username: String,
    pub host: String,
    pub port: u16,
    pub message_sender: Option<mpsc::UnboundedSender<String>>,
    pub gui_receiver: Option<mpsc::UnboundedReceiver<String>>,
    pub is_connected: bool,
    pub user_colors: HashMap<String, egui::Color32>
}

impl ChatApp {
    pub fn default_with_server(settings: &ServerSettings) -> Self {
        Self {
            messages: vec![],
            input: String::new(),
            username: String::new(),
            host: settings.host.clone(),
            port: settings.port,
            message_sender: None,
            gui_receiver: None,
            is_connected: false,
            user_colors: HashMap::new()
        }
    }
}

impl Default for ChatApp {
    fn default() -> Self {
        Self {
            messages: vec![],
            input: String::new(),
            username: String::new(),
            host: "127.0.0.1".to_string(),
            port: 9001,
            message_sender: None,
            gui_receiver: None,
            is_connected: false,
            user_colors: HashMap::new()
        }
    }
}

impl App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.receive_messages();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Chat Application");
            self.display_connection_settings(ui);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.display_input(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_messages(ui);
        });
    }
}

impl ChatApp {
    fn receive_messages(&mut self) {
        if let Some(gui_receiver) = &mut self.gui_receiver {
            while let Ok(message) = gui_receiver.try_recv() {
                let chat_message: ChatMessage = serde_json::from_str(&message).unwrap();

                if !self.user_colors.contains_key(&chat_message.username) {
                    let color = egui::Color32::from_rgb(
                        rand::thread_rng().gen_range(0..=255),
                        rand::thread_rng().gen_range(0..=255),
                        rand::thread_rng().gen_range(0..=255),
                    );
                    self.user_colors.insert(chat_message.username.clone(), color);
                }

                self.messages.push(chat_message);
            }
        }
    }

    fn display_connection_settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Host:");
            ui.text_edit_singleline(&mut self.host);
            ui.label("Port:");
            ui.add(egui::DragValue::new(&mut self.port).speed(1));
        });

        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut self.username);
        });

        if !self.is_connected && ui.button("Connect").clicked() {
            self.connect_to_server();
        }
    }

    fn display_messages(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            for message in &self.messages {
                let is_own_message = message.username == self.username;
                let color = if is_own_message {
                    egui::Color32::GOLD
                } else {
                    *self.user_colors.get(&message.username).unwrap()
                };
                ui.horizontal_wrapped(|ui| {
                    ui.colored_label(color, &message.username);
                    ui.label(":");
                    ui.label(&message.content);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            Local::now()
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string(),
                        );
                    });
                });
                ui.end_row();
            }
        });
    }

    fn display_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let available_width = ui.available_width() - ui.spacing().item_spacing.x - ui.spacing().button_padding.x * 2.0 - 50.0;
            let text_edit = egui::TextEdit::multiline(&mut self.input)
                .desired_rows(2)
                .desired_width(available_width);
            ui.add(text_edit);

            if ui.input(|i| i.key_pressed(egui::Key::Enter) && 
                !i.modifiers.shift ) {
                self.send_message();
            }

            if ui.button("Send").clicked() {
                self.send_message();
            }
        });
    }

    fn connect_to_server(&mut self) {
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        let (gui_sender, gui_receiver) = mpsc::unbounded_channel();
        self.message_sender = Some(message_sender);
        self.gui_receiver = Some(gui_receiver);
        self.is_connected = true;

        let host = self.host.clone();
        let port = self.port;
        let username = self.username.clone();

        std::thread::spawn(move || {
            let server_address = format!("ws://{}:{}", host, port);
            connect_to_server(server_address, username, message_receiver, gui_sender)
        });
    }

    fn send_message(&mut self) {
        if !self.input.trim().is_empty() {
            if let Some(message_sender) = &self.message_sender {
                message_sender.send(self.input.clone()).unwrap();
                self.messages.push(ChatMessage {
                    username: self.username.clone(),
                    content: self.input.clone(),
                });
                self.input.clear();
            }
        }
    }
}
