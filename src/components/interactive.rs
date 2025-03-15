use crate::{action::Action, app::Mode};
use color_eyre::Result;
use std::collections::VecDeque;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Default)]
pub struct Interactive<T> {
    pub limit: usize,
    pub elems_rx: Option<UnboundedReceiver<Vec<T>>>,
    pub elems: VecDeque<Vec<T>>,
    pub index: usize,
    pub mode: Mode,
}

impl<T> Interactive<T> {
    pub fn get(&self) -> Option<&Vec<T>> {
        self.elems.get(self.index)
    }
    pub fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Down => {
                self.mode = Mode::Interactive;
                self.index = self
                    .index
                    .saturating_add(1)
                    .min(self.elems.len().saturating_sub(1));
            }
            Action::Up => {
                self.mode = Mode::Interactive;
                self.index = self.index.saturating_sub(1);
            }
            Action::Follow => {
                self.mode = Mode::Follow;
                self.index = 0usize;
            }
            Action::Tick => {
                if matches!(self.mode, Mode::Follow) {
                    if let Ok(elems) = self.elems_rx.as_mut().unwrap().try_recv() {
                        self.elems.push_front(elems);
                    }
                    if self.elems.len() > self.limit {
                        self.elems.pop_back();
                    }
                }
            }
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }
}
