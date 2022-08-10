use dioxus::prelude::*;
use std::f32::consts::PI;

fn main(){
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        WavContainer {}
    ))
}

fn save_wav(){
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100, 
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0 .. 441000).map(|x: i32| -> f32 { (x as f32) / 44100.0}){
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = 1.0;
        writer.write_sample(sample * amplitude).unwrap();
    }
}

#[allow(non_snake_case)] // Dioxus component -- should be capital
fn WavContainer(cx: Scope) -> Element {
    cx.render(rsx! {
        div
        {
            h1 {"Hello World!"}
            button
            {
                onclick: move |_| save_wav(),
            }

        }

    })
}
