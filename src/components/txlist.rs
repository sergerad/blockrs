use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::Component;
use crate::{action::Action, config::Config, types::Transaction};

#[derive(Default)]
pub struct TxList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    transaction_rx: Option<UnboundedReceiver<Vec<Transaction>>>,
    transactions: Vec<Transaction>,
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
                    self.transactions = transactions;
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
        //let messages: Vec<ListItem> = self
        //    .transactions
        //    .iter()
        //    .map(|tx| {
        //        let content = Line::from(Span::raw(tx.to_string()));
        //        ListItem::new(content)
        //    })
        //    .collect();
        //let messages =
        //    List::new(messages).block(ratatui::widgets::Block::bordered().title("Transactions"));
        //frame.render_widget(messages, area);

        //let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
        let mut rows = Vec::new();
        for tx in &self.transactions {
            rows.push(Row::new(vec![
                tx.hash.to_full_string(),
                tx.from.to_string(),
                tx.to.to_string(),
                tx.value.to_string(),
            ]));
        }
        // Columns widths are constrained in the same way as Layout...
        let widths = [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ];
        let table = Table::new(rows, widths)
            // ...and they can be separated by a fixed spacing.
            .column_spacing(2)
            // You can set the style of the entire Table.
            .style(Style::new().blue())
            // It has an optional header, which is simply a Row always visible at the top.
            .header(
                Row::new(vec!["hash", "from", "to", "value"])
                    .style(Style::new().bold())
                    // To add space between the header and the rest of the rows, specify the margin
                    .bottom_margin(1),
            )
            // It has an optional footer, which is simply a Row always visible at the bottom.
            //.footer(Row::new(vec!["blockies"]))
            // As any other widget, a Table can be wrapped in a Block.
            .block(Block::bordered().title("Transactions"))
            // The selected row, column, cell and its content can also be styled.
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(">>");
        frame.render_widget(table, area);
        Ok(())
    }
}
