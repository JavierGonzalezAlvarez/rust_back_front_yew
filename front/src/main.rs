use yew::prelude::*;
mod form;
use form::Form;

enum Msg {
    Save,
    Cancel,
}

struct MyComponent;

impl Component for MyComponent {
    type Message = Msg;
    //type Properties = Props;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MyComponent
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Save => {
                //api                
                true
            }
            Msg::Cancel => {                
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {        
        let link = ctx.link();
        let header = "Main Component";

        html! {
            <main>
                <div>
                    <h1 class="color">{ header }</h1>
                </div>
                <div>
                    <Form />
                </div>
                <div>
                    <button onclick ={link.callback(|_| Msg::Save)}>{ "Save" }</button>
                    <button onclick ={link.callback(|_| Msg::Cancel)}>{ "Cancel" }</button>                    
                </div>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<MyComponent>();
}
