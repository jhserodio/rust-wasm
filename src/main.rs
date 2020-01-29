use yew::{
    html,
    Callback,
    ClickEvent,
    Component,
    ComponentLink,
    Html,
    ShouldRender
};

struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Cricouuu!"  } else { "Crica nheuu"  };
        let hello = String::from("Hello world");

        html! {
            <div>
                <h1>{ hello }</h1>
                <button onclick=&self.onclick> { button_text  }</button>
            </div>
        }
    }
}


fn main() {
    yew::start_app::<App>();
}
