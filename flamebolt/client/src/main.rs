use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{
    Calendar, Create, Dashboard, Home, PageNotFound, Profile, SideBar, TodoLists,
};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/create")]
    Create,
    #[at("/todo-lists")]
    TodoLists,
    #[at("/calendar")]
    Calendar,
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Create => html! { <Create /> },
        Route::TodoLists => html! { <TodoLists /> },
        Route::Calendar => html! { <Calendar /> },
        Route::Profile => html! { <Profile /> },
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
