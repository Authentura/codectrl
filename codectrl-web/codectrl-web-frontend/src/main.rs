use yew::prelude::*;

// enum Msg {
//     NoOp,
// }
//
struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self {} }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "CodeCtrl" }</h1>
            </div>
        }
    }
}

fn main() { yew::start_app::<Model>(); }
