use crate::hero::Hero;
use egui::*;

pub struct Team {
    pub name: String,
    pub heros: Vec<Hero>,
}

impl Team {
    pub fn with_members(name: impl Into<String>, heros: &[Hero]) -> Self {
        Self {
            name: name.into(),
            heros: heros.to_vec(),
        }
    }
}

pub fn draw_team(ui: &mut Ui, team: &mut Team) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(&team.name).size(32.));
        for hero in &mut team.heros {
            ui.add(hero);
        }
    });
}
