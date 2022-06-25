use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct User {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub comments: String,
}

#[derive(Properties, PartialEq)]
struct UserListProps {
    user: Vec<User>,
}

#[function_component(UsersList)]
fn user_list(UserListProps { user }: &UserListProps) -> Html {
    user.iter()
        .map(|user| {
            let user = user.clone();
            html! {
                <p>{format!("{} - {} - {} - {}",
                user.name,
                user.phone,
                user.email,
                user.comments
            )}</p>
            }
        })
        .collect()
}

#[function_component(Get)]
pub fn app() -> Html {
    let user = use_state(|| vec![]);
    {
        let user = user.clone();
        use_effect_with_deps(
            move |_| {
                let user = user.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data: Vec<User> = Request::get("http://0.0.0.0:8000/get_all")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    user.set(fetched_data);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <h2>{ "Get Component" }</h2>
            <div>
                <h3>{"Data"}</h3>
                <UsersList user={(*user).clone()} />
            </div>
        </>
    }
}
