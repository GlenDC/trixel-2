//! Trixel Core Painting Functionality.

/*
Trixel, a FOSS trigonometric painting app.
Copyright (C) 2021  Glen Henri J. De Cauwsemaeker and Elizabeth C. Gonzales Belsuzarri.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/

use egui::{self, Color32, Frame, Ui, Sense, DragValue};
use epaint::{Mesh, TextureId, Shape};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct Painting {
    size: f32,
    color: Color32,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            size: 1.0,
            color: Color32::LIGHT_BLUE,
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.add(DragValue::new(&mut self.size).speed(0.1).clamp_range(0.1..=5.0))
                .on_hover_text("Size");
            ui.color_edit_button_srgba(&mut self.color);
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), Sense::drag());

        let mut shapes = vec![];

        let mut mesh = Mesh::with_texture(TextureId::Egui);
        mesh.add_triangle(0, 1, 2);
        let center = response.rect.center();
        let offset_x = response.rect.width() / 4.0 * self.size;
        let offset_y = response.rect.height() / 4.0 * self.size;
        mesh.colored_vertex(egui::pos2(center.x, center.y + offset_y), self.color);
        mesh.colored_vertex(egui::pos2(center.x - offset_x, center.y - offset_y), self.color);
        mesh.colored_vertex(egui::pos2(center.x + offset_x, center.y - offset_y), self.color);
        shapes.push(Shape::Mesh(mesh));

        painter.extend(shapes);

        response.mark_changed();
        response
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        self.ui_control(ui);
        Frame::dark_canvas(ui.style()).show(ui, |ui| {
            self.ui_content(ui);
        });
    }
}
