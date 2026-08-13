pub type Cel = &'static [&'static str];

pub struct Game {
    pub name: &'static str,
    pub blurb: &'static str,
    pub id: &'static str,
    pub hint: &'static str,
    pub playable: bool,
    pub tiles: bool,
    pub art: &'static [Cel],
}
