use std::collections::VecDeque;

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{action::Action, config::Config, types::Transaction};

#[derive(Default)]
pub struct TxList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    transaction_rx: Option<UnboundedReceiver<Transaction>>,
    transactions: VecDeque<Transaction>,
}

impl TxList {
    pub fn new(transaction_rx: UnboundedReceiver<Transaction>) -> Self {
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
                while let Ok(block) = self.transaction_rx.as_mut().unwrap().try_recv() {
                    self.transactions.push_back(block);
                    if self.transactions.len() > 30 {
                        self.transactions.pop_front();
                    }
                }
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let messages: Vec<ListItem> = self
            .transactions
            .iter()
            .map(|tx| {
                let content = Line::from(Span::raw(tx.to_string()));
                ListItem::new(content)
            })
            .collect();
        let messages =
            List::new(messages).block(ratatui::widgets::Block::bordered().title("Transactions"));
        frame.render_widget(messages, area);
        Ok(())
    }
}
