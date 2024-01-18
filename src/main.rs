#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_audio::plot_xy::*;
use ringbuf::HeapRb;

// use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    //  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    plot_xy: PlotXY,
    progress: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            plot_xy: PlotXY::new(64.0, 64),
            progress: 0,
        }
    }
}
use std::f32::consts::PI;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let mut plot_rect = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            // if ui.button("Save Plot").clicked() {
            //     println!("--here--")
            // }

            // simulate data
            let data: Vec<(f32, f32)> = (self.progress..=self.progress + 10)
                .map(|i| (2.0 * PI * i as f32 / 40.0, 0.0))
                .collect();

            self.plot_xy.ui_content(ui, &data);
            self.progress = (self.progress + 10) % 40;
        });

        ctx.request_repaint();

        // // Check for returned screenshot:
        // let screenshot = ctx.input(|i| {
        //     for event in &i.raw.events {
        //         if let egui::Event::Screenshot { image, .. } = event {
        //             return Some(image.clone());
        //         }
        //     }
        //     None
        // });

        // if let (Some(screenshot), Some(plot_location)) = (screenshot, plot_rect) {
        //     if let Some(mut path) = rfd::FileDialog::new().save_file() {
        //         path.set_extension("png");

        //         // for a full size application, we should put this in a different thread,
        //         // so that the GUI doesn't lag during saving

        //         let pixels_per_point = ctx.pixels_per_point();
        //         let plot = screenshot.region(&plot_location, Some(pixels_per_point));
        //         // save the plot to png
        //         image::save_buffer(
        //             &path,
        //             plot.as_raw(),
        //             plot.width() as u32,
        //             plot.height() as u32,
        //             image::ColorType::Rgba8,
        //         )
        //         .unwrap();
        //         eprintln!("Image saved to {path:?}.");
        //     }
        // }
    }
}
