#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::{Color32, RichText};
use rfd;
use std::{error::Error, path::{self, PathBuf}};
use eframe::egui;
use csv;
use monet::{self, GraphPaper};

use std::fs::File;
use std::io::Write;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Monet",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Debug, PartialEq)]
enum AxisKind {
    Linear,
    Log
}

struct AxisData {
    axis_kind: AxisKind,
    // For linear
    h_great_split: u32,
    h_short_split: u32,
    max_value: f32,
    // For log
    base: f32,
    from: i32,
    to  : i32,
    tick: u32
}

fn read_csv_columns(csv_path: &PathBuf) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;

    let headers = rdr.headers()?.clone();
    let num_columns = headers.len();
    let mut columns: Vec<Vec<String>> = vec![Vec::new(); num_columns];

    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            if let Some(col) = columns.get_mut(i) {
                col.push(field.to_string());
            }
        }
    }

    Ok(columns)
}

impl Default for AxisData {
    fn default() -> Self {
        Self {
            axis_kind: AxisKind::Linear,

            h_great_split: 10,
            h_short_split: 5,
            max_value    : 10.0,

            base :  10.0,
            from : -1,
            to   :  2,
            tick :  10
        }
    }
}

struct MyApp {
    graph_name: String,
    csv_path: Option<path::PathBuf>,
    out_path: Option<path::PathBuf>,
    columns: Vec<(u32, u32)>,
    parse_error: String,
    x: AxisData,
    y: AxisData
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            graph_name: String::new(),
            csv_path: None,
            out_path: None,
            columns: vec![(0, 0)],
            parse_error: String::new(),
            x: AxisData::default(),
            y: AxisData::default(),
        }
    }
}

const V_SEPARATION:f32 = 10.0;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The heading
            ui.heading("Monet");

            // Specify csv path
            ui.horizontal(|hui| {
                if hui.button("Load a CSV").clicked() {
                    self.csv_path = rfd::FileDialog::new().pick_file();
                }
                hui.label(match &self.csv_path {
                    Some(s) => s.to_str().unwrap(),
                          None => "Unlocated"
                })
            });

            // Specify out path
            ui.horizontal(|hui| {
                if hui.button("Specify Out").clicked() {
                    self.out_path = rfd::FileDialog::new().pick_folder();
                }
                hui.label(match &self.out_path {
                    Some(s) => s.to_str().unwrap(),
                          None => "Unlocated"
                })
            });

            ui.add_space(V_SEPARATION);

            // Select X Axis Type
            egui::ComboBox::from_label("X's axis type")
            .selected_text(format!("{:?}", self.x.axis_kind))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.x.axis_kind,
                    AxisKind::Linear,
                    "Linear"
                );
                ui.selectable_value(
                    &mut self.x.axis_kind,
                    AxisKind::Log,
                    "Log"
                );
            });

            // Select Y Axis Type
            egui::ComboBox::from_label("Y's axis type")
            .selected_text(format!("{:?}", self.y.axis_kind))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.y.axis_kind,
                    AxisKind::Linear,
                    "Linear"
                );
                ui.selectable_value(
                    &mut self.y.axis_kind,
                    AxisKind::Log,
                    "Log"
                );
            });

            ui.add_space(V_SEPARATION);

            if let Some(p) = &self.csv_path.clone() {
                ui.label("Fields means each a corresponding csv column");
                if ui.button("Add line").clicked() {
                    self.columns.push((0, 0));
                }
                for i in 0..self.columns.len() {
                    ui.horizontal(|hui| {
                        if self.columns.get(i).is_some() {
                            hui.add(egui::DragValue::new(&mut self.columns[i].0));
                            hui.add(egui::DragValue::new(&mut self.columns[i].1));
                            if hui.button("Delete").clicked() {
                                self.columns.remove(i);
                            };
                        }
                    });
                }

                ui.add_space(V_SEPARATION);

                if ui.button("Export SVG").clicked() {
                    if let Err(e) = self.compile(p) {
                        self.parse_error = e;
                    }
                }
            }

            ui.label(RichText::from(&self.parse_error).color(Color32::LIGHT_RED));
        });
    }
}

impl MyApp {
    fn compile(&mut self, csv_path:&PathBuf) -> Result<(), String> {
        if let Ok(_) = csv::Reader::from_path(csv_path) {
            let mut graph_paper = monet::GraphPaper {
                name: self.graph_name.clone(),
                size: monet::graph_paper::A4,
                points: Vec::new(),
                margin: 100.0,
                stroke_width: 3.0,
                great_split_length: 50.0,
                short_split_length: 25.5,
            };
            if let Some(s) = &self.csv_path {
                let read_csv = read_csv_columns(s)
                .or(Err("Failed to parse CSV into vector."))?;
                for i in 0..self.columns.len() {
                    let c = self.columns.get(i).unwrap();
                    let x:Option<&Vec<String>> = read_csv.get(c.0 as usize);
                    let y:Option<&Vec<String>> = read_csv.get(c.1 as usize);
                    let points = match (x, y) {
                        (Some(x), Some(y)) => {
                            x.iter().zip(y)
                            .filter_map(|f| {
                                match (f.0.parse::<f32>(), f.1.parse::<f32>()) {
                                    (Ok(x), Ok(y)) => Some(monet::Vec2::vec2(x, y)),
                                        (_, _) => None
                                }
                            }).collect::<Vec<monet::Vec2>>()
                        },
                        (_, _) => return Err(format!(
                            "The located column, {} and {} doesn't exit.", c.0, c.1
                        ))
                    };
                    graph_paper.points.extend(points);
                }
                let serialised = self.serialise(graph_paper);
                if let Some(p) = &self.out_path {
                    if let Err(e) = self.out(
                        serialised,
                        &p.join(PathBuf::from(
                            format!("{}.svg", self.graph_name))
                        )
                    ) {
                        return Err(e.to_string());
                    };
                } else {
                    return Err("Out path wasn't specified".to_string())
                }
            } else {
                return Err("The specified csv doesn't exit".to_string());
            }
        }
        Ok(())
    }

    fn serialise(&self, graph_paper:GraphPaper) -> String {
        monet::graph_paper::Graph {
            graph_paper: graph_paper,
            x_scale: match self.x.axis_kind {
                AxisKind::Linear => Box::new(monet::XLinearScale {
                    h_great_split: self.x.h_great_split,
                    h_short_split: self.x.h_short_split,
                    max_value    : self.x.max_value
                }),
                AxisKind::Log => Box::new(monet::XLogScale {
                    base: self.x.base,
                    from: self.x.from,
                    to  : self.x.to,
                    tick: self.x.tick
                })
            },
            y_scale: match self.y.axis_kind {
                AxisKind::Linear => Box::new(monet::YLinearScale {
                    v_great_split: self.y.h_great_split,
                    v_short_split: self.y.h_short_split,
                    max_value    : self.y.max_value
                }),
                AxisKind::Log => Box::new(monet::YLogScale {
                    base: self.y.base,
                    from: self.y.from,
                    to  : self.y.to,
                })
            }
        }.serialise()
    }

    fn out(&self, serialised:String, path:&PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        write!(file, "{}", &serialised)?;
        file.flush()?;
        Ok(())
    }
}
