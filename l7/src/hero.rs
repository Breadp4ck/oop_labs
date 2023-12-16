use egui::*;
use log::*;

const WIDGET_HERO_BORDER_INNER: f32 = 2.;

#[derive(Clone)]
pub struct Hero {
    pub selected: bool,
    pub name: String,
    pub img_path: String,
}

impl Hero {
    pub fn new(name: impl Into<String>, img_path: impl Into<String>) -> Self {
        Hero {
            selected: false,
            name: name.into(),
            img_path: img_path.into(),
        }
    }
}

impl Widget for &mut Hero {
    fn ui(self, ui: &mut Ui) -> Response {
        // Выделяем пространство под виджет с карточкой
        let desired_size = ui.spacing().interact_size.y * egui::vec2(18.0, 4.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        // Если на виджет кликнули, то обновляем состояние карточки героя
        if response.clicked() {
            info!("Value changed!");
            self.selected = !self.selected;
            response.mark_changed(); // report back that the value changed
        }

        // Если виджет виден, то отрисовываем
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact_selectable(&response, self.selected); // стиль
            let rect = rect.expand(visuals.expansion); // расширяем пространство в соответсвии со стилем (из-за border и т.д.)

            // область с картинкой героя
            let image_rect = Rect::from_min_size(
                rect.min + Vec2::new(WIDGET_HERO_BORDER_INNER, WIDGET_HERO_BORDER_INNER),
                Vec2::new(
                    rect.height() - WIDGET_HERO_BORDER_INNER * 2.,
                    rect.height() - WIDGET_HERO_BORDER_INNER * 2.,
                ),
            );

            // Дополнительные параметры для отрисовки интерфейса
            let radius = 0.2 * rect.height();
            let name_size = 28.;
            let name_font = FontId::monospace(name_size);
            let name_at = image_rect.min
                + Vec2::new(
                    image_rect.width() + name_size / 2.,
                    image_rect.height() / 2.,
                );

            // Далее просто рисуется интерфейс

            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);

            let image = egui::Image::new(&self.img_path)
                .rounding(radius - WIDGET_HERO_BORDER_INNER)
                .fit_to_exact_size(Vec2::new(image_rect.height(), image_rect.height()));

            image.paint_at(ui, image_rect);

            ui.painter().text(
                name_at,
                Align2::LEFT_CENTER,
                &self.name,
                name_font,
                visuals.text_color(),
            );
        }

        response // Информация об интерфейсе и событиях
    }
}
