use yew::{Html, Properties, component, html};

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub toggle: bool,
}

#[component]
pub fn SideBar(props: &SideBarProps) -> Html {
    let _ = props.toggle;
    html! {
        <aside class="sidebar">
            <p>{"Sidebar"}</p>
        </aside>
    }
}
