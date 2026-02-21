use yew::prelude::*;
use yew_router::prelude::*;

use crate::{Route, Theme};

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub theme: Theme,
    pub on_toggle_theme: Callback<()>,
}

#[component]
pub fn SideBar(props: &SideBarProps) -> Html {
    let route = use_route::<Route>();
    let collapsed = use_state_eq(|| false);

    let on_toggle_sidebar = {
        let collapsed = collapsed.clone();
        Callback::from(move |_: MouseEvent| collapsed.set(!*collapsed))
    };

    let nav_class = classes!("sidebar", (*collapsed).then_some("close"));
    let toggle_btn_class = classes!((*collapsed).then_some("rotate"));

    let active_dashboard = route.as_ref() == Some(&Route::Dashboard);
    let active_pool = route.as_ref() == Some(&Route::Pool);
    let active_server = route.as_ref() == Some(&Route::Server);
    let active_projects = route.as_ref() == Some(&Route::Projects);
    let active_workload = route.as_ref() == Some(&Route::Workload);

    html! {
        <nav id="sidebar" class={nav_class}>
            <ul>
                <li>
                    <span class="logo">{"Flamebolt"}</span>
                    <button type="button" id="toggle-btn" class={toggle_btn_class} onclick={on_toggle_sidebar}>
                        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor">
                            <path d="m313-480 155 156q11 11 11.5 27.5T468-268q-11 11-28 11t-28-11L228-452q-6-6-8.5-13t-2.5-15q0-8 2.5-15t8.5-13l184-184q11-11 27.5-11.5T468-692q11 11 11 28t-11 28L313-480Zm264 0 155 156q11 11 11.5 27.5T732-268q-11 11-28 11t-28-11L492-452q-6-6-8.5-13t-2.5-15q0-8 2.5-15t8.5-13l184-184q11-11 27.5-11.5T732-692q11 11 11 28t-11 28L577-480Z"/>
                        </svg>
                    </button>
                </li>
                <li class={classes!(active_dashboard.then_some("active"))}>
                    <Link<Route> to={Route::Dashboard} classes={classes!("sidebar-link")}>
                        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor">
                            <path d="M520-640v-160q0-17 11.5-28.5T560-840h240q17 0 28.5 11.5T840-800v160q0 17-11.5 28.5T800-600H560q-17 0-28.5-11.5T520-640ZM120-480v-320q0-17 11.5-28.5T160-840h240q17 0 28.5 11.5T440-800v320q0 17-11.5 28.5T400-440H160q-17 0-28.5-11.5T120-480Zm400 320v-320q0-17 11.5-28.5T560-520h240q17 0 28.5 11.5T840-480v320q0 17-11.5 28.5T800-120H560q-17 0-28.5-11.5T520-160Zm-400 0v-160q0-17 11.5-28.5T160-360h240q17 0 28.5 11.5T440-320v160q0 17-11.5 28.5T400-120H160q-17 0-28.5-11.5T120-160Zm80-360h160v-240H200v240Zm400 320h160v-240H600v240Zm0-480h160v-80H600v80ZM200-200h160v-80H200v80Zm160-320Zm240-160Zm0 240ZM360-280Z"/>
                        </svg>
                        <span>{"Dashboard"}</span>
                    </Link<Route>>
                </li>
                <li class={classes!(active_pool.then_some("active"))}>
                    <Link<Route> to={Route::Pool} classes={classes!("sidebar-link")}>
                        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24" fill="currentColor">
                            <path d="M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1zM7 19c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zM20 3H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zM7 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z"/>
                        </svg>
                        <span>{"Pool"}</span>
                    </Link<Route>>
                </li>
                <li class={classes!(active_server.then_some("active"))}>
                    <Link<Route> to={Route::Server} classes={classes!("sidebar-link")}>
                        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24" fill="currentColor">
                            <path d="M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z"/>
                        </svg>
                        <span>{"Server"}</span>
                    </Link<Route>>
                </li>
                <li class={classes!(active_projects.then_some("active"))}>
                    <Link<Route> to={Route::Projects} classes={classes!("sidebar-link")}>
                        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24" fill="currentColor">
                            <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                        </svg>
                        <span>{"Projects"}</span>
                    </Link<Route>>
                </li>
                <li class={classes!(active_workload.then_some("active"))}>
                    <Link<Route> to={Route::Workload} classes={classes!("sidebar-link")}>
                        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24" fill="currentColor">
                            <path d="M3 17v2h6v-2H3zM3 5v2h10V5H3zm10 16v-2h8v-2h-8v-2h-2v6h2zM7 9v2H3v2h4v2h2V9H7zm14 4v-2H11v2h10zm-6-4h2V7h4V5h-4V3h-2v6z"/>
                        </svg>
                        <span>{"Workload"}</span>
                    </Link<Route>>
                </li>
            </ul>
            <div class="sidebar-footer">
                <button type="button" class="sidebar-user">
                    <span class="sidebar-user-avatar" aria-hidden="true"></span>
                    <span class="sidebar-user-name">{"User"}</span>
                </button>
                <button type="button" class="theme-toggle-btn" onclick={props.on_toggle_theme.reform(|_| ())} title={if props.theme == Theme::Dark { "Tema claro" } else { "Tema escuro" }}>
                    {if props.theme == Theme::Dark {
                        html! {
                            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" aria-hidden="true">
                                <path d="M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 100-96 58 57-61 140ZM114-304l97-101 56 52-100 96-53-47Zm644 492 61-140-96-100 59-57 140 61ZM480-480Z"/>
                            </svg>
                        }
                    } else {
                        html! {
                            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" aria-hidden="true">
                                <path d="M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q14 0 27.5 1t26.5 3q-41 29-65.5 75.5T418-660q0 90 63 153t153 63q55 0 101-24.5t75-65.5q2 13 3 26.5t1 27.5q0 150-105 255T480-120Zm0-80q88 0 158-48.5T740-375q-20 5-40 8t-40 3q-123 0-209.5-86.5T364-660q0-20 3-40t8-40q-78 32-126.5 102T200-480q0 116 82 198t198 82Zm-10-270Z"/>
                            </svg>
                        }
                    }}
                </button>
            </div>
        </nav>
    }
}
