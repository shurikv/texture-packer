struct Size(u16, u16);

struct Position(u16, u16);

pub struct Sprite<'a> {
    name: &'a str,
    original_size: Size,
    trimmed_size: Option<Size>,
    bytes: Option<&'a [u8]>,
}

pub struct AtlasCell<'a> {
    sprite: Sprite<'a>,
    position: Position,
}

pub struct Atlas<'a> {
    size: Size,
    sprites: Vec<AtlasCell<'a>>,
}