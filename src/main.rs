use dioxus::prelude::*;

pub mod uilib;
pub mod wav_io;

fn main(){
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        uilib::TestComponent::WavContainer {}
    ))
}
