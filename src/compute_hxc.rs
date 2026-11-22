use eframe::egui;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub struct ComputeHxC {
    data_path: String,
    m: i32,
    t: i32,
    matlab_output: String,
    rx: Option<Receiver<String>>,
}

impl Default for ComputeHxC {
    fn default() -> Self {
        Self {
            m: 2,
            t: 1,
            data_path: String::from(""),
            matlab_output: String::new(),
            rx: None,
        }
    }
}

impl ComputeHxC {
    //Calling matlab function, only on linux for now
    fn run_matlab(&mut self, ctx: egui::Context) {
        //Caution: you need to add the path of the function to the MATLAB path
        let matlab_scripts_dir = "/home/gaspichou/Documents/git/DBT2-permutation-entropy/Codes/HXC";

        let matlab_cmd = format!(
            "try, \
                        addpath('{dir}'); \
                        HXC_standard_deviation({m}, {t}, '{data}'); \
                        if ~isempty(get(groot, 'CurrentFigure')), waitfor(gcf); end; \
                     catch ME, \
                        disp(getReport(ME)); \
                     end; \
                     quit;",
            dir = matlab_scripts_dir,
            m = self.m,
            t = self.t,
            data = self.data_path
        );

        // Communication between MATLAB thread and UI
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        self.matlab_output = String::from("MATLAB computing...");

        thread::spawn(move || {
            let mut child = Command::new("matlab")
                .args(["-nodesktop", "-nosplash", "-r", &matlab_cmd])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to execute MATLAB");

            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                if let Ok(line_str) = line {
                    let _ = tx.send(line_str + "\n");
                    ctx.request_repaint();
                }
            }

            let _ = child.wait(); // Wait for the process to actually finish
            let _ = tx.send(String::from("\nMATLAB Finished."));
            ctx.request_repaint();
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(rx) = &self.rx {
            // Read all pending messages in the queue
            while let Ok(output) = rx.try_recv() {
                self.matlab_output.push_str(&output);

                // If the process is done, drop the receiver to re-enable the button
                if output.contains("MATLAB Finished.") {
                    self.rx = None;
                    break;
                }
            }
        }

        ui.heading("Entropy and Complexity Computing");

        ui.horizontal(|ui| {
            ui.label("Data Path: ");
            ui.add(
                egui::TextEdit::singleline(&mut self.data_path).hint_text("~/project/example.mat"),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Embedding Dimension: ");
            ui.add(egui::DragValue::new(&mut self.m).range(0..=100));
        });

        ui.horizontal(|ui| {
            ui.label("Embedding Time Delay: ");
            ui.add(egui::DragValue::new(&mut self.t).range(0..=100));
        });

        // Deactivate the button if computation in progress
        let is_idle = self.rx.is_none();
        if ui
            .add_enabled(is_idle, egui::Button::new("Run MATLAB"))
            .clicked()
        {
            self.run_matlab(ui.ctx().clone());
        }

        ui.add_space(10.0);
        ui.separator();
        ui.label("MATLAB output :");

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                ui.monospace(&self.matlab_output);
            });
    }
}
