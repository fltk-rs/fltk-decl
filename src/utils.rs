use crate::Widget;
use fltk::{prelude::*, *};

macro_rules! handle_text {
    ($w: ident, $widget: ident) => {
        if let Some(col) = &$w.textcolor {
            if let Ok(col) = enums::Color::from_hex_str(col) {
                $widget.set_text_color(col);
            }
        }
        if let Some(f) = $w.textfont {
            if f < 14 {
                $widget.set_text_font(unsafe { std::mem::transmute(f) });
            }
        }
        if let Some(sz) = $w.textsize {
            $widget.set_text_size(sz);
        }
    };
}

pub(crate) fn handle_w<T>(w: &Widget, widget: &mut T)
where
    T: Clone + Send + Sync + WidgetExt + 'static,
{
    if let Some(id) = &w.id {
        widget.set_id(id);
    }
    if let Some(label) = &w.label {
        widget.set_label(label);
    }
    if w.x.is_some() || w.y.is_some() || w.w.is_some() || w.h.is_some() {
        widget.resize(
            w.x.unwrap_or(widget.x()),
            w.y.unwrap_or(widget.y()),
            w.w.unwrap_or(widget.w()),
            w.h.unwrap_or(widget.h()),
        );
    }
    if let Some(fixed) = w.fixed {
        if let Some(parent) = widget.parent() {
            if let Some(mut flex) = group::Flex::from_dyn_widget(&parent) {
                flex.fixed(widget, fixed);
            }
        }
    }
    if let Some(margin) = w.margin {
        if let Some(mut flex) = group::Flex::from_dyn_widget(widget) {
            flex.set_margin(margin);
        }
    }
    if w.left.is_some() || w.top.is_some() || w.right.is_some() || w.bottom.is_some() {
        if let Some(mut flex) = group::Flex::from_dyn_widget(widget) {
            let old = flex.margins();
            flex.set_margins(
                w.left.unwrap_or(old.0),
                w.top.unwrap_or(old.1),
                w.right.unwrap_or(old.2),
                w.bottom.unwrap_or(old.3),
            );
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
            } else if let Some(parent) = widget.parent() {
                parent.resizable(widget);
            }
        }
    }
    if let Some(tip) = &w.tooltip {
        widget.set_tooltip(tip);
    }
    if let Some(path) = &w.image {
        widget.set_image(image::SharedImage::load(path).ok());
    }
    if let Some(path) = &w.deimage {
        widget.set_deimage(image::SharedImage::load(path).ok());
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
    if let Some(f) = &w.frame {
        if let Some(f) = crate::frames::get_frame(f) {
            widget.set_frame(unsafe { std::mem::transmute(f as i32) });
        }
    }
    if let Some(mut b) = button::Button::from_dyn_widget(widget) {
        if let Some(f) = &w.downframe {
            if let Some(f) = crate::frames::get_frame(f) {
                b.set_down_frame(unsafe { std::mem::transmute(f as i32) });
            }
        }
        if let Some(f) = &w.shortcut {
            if let Ok(sh) = f.parse::<i32>() {
                b.set_shortcut(unsafe { std::mem::transmute(sh) });
            }
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
            let mut c = group::Flex::default_fill().column();
            handle_w(w, &mut c);
        }
        "Row" => {
            let mut c = group::Flex::default_fill().row();
            handle_w(w, &mut c);
        }
        "Button" => {
            let mut b = button::Button::default_fill();
            handle_w(w, &mut b);
        }
        "CheckButton" => {
            let mut b = button::CheckButton::default_fill();
            handle_w(w, &mut b);
        }
        "RadioButton" => {
            let mut b = button::RadioButton::default_fill();
            handle_w(w, &mut b);
        }
        "ToggleButton" => {
            let mut b = button::ToggleButton::default_fill();
            handle_w(w, &mut b);
        }
        "RadioRoundButton" => {
            let mut b = button::RadioRoundButton::default_fill();
            handle_w(w, &mut b);
        }
        "ReturnButton" => {
            let mut b = button::ReturnButton::default_fill();
            handle_w(w, &mut b);
        }
        "Frame" => {
            let mut f = frame::Frame::default_fill();
            handle_w(w, &mut f);
        }
        "Group" => {
            let mut f = group::Group::default_fill();
            handle_w(w, &mut f);
        }
        "Pack" => {
            let mut f = group::Pack::default_fill();
            handle_w(w, &mut f);
        }
        "Tile" => {
            let mut f = group::Tile::default_fill();
            handle_w(w, &mut f);
        }
        "Tabs" => {
            let mut f = group::Tabs::default_fill();
            handle_w(w, &mut f);
        }
        "Scroll" => {
            let mut f = group::Scroll::default_fill();
            handle_w(w, &mut f);
        }
        "ColorChooser" => {
            let mut f = group::ColorChooser::default_fill();
            handle_w(w, &mut f);
        }
        "TextDisplay" => {
            let mut f = text::TextDisplay::default_fill();
            handle_text!(w, f);
            let buf = text::TextBuffer::default();
            f.set_buffer(buf);
            handle_w(w, &mut f);
        }
        "TextEditor" => {
            let mut f = text::TextEditor::default_fill();
            handle_text!(w, f);
            let buf = text::TextBuffer::default();
            f.set_buffer(buf);
            handle_w(w, &mut f);
        }
        "Input" => {
            let mut f = input::Input::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "IntInput" => {
            let mut f = input::IntInput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "FloatInput" => {
            let mut f = input::FloatInput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "SecretInput" => {
            let mut f = input::SecretInput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "FileInput" => {
            let mut f = input::FileInput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "MultilineInput" => {
            let mut f = input::MultilineInput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Output" => {
            let mut f = output::Output::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "MultilineOutput" => {
            let mut f = output::Output::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "MenuBar" => {
            let mut f = menu::MenuBar::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "SysMenuBar" => {
            let mut f = menu::SysMenuBar::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Choice" => {
            let mut f = menu::Choice::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Slider" => {
            let mut f = valuator::Slider::default_fill();
            handle_w(w, &mut f);
        }
        "NiceSlider" => {
            let mut f = valuator::NiceSlider::default_fill();
            handle_w(w, &mut f);
        }
        "FillSlider" => {
            let mut f = valuator::FillSlider::default_fill();
            handle_w(w, &mut f);
        }
        "ValueSlider" => {
            let mut f = valuator::ValueSlider::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Dial" => {
            let mut f = valuator::Dial::default_fill();
            handle_w(w, &mut f);
        }
        "LineDial" => {
            let mut f = valuator::LineDial::default_fill();
            handle_w(w, &mut f);
        }
        "FillDial" => {
            let mut f = valuator::FillDial::default_fill();
            handle_w(w, &mut f);
        }
        "Counter" => {
            let mut f = valuator::Counter::default_fill();
            handle_w(w, &mut f);
        }
        "Scrollbar" => {
            let mut f = valuator::Scrollbar::default_fill();
            handle_w(w, &mut f);
        }
        "Roller" => {
            let mut f = valuator::Roller::default_fill();
            handle_w(w, &mut f);
        }
        "Adjuster" => {
            let mut f = valuator::Adjuster::default_fill();
            handle_w(w, &mut f);
        }
        "ValueInput" => {
            let mut f = valuator::ValueInput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "ValueOutput" => {
            let mut f = valuator::ValueOutput::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "HorSlider" => {
            let mut f = valuator::HorSlider::default_fill();
            handle_w(w, &mut f);
        }
        "HorNiceSlider" => {
            let mut f = valuator::HorNiceSlider::default_fill();
            handle_w(w, &mut f);
        }
        "HorFillSlider" => {
            let mut f = valuator::HorFillSlider::default_fill();
            handle_w(w, &mut f);
        }
        "HorValueSlider" => {
            let mut f = valuator::HorValueSlider::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Browser" => {
            let mut f = browser::Browser::default_fill();
            handle_w(w, &mut f);
        }
        "SelectBrowser" => {
            let mut f = browser::SelectBrowser::default_fill();
            handle_w(w, &mut f);
        }
        "HoldBrowser" => {
            let mut f = browser::HoldBrowser::default_fill();
            handle_w(w, &mut f);
        }
        "FileBrowser" => {
            let mut f = browser::FileBrowser::default_fill();
            handle_w(w, &mut f);
        }
        "CheckBrowser" => {
            let mut f = browser::CheckBrowser::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "MultiBrowser" => {
            let mut f = browser::MultiBrowser::default_fill();
            handle_w(w, &mut f);
        }
        "Table" => {
            let mut f = table::Table::default_fill();
            handle_w(w, &mut f);
        }
        "TableRow" => {
            let mut f = table::TableRow::default_fill();
            handle_w(w, &mut f);
        }
        "Tree" => {
            let mut f = tree::Tree::default_fill();
            handle_w(w, &mut f);
        }
        "Spinner" => {
            let mut f = misc::Spinner::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Chart" => {
            let mut f = misc::Chart::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "Progress" => {
            let mut f = misc::Progress::default_fill();
            handle_w(w, &mut f);
        }
        "InputChoice" => {
            let mut f = misc::InputChoice::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        "HelpView" => {
            let mut f = misc::HelpView::default_fill();
            handle_text!(w, f);
            handle_w(w, &mut f);
        }
        _ => (),
    };
}
