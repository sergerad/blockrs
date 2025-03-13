use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{
    action::Action,
    config::Config,
    types::{Abridged, Transaction},
};

#[derive(Default)]
pub struct TxList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    transaction_rx: Option<UnboundedReceiver<Vec<Transaction>>>,
    transactions: Vec<Transaction>,
    value_column_name: String,
}

impl TxList {
    pub fn new(transaction_rx: UnboundedReceiver<Vec<Transaction>>) -> Self {
        Self {
            transaction_rx: transaction_rx.into(),
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
        match action {
            Action::Tick => {
                if let Ok(transactions) = self.transaction_rx.as_mut().unwrap().try_recv() {
                    if !transactions.is_empty() {
                        self.transactions = transactions;
                        if self.value_column_name.is_empty() {
                            self.value_column_name = self.transactions[0].unit.to_uppercase();
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
            .transactions
            .iter()
            .map(|tx| {
                Row::new(vec![
                    tx.hash.clone(),
                    tx.from.abridged(),
                    tx.to.abridged(),
                    tx.value.clone(),
                ])
            })
            .collect();
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
                Row::new(vec!["HASH", "FROM", "TO", &self.value_column_name])
                    .style(Style::new().bold().italic()), // To add space between the header and the rest of the rows, specify the margin
                                                          //.bottom_margin(1),
            )
            .block(ratatui::widgets::Block::bordered().title("TRANSACTIONS"))
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");
        frame.render_widget(table, area);
        Ok(())
    }
}
