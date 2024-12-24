#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
#![feature(never_type)]

use eframe::egui;

fn main() -> eframe::Result {
    let _synth = chipbox_synth::Graph::<!, u8>::with_capacity(0, 0);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "chipbox-synth example",
        options,
        Box::new(|_| Ok(Box::<App>::default())),
    )
}

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });
    }
}
