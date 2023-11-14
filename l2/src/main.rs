use std::fmt::Display;

use iced::widget::{button, column, container, radio, row, text};
use iced::{Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
    Classic,
    Cheese,
    Cesar,
    Meat,
    Beef,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Classic => write!(f, "Классическая"),
            Kind::Cheese => write!(f, "Сырная"),
            Kind::Cesar => write!(f, "Цезарь"),
            Kind::Meat => write!(f, "Мясная"),
            Kind::Beef => write!(f, "С говядиной"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Small => write!(f, "Мини"),
            Size::Medium => write!(f, "Средняя"),
            Size::Large => write!(f, "Большая"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Shaurma {
    kind: Kind,
    size: Size,
}

impl Shaurma {
    pub fn new(kind: Kind, size: Size) -> Self {
        Self { kind, size }
    }

    pub fn price(&self) -> u16 {
        match (self.kind, self.size) {
            (Kind::Classic, Size::Small) => 100,
            (Kind::Classic, Size::Medium) => 200,
            (Kind::Classic, Size::Large) => 200,
            (Kind::Cheese, Size::Small) => 200,
            (Kind::Cheese, Size::Medium) => 100,
            (Kind::Cheese, Size::Large) => 200,
            (Kind::Cesar, Size::Small) => 200,
            (Kind::Cesar, Size::Medium) => 100,
            (Kind::Cesar, Size::Large) => 200,
            (Kind::Meat, Size::Small) => 200,
            (Kind::Meat, Size::Medium) => 100,
            (Kind::Meat, Size::Large) => 200,
            (Kind::Beef, Size::Small) => 100,
            (Kind::Beef, Size::Medium) => 200,
            (Kind::Beef, Size::Large) => 200,
        }
    }
}

struct App {
    shaurma: Shaurma,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SelectKind(Kind),
    SelectSize(Size),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            shaurma: Shaurma::new(Kind::Classic, Size::Small),
        }
    }

    fn title(&self) -> String {
        String::from("Стекляшка")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectKind(kind) => {
                self.shaurma.kind = kind;
            }
            Message::SelectSize(size) => {
                self.shaurma.size = size;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let select_kind = [
            Kind::Classic,
            Kind::Cesar,
            Kind::Cheese,
            Kind::Meat,
            Kind::Beef,
        ]
        .iter()
        .fold(
            column![text("Выберете шаурму")].spacing(10),
            |column, kind| {
                column.push(radio(
                    format!("{kind}"),
                    *kind,
                    Some(self.shaurma.kind),
                    Message::SelectKind,
                ))
            },
        );

        let select_size = [Size::Small, Size::Medium, Size::Large].iter().fold(
            column![text("Выберете размер")].spacing(10),
            |column, size| {
                column.push(radio(
                    format!("{size}"),
                    *size,
                    Some(self.shaurma.size),
                    Message::SelectSize,
                ))
            },
        );

        let price = self.shaurma.price().to_string();

        container(
            column![
                row![select_kind, select_size].spacing(10),
                text(format!("{price} рублей")),
                button("Заказать!")
            ]
            .align_items(iced::Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
