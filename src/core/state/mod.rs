pub mod page;

pub trait State {
    fn available(&self, next_state: impl State) -> bool;
}
