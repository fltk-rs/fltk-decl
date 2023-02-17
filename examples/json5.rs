use fltk::{prelude::*, *};
use fltk_decl::DeclarativeApp;

#[derive(Clone, Copy)]
struct State {
    count: i32,
}

impl State {
    pub fn increment(&mut self, val: i32) {
        let mut result: frame::Frame = app::widget_from_id("result").unwrap();
        self.count += val;
        result.set_label(&self.count.to_string());
    }
}

fn inc_btn_cb(_b: &mut button::Button) {
    let state = app::GlobalState::<State>::get();
    state.with(|s| s.increment(1));
}

fn dec_btn_cb(_b: &mut button::Button) {
    let state = app::GlobalState::<State>::get();
    state.with(|s| s.increment(-1));
}

fn main() {
    app::GlobalState::new(State { count: 0 });
    DeclarativeApp::new(200, 300, "MyApp", "examples/gui.json5")
        .run(true, |_| {
            let mut inc: button::Button = app::widget_from_id("inc").unwrap();
            let mut dec: button::Button = app::widget_from_id("dec").unwrap();
            inc.set_callback(inc_btn_cb);
            dec.set_callback(dec_btn_cb);
        })
        .unwrap();
}
