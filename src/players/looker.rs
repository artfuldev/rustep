use crate::core::{Game, Position};

pub trait Looker: LookerClone {
    fn moves(&mut self, game: &Game) -> Vec<Position>;
}

pub trait LookerClone {
    fn clone_box(&self) -> Box<dyn Looker>;
}

impl<T> LookerClone for T
where
    T: 'static + Looker + Clone,
{
    fn clone_box(&self) -> Box<dyn Looker> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Looker> {
    fn clone(&self) -> Box<dyn Looker> {
        self.clone_box()
    }
}
