use crossterm::event::KeyCode;
use ratatui::{layout::{Constraint, Layout, Position}, text, widgets::{Block, Paragraph, TableState}};
use regex::Regex;

use crate::modules::TextInput;

use super::{input_functions, rendering::{self, single_line_input}, RenderableModule, SanitizedForm, SelectedModule};

pub struct UserModule {}

pub enum UserModules {
    UserList,
    UserEditing(SelectedInput)
}

#[derive(Clone)]
pub enum SelectedInput {
    Name
}

impl Default for SelectedInput {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Default)]
pub struct UserModuleData {
    user_list: Vec<String>,
    user_table_state: TableState,
    editable_user_data: UserEntry,
    selected_user: UserEntry
}

#[derive(Default)]
struct UserEntry {
    pub name: TextInput
}

impl SanitizedForm for UserEntry {
    fn verify_input(&self) -> bool {
       self.name.valid 
    }

    fn update_validity(&mut self) -> () {
        let name_regex = Regex::new(r".+").unwrap();
        self.name.valid = name_regex.is_match(&self.name.text);
    }

    fn clear_form(&mut self) -> () {
        *self = Self::default();
    }
}

impl RenderableModule for UserModule {
    async fn input_handling(app: &mut crate::app::App, key: crossterm::event::KeyEvent) -> () {
        if let SelectedModule::UserModule(module_status) = &app.selected_module {
            let editable_data = &mut app.module_data.user_module.editable_user_data;
            match module_status {
                UserModules::UserList => {
                    match key.code {
                        KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('c') => app.selected_module = SelectedModule::UserModule(UserModules::UserEditing(SelectedInput::default())),
                        _ => {}
                    }
                },
                UserModules::UserEditing(selected_input) => {
                    let input = match selected_input {
                        SelectedInput::Name => &mut editable_data.name,
                    };

                    match key.code {
                        KeyCode::Esc => app.selected_module = SelectedModule::UserModule(UserModules::UserList),
                        KeyCode::Backspace => input_functions::delete_char(input),
                        KeyCode::Left => input_functions::move_cursor_left(input),
                        KeyCode::Right => input_functions::move_cursor_right(input),
                        KeyCode::Char(c) => input_functions::enter_char(input, c),
                        KeyCode::Enter => {
                            if editable_data.verify_input() {
                                // Send to server
                                let url = format!("http://{}:{}/api/users", app.module_data.server_module.selected_server.address.text, app.module_data.server_module.selected_server.port.text);
                                server_functions::send_new_user(url, &app.http_client, editable_data).await;
                                // Clear form fields
                                editable_data.clear_form();
                                // Refresh User List
                                // Change state back to list
                                app.selected_module = SelectedModule::UserModule(UserModules::UserList)
                            }
                        }
                        _ => {}
                    }
                    editable_data.update_validity();
                },
            }
        }
    }

    fn render(app: &mut crate::app::App, frame: &mut ratatui::Frame) -> () {
        let rects = Layout::vertical([Constraint::Fill(10), Constraint::Length(3)]).split(frame.area());

        // Main list goes here
        
        let footer_contents = text::Line::from("Esc: Exit | Enter: Select User | Arrow Keys: Navigation | c: Create New User | r: Refresh User List");
        let keybinds = Paragraph::new(footer_contents).block(Block::bordered().title("Keybinds")).centered();
        frame.render_widget(keybinds, rects[1]);

        if let SelectedModule::UserModule(UserModules::UserEditing(_selected_input)) = &app.selected_module {
            let area = rendering::popup_area_lengths(frame.area(), 37, 6);

            let rects = Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).split(area);
            
            let editable_data = &app.module_data.user_module.editable_user_data;

            let input_name = single_line_input(editable_data.name.text.as_str(), "User Name", editable_data.name.valid);
            frame.render_widget(input_name, rects[0]);

            frame.set_cursor_position(Position::new(rects[0].x + editable_data.name.index as u16 + 1, rects[0].y + 1));

            let footer_contents = text::Line::from("Esc: Back | Enter: Confirm User");
            let keybinds = Paragraph::new(footer_contents).block(Block::bordered().title("Keybinds")).centered();
            frame.render_widget(keybinds, rects[1]);
        }
    }
}

mod server_functions {
    use super::UserEntry;

    pub async fn send_new_user(url: String, http_client: &reqwest::Client, user: &mut UserEntry) {
        let params = [("name".to_string(), user.name.text.clone())];
        http_client.post(url).form(&params).send().await.unwrap();
    }

    pub async fn get_user_list(url: String, http_client: &reqwest::Client) {
        let users = http_client.get(url).await
    }
}
