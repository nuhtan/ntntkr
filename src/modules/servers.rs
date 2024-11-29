use crossterm::event::KeyCode;
use ratatui::{crossterm::event, layout::{Constraint, Layout, Position}, style::{Style, Stylize}, text, widgets::{Block, Paragraph, Row, Table, TableState}};
use regex::Regex;

use crate::{app::App, config::config::Server};
use crate::modules::TextInput;

use super::{input_functions, rendering::{self, single_line_input}, users::UserModules, RenderableModule, SanitizedForm, SelectedModule};

pub enum ServerModules {
    ServerList,
    ServerEditing(SelectedInput)
}

#[derive(Clone)]
pub enum SelectedInput {
    Name,
    Address,
    Port
}

impl Default for SelectedInput {
    fn default() -> Self {
        SelectedInput::Name
    }
}

impl SelectedInput {
    fn next(self) -> Self {
        match self {
            SelectedInput::Name => SelectedInput::Address,
            SelectedInput::Address => SelectedInput::Port,
            SelectedInput::Port => SelectedInput::Name,
        }
    }

    fn prev(self) -> Self {
        match self {
            SelectedInput::Name => SelectedInput::Port,
            SelectedInput::Address => SelectedInput::Name,
            SelectedInput::Port => SelectedInput::Address,
        }
    }
}

#[derive(Default)]
pub struct ServerModuleData {
    pub server_list: Vec<ServerEntry>,
    servers_table_state: TableState,
    selected_server: ServerEntry,
    editable_server_data: ServerEntry,
}

#[derive(Clone, Default)]
pub struct ServerEntry {
    name: TextInput,
    address: TextInput,
    port: TextInput
}

impl From<&Server> for ServerEntry {
    fn from(value: &Server) -> Self {
        Self {
            name: TextInput::new(value.name.clone()),
            address: TextInput::new(value.address.clone()),
            port: TextInput::new(value.port.clone()),
        }
    }
}

impl SanitizedForm for ServerEntry {
    fn verify_input(&self) -> bool {
        self.name.valid && self.address.valid && self.port.valid
    }

    fn update_validity(&mut self) -> () {
        let name_regex = Regex::new(r".+").unwrap();
        let addr_regex = Regex::new(r"([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})|(localhost)").unwrap();
        let port_regex = Regex::new(r"[0-9]{1,5}").unwrap();
        
        self.name.valid = name_regex.is_match(&self.name.text);
        self.address.valid = addr_regex.is_match(&self.address.text);
        self.port.valid = port_regex.is_match(&self.port.text);
    }

    fn clear_form(&mut self) -> () {
        *self = ServerEntry::default();
    }
}

pub struct ServerModule {}

impl RenderableModule for ServerModule {
    fn input_handling(app: &mut App, key: event::KeyEvent) -> () {
        if let SelectedModule::ServerModule(module_status) = &app.selected_module {
            let editable_data = &mut app.module_data.server_module.editable_server_data;
            match module_status {
                ServerModules::ServerList => {
                    match key.code {
                        KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('c') => app.selected_module = SelectedModule::ServerModule(ServerModules::ServerEditing(SelectedInput::default())),
                        KeyCode::Enter => {
                            app.module_data.server_module.selected_server = app.module_data.server_module.server_list[app.module_data.server_module.servers_table_state.selected().unwrap()].clone();
                            app.selected_module = SelectedModule::UserModule(UserModules::UserList);
                        },
                        KeyCode::Up => app.module_data.server_module.servers_table_state.select_previous(),
                        KeyCode::Down => app.module_data.server_module.servers_table_state.select_next(),
                        _ => {}
                    }
                },
                ServerModules::ServerEditing(selected_input) => {

                    let input = match selected_input {
                        SelectedInput::Name => &mut editable_data.name,
                        SelectedInput::Address => &mut editable_data.address,
                        SelectedInput::Port => &mut editable_data.port,
                    };
                    
                    match key.code {
                        KeyCode::Esc => app.selected_module = SelectedModule::ServerModule(ServerModules::ServerList),
                        KeyCode::Up => {
                            if let SelectedModule::ServerModule(ServerModules::ServerEditing(inputs)) = &app.selected_module {
                                app.selected_module = SelectedModule::ServerModule(ServerModules::ServerEditing(inputs.clone().prev()));
                            }
                        },
                        KeyCode::Down => {
                            if let SelectedModule::ServerModule(ServerModules::ServerEditing(inputs)) = &app.selected_module {
                                app.selected_module = SelectedModule::ServerModule(ServerModules::ServerEditing(inputs.clone().next()));
                            }
                        },
                        KeyCode::Backspace => input_functions::delete_char(input),
                        KeyCode::Left => input_functions::move_cursor_left(input),
                        KeyCode::Right => input_functions::move_cursor_right(input),
                        KeyCode::Char(c) => input_functions::enter_char(input, c),
                        KeyCode::Enter => {
                            if editable_data.verify_input() {
                                // Add to config file
                                let config_server = Server {
                                    name: editable_data.name.text.clone(),
                                    address: editable_data.address.text.clone(),
                                    port: editable_data.port.text.clone(),
                                };
                                app.config.add_new_server(config_server);

                                // update app's server list
                                app.module_data.server_module.server_list.push(editable_data.clone());
                                
                                // clear form fields
                                editable_data.clear_form();

                                // Change state to now go back to the server list
                                app.selected_module = SelectedModule::ServerModule(ServerModules::ServerList);
                            }
                        }
                        _ => {},
                    }
                    editable_data.update_validity();
                },
            }
        }
    }

