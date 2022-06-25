use yew::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;

#[derive(Clone, PartialEq, Properties)]
pub struct PropsName {
    pub value: String,
    pub on_change_name: Callback<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PropsEmail {
    pub value: String,
    pub on_change_email: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

#[function_component(TextInputName)]
pub fn text_input_name(props: &PropsName) -> Html {
    let PropsName {
        value,
        on_change_name,
    } = props.clone();
    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change_name.emit(get_value_from_input_event(input_event));
    });
    html! {
        <input type="text" {value} {oninput} />
    }
}

#[function_component(TextInputEmail)]
pub fn text_input_email(props: &PropsEmail) -> Html {
    let PropsEmail {
        value,
        on_change_email,
    } = props.clone();
    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change_email.emit(get_value_from_input_event(input_event));
    });
    html! {
        <input type="text" {value} {oninput} />
    }
}

pub enum Msg {
    SetName(String),
    SetEmail(String),
}

pub struct Form {
    _id: i32,
    name: String,
    email: String,
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _id: 0,
            name: String::from(""),
            email: String::from(""),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_name = ctx.link().callback(Msg::SetName);
        let on_change_email = ctx.link().callback(Msg::SetEmail);
        let header = "Form Component";
        html! {
            <main>
                <br/><hr/><br/>
                <div>
                    <h2>{ header }</h2>
                </div>
                <div>
                    {"Enter a new Name: "}
                    <TextInputName {on_change_name} value={self.name.clone()} />
                </div>
                <b></b>
                <div>
                    {"Enter a new Email below: "}
                    <TextInputEmail {on_change_email} value={self.email.clone()} />
                </div>
            </main>
        }
    }
}
