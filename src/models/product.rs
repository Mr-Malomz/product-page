use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq)]
pub struct RootComposition {
    pub composition: Composition,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Composition {
    pub slots: Slots,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Slots {
    pub flowers: Vec<Flower>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Flower {
    pub parameters: Parameters,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Parameters {
    pub img: Img,
    pub name: Name,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Img {
    pub value: Vec<Value>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Value {
    pub alt: String,
    pub url: String,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Name {
    pub value: String,
}