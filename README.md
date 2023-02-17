# fltk-decl
Use a declarative language (json5, json, xml) to describe your fltk-rs gui, with support for hot-reloading. 

## Usage
In your Cargo.toml:
```toml
[dependencies]
fltk-decl = "0.1"
```
Create a json file, let's call it gui.json.
```json
{
    "$schema": "https://raw.githubusercontent.com/MoAlyousef/fltk-decl/main/schemas/fltk-schema.json",
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
Notice we point to the schema to get auto-completion and hinting on vscode, otherwise it's optional.

Note that this crate uses json5, so you could just as easily change your gui.json to gui.json5:
```json5
{
    // main column
    widget: "Column",
    id: "my_column",
    children: [
        {
            // our button
            widget: "Button",
            color: "#ff0000",
            id: "my_button",
        }
    ],
}
```
However, you lose vscode's auto-completion since json5 extensions in vscode don't support schemas.

You could also use xml:
`gui.xml`
```xml
<?xml version="1.0" encoding="UTF-8"?>
<root>
    <widget>Column</widget>
    <children>
        <widget>Button</widget>
        <label>Inc</label>
        <fixed>60</fixed>
        <id>inc</id>
        <labelcolor>#0000ff</labelcolor>
    </children>
</root>
```

Import it into your app:
```rust
use fltk_decl::DeclarativeApp;

fn main() {
    // use the filetype and extension that you require
    DeclarativeApp::new(200, 300, "MyApp", "gui.json").run(true, |_main_win| {});
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
    DeclarativeApp::new(200, 300, "MyApp", "examples/gui.json")
        .run(true, |_win| {
            let mut inc: button::Button = app::widget_from_id("inc").unwrap();
            let mut dec: button::Button = app::widget_from_id("dec").unwrap();
            inc.set_callback(inc_btn_cb);
            dec.set_callback(dec_btn_cb);
        })
        .unwrap();
}
```

