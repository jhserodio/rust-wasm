use yew::prelude::*;

pub struct Item {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub name: String,
    pub value: i32,
}

impl Component for Item {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Item { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="item">
                { &self.props.value }
            </div>
        }
    }
}
