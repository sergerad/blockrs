use std::collections::{HashMap, VecDeque};

use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{
    action::Action,
    app::Mode,
    config::Config,
    types::{Abridged, Account},
};

#[derive(Default)]
pub struct AccList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    account_rx: Option<UnboundedReceiver<Vec<Account>>>,
    accounts: VecDeque<Vec<Account>>,
    accounts_idx: usize,
    mode: Mode,
}

impl AccList {
    pub fn new(account_rx: UnboundedReceiver<Vec<Account>>) -> Self {
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
            Action::Down => {
                self.mode = Mode::Interactive;
                self.accounts_idx = self
                    .accounts_idx
                    .saturating_add(1)
                    .min(self.accounts.len().saturating_sub(1));
            }
            Action::Up => {
                self.mode = Mode::Interactive;
                self.accounts_idx = self.accounts_idx.saturating_sub(1);
            }
            Action::Follow => {
                self.mode = Mode::Follow;
                self.accounts_idx = 0usize;
            }
            Action::Tick => {
                if matches!(self.mode, Mode::Follow) {
                    if let Ok(accounts) = self.account_rx.as_mut().unwrap().try_recv() {
                        self.accounts.push_front(accounts);
                    }
                    // TODO: parameterize max
                    if self.accounts.len() > 20 {
                        self.accounts.pop_back();
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
            if let Some(accounts) = self.accounts.get(self.accounts_idx) {
                accounts
                    .iter()
                    .map(|acc| {
                        Row::new(vec![
                            acc.address.abridged(),
                            acc.unit.clone(),
                            acc.balance.clone(),
                        ])
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        };
        let widths = [
            Constraint::Min(11),
            Constraint::Min(5),
            Constraint::Percentage(100),
        ];
        let table = Table::new(rows, widths)
            .column_spacing(2)
            .style(Style::new().green())
            //.header(
            //    Row::new(vec!["addr", "balance", "units"])
            //        .style(Style::new().bold())
            //        // To add space between the header and the rest of the rows, specify the margin
            //        .bottom_margin(1),
            //)
            .block(
                ratatui::widgets::Block::bordered()
                    .title("BALANCES")
                    .title_alignment(Alignment::Right),
            )
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");
        frame.render_widget(table, area);
        Ok(())
    }
}
