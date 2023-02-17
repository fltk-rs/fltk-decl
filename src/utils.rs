use fltk::{prelude::*, *};
use crate::Widget;

pub(crate) fn load(path: &str) -> Option<Widget> {
    let s = std::fs::read_to_string(path).expect("Invalid path!");
    if path.ends_with(".xml") {
        serde_xml_rs::from_str(&s).ok()
    } else {
        serde_json5::from_str(&s).ok()
    }
}

pub(crate) fn handle_w<T>(w: &Widget, widget: &mut T)
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
        widget.set_tooltip(tip);
    }
    if let Some(path) = &w.image {
        widget.set_image(Some(
            image::SharedImage::load(path).expect("Failed to load image!"),
        ));
    }
    if let Some(path) = &w.deimage {
        widget.set_deimage(Some(
            image::SharedImage::load(path).expect("Failed to load image!"),
        ));
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
    if let Some(mut b) = button::Button::from_dyn_widget(widget) {
        if let Some(f) = w.downframe {
            if f < 50 {
                b.set_down_frame(unsafe { std::mem::transmute(f) });
            }
        }
        if let Some(f) = &w.shortcut {
            b.set_shortcut(unsafe {
                std::mem::transmute(f.parse::<i32>().expect("Failed to parse shortcut!"))
            });
        }
    }
    if let Some(mut b) = valuator::Slider::from_dyn_widget(widget) {
        if let Some(sz) = w.minimum {
            b.set_minimum(sz);
        }
        if let Some(sz) = w.maximum {
            b.set_maximum(sz);
        }
        if let Some(sz) = w.slidersize {
            b.set_slider_size(sz as _);
        }
        if let Some(sz) = w.step {
            b.set_step(sz, 1);
        }
    }
    if let Some(gap) = w.pad {
        if let Some(mut b) = group::Flex::from_dyn_widget(widget) {
            b.set_pad(gap);
        }
    }
    if let Some(grp) = group::Group::from_dyn_widget(widget) {
        grp.end();
    }
}

pub(crate) fn transform(w: &Widget) {
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