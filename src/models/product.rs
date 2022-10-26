use serde::Deserialize;

//modeling image data
#[derive(Clone, Deserialize, PartialEq)]
pub struct ImageData {
    pub alt: String,
    pub url: String,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Value {
    pub value: Vec<ImageData>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Img {
    pub value: Value,
}

//modelling image text
#[derive(Clone, Deserialize, PartialEq)]
pub struct Name {
    pub name: String,
}

//putting it all together
#[derive(Clone, Deserialize, PartialEq)]
pub struct Flower {
    pub img: Img,
    pub name: Name,
}

//List of flowers
#[derive(Clone, Deserialize, PartialEq)]
pub struct Flowers {
    pub flower: Vec<Flower>,
}
