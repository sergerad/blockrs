use crate::action::Action;
use color_eyre::Result;
use std::collections::VecDeque;
use tokio::sync::mpsc::UnboundedReceiver;

/// The maximum number of list elements that can be stored interactive components
/// at any moment in time.
const LIMIT: usize = 1000;

#[derive(Default, Clone, Debug)]
pub enum Mode {
    #[default]
    Follow,
    Interactive,
}

type Receiver<T> = UnboundedReceiver<Vec<T>>;

#[derive(Default)]
pub struct Interactive<T> {
    pub elems_rx: Option<Receiver<T>>,
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
                    if self.elems.len() > LIMIT {
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
