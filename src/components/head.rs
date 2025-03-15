use std::{
    collections::VecDeque,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{action::Action, app::Mode, config::Config, types::Block};

#[derive(Default)]
pub struct Head {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    block_rx: Option<UnboundedReceiver<Block>>,
    blocks: VecDeque<Block>,
    block_idx: usize,
    mode: Mode,
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
            Action::Down => {
                self.mode = Mode::Interactive;
                self.block_idx = self
                    .block_idx
                    .saturating_add(1)
                    .min(self.blocks.len().saturating_sub(1));
            }
            Action::Up => {
                self.mode = Mode::Interactive;
                self.block_idx = self.block_idx.saturating_sub(1);
            }
            Action::Follow => {
                self.mode = Mode::Follow;
                self.block_idx = 0usize;
            }
            Action::Tick => {
                if matches!(self.mode, Mode::Follow) {
                    if let Ok(block) = self.block_rx.as_mut().unwrap().try_recv() {
                        self.blocks.push_front(block);
                        // TODO: parameterize max
                        if self.blocks.len() > 20 {
                            self.blocks.pop_back();
                        }
                    }
                }
            }
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let rows: Vec<_> = self
            .blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                let timestamp = UNIX_EPOCH + Duration::from_secs(block.timestamp);
                let datetime = DateTime::<Utc>::from(timestamp);
                let timestamp = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                let row = Row::new(vec![
                    block.number.to_string(),
                    timestamp,
                    block.hash.clone(),
                ]);
                if i == self.block_idx {
                    row.white()
                } else {
                    row.blue()
                }
            })
            .collect();
        let widths = [
            Constraint::Min(10),
            Constraint::Min(20),
            Constraint::Percentage(100),
        ];
        let table = Table::new(rows, widths)
            .column_spacing(2)
            .style(Style::new().blue())
            //.header(
            //    Row::new(vec!["number", "hash", "time"])
            //        .style(Style::new().bold())
            //        // To add space between the header and the rest of the rows, specify the margin
            //        .bottom_margin(1),
            //)
            //.footer(Row::new(vec!["blockies"]))
            .block(ratatui::widgets::Block::bordered().title("HEAD"))
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");
        frame.render_widget(table, area);

        Ok(())
    }
}
