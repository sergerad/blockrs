use super::{
    interactive::{Interactive, Mode},
    Component,
};
use crate::{
    action::Action,
    config::Config,
    types::{Block, BlockReceiver},
};
use chrono::{DateTime, Utc};
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, UNIX_EPOCH};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct Head {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    interact: Interactive<Block>,
}

impl Head {
    pub fn new(block_rx: BlockReceiver) -> Self {
        Self {
            interact: Interactive {
                elems_rx: block_rx.into(),
                ..Default::default()
            },
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
        self.interact.update(action)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Map the latest head block to a row.
        let rows = {
            // Get the list of blocks (always single block).
            if let Some(blocks) = self.interact.get() {
                // Map blocks to rows.
                blocks
                    .iter()
                    .map(|block| {
                        // Construct up/down indicator based on currently-highlighted
                        // block's position in the list of all blocks observed so far.
                        let indicator = match self.interact.index {
                            _i if self.interact.elems.len() == 1 => "           ",
                            i if i > 0 && i < self.interact.elems.len() - 1 => "▲ NEXT ▼ PREV",
                            i if i == 0 && self.interact.elems.len() > 1 => "     ▼ PREV",
                            i if i == self.interact.elems.len() - 1 => "▲ NEXT       ",
                            _ => "           ",
                        };
                        let timestamp = UNIX_EPOCH + Duration::from_secs(block.timestamp);
                        let datetime = DateTime::<Utc>::from(timestamp);
                        let timestamp = datetime.format("%H:%M:%S").to_string();
                        Row::new(vec![
                            indicator.to_string(),
                            block.number.to_string(),
                            timestamp,
                            block.hash.clone(),
                        ])
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        };

        // Construct the table.
        let widths = [
            Constraint::Min(14),         // Indicator.
            Constraint::Min(10),         // Block number.
            Constraint::Min(10),         // Timestamp.
            Constraint::Percentage(100), // Hash.
        ];
        let title = match self.interact.mode {
            Mode::Follow => "HEAD",
            Mode::Interactive => "PAUSED",
        };
        let table = Table::new(rows, widths)
            .column_spacing(2)
            .style(Style::new().blue())
            .block(ratatui::widgets::Block::bordered().title(title))
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");

        // Render.
        frame.render_widget(table, area);
        Ok(())
    }
}
