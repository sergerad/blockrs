use std::time::Duration;

use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::Rect,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc,
    time::{interval, Instant},
};
use tracing::{debug, info};

use crate::{
    action::Action,
    components::{acclist::AccList, error::Error, head::Head, txlist::TxList, Component},
    config::Config,
    monitor::ChainMonitor,
    providers::ChainProvider,
    tui::{Event, Tui},
};

pub struct App<P> {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    should_suspend: bool,
    setting: Setting,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    monitor: Option<ChainMonitor<P>>,
    error_timestamp: Instant,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Setting {
    #[default]
    Default,
}

impl<P: ChainProvider + Send + Sync + 'static> App<P> {
    pub fn new(tick_rate: f64, frame_rate: f64, provider: P, config: Config) -> Result<Self> {
        let mut monitor = ChainMonitor::new(provider);
        let (block_rx, transaction_rx, account_rx) = monitor.receivers();
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            tick_rate,
            frame_rate,
            config,
            components: vec![
                Box::new(Head::new(block_rx)),
                Box::new(AccList::new(account_rx)),
                Box::new(TxList::new(transaction_rx)),
            ],
            should_quit: false,
            should_suspend: false,
            setting: Setting::Default,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            monitor: monitor.into(),
            error_timestamp: Instant::now(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        // Run chain monitor loop.
        let mut monitor = self.monitor.take().unwrap();
        let tick_rate = self.config.app.tick_rate;
        let provider_action_tx = self.action_tx.clone();
        tokio::task::spawn(async move {
            let mut tick_interval = interval(tick_rate);
            loop {
                tick_interval.tick().await;
                if let Err(e) = monitor.run().await {
                    provider_action_tx
                        .send(Action::Error(e.to_string()))
                        .unwrap();
                }
            }
        });

        // Run main app loop.
        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.setting) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                    if self.is_error()
                        && Instant::now().duration_since(self.error_timestamp)
                            > Duration::from_secs(4)
                    {
                        self.components.pop();
                    }
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::Error(ref err) => {
                    self.error_timestamp = Instant::now();
                    if self.is_error() {
                        self.components[3] = Box::new(Error::new(err.to_string()));
                    } else {
                        self.components.push(Box::new(Error::new(err.to_string())));
                    }
                }
                _ => {}
            }
            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    self.action_tx.send(action)?
                };
            }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(frame.area());

            let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(outer_layout[0]);

            let mut areas = vec![inner_layout[0], inner_layout[1], outer_layout[1]];
            if self.is_error() {
                areas.push(Self::popup_area(frame.area(), 50, 30));
            }
            for (i, component) in self.components.iter_mut().enumerate() {
                if let Err(err) = component.draw(frame, areas[i]) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;
        Ok(())
    }

    /// Returns true if the app is in an error state.
    fn is_error(&self) -> bool {
        self.components.len() == 4
    }

    /// Returns a centered rect using up certain percentage of the available rect `r`.
    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}
