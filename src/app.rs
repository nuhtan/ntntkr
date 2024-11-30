use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, EventStream};
use ratatui::{DefaultTerminal, Frame};
use futures::StreamExt;
use reqwest::Client;

use crate::{config::config::Config, modules::{servers::{ServerEntry, ServerModule, ServerModules}, users::UserModule, ModuleData, RenderableModule, SelectedModule}};

pub struct App {
    pub should_quit: bool,
    pub selected_module: SelectedModule,
    pub module_data: ModuleData,
    pub config: Config,
    pub http_client: Client
}

impl App {
    const FPS: f32 = 60.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.config.initialize();
        
        self.module_data.server_module.server_list = self.config.servers.iter().map(|s| ServerEntry::from(s)).collect();

        let period = Duration::from_secs_f32(1.0 / Self::FPS);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event).await,
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.selected_module {
            SelectedModule::ServerModule(_) => ServerModule::render(self, frame),
            SelectedModule::UserModule(_) => UserModule::render(self, frame),
        }
    }

    async fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            match self.selected_module {
                SelectedModule::ServerModule(_) => ServerModule::input_handling(self, *key).await,
                SelectedModule::UserModule(_) => UserModule::input_handling(self, *key).await,
            };
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            module_data: ModuleData::default(),
            selected_module: SelectedModule::ServerModule(ServerModules::ServerList),
            config: Config::default(),
            http_client: reqwest::Client::new(),
        }
    }
}
