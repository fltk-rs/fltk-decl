// run using `cargo run --example menu --features=json5`

use fltk::{prelude::*, *};
use fltk_decl::DeclarativeApp;

fn main() {
    DeclarativeApp::new_json5(200, 300, "MyApp", "examples/menu.json")
        .run(|_win| {
            if let Some(mut choice) = app::widget_from_id::<menu::Choice>("choice") {
                choice.add_choice("JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC");
                choice.set_callback(|c| {
                    if let Some(mut label) = app::widget_from_id::<frame::Frame>("label") {
                        label.set_label(&format!("{:?}", c.choice()));
                    }
                });
            }
        })
        .unwrap();
}
