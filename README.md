# fltk-decl
Use a declarative language (json5, json, yaml, xml, toml, s-exp) to describe your fltk-rs gui, with support for hot-reloading of your gui file. The crate is designed to be as permissive as possible. So wrong keys or values will be ignored. Normally only changing a widget's id at runtime would cause an error!

## Usage

### Using a fixed format, using features
Assuming we're using json for our gui description, we'll pull fltk-decl and fltk. We'll specify the feature that we need to be json since we're using json. In your Cargo.toml:
```toml
[dependencies]
fltk-decl = { version = "0.2", features = ["json"] }
fltk = "1.3.32"
```
For other formats, replace the json feature with your required feature. Possible values (json, json5, yaml, xml).

Since we're gonna use json, we'll create a json file and let's call it gui.json:
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

Import it into your app:
```rust,ignore
use fltk_decl::{DeclarativeApp, Widget};

fn main() {
    // use the filetype and extension that you require.
    // `run` a callback that runs at least once, or whenever the gui file changes.
    DeclarativeApp::new_json(200, 300, "MyApp", "examples/gui.json").run(|_| {}).unwrap();
}
```
Notice we use new_json which is made available by the json feature. The constructor comes in the form of `DeclarativeApp::new` or `DeclarativeApp::new_<feature>`, ex. new_json5, new_yaml, new_xml!

To handle callbacks:
```rust,ignore
use fltk::{prelude::*, *};
use fltk_decl::{DeclarativeApp, Widget};

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
    let val = if b.label() == "Inc" {
        1
    } else {
        -1
    };
    state.with(move |s| s.increment(val));
}

fn main() {
    app::GlobalState::new(State { count: 0 });
    DeclarativeApp::new_json(200, 300, "MyApp", PATH)
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
```

### Flexible, using any serde format and using a loading function (requires no features)
Assuming we're using json for our gui description, we'll pull fltk-decl, fltk and the deserialization library that we require, in this case it's serde_json:
In your Cargo.toml:
```toml
[dependencies]
fltk-decl = "0.2"
fltk = "1.3.32"
serde_json = "1"
```

For the other data formats, you can pull the respective deserialization library:
```toml
serde_json5 = "0.1" # for json5
serde-xml-rs = "0.6" # for xml
serde_yaml = "0.9" # for yaml
toml = "0.7" # for toml
serde-lexpr = "0.1.2" # for an s-expression description
```

Since we're gonna use json, we'll create a json file and let's call it gui.json:
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

Import it into your app:
```rust,no_run
use fltk_decl::{DeclarativeApp, Widget};

// declare how you would like to deserialize
fn load_fn(path: &'static str) -> Option<Widget> {
    let s = std::fs::read_to_string(path).ok()?;
    // We want to see the serde error on the command line while we're developing
    serde_json5::from_str(&s).map_err(|e| eprintln!("{e}")).ok()
}

fn main() {
    // use the filetype and extension that you require.
    // `run` a callback that runs at least once, or whenever the gui file changes.
    DeclarativeApp::new(200, 300, "MyApp", "examples/gui.json", load_fn).run(|_| {}).unwrap();
}
```

To handle callbacks:
```rust
use fltk::{prelude::*, *};
use fltk_decl::{DeclarativeApp, Widget};

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
    let val = if b.label() == "Inc" {
        1
    } else {
        -1
    };
    state.with(move |s| s.increment(val));
}

fn load_fn(path: &'static str) -> Option<Widget> {
    let s = std::fs::read_to_string(path).ok()?;
    serde_json5::from_str(&s).map_err(|e| eprintln!("{e}")).ok()
}

fn main() {
    app::GlobalState::new(State { count: 0 });
    DeclarativeApp::new(200, 300, "MyApp", PATH, load_fn)
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
```

## Other data formats:

You can choose json5 (to benefit from comments, trailing commas and unquoted keys!):
```json5
{
    // main column
    widget: "Column",
    children: [
        {
            // our button
            widget: "Button",
            label: "Click me",
            color: "#ff0000",
            id: "my_button",
        }
    ],
}
```
However, you lose vscode's auto-completion since json5 extensions in vscode don't support schemas.

