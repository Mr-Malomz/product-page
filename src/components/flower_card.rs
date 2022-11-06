use yew::prelude::*;

use crate::models::product::Flower;

#[derive(Properties, PartialEq)]
pub struct FlowerCardProp {
    pub flower: Flower,
}

#[function_component(FlowerCard)]
pub fn flower_card(FlowerCardProp { flower }: &FlowerCardProp) -> Html {
    let name = flower.parameters.name.value.clone();
    let image_url = flower.parameters.img.value[0].url.clone();
    let image_alt = flower.parameters.img.value[0].alt.clone();

    html! {
        <div class="col-md-6 col-lg-4 col-xl-3 mb-5">
            <div class="card" style="width: 18rem;">
                <img src={image_url} class="card-img-top" alt={image_alt} />
                <div class="card-body">
                    <h5 class="card-title">{name}</h5>
                </div>
            </div>
        </div>
    }
}
