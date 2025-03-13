use std::collections::HashMap;

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{action::Action, config::Config, types::Account};

#[derive(Default)]
pub struct AccList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    account_rx: Option<UnboundedReceiver<Account>>,
    accounts: HashMap<String, Account>,
}

impl AccList {
    pub fn new(account_rx: UnboundedReceiver<Account>) -> Self {
        Self {
            account_rx: account_rx.into(),
            ..Default::default()
        }
    }
}

impl Component for AccList {
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
                while let Ok(account) = self.account_rx.as_mut().unwrap().try_recv() {
                    self.accounts.insert(account.address.to_string(), account);
                }
            }
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let messages: Vec<ListItem> = self
            .accounts
            .values()
            .map(|acc| {
                let content = Line::from(Span::raw(acc.to_string()));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages)
            .block(ratatui::widgets::Block::bordered().title("Balances"))
            .style(Style::new().green().italic());
        frame.render_widget(messages, area);
        Ok(())
    }
}