You could also use yaml (optionally along with a schema for autocompletion and validation):
```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/MoAlyousef/fltk-decl/main/schemas/fltk-schema.yaml

widget: Column
children:
- widget: Button
  label: Inc
  fixed: 60
  id: inc
  labelcolor: "#0000ff"
- widget: Row
  children:
  - widget: Frame
    fixed: 30
  - widget: Frame
    label: '0'
    id: result
    labelcolor: "#ff0000"
  - widget: Frame
    fixed: 30
- widget: Button
  label: Dec
  fixed: 60
  id: dec
```

You could also use xml:
`gui.xml`
```xml
<?xml version="1.0" encoding="UTF-8"?>
<root>
    <widget>Column</widget>
    <children>
        <widget>Button</widget>
        <label>Click Me</label>
        <id>my_button</id>
        <labelcolor>#0000ff</labelcolor>
    </children>
</root>
```

or toml!
```toml
widget = "Column"

[[children]]
widget = "Button"
label = "Click Me"
id = "my_button"
```

or s-expression format:
```scheme
(
    (widget . "Column") 
    (children (
        (
            (widget . "Button") 
            (label "Inc") 
            (id "inc") 
            (fixed 60) 
            (labelcolor "#0000ff")
        )
        (
            (widget . "Frame")
            (id "result")
            (label "0")
        )
        (
            (widget . "Button") 
            (label "Dec") 
            (id "dec") 
            (fixed 60) 
            (labelcolor "#ff0000")
        )
    ))
)
```

## Supported properties:
- widget: (Required) The widget type (string)
- label: The widget label (string)
- fixed: Whether the widget is fixed inside a Flex (integer)
- x: x coordinate
- y: y coordinate
- w: width
- h: height
- margin: A flex's margin
- left: A flex's left margin
- top: A flex's top margin
- right: A flex's right margin
- bottom: A flex's bottom margin
- id: The widget's id (string)
- labelcolor: The widget's label color (string, format #xxxxxx)
- color: The widget's color (string, format #xxxxxx)
- selectioncolor: The widget's selection color (string, format #xxxxxx)
- hide: Whether the widget is hidden (bool)
- visible: Whether the widget is visible (bool)
- deactivate: Whether the widget is deactivated (bool)
- resizable: Whether the widget is the resiable widget in a group (bool)
- tooltip: The widget's tooltip (string)
- image: A path to an image for the widget (string)
- deimage: A path to an image (deactivated) for the widget (string)
- labelfont: The label font (integer)
- labelsize: The label size (integer)
- align: The label's alignment (integer)
- when: The widget's callback trigger (integer)
- frame: The widget's frame type (string)
- downframe: The widget's down_frame type, for buttons (string)
- shortcut: The widget's shortcut, for buttons (string)
- pad: The Flex's padding (integer)
- minimun: The valuator's minimum value (floating point number)
- maximum: The valuator's maximum value (floating point number)
- slidersize: The valuator's slider size (floating point number)
- step: The valuator's step (floating point number)
- textcolor: The widget's text color (string)
- textsize: The widget's text size (integer)
- textfont: The widget's font (integer)
- children: an array of widgets representing the children of the widget (array of objects)

## Supported widgets:
- Column (Flex column)
- Row (Flex row)
- Button 
- CheckButton 
- RadioButton 
- ToggleButton 
- RadioRoundButton 
- ReturnButton 
- Frame 
- Group 
- Pack 
- Tile 
- Tabs 
- Scroll 
- ColorChooser 
- TextDisplay
- TextEditor
- Input 
- IntInput 
- FloatInput 
- SecretInput 
- FileInput 
- MultilineInput 
- Output 
- MultilineOutput 
- MenuBar 
- SysMenuBar 
- Choice 
- Slider 
- NiceSlider 
- FillSlider 
- ValueSlider 
- Dial 
- LineDial 
- FillDial 
- Counter 
- Scrollbar 
- Roller 
- Adjuster 
- ValueInput 
- ValueOutput 
- HorSlider 
- HorNiceSlider 
- HorFillSlider 
- HorValueSlider 
- Browser 
- SelectBrowser 
- HoldBrowser 
- FileBrowser 
- CheckBrowser 
- MultiBrowser 
- Table 
- TableRow 
- Tree 
- Spinner 
- Chart 
- Progress 
- InputChoice 
- HelpView 
