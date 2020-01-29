mod button;

use button::Button;

use yew::{
    html,
    Callback,
    ClickEvent,
    Component,
    ComponentLink,
    Html,
    ShouldRender
};

pub struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

pub enum Msg {
    Click,
}


impl App {
    fn classes(&self) -> String {
        String::from("carambolas quadradas")
    }
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
                self.clicked = !self.clicked;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Cricouuu!"  } else { "Crica nheuu"  };

        html! {
            <div class=self.classes()>
                <h1 class=("carambolas", "quadradas")>{"Hello World with Literals"}</h1>
                <button class=vec!["never", "die"] onclick=&self.onclick> { button_text  }</button>
                <Button />
                // <List>
                //     <Item value={21} />
                //     <Item value={3} />
                //     <Item value={65} />
                //     <Item value={123} />
                // </List>
            </div>
        }
    }
}
