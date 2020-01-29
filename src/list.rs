use crate::{item::Item};
use yew::html::{ChildrenWithProps, ComponentLink};
use yew::prelude::*;

pub struct List(Props);

#[derive(Properties)]
pub struct Props {
    pub children: ChildrenWithProps<Item>,
}

pub enum Variants {
    Item(<ListItem as Component>::Properties),
    Header(<ListHeader as Component>::Properties),
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Item { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <ul>{self.view_items()}</ul>
        }
    }
}

impl List {
    fn view_items(&self) -> Html {
        html! {{
            for self.props.children.iter().map(|mut item| {
                item.props.value = format!("item-{}", item.props.value)
            })
        }}
    }
}
