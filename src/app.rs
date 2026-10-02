use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment, components::{Outlet, ParentRoute, Route, Router, Routes},
};

use crate::panic::{EmployeeProvider, PanickyThing};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/local-resource-panic.css"/>

        // sets the document title
        <Title text="🌊"/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.">
                <ParentRoute path=StaticSegment("") view=RouteShell>
                    <Route path=StaticSegment("/") view=HomePage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn RouteShell() -> impl IntoView {
    view! {
        <EmployeeProvider>
            <ShowLet some=move || { use_context::<String>() } let:employee>
                "Hello " {employee}
            </ShowLet>
            <Outlet/>
        </EmployeeProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <PanickyThing />
    }
}
