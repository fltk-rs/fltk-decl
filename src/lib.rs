#![doc = include_str!("../README.md")]

use fltk::{prelude::*, *};
use notify::{
    event::{AccessKind, AccessMode, EventKind},
    Event, RecursiveMode, Watcher,
};
use serde_derive::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Widget {
    widget: String,
    label: Option<String>,
    id: Option<String>,
    fixed: Option<i32>,
    color: Option<String>,
    labelcolor: Option<String>,
    children: Option<Vec<Widget>>,
    hide: Option<bool>,
    deactivate: Option<bool>,
    visible: Option<bool>,
    resizable: Option<bool>,
    selectioncolor: Option<String>,
    tooltip: Option<String>,
    image: Option<String>,
    deimage: Option<String>,
    value: Option<String>,
    labelfont: Option<u32>,
    labelsize: Option<i32>,
    align: Option<i32>,
    when: Option<i32>,
    frame: Option<u32>,
}

/// Entry point for your declarative app
#[derive(Debug, Clone)]
pub struct DeclarativeApp {
    a: app::App,
    w: i32,
    h: i32,
    label: String,
    #[allow(dead_code)]
    path: String,
    widget: Option<Widget>,
}

fn load(path: &str) -> Option<Widget> {
    let s = std::fs::read_to_string(path).expect("Invalid path!");
    if path.ends_with(".xml") {
        serde_xml_rs::from_str(&s).ok()
    } else {
        serde_json5::from_str(&s).ok()
    }
}

fn handle_w<T>(w: &Widget, widget: &mut T)
where
    T: Clone + Send + Sync + WidgetExt + 'static,
{
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
    if let Some(col) = &w.selectioncolor {
        if let Ok(col) = enums::Color::from_hex_str(col) {
            widget.set_selection_color(col);
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
    if let Some(v) = w.hide {
        if v {
            widget.hide();
        }
    }
    if let Some(v) = w.deactivate {
        if v {
            widget.deactivate();
        }
    }
    if let Some(v) = w.visible {
        if v {
            widget.deactivate();
        }
    }
    if let Some(v) = w.resizable {
        if v {
            if let Some(mut grp) = widget.as_group() {
                grp.make_resizable(true);
            } else {
                let parent = widget.parent().unwrap();
                parent.resizable(widget);
            }
        }
    }
    if let Some(tip) = &w.tooltip {
        widget.set_tooltip(&tip);
    }
    if let Some(path) = &w.image {
        widget.set_image(Some(image::SharedImage::load(path).expect("Failed to load image!")));
    }
    if let Some(path) = &w.deimage {
        widget.set_deimage(Some(image::SharedImage::load(path).expect("Failed to load image!")));
    }
    if let Some(sz) = w.labelsize {
        widget.set_label_size(sz);
    }
    if let Some(a) = w.align {
        widget.set_align(unsafe { std::mem::transmute(a) });
    }
    if let Some(a) = w.when {
        widget.set_trigger(unsafe { std::mem::transmute(a) });
    }
    if let Some(f) = w.labelfont {
        if f < 14 {
            widget.set_label_font(unsafe { std::mem::transmute(f) });
        }
    }
    if let Some(f) = w.frame {
        if f < 50 {
            widget.set_frame(unsafe { std::mem::transmute(f) });
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
    /// Instantiate a new declarative app
    pub fn new(w: i32, h: i32, label: &str, path: &str) -> Self {
        let json = load(path).expect("Failed to load widget data!");
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

    /// Run your declarative app.
    /// The bool flag determines whether hot-reloading is enabled.
    /// The callback exposes the app's main window
    pub fn run<F: FnMut(&mut window::Window) + 'static>(
        &self,
        hot_reload: bool,
        mut run_cb: F,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        if hot_reload {
            let flag = Arc::new(AtomicBool::new(true));
            app::add_timeout3(0.0, {
                let flag = flag.clone();
                let mut win = win.clone();
                move |_t| {
                    if flag.load(Ordering::Relaxed) {
                        run_cb(&mut win);
                        flag.store(false, Ordering::Relaxed);
                    }
                    app::repeat_timeout3(0.1, _t);
                }
            });

            let path = self.path.clone();
            let mut watcher =
                notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                    Ok(event) => {
                        if let EventKind::Access(AccessKind::Close(mode)) = event.kind {
                            if mode == AccessMode::Write {
                                if let Some(wid) = load(&path) {
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
                })?;
            watcher.watch(
                std::path::Path::new(&self.path),
                RecursiveMode::NonRecursive,
            )?;
        }

        self.a.run()?;
        Ok(())
    }
}
