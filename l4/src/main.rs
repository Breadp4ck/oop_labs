use std::cell::RefCell;
use std::rc::Rc;

use gtk::pango::WrapMode;
use gtk::{glib, Application, ApplicationWindow, CheckButton, DropDown, Label, Orientation};
use gtk::{prelude::*, Entry};
use gtk4 as gtk;

use base64::{engine::general_purpose, Engine as _};
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

/// Способ отображения хэшированных данных
#[derive(Clone, Copy)]
pub enum HashPreviewOption {
    Base64,
    Hex,
}

/// Хэш-алгоритм
#[derive(Clone, Copy)]
pub enum HashAlgorythm {
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

/// Список хэш-алгортмов для drop-down
const HASH_ALGORYTHMS: [&str; 4] = ["SHA3-224", "SHA3-256", "SHA3-384", "SHA3-512"];

/// Модель приложения
#[derive(Clone)]
pub struct Hasher {
    pub algo: HashAlgorythm,
    pub option: HashPreviewOption,
    pub text: String,
    data: Vec<u8>,
}

impl Hasher {
    pub fn new(algo: HashAlgorythm, option: HashPreviewOption) -> Self {
        Self {
            algo,
            option,
            text: "".to_string(),
            data: vec![],
        }
    }

    /// Захэшировать текст в зависимости от свойства algo
    pub fn calculate(&mut self) {
        // Если текста нет, ничего не делать
        if self.text.is_empty() {
            return;
        }

        self.data = match self.algo {
            HashAlgorythm::Sha3_224 => Sha3_224::new().chain_update(&self.text).finalize().to_vec(),
            HashAlgorythm::Sha3_256 => Sha3_256::new().chain_update(&self.text).finalize().to_vec(),
            HashAlgorythm::Sha3_384 => Sha3_384::new().chain_update(&self.text).finalize().to_vec(),
            HashAlgorythm::Sha3_512 => Sha3_512::new().chain_update(&self.text).finalize().to_vec(),
        };
    }

    /// Отобразить захэшированный текст в зависимости от свойства option
    /// Если текста нет, то ничего не произойдёт
    pub fn view(&self) -> String {
        // Если хэша, ничего не делать
        if self.data.is_empty() {
            return "".to_string();
        }

        match self.option {
            HashPreviewOption::Base64 => general_purpose::STANDARD_NO_PAD.encode(&self.data),
            HashPreviewOption::Hex => hex::encode(&self.data),
        }
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.hasher.myhashapp")
        .build();

    app.connect_activate(|app| ui(app));
    app.run()
}

fn ui(app: &Application) {
    // Соответствующие GUI элементы
    let input_text_entry = Entry::builder().build();
    let hash_algo_drop_down = DropDown::from_strings(&HASH_ALGORYTHMS);
    let hash_option_check_box = CheckButton::with_label("Base64");
    let hash_value_label = Label::builder()
        .wrap(true)
        .wrap_mode(WrapMode::Char)
        .selectable(true)
        .width_chars(64)
        .max_width_chars(64)
        .build();

    // Контейнер для всех элементов
    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();
    container.append(&input_text_entry);
    container.append(&hash_algo_drop_down);
    container.append(&hash_option_check_box);
    container.append(&Label::new(Some("Значение хэша:")));
    container.append(&hash_value_label);

    // Мы создаём счётчик ссылок Rc и вкладываем в AppState возможность
    // изменяться (mutate) в нескольких местах RefCell (Rust по умолчанию запрещает такое).
    let hasher = Rc::new(RefCell::new(Hasher::new(
        HashAlgorythm::Sha3_256,
        HashPreviewOption::Hex,
    )));

    // Подписываемся на событие изменения текстового ввода
    input_text_entry.connect_changed(
        // Это капец. GTK это ООП-библиотека на чистом C, что уже звучит страшно.
        // А Rust язык с очень строгой типизацией и статической сборкой мусора (без сборщика мусора).
        // Из-за этого для "удобства" вводится такой макрос, который клонирует объекты, которые можно
        // изменять в нескольких местах (грубо говоря).
        // Я не думаю, что я в ближайшее время буду пробовать GTK ещё раз, в общем.
        glib::clone!(@strong hasher, @weak hash_value_label => move |entry| {
            let text = entry.text();

            // Одалживаем состояние приложения
            let mut hasher = hasher.borrow_mut();

            // Сохраняем текст
            hasher.text = text.to_string();

            // Хэшируем текст и отображаем хэш в интерфейсе
            hasher.calculate();
            hash_value_label.set_label(&hasher.view());
        }),
    );

    // Подписываемся на событие выбора варианта из выпадающего списка
    hash_algo_drop_down.connect_selected_notify(
        glib::clone!(@strong hasher, @weak hash_value_label => move |drop_down| {
            // Одалживаем состояние приложения
            let mut hasher = hasher.borrow_mut();

            // Выбираем алгоритм в соответствии с номером варианта из списка
            hasher.algo = match drop_down.selected() {
                0 => HashAlgorythm::Sha3_224,
                1 => HashAlgorythm::Sha3_256,
                2 => HashAlgorythm::Sha3_384,
                3 => HashAlgorythm::Sha3_512,
                _ => panic!("There is no such drop down value!"),
            };

            // Хэшируем текст и отображаем хэш в интерфейсе
            hasher.calculate();
            hash_value_label.set_label(&hasher.view());
        }),
    );

    // Подписываемся на событие нажатия на кнопку
    hash_option_check_box.connect_toggled(move |button| {
        // Одалживаем состояние приложения
        let mut hasher = hasher.borrow_mut();

        // Выбираем алгоритм отображения
        hasher.option = if button.is_active() {
            HashPreviewOption::Base64
        } else {
            HashPreviewOption::Hex
        };

        // Хэшируем текст и отображаем хэш в интерфейсе
        hasher.calculate();
        hash_value_label.set_label(&hasher.view());
    });

    // Запихиваем всё в окно и показываем его
    let window = ApplicationWindow::builder()
        .application(app)
        .title("SHA3 hasher")
        .child(&container)
        .build();
    window.present();
}