    fn render(app: &mut App, frame: &mut ratatui::Frame) -> () {
        let rects = Layout::vertical([Constraint::Fill(10), Constraint::Length(3)]).split(frame.area());
        
        let rows: Vec<Row> = app.module_data.server_module.server_list.iter().map(|f| Row::new(vec![f.name.text.clone(), f.address.text.clone(), f.port.text.clone()])).collect();
        let widths = [Constraint::Length(15), Constraint::Length(15), Constraint::Length(5)];

        let servers_table = Table::new(rows, widths).block(Block::bordered().title("Select Server")).row_highlight_style(Style::new().reversed()).highlight_symbol(">>");

        frame.render_stateful_widget(servers_table, rects[0], &mut app.module_data.server_module.servers_table_state);

        if app.module_data.server_module.server_list.len() > 0 && app.module_data.server_module.servers_table_state.selected().is_none() {
            app.module_data.server_module.servers_table_state.select_next();
        }

        let footer_contents = text::Line::from("Esc: Exit | Enter: Select Server | Arrow Keys: Navigation | c: Create New Server Entry");
        let keybinds = Paragraph::new(footer_contents).block(Block::bordered().title("Keybinds")).centered();
        frame.render_widget(keybinds, rects[1]);
        
        if let SelectedModule::ServerModule(ServerModules::ServerEditing(selected_input)) = &app.selected_module {
            let area = rendering::popup_area_lengths(frame.area(), 37, 12);
                    
            let rects = Layout::vertical([Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)]).split(area);
            
            let editable_data = &app.module_data.server_module.editable_server_data;

            let input_name = single_line_input(editable_data.name.text.as_str(), "Server Name", editable_data.name.valid);
                    frame.render_widget(input_name, rects[0]);

            let input_address = single_line_input(editable_data.address.text.as_str(), "Server Address", editable_data.address.valid);
                    frame.render_widget(input_address, rects[1]);

            let input_port = single_line_input(editable_data.port.text.as_str(), "Server Port", editable_data.port.valid);
                    frame.render_widget(input_port, rects[2]);

            let (rect, index) = match selected_input {
                SelectedInput::Name => (0, editable_data.name.index as u16),
                SelectedInput::Address => (1, editable_data.address.index as u16),
                SelectedInput::Port => (2, editable_data.port.index as u16),
            };
            frame.set_cursor_position(Position::new(rects[rect].x + index + 1, rects[rect].y + 1));

            let footer_contents = text::Line::from("Esc: Back | Enter: Confirm Server");
            let keybinds = Paragraph::new(footer_contents).block(Block::bordered().title("Keybinds")).centered();
            frame.render_widget(keybinds, rects[3]);
            
        }
    }
}
