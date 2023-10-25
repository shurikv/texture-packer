use crate::sprite::Atlas;

mod export;

pub trait Export {
    fn export(&self, atlas: Atlas);
}