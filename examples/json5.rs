use fltk::{prelude::*, *};
use fltk_decl::DeclarativeApp;

#[derive(Clone, Copy)]
struct State {
    count: i32,
}

fn current_value(val: i32) -> String {
    let state = app::GlobalState::get();
    state
        .with(move |s: &mut State| {
            s.count += val;
            s.count
        })
        .to_string()
}

fn main() {
    app::GlobalState::new(State { count: 0 });
    DeclarativeApp::new(200, 300, "MyApp", "examples/gui.json5")
        .run(true, |_| {
            let mut inc: button::Button = app::widget_from_id("inc").unwrap();
            let mut dec: button::Button = app::widget_from_id("dec").unwrap();
            let mut result: frame::Frame = app::widget_from_id("result").unwrap();
            inc.set_callback({
                let mut result = result.clone();
                move |_| {
                    result.set_label(&current_value(1));
                }
            });
            dec.set_callback(move |_| {
                result.set_label(&current_value(-1));
            });
        })
        .unwrap();
}
