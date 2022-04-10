mod evolve;
mod game;

pub(crate) use game::eval_model;
pub use evolve::evolve;

#[derive(Copy, Clone, Debug, Default)]
pub struct Model(u16);
// pub struct Model([u8; 16]);
