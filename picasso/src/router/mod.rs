use {
    crate::views::{Blog, Home},
    dioxus::prelude::*,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[component]
fn WebNavbar() -> Element {
    rsx! {
        Link { to: Route::Home {}, "outsidev" }

        Link { to: Route::Blog { id: 1 }, "Blog" }

        Outlet::<Route> {}
    }
}
