use std::{fmt::Display, io::Write};

// Перечисление, содержащее информацию о поле.
// Автоматически реализует типажи (интерфейсы) сравнения полей между собой.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Gender {
    Male,
    Female,
}

// Явная реализайция типажа Display, указывающая на то,
// как представлять данные в виде строки
impl Display for Gender {
    // Трейт требует реализации метода (метод, потому что есть &self -- это как this)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match это switch, но из мира функционального программирования.
        // write!() это макрос, как и println!(), panic!() и другие штуки с '!'.
        match self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
        }
    }
}

// Структура (класс) клиента. Содержит 4 приватных поля.
pub struct Client {
    forename: String, // Строка
    surname: String,  // Строка
    gender: Gender,   // Указанное выше перечисление
    age: u8,          // unsigned char из C; 8 потому что 8 бит, u потому что unsigned
}

// Реализация методов и функций для клиента
impl Client {
    // В расте для структур нет конструкторов как таковых, но эту роль
    // внегласно занимает метод new(), который возвращает Self -- в данном случае сокращение для Client
    pub fn new(forename: String, surname: String, gender: Gender, age: u8) -> Self {
        // Инициализация структуры Client. Если бы имена полей отличались от передаваемых значений,
        // то тогда бы использовался обычный синтаксис, а не сокращённый:
        // Self { forename: firstname, surname: secondname, gender: sex, age: years_old }
        Self {
            forename,
            surname,
            gender,
            age,
        }
    }
}

// Явная реализайция типажа Display, указывающая на то,
// как представлять данные в виде строки
impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {}, {} years old",
            self.forename, self.surname, self.gender, self.age
        )
    }
}

fn main() {
    // Создаём список (вектор) клиентов
    let mut clients = [
        Client::new("Александр".into(), "Шувалов".into(), Gender::Male, 24),
        Client::new("Ольга".into(), "Куликова".into(), Gender::Female, 32),
        Client::new("Олег".into(), "Куликов".into(), Gender::Male, 28),
        Client::new("Николай".into(), "Терехов".into(), Gender::Male, 21),
        Client::new("Анастасия".into(), "Назарова".into(), Gender::Female, 23),
        Client::new("Александр".into(), "Косаерв".into(), Gender::Male, 38),
        Client::new("Анастасия".into(), "Гросс".into(), Gender::Female, 31),
        Client::new("Валерия".into(), "Чёрная".into(), Gender::Female, 21),
        Client::new("Дарья".into(), "Иванова".into(), Gender::Female, 18),
        Client::new("Владислав".into(), "Кутузов".into(), Gender::Male, 29),
        Client::new("Никита".into(), "Гросс".into(), Gender::Male, 33),
        Client::new("Мария".into(), "Бойко".into(), Gender::Female, 40),
    ];

    // Сортируем клиентов по имени и фамилии
    clients.sort_by(|c1, c2| {
        c1.forename
            .cmp(&c2.forename)
            .then(c1.surname.cmp(&c2.surname))
    });

    // Создаём файл для записи
    let mut file = std::fs::File::create("fsa.txt").unwrap();

    // Записываем клиентов в файл
    for client in &clients {
        write!(file, "{}\n", client).unwrap();
    }

    // Сортируем клиентов по возрасту
    clients.sort_by(|c1, c2| c1.age.cmp(&c2.age));

    // Создаём файл для записи
    let mut file = std::fs::File::create("asf_m.txt").unwrap();

    // Записываем в файл только мужчин
    for client in clients.iter().filter(|c| c.gender == Gender::Male) {
        write!(file, "{}\n", client).unwrap();
    }
}
