use std::collections::VecDeque;

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{
    action::Action,
    app::Mode,
    config::Config,
    types::{Abridged, Transaction},
};

#[derive(Default)]
pub struct TxList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    transaction_rx: Option<UnboundedReceiver<Vec<Transaction>>>,
    transactions: VecDeque<Vec<Transaction>>,
    transactions_idx: usize,
    value_column_name: String,
    mode: Mode,
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
            Action::Down => {
                self.mode = Mode::Interactive;
                self.transactions_idx = self
                    .transactions_idx
                    .saturating_add(1)
                    .min(self.transactions.len().saturating_sub(1));
            }
            Action::Up => {
                self.mode = Mode::Interactive;
                self.transactions_idx = self.transactions_idx.saturating_sub(1);
            }
            Action::Follow => {
                self.mode = Mode::Follow;
                self.transactions_idx = 0usize;
            }
            Action::Tick => {
                if matches!(self.mode, Mode::Follow) {
                    if let Ok(transactions) = self.transaction_rx.as_mut().unwrap().try_recv() {
                        if !transactions.is_empty() {
                            self.transactions.push_front(transactions);
                            // TODO: parameterize max
                            if self.transactions.len() > 20 {
                                self.transactions.pop_back();
                            }
                            if self.value_column_name.is_empty() {
                                self.value_column_name =
                                    self.transactions[0][0].unit.to_uppercase();
                            }
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
        let rows = {
            if let Some(transactions) = self.transactions.get(self.transactions_idx) {
                transactions
                    .iter()
                    .map(|tx| {
                        Row::new(vec![
                            tx.hash.clone(),
                            tx.from.abridged(),
                            tx.to.abridged(),
                            tx.value.clone(),
                        ])
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
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
                Row::new(vec!["HASH", "FROM", "TO", &self.value_column_name])
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
