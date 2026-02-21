use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{
    Dashboard, PageNotFound, Pool, Projects, SideBar, Server, Workload,
};

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
    html! {
        <BrowserRouter>
            <div class="app-layout">
                <SideBar />
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
