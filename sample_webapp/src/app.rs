use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    
    fn create(_: Self::Properties, _:ComponentLink<Self>) -> Self {
        App {}
    }

    fn view(&self) -> Html {
        html! {
            <p>{"Hello World!"}</p>
        }
    }

    fn update(&mut self, _:Self::Message) -> bool {
        true
    }

    fn change(&mut self, _:Self::Properties) -> bool {
        true
    }
}