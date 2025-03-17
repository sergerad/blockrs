use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{interactive::Interactive, Component};
use crate::{
    action::Action,
    config::Config,
    types::{Abridged, Transaction, TransactionReceiver},
};

#[derive(Default)]
pub struct TxList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    interact: Interactive<Transaction>,
}

impl TxList {
    pub fn new(transactions_rx: TransactionReceiver) -> Self {
        Self {
            interact: Interactive {
                elems_rx: transactions_rx.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Component for TxList {
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
        let (rows, value_col_name) = {
            if let Some(transactions) = self.interact.get() {
                let rows = transactions
                    .iter()
                    .map(|tx| {
                        Row::new(vec![
                            tx.hash.clone(),
                            tx.from.abridged(),
                            tx.to.abridged(),
                            tx.value.clone(),
                        ])
                    })
                    .collect::<Vec<_>>();
                (
                    rows,
                    transactions
                        .first()
                        .map(|tx| tx.units.clone().to_uppercase())
                        .unwrap_or_default(),
                )
            } else {
                (Vec::new(), "".to_string())
            }
        };
        let widths = [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ];
        let table = Table::new(rows, widths)
            .column_spacing(2)
            .style(Style::new().blue())
            .header(
                Row::new(vec!["HASH", "FROM", "TO", value_col_name.as_str()])
                    .style(Style::new().bold().italic()),
            )
            .block(
                ratatui::widgets::Block::bordered()
                    .title_bottom("TRANSACTIONS")
                    .title_alignment(Alignment::Center),
            )
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");
        frame.render_widget(table, area);
        Ok(())
    }
}
