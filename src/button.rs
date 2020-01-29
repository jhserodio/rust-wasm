use yew::html::ComponentLink;
use yew::prelude::*;

pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Button {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div>
                <button>{format!("Buttom without props")}</button>
            </div>
        }
    }
}
