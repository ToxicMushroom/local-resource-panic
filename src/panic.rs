use leptos::children::ChildrenFn;
use leptos::context::provide_context;
use leptos::control_flow::{Await, ShowLet};
use leptos::prelude::ElementChild;
use leptos::reactive::traits::Get;
use leptos::server_fn::codec::Cbor;
use leptos::suspense::Transition;
use leptos::tachys::view::any_view::IntoAny;
use leptos::{component, server::LocalResource, server_fn::ServerFnError, IntoView};
use leptos::{server, view};

#[component]
pub fn PanickyThing() -> impl IntoView {
    let quotas = LocalResource::new(move || {
        load_days_left_in_quota_for_year()
    });

    view! {
        <Transition>
            {move || {
                match quotas.get() {
                    Some(_) => {
                        view! {
                            "hello"
                        }.into_any()
                    },
                    None => view! {
                        <div>
                            "heloo2"
                        </div>
                    }.into_any(),
                }
            }}
        </Transition>
    }
}

#[server(endpoint = "load_days_left_in_quota_for_year", input = Cbor, output = Cbor)]
pub async fn load_days_left_in_quota_for_year() -> Result<String, ServerFnError> {
    Err(ServerFnError::ServerError("teapot".to_string()))
}


#[component]
pub fn EmployeeProvider(children: ChildrenFn) -> impl IntoView {
    view! {
        <Await
            future=get_self()
            let:self_resp
        >
            <ShowLet
                some=self_resp.clone().ok()
                fallback=|| "failed fetching employee-info and permissions"
                children=move |employee| {
                    provide_context(employee);
                    view! {
                        {children()}
                    }
                }
            />
        </Await>
    }
}

#[server(endpoint = "get_self", input = Cbor, output = Cbor)]
pub async fn get_self() -> Result<String, ServerFnError> {
    Ok("Merlin".to_string())
}