use ratatui::{layout::{Constraint, Layout}, text, widgets::{Block, Paragraph, TableState}};

use crate::modules::TextInput;

use super::{RenderableModule, SanitizedForm, SelectedModule};

pub struct UserModule {}

pub enum UserModules {
    UserList,
    UserEditing(SelectedInput)
}

#[derive(Clone)]
pub enum SelectedInput {
    Name
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
    name: TextInput
}

impl SanitizedForm for UserEntry {
    fn verify_input(&self) -> bool {
        todo!()
    }

    fn update_validity(&mut self) -> () {
        todo!()
    }

    fn clear_form(&mut self) -> () {
        todo!()
    }
}

impl RenderableModule for UserModule {
    fn input_handling(app: &mut crate::app::App, key: crossterm::event::KeyEvent) -> () {
        todo!()
    }

    fn render(app: &mut crate::app::App, frame: &mut ratatui::Frame) -> () {
        let rects = Layout::vertical([Constraint::Fill(10), Constraint::Length(3)]).split(frame.area());

        // Main list goes here
        
        let footer_contents = text::Line::from("Esc: Exit | Enter: Select User | Arrow Keys: Navigation | c: Create New User | r: Refresh User List");
        let keybinds = Paragraph::new(footer_contents).block(Block::bordered().title("Keybinds")).centered();
        frame.render_widget(keybinds, rects[1]);

        if let SelectedModule::UserModule(UserModules::UserEditing(selected_input)) = &app.selected_module {

        }
    }
}
