use fltk_decl::DeclarativeApp;

const GUI: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <widget>Column</widget>
    <children>
        <widget>Button</widget>
        <label>Click Me</label>
        <id>my_button</id>
        <labelcolor>#0000ff</labelcolor>
    </children>
</root>"#;

fn main() {
    // use the filetype and extension that you require.
    // `run` a callback that runs at least once, or whenever the gui file changes.
    DeclarativeApp::new_inline(200, 300, "MyApp", serde_xml_rs::from_str(GUI).ok())
        .run_once(|_| {})
        .unwrap();
}
