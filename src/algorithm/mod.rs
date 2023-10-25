mod grid;

use crate::sprite::Sprite;

pub trait PackAlgorithm {
    fn pack(&self, sprites: Vec<Sprite>);
}