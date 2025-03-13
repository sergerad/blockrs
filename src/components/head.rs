use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use color_eyre::{owo_colors::OwoColorize, Result};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{action::Action, config::Config, types::Block};

#[derive(Default)]
pub struct Head {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    block_rx: Option<UnboundedReceiver<Block>>,
    block: Option<Block>,
}

impl Head {
    pub fn new(block_rx: UnboundedReceiver<Block>) -> Self {
        Self {
            block_rx: block_rx.into(),
            ..Default::default()
        }
    }
}

impl Component for Head {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                if let Ok(block) = self.block_rx.as_mut().unwrap().try_recv() {
                    self.block.replace(block);
                }
            }
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(block) = &self.block {
            let timestamp = UNIX_EPOCH + Duration::from_secs(block.timestamp);
            let datetime = DateTime::<Utc>::from(timestamp);
            let timestamp = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            let header = Row::new(vec![
                block.number.to_string(),
                timestamp,
                block.hash.clone(),
            ]);
            let widths = [
                Constraint::Percentage(5),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
            ];
            let table = Table::new(vec![header], widths)
                .column_spacing(2)
                .style(Style::new().blue())
                //.header(
                //    Row::new(vec!["number", "hash", "time"])
                //        .style(Style::new().bold())
                //        // To add space between the header and the rest of the rows, specify the margin
                //        .bottom_margin(1),
                //)
                // It has an optional footer, which is simply a Row always visible at the bottom.
                //.footer(Row::new(vec!["blockies"]))
                .block(ratatui::widgets::Block::bordered().title("HEAD"))
                .row_highlight_style(Style::new().reversed())
                .column_highlight_style(Style::new().red())
                .cell_highlight_style(Style::new().blue())
                .highlight_symbol(">>");
            frame.render_widget(table, area);
        }
        Ok(())
    }
}
