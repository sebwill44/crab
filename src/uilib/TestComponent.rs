use dioxus::prelude::*;

#[allow(non_snake_case)] // Dioxus component -- should be capital
pub fn WavContainer(cx: Scope) -> Element {
    cx.render(rsx! {
        div
        {
            h1 {"Hello World!"}
            button
            {
                onclick: move |_| crate::wav_io::save_wav::save_wav(),
            }

        }

    })
}