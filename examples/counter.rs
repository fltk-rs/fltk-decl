use fltk::{prelude::*, *};
use fltk_decl::DeclarativeApp;

// use the extension you require!
const PATH: &str = "examples/gui.json";

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

fn btn_cb(b: &mut button::Button) {
    let state = app::GlobalState::<State>::get();
    let val = if b.label() == "Inc" { 1 } else { -1 };
    state.with(move |s| s.increment(val));
}

fn main() {
    app::GlobalState::new(State { count: 0 });
    DeclarativeApp::new(200, 300, "MyApp", PATH)
        .run(|_win| {
            app::set_scheme(app::Scheme::Oxy);
            if let Some(mut btn) = app::widget_from_id::<button::Button>("inc") {
                btn.set_callback(btn_cb);
            }
            if let Some(mut btn) = app::widget_from_id::<button::Button>("dec") {
                btn.set_callback(btn_cb);
            }
        })
        .unwrap();
}
