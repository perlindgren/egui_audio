// plot_xy.rs
// plotting x and y values similar to oscilloscope view.

use egui::{emath::RectTransform, *};
use epaint::{PathShape, RectShape, Rounding};
use ringbuf::{HeapRb, Rb};

pub struct PlotXY {
    diameter: f32,
    // history
    rb: HeapRb<(f32, f32)>,
    // frame: Rect,
}

impl PlotXY {
    pub fn new(size: f32, rb: usize) -> Self {
        Self {
            diameter: size,
            rb: HeapRb::new(rb),
        }
    }

    /// main panel
    pub fn ui_content<'b>(&mut self, ui: &mut Ui, data: &'b [(f32, f32)]) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter((self.diameter, self.diameter).into(), Sense::click());

        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, (1.0, 1.0).into()),
            response.rect,
        );

        let center =
            Rect::from_two_pos(to_screen * (0.5, 0.5).into(), to_screen * (1.0, 1.0).into());

        let to_center =
            RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, (1.0, 1.0).into()), center);

        // compute left/right sample
        let left: Vec<Pos2> = data
            .iter()
            .map(|(left, _)| to_center * Pos2::new(left.cos(), left.sin()))
            .collect();

        let stroke = Stroke::new(1.0, Color32::WHITE);

        // println!("rect {:?}", response.rect);

        painter.add(RectShape::filled(
            response.rect,
            Rounding::ZERO,
            Color32::GRAY,
        ));

        painter.add(RectShape::filled(center, Rounding::ZERO, Color32::BLUE));

        // paint left sample
        // painter.add(PathShape::line(left, stroke));

        let mut rb_data = vec![];

        // copy rb_data
        while let Some(d) = self.rb.pop() {
            rb_data.push(d);
        }

        // compute left/right sample
        let left: Vec<Pos2> = rb_data
            .iter()
            .map(|(left, _)| to_center * Pos2::new(left.cos(), left.sin()))
            .collect();

        painter.add(PathShape::line(left, stroke));
        // // paint right sample
        // painter.add(PathShape::line(right, config.stroke_sample));

        // push to rb
        for d in data {
            self.rb.push(*d).unwrap();
        }

        response
    }
}
