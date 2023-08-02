#![doc = include_str!("../README.md")]

use fltk::{prelude::*, *};
use notify::{
    event::{AccessKind, AccessMode, DataChange, EventKind, ModifyKind},
    Event, RecursiveMode, Watcher,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

mod frames;
mod utils;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Widget {
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
    labelfont: Option<u32>,
    labelsize: Option<i32>,
    align: Option<i32>,
    when: Option<i32>,
    frame: Option<String>,
    downframe: Option<String>,
    shortcut: Option<String>,
    pad: Option<i32>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    step: Option<f64>,
    slidersize: Option<f64>,
    textfont: Option<i32>,
    textsize: Option<i32>,
    textcolor: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    w: Option<i32>,
    h: Option<i32>,
    margin: Option<i32>,
    left: Option<i32>,
    top: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
}

/// Entry point for your declarative app
#[derive(Debug, Clone)]
pub struct DeclarativeApp {
    a: app::App,
    w: i32,
    h: i32,
    label: String,
    #[allow(dead_code)]
    path: Option<&'static str>,
    widget: Option<Widget>,
    load_fn: fn(&'static str) -> Option<Widget>,
}

impl DeclarativeApp {
    /// Instantiate a new declarative app
    pub fn new(
        w: i32,
        h: i32,
        label: &str,
        path: &'static str,
        load_fn: fn(&'static str) -> Option<Widget>,
    ) -> Self {
        let widget = load_fn(path);
        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        Self {
            a,
            w,
            h,
            label: label.to_string(),
            path: Some(path),
            widget,
            load_fn,
        }
    }

    #[cfg(feature = "json")]
    pub fn new_json(w: i32, h: i32, label: &str, path: &'static str) -> Self {
        fn load_fn(path: &'static str) -> Option<Widget> {
            let s = std::fs::read_to_string(path).ok()?;
            serde_json::from_str(&s).map_err(|e| eprintln!("{e}")).ok()
        }
        Self::new(w, h, label, path, load_fn)
    }

    #[cfg(feature = "json5")]
    pub fn new_json5(w: i32, h: i32, label: &str, path: &'static str) -> Self {
        fn load_fn(path: &'static str) -> Option<Widget> {
            let s = std::fs::read_to_string(path).ok()?;
            serde_json5::from_str(&s).map_err(|e| eprintln!("{e}")).ok()
        }
        Self::new(w, h, label, path, load_fn)
    }

    #[cfg(feature = "xml")]
    pub fn new_xml(w: i32, h: i32, label: &str, path: &'static str) -> Self {
        fn load_fn(path: &'static str) -> Option<Widget> {
            let s = std::fs::read_to_string(path).ok()?;
            serde_xml_rs::from_str(&s)
                .map_err(|e| eprintln!("{e}"))
                .ok()
        }
        Self::new(w, h, label, path, load_fn)
    }

    #[cfg(feature = "yaml")]
    pub fn new_yaml(w: i32, h: i32, label: &str, path: &'static str) -> Self {
        fn load_fn(path: &'static str) -> Option<Widget> {
            let s = std::fs::read_to_string(path).ok()?;
            serde_yaml::from_str(&s).map_err(|e| eprintln!("{e}")).ok()
        }
        Self::new(w, h, label, path, load_fn)
    }

    /// Instantiate a new declarative app
    pub fn new_inline(w: i32, h: i32, label: &str, widget: Option<Widget>) -> Self {
        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        Self {
            a,
            w,
            h,
            label: label.to_string(),
            path: None,
            widget,
            load_fn: |_| None,
        }
    }

    /// Run your declarative app.
    /// The callback exposes the app's main window
    pub fn run<F: FnMut(&mut window::Window) + 'static>(
        &self,
        mut run_cb: F,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.path {
            let mut win = window::Window::default()
                .with_size(self.w, self.h)
                .with_label(&self.label);
            if let Some(widget) = &self.widget {
                utils::transform(widget);
            }
            win.end();
            win.show();

            if let Some(mut frst) = win.child(0) {
                frst.resize(0, 0, win.w(), win.h());
                win.resizable(&frst);
            }

            run_cb(&mut win);

            let flag = Arc::new(AtomicBool::new(false));
            app::add_timeout3(0.1, {
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

            let load_fn = self.load_fn;
            let mut watcher = notify::recommended_watcher({
                let path = <&str>::clone(path);
                move |res: Result<Event, notify::Error>| match res {
                    Ok(event) => {
                        let mut needs_update = false;
                        match event.kind {
                            EventKind::Access(AccessKind::Close(mode)) => {
                                if mode == AccessMode::Write {
                                    needs_update = true;
                                }
                            }
                            EventKind::Modify(ModifyKind::Data(DataChange::Content)) => {
                                needs_update = true;
                            }
                            _ => (),
                        }
                        if needs_update {
                            if let Some(wid) = (load_fn)(path) {
                                win.clear();
                                win.begin();
                                utils::transform(&wid);
                                win.end();
                                if let Some(mut frst) = win.child(0) {
                                    frst.resize(0, 0, win.w(), win.h());
                                    win.resizable(&frst);
                                }
                                app::redraw();
                                flag.store(true, Ordering::Relaxed);
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            })?;
            watcher.watch(&PathBuf::from(path), RecursiveMode::NonRecursive)?;

            self.a.run()?;
        } else {
            self.run_once(run_cb)?;
        }
        Ok(())
    }

    /// Run the app without hot-reloading!
    pub fn run_once<F: FnMut(&mut window::Window) + 'static>(
        &self,
        mut run_cb: F,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut win = window::Window::default()
            .with_size(self.w, self.h)
            .with_label(&self.label);
        if let Some(widget) = &self.widget {
            utils::transform(widget);
        }
        win.end();
        win.show();

        if let Some(mut frst) = win.child(0) {
            frst.resize(0, 0, win.w(), win.h());
            win.resizable(&frst);
        }

        run_cb(&mut win);

        self.a.run()?;
        Ok(())
    }

    /// Just load the image of the window
    pub fn dump_image(&self) {
        let mut win = window::Window::default()
            .with_size(self.w, self.h)
            .with_label(&self.label);
        if let Some(widget) = &self.widget {
            utils::transform(widget);
        }
        win.end();
        win.show();

        if let Some(mut frst) = win.child(0) {
            frst.resize(0, 0, win.w(), win.h());
            win.resizable(&frst);
        }
        let sur = surface::SvgFileSurface::new(win.w(), win.h(), "temp.svg");
        surface::SvgFileSurface::push_current(&sur);
        draw::set_draw_color(enums::Color::White);
        draw::draw_rectf(0, 0, win.w(), win.h());
        sur.draw(&win, 0, 0);
        surface::SvgFileSurface::pop_current();
    }
}
