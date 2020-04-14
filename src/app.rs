use web_sys;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    clicked: bool,
    number: i32,
}

pub enum Msg {
    Click,
    Change(i32),
    Error(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            clicked: false,
            number: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true
            }
            Msg::Change(num) => {
                self.number = num;
                true
            }
            Msg::Error(reason) => {
                web_sys::console::log(reason);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Eggs" } else { "Spam" };
        let text = match self.number {
            0 => "Tallet er null".into(),
            _ => format!("Tallet er {}", self.number),
        };
        html! {
            <div>
                <input type="number" onchange=self.link.callback(|e: ChangeData| {
                    match e {
                        ChangeData::Value(s) => {
                            match s.parse() {
                                Ok(num) => Msg::Change(num),
                                Err(reason) => Msg::Error(reason.to_string())
                            }
                        },
                        _ => unreachable!(),
                    }
                }) />
                <button onclick=self.link.callback(|_| Msg::Click)>{button_text}</button>
                <p>{text}</p>
            </div>
        }
    }
}
