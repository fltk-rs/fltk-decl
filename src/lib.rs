#![doc = include_str!("../README.md")]

use fltk::{prelude::*, *};
use notify::{
    event::{AccessKind, AccessMode, EventKind},
    Event, RecursiveMode, Watcher,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

mod utils;

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
    labelfont: Option<u32>,
    labelsize: Option<i32>,
    align: Option<i32>,
    when: Option<i32>,
    frame: Option<u32>,
    downframe: Option<u32>,
    shortcut: Option<String>,
    pad: Option<i32>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    step: Option<f64>,
    slidersize: Option<f64>,
}

/// Entry point for your declarative app
#[derive(Debug, Clone)]
pub struct DeclarativeApp {
    a: app::App,
    w: i32,
    h: i32,
    label: String,
    #[allow(dead_code)]
    path: PathBuf,
    widget: Option<Widget>,
}

impl DeclarativeApp {
    /// Instantiate a new declarative app
    pub fn new<P: AsRef<Path>>(w: i32, h: i32, label: &str, path: P) -> Self {
        let json = utils::load(path.as_ref()).expect("Failed to load widget data!");
        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        Self {
            a,
            w,
            h,
            label: label.to_string(),
            path: PathBuf::from(path.as_ref()),
            widget: Some(json),
        }
    }

    /// Run your declarative app.
    /// The callback exposes the app's main window
    pub fn run<F: FnMut(&mut window::Window) + 'static>(
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

        let path = self.path.clone();
        let mut watcher =
            notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    if let EventKind::Access(AccessKind::Close(mode)) = event.kind {
                        if mode == AccessMode::Write {
                            if let Some(wid) = utils::load(&path) {
                                win.clear();
                                win.begin();
                                utils::transform(&wid);
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
        watcher.watch(&self.path, RecursiveMode::NonRecursive)?;

        self.a.run()?;
        Ok(())
    }

    #[doc(hidden)]
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
