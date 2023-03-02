use fltk::{prelude::*, *};
use fltk_decl::{DeclarativeApp, Widget};
use std::path::PathBuf;

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

fn run_cb(_win: &mut window::Window) {
    app::set_scheme(app::Scheme::Oxy);
    if let Some(mut btn) = app::widget_from_id::<button::Button>("inc") {
        btn.set_callback(btn_cb);
    }
    if let Some(mut btn) = app::widget_from_id::<button::Button>("dec") {
        btn.set_callback(btn_cb);
    }
}

fn load_fn(p: &'static str) -> Option<Widget> {
    let path = PathBuf::from(p);
    let ext = path.extension().unwrap().to_str().unwrap();
    let s = std::fs::read_to_string(p).unwrap();
    match ext {
        "xml" => serde_xml_rs::from_str(&s).ok(),
        "toml" => toml::from_str(&s).ok(),
        "yaml" => serde_yaml::from_str(&s).ok(),
        "scm" => serde_lexpr::from_str(&s).ok(),
        _ => serde_json5::from_str(&s).ok(),
    }
}

fn main() {
    app::GlobalState::new(State { count: 0 });
    DeclarativeApp::new(200, 300, "MyApp", PATH, load_fn)
        .run(run_cb)
        .unwrap();
}
