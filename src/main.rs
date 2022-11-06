use components::{flower_card::FlowerCard, header::Header, loader::Loader, message::Message};
use models::product::{RootComposition, Slots};
use reqwest::{header, Client, Error};
use yew::prelude::*;

mod components;
mod models;

#[function_component(App)]
fn app() -> Html {
    let flowers: UseStateHandle<Option<Slots>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        //create copies of the states
        let flowers = flowers.clone();
        let error = error.clone();

        //construct uniform api endpoint
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("x-api-key", "<REPLACE WITH API KEY>".parse().unwrap());

        let url = format!(
            "https://uniform.app/api/v1/canvas?projectId={id}&slug={slug}&state=64",
            id = "<REPLACE WITH PROJECT ID>",
            slug = "/flowers"
        );

        use_effect_with_deps(
            move |_| {
                let client = Client::new();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_flowers = client.get(url).headers(headers).send().await;

                    match fetched_flowers {
                        Ok(response) => {
                            let json = response.json::<RootComposition>().await;
                            match json {
                                Ok(data) => flowers.set(Some(data.composition.slots)),
                                Err(e) => error.set(Some(e)),
                            }
                        }
                        Err(e) => error.set(Some(e)),
                    }
                });
                || ()
            },
            (),
        );
    }

    let flower_list_logic = match flowers.as_ref() {
        Some(flowers) => flowers
            .flowers
            .iter()
            .map(|flower| {
                html! {
                    <FlowerCard flower={flower.clone()}/>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(e) => {
                println!("{}", e);
                html! {
                    <Message text={"Error getting list of users"} css_class={"text-danger"}/>
                }
            }
            None => {
                html! {
                  <Loader />
                }
            }
        },
    };

    html! {
        <>
            <Header />
            <section class="section-products mt-5">
                <div class="container">
                    <div class="row justify-content-center text-center">
                        <div class="col-md-8 col-lg-6">
                            <div class="header">
                                <h5 class="">{"Popular Product List"}</h5>
                            </div>
                        </div>
                    </div>
                    <div class="row">
                        {flower_list_logic}
                    </div>
                </div>
            </section>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
