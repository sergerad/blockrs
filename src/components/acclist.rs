use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{
    action::Action,
    config::Config,
    types::{Abridged, Account, AccountReceiver},
};

use crate::components::interactive::Interactive;

#[derive(Default)]
pub struct AccList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    interact: Interactive<Account>,
}

impl AccList {
    pub fn new(account_rx: AccountReceiver) -> Self {
        Self {
            interact: Interactive {
                elems_rx: account_rx.into(),
                ..Default::default()
            },
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
        self.interact.update(action)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let rows = {
            if let Some(accounts) = self.interact.get() {
                accounts
                    .iter()
                    .map(|acc| {
                        Row::new(vec![
                            acc.address.abridged(),
                            acc.units.clone(),
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
