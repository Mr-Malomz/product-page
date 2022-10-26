use components::header::Header;
use yew::prelude::*;

mod components;
mod models;

#[function_component(App)]
fn app() -> Html {
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

                    </div>
                </div>
            </section>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
