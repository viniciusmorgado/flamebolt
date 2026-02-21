use yew::prelude::*;

mod components;
use components::SideBar;

#[component]
fn App() -> Html {
    html! {
        <>
            <main>
                <SideBar toggle={true} />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
