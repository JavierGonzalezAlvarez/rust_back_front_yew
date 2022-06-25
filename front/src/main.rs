use yew::prelude::*;

mod form;
use form::Form;

mod get;
use get::Get;

enum Msg {
    Save,
    Cancel,
}

#[derive(Clone, PartialEq)]
struct MyComponent {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub comments: String,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: String::from(""),
            phone: String::from(""),
            email: String::from(""),
            comments: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Save => {
                //some action
                true
            }
            Msg::Cancel => {
                //some action
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let header = "Main Component - Back and Front in Rust (Activeweb & Yew)";

        html! {
            <main>
                <div>
                    <h1 class="red">{ header }</h1>
                </div>
                <div>
                    <Form />
                </div>
                <div>
                    <button onclick ={link.callback(|_| Msg::Save)}>{ "Save" }</button>
                    <button onclick ={link.callback(|_| Msg::Cancel)}>{ "Cancel" }</button>
                </div>
                <div>
                    <Get />
                </div>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<MyComponent>();
}
