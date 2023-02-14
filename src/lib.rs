use fltk::{prelude::*, *};
use notify::{
    event::{AccessKind, AccessMode, EventKind},
    Event, RecursiveMode, Watcher,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Widget {
    widget: String,
    label: Option<String>,
    id: Option<String>,
    fixed: Option<i32>,
    color: Option<String>,
    labelcolor: Option<String>,
    children: Option<Vec<Widget>>,
}

#[derive(Debug, Clone)]
pub struct DeclarativeApp {
    a: app::App,
    w: i32,
    h: i32,
    label: String,
    path: String,
    widget: Option<Widget>,
}

fn load(path: &str) -> Result<Widget, Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&s)?)
}

fn handle_w<T>(w: &Widget, widget: &mut T) where T: Clone + Send + Sync + WidgetExt + 'static {
    if let Some(id) = &w.id {
        widget.set_id(Box::leak(id.clone().into_boxed_str()));
    }
    if let Some(label) = &w.label {
        widget.set_label(label);
    }
    if let Some(fixed) = w.fixed {
        if let Some(parent) = widget.parent() {
            if let Some(mut flex) = group::Flex::from_dyn_widget(&parent) {
                flex.set_size(widget, fixed);
            }
        }
    }
    if let Some(col) = &w.color {
        if let Ok(col) = enums::Color::from_hex_str(col) {
            widget.set_color(col);
        }
    }
    if let Some(col) = &w.labelcolor {
        if let Ok(col) = enums::Color::from_hex_str(col) {
            widget.set_label_color(col);
        }
    }
    if let Some(children) = &w.children {
        for c in children {
            transform(c);
        }
    }
    if let Some(grp) = group::Group::from_dyn_widget(widget) {
        grp.end();
    }
}

fn transform(w: &Widget) {
    match w.widget.as_str() {
        "Column" => {
            let mut c = group::Flex::default().column();
            handle_w(w, &mut c);
        }
        "Row" => {
            let mut c = group::Flex::default().row();
            handle_w(w, &mut c);
        }
        "Button" => {
            let mut b = button::Button::default();
            handle_w(w, &mut b);
        }
        "Frame" => {
            let mut f = frame::Frame::default();
            handle_w(w, &mut f);
        }
        _ => (),
    };
}

impl DeclarativeApp {
    pub fn new(w: i32, h: i32, label: &str, path: &str) -> Self {
        let json = load(path).unwrap();
        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        Self {
            a,
            w,
            h,
            label: label.to_string(),
            path: path.to_string(),
            widget: Some(json),
        }
    }

    pub fn run<F: FnMut() + 'static>(&self, mut run_cb: F) {
        let mut win = window::Window::default()
            .with_size(self.w, self.h)
            .with_label(&self.label);
        if let Some(widget) = &self.widget {
            transform(widget);
        }
        win.end();
        win.show();

        if let Some(mut frst) = win.child(0) {
            frst.resize(0, 0, win.w(), win.h());
            win.resizable(&frst);
        }

        let flag = Arc::new(AtomicBool::new(true));
        app::add_timeout3(0.1, {
            let flag = flag.clone();
            move |t| {
                if flag.load(Ordering::Relaxed) {
                    run_cb();
                    flag.store(false, Ordering::Relaxed);
                }
                app::repeat_timeout3(0.1, t);
            }
        });

        let path = self.path.clone();

        let mut watcher =
            notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    if let EventKind::Access(AccessKind::Close(mode)) = event.kind {
                        if mode == AccessMode::Write {
                            if let Ok(wid) = load(&path) {
                                win.clear();
                                win.begin();
                                transform(&wid);
                                win.end();
                                if let Some(mut frst) = win.child(0) {
                                    frst.resize(0, 0, win.w(), win.h());
                                    win.resizable(&frst);
                                }
                                flag.store(true, Ordering::Relaxed);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("{}", e),
            })
            .unwrap();
        watcher
            .watch(Path::new(&self.path), RecursiveMode::NonRecursive)
            .unwrap();

        self.a.run().unwrap();
    }
}
