struct Item {
    name: String,
    ascii_icon: char,
    tags: Vec<Tags>,
}

pub enum Tags {
    Solid,
    Ingredient,
    Tool,
    Metal,
    Organic,
    Magical,
    Mundane,
    Large,
    Small,
}
