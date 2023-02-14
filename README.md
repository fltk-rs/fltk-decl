# fltk-decl

## Usage
Create a json file, let's call it gui.json.
```json
{
    "widget": "Column",
    "children": [
        {
            "widget": "Button",
            "label": "Inc",
            "fixed": 60,
            "id": "inc",
            "labelcolor": "#0000ff"

        },
        {
            "widget": "Row",
            "children": [
                {
                    "widget": "Frame",
                    "fixed": 30
                },
                {
                    "widget": "Frame",
                    "label": "0",
                    "id": "result",
                    "labelcolor": "#ff0000"
                },
                {
                    "widget": "Frame",
                    "fixed": 30
                }
            ]
        },
        {
            "widget": "Button",
            "label": "Dec",
            "fixed": 60,
            "id": "dec"
        }
    ]
}
```

Import it into your app:
```rust
use fltk_decl::DeclarativeApp;

fn main() {
    DeclarativeApp::new(400, 300, "MyApp", "gui.json").run(|| {});
}
```

To handle callbacks:
```rust
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
    DeclarativeApp::new(400, 300, "MyApp", "gui.json").run(|| {
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
    });
}
```