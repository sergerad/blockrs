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

/// Contains all the data and logic required to respond to user inputs into the app.
///
/// UI content is managed as a list of lists. One list is pointed to as being active
/// at any one point in time.
#[derive(Default)]
pub struct Interactive<T> {
    /// Lists of data elements required by the UI.
    pub elems: VecDeque<Vec<T>>,
    /// Means of receiving new lists of elements for the UI.
    pub elems_rx: Option<Receiver<T>>,
    /// Index into the currently active list of elements.
    pub index: usize,
    /// Whether this component is expected to be updating (follow) or not.
    pub mode: Mode,
}

impl<T> Interactive<T> {
    /// Get the list of elements that are currently active for the UI.
    pub fn get(&self) -> Option<&Vec<T>> {
        self.elems.get(self.index)
    }

    /// Respond to user actions and app ticks.
    pub fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Down => {
                // Enter interactive mode and move down the list of elements.
                self.mode = Mode::Interactive;
                self.index = self
                    .index
                    .saturating_add(1)
                    .min(self.elems.len().saturating_sub(1));
            }
            Action::Up => {
                // Enter interactive mode and move up the list of elements.
                self.mode = Mode::Interactive;
                self.index = self.index.saturating_sub(1);
            }
            Action::Follow => {
                // Enter follow mode and point to newest list of elements.
                self.mode = Mode::Follow;
                self.index = 0usize;
            }
            Action::Tick => {
                // Update elements list during follow mode.
                if matches!(self.mode, Mode::Follow) {
                    if let Ok(elems) = self.elems_rx.as_mut().unwrap().try_recv() {
                        self.elems.push_front(elems);
                        // Pop the oldest element out of the list.
                        if self.elems.len() > LIMIT {
                            self.elems.pop_back();
                        }
                    }
                }
            }
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }
}
