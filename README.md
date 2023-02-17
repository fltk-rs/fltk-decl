# fltk-decl
Use a declarative language (json5, json, xml, toml) to describe your fltk-rs gui, with support for hot-reloading of your gui file. 

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

Note that this crate uses json5, so you could just as easily change your gui.json to gui.json5 (to benefit from comments, trailing commas and unquoted keys!):
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
```
widget = "Column"

[[children]]
widget = "Button"
label = "Click Me"
id = "my_button"
```

Import it into your app:
```rust
use fltk_decl::DeclarativeApp;

fn main() {
    // use the filetype and extension that you require
    // `run` takes a bool and a callback, the bool indicates you want hot-reloading.
    // The callback runs at least once, or whenever the gui file changes.
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

## Supported properties:
- widget: (Required) The widget type (string)
- label: The widget label (string)
- fixed: Whether the widget is fixed inside a Flex (integer)
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
- frame: The widget's frame type (integer)
- downframe: The widget's down_frame type, for buttons (integer)
- shortcut: The widget's shortcut, for buttons (string)
- pad: The Flex's padding (integer)
- minimun: The valuator's minimum value (floating point number)
- maximum: The valuator's maximum value (floating point number)
- slidersize: The valuator's slider size (floating point number)
- step: The valuator's step (floating point number)
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