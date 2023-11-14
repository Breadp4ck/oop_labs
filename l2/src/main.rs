use std::fmt::Display;

// Библиотека для интерфейса
use iced::widget::{button, checkbox, column, container, radio, row, text};
use iced::{Color, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    // Интерфейс построен с помощью библиотеки Iced.
    // Iced использует Elm модель:
    // - Приложение находится в некотором состоянии (State)
    // - Пользователь взаимодействут с приложением при помощи сообщений (Messages)
    // - Для отображения состояний используются виджеты, которые предоставляют
    //   пользователю возможность создавать сообщения (View logic)
    // - Сообщения обновляют состояние приложения (Update logic)
    App::run(Settings::default())
}

// Модель шаурмы
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
    Classic,
    Cheese,
    Cesar,
    Meat,
    Beef,
}

// Реализация Display для текстового представления модели шаурмы
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

// Размер шаурмы
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Size {
    Small,
    Medium,
    Large,
}

// Реализация Display для текстового представления размера шаурмы
impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Small => write!(f, "Мини"),
            Size::Medium => write!(f, "Средняя"),
            Size::Large => write!(f, "Большая"),
        }
    }
}

// Добавки
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Supplements {
    Cheese,
    Mushrooms,
    Jalopenos,
}

// Шаурма
pub struct Shaurma {
    kind: Kind,
    size: Size,
    cheese: bool,
    mushrooums: bool,
    jalopenos: bool,
}

impl Shaurma {
    // Конструктор шаурмы
    pub fn new(kind: Kind, size: Size) -> Self {
        Self {
            kind,
            size,
            cheese: false,
            mushrooums: false,
            jalopenos: false,
        }
    }

    // Подсчёт цены для шаурмы
    pub fn price(&self) -> u32 {
        let mut price = match (self.kind, self.size) {
            (Kind::Classic, Size::Small) => 100,
            (Kind::Classic, Size::Medium) => 140,
            (Kind::Classic, Size::Large) => 170,
            (Kind::Cheese, Size::Small) => 130,
            (Kind::Cheese, Size::Medium) => 160,
            (Kind::Cheese, Size::Large) => 190,
            (Kind::Cesar, Size::Small) => 125,
            (Kind::Cesar, Size::Medium) => 155,
            (Kind::Cesar, Size::Large) => 185,
            (Kind::Meat, Size::Small) => 130,
            (Kind::Meat, Size::Medium) => 160,
            (Kind::Meat, Size::Large) => 190,
            (Kind::Beef, Size::Small) => 145,
            (Kind::Beef, Size::Medium) => 175,
            (Kind::Beef, Size::Large) => 215,
        };

        if self.cheese {
            price += 15;
        }

        if self.mushrooums {
            price += 25;
        }

        if self.jalopenos {
            price += 20;
        }

        price
    }
}

// Виджет-приложение
struct App {
    shaurma: Shaurma,
    ordered: bool,
}

// Возможные сообщения/события в рамках виджета-приложения
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    SelectKind(Kind),
    SelectSize(Size),
    AdditionalCheese(bool),
    AdditionalMushrooms(bool),
    AdditionalJalopenos(bool),
    Order,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            shaurma: Shaurma::new(Kind::Classic, Size::Small),
            ordered: false,
        }
    }

    fn title(&self) -> String {
        String::from("Шаурма")
    }

    // Реагирование на сообщения
    fn update(&mut self, message: Message) {
        if message != Message::Order {
            self.ordered = false;
        }

        match message {
            Message::SelectKind(kind) => {
                self.shaurma.kind = kind;
            }
            Message::SelectSize(size) => {
                self.shaurma.size = size;
            }
            Message::AdditionalCheese(cheese) => {
                self.shaurma.cheese = cheese;
            }
            Message::AdditionalMushrooms(mushrooms) => {
                self.shaurma.mushrooums = mushrooms;
            }
            Message::AdditionalJalopenos(jalopenos) => {
                self.shaurma.jalopenos = jalopenos;
            }
            Message::Order => {
                self.ordered = true;
            }
        }
    }

    // Рисование виджетов
    fn view(&self) -> Element<Message> {
        // Модель шаурмы
        let select_kind = [
            Kind::Classic,
            Kind::Cesar,
            Kind::Cheese,
            Kind::Meat,
            Kind::Beef,
        ]
        .iter()
        .fold(
            column![text("Выберите шаурму")].spacing(10),
            |column, kind| {
                column.push(radio(
                    format!("{kind}"),
                    *kind,
                    Some(self.shaurma.kind),
                    Message::SelectKind,
                ))
            },
        );

        // Размер шаурмы
        let select_size = [Size::Small, Size::Medium, Size::Large].iter().fold(
            column![text("Выберите размер")].spacing(10),
            |column, size| {
                column.push(radio(
                    format!("{size}"),
                    *size,
                    Some(self.shaurma.size),
                    Message::SelectSize,
                ))
            },
        );

        // Добавки
        let select_supplements = column![
            text("Выберите добавку"),
            checkbox("Сыр", self.shaurma.cheese, Message::AdditionalCheese),
            checkbox(
                "Грибы",
                self.shaurma.mushrooums,
                Message::AdditionalMushrooms
            ),
            checkbox(
                "Халопеньо",
                self.shaurma.jalopenos,
                Message::AdditionalJalopenos
            )
        ]
        .spacing(10);

        // Итоговая цена шаурмы
        let price = self.shaurma.price().to_string();

        // Кнопка "Заказать" или сообщение о том, что уже заказано
        let order = if self.ordered {
            column![text("Шаурма заказана!").size(23.8)] // При 23 и 24 дёргаются на один пиксель разные части интерфейса. Да, это костыль
        } else {
            column![button("Заказать!").on_press(Message::Order)]
        };

        // Сборка элементов интерфейса
        container(
            column![
                text("Шаурма").size(57).style(Color::from([0.15, 0.1, 0.1])),
                row![select_kind, select_size, select_supplements].spacing(20),
                text(format!("{price} рублей")),
                order,
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
