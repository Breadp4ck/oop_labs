use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, SystemTime},
};

use macroquad::prelude::*;

const DELTA: f32 = 1. / 60.; // Физический тик
const SPEED: f32 = 200.0; // Скорость перемещения звезды

/// Метод draw используется для отрисовки объекта на экран,
/// а значит, эта функция должна вызываться только из оснвоного потока.
/// В принципе, он нужен для того, чтобы хоть как-то отделить логику
/// локального потока и основного. Только основной поток может напрямую
/// использовать macroquad (библиотека для создания игр).
trait Drawable {
    fn draw(&self);
}

/// Планета, которая вращается вокург звезды [Star]
#[derive(Clone, Copy)]
pub struct Planet {
    position: Vec2,
    offset: Vec2,
    time: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
}

impl Planet {
    pub fn new_at(position: Vec2) -> Self {
        Self {
            position,
            offset: Vec2::ZERO,
            amplitude: 100.,
            speed: 1.,
            radius: 10.,
            time: 0.,
            color: GRAY,
        }
    }

    /// Обновление состояния, локальный поток
    pub fn update(&mut self, delta: f32) {
        self.time += delta * self.speed;

        self.offset.x = self.amplitude * self.time.sin();
        self.offset.y = self.amplitude * self.time.cos();
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.position = center;
    }
}

impl Drawable for Planet {
    fn draw(&self) {
        draw_circle(
            self.position.x + self.offset.x,
            self.position.y + self.offset.y,
            self.radius,
            self.color,
        );
    }
}

#[derive(Default)]
pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

pub struct Star {
    velocity: Vec2,
    position: Vec2,
}

impl Star {
    pub fn new_at(position: Vec2) -> Self {
        Self {
            velocity: Vec2::ZERO,
            position,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Обработка ввода
    pub fn handle_input(&mut self, input: &Input) {
        let mut direction = Vec2::ZERO;

        if input.up {
            direction.y -= 1.0;
        }
        if input.down {
            direction.y += 1.0;
        }
        if input.left {
            direction.x -= 1.0;
        }
        if input.right {
            direction.x += 1.0;
        }

        self.velocity = direction * SPEED;
    }

    /// Обновление состояния
    pub fn update(&mut self, delta: f32) {
        self.position += delta * self.velocity;
    }
}

impl Drawable for Star {
    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 25.0, YELLOW);
    }
}

/// Подождать один физический тик
pub fn physics_tick() -> f32 {
    let now = SystemTime::now();
    sleep(Duration::from_secs_f32(DELTA));

    let delta = now.elapsed().unwrap();

    delta.as_secs_f32()
}

#[macroquad::main("Star System")]
async fn main() {
    // Создание объектов и оборачивание их в структуры данных, предназначенные
    // для общения между разными потоками (Arc и Mutex)

    let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

    let input = Arc::new(Mutex::new(Input::default()));
    let input_local = input.clone();

    let star = Arc::new(Mutex::new(Star::new_at(center)));
    let star_local = star.clone();

    let mut planets = [
        Planet::new_at(center + Vec2::new(50., 0.)),
        Planet::new_at(center - Vec2::new(50., 0.)),
        Planet::new_at(center + Vec2::new(0., 50.)),
        Planet::new_at(center - Vec2::new(0., 50.)),
    ];

    planets[0].amplitude = 90.;
    planets[1].amplitude = 120.;
    planets[2].amplitude = 145.;
    planets[3].amplitude = 195.;

    planets[0].speed = 1.81;
    planets[1].speed = 1.63;
    planets[2].speed = 1.31;
    planets[3].speed = 1.17;

    planets[0].radius = 8.0;
    planets[1].radius = 10.0;
    planets[2].radius = 13.0;
    planets[3].radius = 18.0;

    planets[0].color = DARKPURPLE;
    planets[1].color = VIOLET;
    planets[2].color = SKYBLUE;
    planets[3].color = DARKGREEN;

    let planets = [
        Arc::new(Mutex::new(planets[0])),
        Arc::new(Mutex::new(planets[1])),
        Arc::new(Mutex::new(planets[2])),
        Arc::new(Mutex::new(planets[3])),
    ];

    // Отдельные потоки для физики Planet

    for planet in &planets {
        let planet = planet.clone();
        let star = star_local.clone();

        std::thread::spawn(move || loop {
            let delta = physics_tick();

            planet
                .lock()
                .unwrap()
                .set_center(star.lock().unwrap().position());
            planet.lock().unwrap().update(delta);
        });
    }

    // Отдельный поток для физики Star

    std::thread::spawn(|| {
        let star = star_local;
        let input = input_local;

        loop {
            let delta = physics_tick();

            let mut star = star.lock().unwrap();
            let input = input.lock().unwrap();

            star.handle_input(&input);
            star.update(delta);
        }
    });

    // Основной игровой цикл

    loop {
        clear_background(Color::new(0.1, 0.05, 0.15, 1.0));

        {
            let mut input = input.lock().unwrap();
            *input = Input {
                up: is_key_down(KeyCode::Up),
                down: is_key_down(KeyCode::Down),
                left: is_key_down(KeyCode::Left),
                right: is_key_down(KeyCode::Right),
            };
        }

        star.lock().unwrap().draw();

        for planet in &planets {
            planet.lock().unwrap().draw();
        }

        next_frame().await
    }
}
