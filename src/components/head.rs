use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::{interactive::Interactive, Component};
use crate::{action::Action, config::Config, types::Block};

#[derive(Default)]
pub struct Head {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    interact: Interactive<Block>,
}

impl Head {
    pub fn new(block_rx: UnboundedReceiver<Vec<Block>>) -> Self {
        Self {
            interact: Interactive {
                elems_rx: block_rx.into(),
                limit: 20, // TODO: add config
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
        let rows = {
            if let Some(blocks) = self.interact.get() {
                blocks
                    .iter()
                    .map(|block| {
                        let timestamp = UNIX_EPOCH + Duration::from_secs(block.timestamp);
                        let datetime = DateTime::<Utc>::from(timestamp);
                        let timestamp = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                        Row::new(vec![
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
