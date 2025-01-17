use crate::{structures::Align, ui, widget::HWidget};
use gtk::{traits::*, *};
use std::fmt::Display;

/// Creates a new label widget.
#[derive(Debug)]
pub struct CavaWidget {
    pub label: Label,
}

unsafe impl Send for CavaWidget {}
unsafe impl Sync for CavaWidget {}

// Implements HWidget for the widget so that we can actually use it.
impl HWidget for CavaWidget {
    fn add(self, name: String, align: Align, left: &Box, centered: &Box, right: &Box) {
        self.label.set_widget_name(&name);
        ui::add_and_align(&self.label, align, left, centered, right);
        ui::CAVA_INSTANCES
            .lock()
            .expect("[ERROR] Couldn't access ui::CAVA_INSTANCES!\n")
            .push(self)
            .expect("[ERROR] You cannot have more than `8` Cava widgets!\n");
    }

    fn update_label_reg(&self, new_content: &(impl Display + Clone)) {
        let final_content = &new_content.to_string();
        // Only redraw if the text wasn't the exact same as final_content.
        if !self.label.text().eq(final_content) {
            self.label.set_text(final_content)
        }
    }
}
