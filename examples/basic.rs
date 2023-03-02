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
    DeclarativeApp::new(200, 300, "MyApp", "examples/gui.json", load_fn)
        .run(|_| {})
        .unwrap();
}
