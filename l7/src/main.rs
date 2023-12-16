use crate::{hero::Hero, team::Team};
use eframe::egui;
use log::*;
use team::draw_team;

mod hero;
mod team;

fn main() {
    // Логгер
    env_logger::init();

    // Настройки приложения
    let native_options = eframe::NativeOptions::default();

    // Запуск
    let _ = eframe::run_native(
        "DnD Party List",
        native_options,
        Box::new(|cc| Box::new(DndPvp::new(cc))),
    );
}

// Приложение
struct DndPvp {
    teams: Vec<Team>,
    start_window_open: bool, // состояние popup-окна, которе открывается по нажатию на кнопку Start
}

impl Default for DndPvp {
    fn default() -> Self {
        Self {
            teams: vec![
                Team::with_members(
                    "Blue",
                    &[
                        Hero::new("Nagat'rok", "file://assets/demon.png"),
                        Hero::new("Nimeza", "file://assets/orc.png"),
                        Hero::new("Soverein", "file://assets/human.png"),
                    ],
                ),
                Team::with_members(
                    "Red",
                    &[
                        Hero::new("Thaldar", "file://assets/dwarf.png"),
                        Hero::new("Greedlock", "file://assets/elf.png"),
                        Hero::new("Piero", "file://assets/transformer.png"),
                    ],
                ),
            ],
            start_window_open: false,
        }
    }
}

impl DndPvp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    // Перекидывание героев в конкретную команду
    fn send_heros_to_team(&mut self, team_to: usize) {
        if team_to >= self.teams.len() {
            warn!("Can't add heros to the team: there is no such team.");
            return;
        }

        // Герои, которых нужно перекинуть
        let mut heroes_to_move = vec![];

        // Перекидывание выбранных героев в другую команду
        for (team_idx, team) in self.teams.iter_mut().enumerate() {
            // Разделяем героев на выбранных и невыбранных
            let (mut selected, mut remain) = team
                .heros
                .clone()
                .into_iter()
                .partition(|hero| hero.selected);

            heroes_to_move.append(&mut selected);

            if team_idx == team_to {
                heroes_to_move.append(&mut remain);
            } else {
                team.heros = remain;
            }
        }

        // Закидываем выбранных героев в нужную команду
        self.teams[team_to].heros = heroes_to_move;

        // Сортируем героев
        for team in self.teams.iter_mut() {
            team.heros.sort_by(|h1, h2| h1.name.cmp(&h2.name));
        }
    }
}

impl eframe::App for DndPvp {
    // Вызывается при переотрисовке
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx); // загрузчик для изображений

        // Popup-окно старта игры
        egui::Window::new("Game Started")
            .movable(false)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .open(&mut self.start_window_open)
            .show(ctx, |ui| {
                // Таблички с командами
                ui.columns(self.teams.len(), |uis| {
                    for (team_idx, team) in self.teams.iter_mut().enumerate() {
                        let ui = &mut uis[team_idx];
                        ui.label(&team.name);
                        ui.separator();

                        for hero in &team.heros {
                            ui.label(&hero.name);
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Заголовок
                ui.heading(
                    egui::RichText::new("DnD PvP")
                        .color(egui::Color32::from_rgb(98, 210, 238))
                        .size(48.),
                );
                ui.add_space(10.);

                // Кнопка старта
                if ui.button(egui::RichText::new("Start").size(24.)).clicked() {
                    self.start_window_open = true;
                }
                ui.add_space(20.);

                // Нужно ли двигать героев
                let mut move_heros = None;

                // Кнопки, по нажатию на которые происходит перекидывание в другую команду
                for (team_idx, team) in self.teams.iter().enumerate() {
                    if ui
                        .button(egui::RichText::new(format!("To {}", team.name)).size(24.))
                        .clicked()
                    {
                        // Нужно перекинуть героев выбранную команду
                        move_heros = Some(team_idx);
                    }
                }

                // Если нужно перекинуть героев, то перекидываем их
                if let Some(team_to) = move_heros {
                    self.send_heros_to_team(team_to);
                }
            });

            ui.add_space(10.);
            ui.separator();

            // Табличка с карточками героев
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    ui.columns(self.teams.len(), |uis| {
                        for (team_idx, team) in self.teams.iter_mut().enumerate() {
                            let ui = &mut uis[team_idx];
                            draw_team(ui, team); // колонку с карточками героев
                        }
                    });
                });
        });
    }
}
