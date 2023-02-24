
mod edit_dist;
use crate::app::edit_dist::edit_dist;

use yew::prelude::*;

use crate::text_input::TextInput;



pub enum Msg {
    SetPhrase1(String),
    SetPhrase2(String),
}

#[derive(Debug, Default)]
pub struct App {
    phrase1: String,
    phrase2: String,
}

impl App {
    fn get_dist(&self) -> usize {
        edit_dist(&self.phrase1, &self.phrase2)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPhrase1(p1) => self.phrase1 = p1,
            Msg::SetPhrase2(p2) => self.phrase2 = p2,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let phrase1_change = ctx.link().callback(Msg::SetPhrase1);
        let phrase2_change = ctx.link().callback(Msg::SetPhrase2);

        html! {
            <main>
                <div class="entry">
                    <div>
                        {"Phrase 1: "}
                    </div>
                    <div>
                        <TextInput on_change={phrase1_change} value={self.phrase1.clone()} />
                    </div>
                </div>
                <div class="entry">
                    <div>
                        {"Phrase 2: "}
                    </div>
                    <div>
                        <TextInput on_change={phrase2_change} value={self.phrase2.clone()} />
                    </div>
                </div>
                <div class="readout">
                    <div>
                        {"Distance: "}
                    </div>
                    <div>
                        {self.get_dist()}
                    </div>
                </div>
                {"Shane Was Here"}
            </main>
        }
    }
}
