use itertools::Itertools;
use yew::prelude::*;

pub struct TopBar {
    left_text: Vec<String>,
    right_text: Vec<String>,
}

impl Component for TopBar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            left_text: "Tropical Zodiac\nEqual Houses\nQuadrants"
                .lines()
                .map(String::from)
                .collect(),
            right_text: "DRAW\nzh 2\nZET9".lines().map(String::from).collect(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="top_bar">
                <div>{
                    for self.left_text.iter().map(|s| html! { s }).intersperse(html! { <br /> })
                }</div>
                <div>{
                    for self.right_text.iter().map(|s| html! { s }).intersperse(html! { <br /> })
                }</div>
            </div>
        }
    }
}
