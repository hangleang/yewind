use yew::prelude::*;
use yewind::{
    button, Button,
    utils::{Color, Size},
};

pub struct Showcase;

pub enum Msg {
    OnClicked(MouseEvent),
}

impl Component for Showcase {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Self }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { unimplemented!() }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender { unimplemented!() }

    fn view(&self) -> Html {
        html! {
            <Button
                // class=Classes::from("focus:outline-none bg-blue-500 rounded text-base text-white py-2 px-5")
                style=button::Style::Filled
                color=Color::Blue(500)
                disabled=true
            >
                { "Default Button" }
            </Button>
        }
    }
}

fn main() {
    yew::start_app::<Showcase>()
}
