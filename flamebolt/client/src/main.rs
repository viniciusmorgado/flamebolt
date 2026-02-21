use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{
    Dashboard, PageNotFound, Pool, Projects, SideBar, Server, Workload,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/pool")]
    Pool,
    #[at("/server")]
    Server,
    #[at("/projects")]
    Projects,
    #[at("/workload")]
    Workload,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <Dashboard /> },
        Route::Pool => html! { <Pool /> },
        Route::Server => html! { <Server /> },
        Route::Projects => html! { <Projects /> },
        Route::Workload => html! { <Workload /> },
        Route::NotFound => html! { <PageNotFound /> },
    }
}
 
#[function_component]
fn App() -> Html {
    let theme = use_state_eq(|| Theme::Dark);
    let on_toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(match *theme {
                Theme::Dark => Theme::Light,
                Theme::Light => Theme::Dark,
            })
        })
    };
    let layout_class = classes!(
        "app-layout",
        if *theme == Theme::Light {
            "theme-light"
        } else {
            "theme-dark"
        }
    );
    html! {
        <BrowserRouter>
            <div class={layout_class}>
                <SideBar theme={*theme} on_toggle_theme={on_toggle_theme} />
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
