use ratatui::{crossterm::event::KeyEvent, Frame};
use servers::{ServerModuleData, ServerModules};
use users::{UserModuleData, UserModules};

use crate::app::App;

pub mod servers;
pub mod users;

pub enum SelectedModule {
    ServerModule(ServerModules),
    UserModule(UserModules),
}

#[derive(Default)]
pub struct ModuleData {
    pub server_module: ServerModuleData,
    user_module: UserModuleData
}

pub trait RenderableModule {
    fn input_handling(app: &mut App, key: KeyEvent) -> ();
    fn render(app: &mut App, frame: &mut Frame) -> ();
}

pub trait SanitizedForm {
    fn verify_input(&self) -> bool;
    fn update_validity(&mut self) -> ();
    fn clear_form(&mut self) -> ();
}

#[derive(Clone)]
pub struct TextInput {
    pub text: String,
    pub index: usize,
    pub valid: bool
}

impl TextInput {
    fn new(text: String) -> Self {
        Self {
            text,
            index: 0,
            valid: false,
        } 
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: Default::default(),
            index: Default::default(),
            valid: false
        }
    }
}

pub mod rendering {
    use ratatui::{layout::{Constraint, Flex, Layout, Rect}, style::{Color, Style}, widgets::{Block, Paragraph}};

    pub fn popup_area_percent(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub fn popup_area_lengths(area: Rect, length_x: u16, length_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Length(length_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(length_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub fn single_line_input<'a>(content: &'a str, title: &'a str, valid: bool) -> Paragraph<'a> {
        Paragraph::new(content).block(Block::bordered().title(title)).style(Style::new().fg(if valid { Color::White } else { Color::Red }))
    }
}

pub mod input_functions {
    use crate::modules::TextInput;

    pub fn move_cursor_left(input: &mut TextInput) {
        let cursor_moved_left = input.index.saturating_sub(1);
        input.index = clamp_cursor(&mut input.text, cursor_moved_left);
    }

    pub fn move_cursor_right(input: &mut TextInput) {
        let cursor_moved_right = input.index.saturating_add(1);
        input.index = clamp_cursor(&mut input.text, cursor_moved_right);
    }

    pub fn enter_char(input: &mut TextInput, new_char: char) {
        let index = byte_index(input);
        input.text.insert(index, new_char);
        move_cursor_right(input);
    }

    fn byte_index(input: &mut TextInput) -> usize {
        input.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(input.index)
            .unwrap_or(input.text.len())
    }

    pub fn delete_char(input: &mut TextInput) {
        let is_not_cursor_leftmost = input.index != 0;
        if is_not_cursor_leftmost {

            let current_index = input.index.clone();
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = input.text.chars().take(from_left_to_current_index);
            let after_char_to_delete = input.text.chars().skip(current_index);

            input.text = before_char_to_delete.chain(after_char_to_delete).collect();
            move_cursor_left(input);
        }
    }

    fn clamp_cursor(input_string: &mut String, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, input_string.chars().count())
    }

    fn reset_cursor(input_index: &mut usize) {
        *input_index = 0;
    }
}
