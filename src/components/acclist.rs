use super::Component;
use crate::components::interactive::Interactive;
use crate::{
    action::Action,
    config::Config,
    types::{Abridged, Account, AccountReceiver},
};
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

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
        // Map the relevant list of accounts to rows.
        let rows = {
            // Get the list of accounts currently pointed to.
            if let Some(accounts) = self.interact.get() {
                // Map accounts to rows.
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

        // Construct the accounts table.
        let widths = [
            Constraint::Min(11),         // Address.
            Constraint::Min(5),          // Units.
            Constraint::Percentage(100), // Balance.
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

        // Render.
        frame.render_widget(table, area);
        Ok(())
    }
}
