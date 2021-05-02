use ::rand::{self, Rng};
use macroquad::prelude::*;
use std::collections::HashSet;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

struct Ship {
    position: Vec2,
    rotation: f32,
    bullets: Vec<Bullet>,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rotation: 0.,
            bullets: vec![],
        }
    }

    fn draw(&mut self) {
        let rotation = self.rotation.to_radians();

        let v1 = Vec2::new(
            self.position.x + rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );

        let v2 = Vec2::new(
            self.position.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );

        let v3 = Vec2::new(
            self.position.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );

        draw_triangle_lines(v1, v2, v3, 2., BLACK);

        self.bullets.iter_mut().for_each(|b| b.update());
        self.bullets.iter_mut().for_each(|b| b.draw());
        // range of bullets
        self.bullets
            .retain(|b| get_time() - b.time_shot_out < 0.25 && !b.collided);
    }

    fn mv(&mut self) {
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            let rotation = self.rotation.to_radians();

            self.position.y += rotation.cos() * -4.;
            self.position.x += rotation.sin() * 4.;
        }

        if is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
            self.shoot();
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rotation -= 5.;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.rotation += 5.;
        }

        // wraping:
        if self.position.x > screen_width() {
            self.position.x = 0.;
        } else if self.position.x < 0. {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.;
        } else if self.position.y < 0. {
            self.position.y = screen_height();
        }
    }

    fn shoot(&mut self) {
        self.bullets.push(Bullet::new(self.position, self.rotation));
    }
}

const BULLET_SIZE: f32 = 2.;

#[derive(Copy, Clone)]
struct Bullet {
    position: Vec2,
    rotation: f32,
    time_shot_out: f64,
    collided: bool,
}

impl Bullet {
    fn new(position: Vec2, rotation: f32) -> Bullet {
        Bullet {
            position,
            rotation,
            time_shot_out: get_time(),
            collided: false,
        }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, BULLET_SIZE, BLACK);
    }

    fn update(&mut self) {
        // spacing of bullets
        let rotation = self.rotation.to_radians();
        self.position.y += rotation.cos() * rand::thread_rng().gen_range(-30.0..-25.);
        self.position.x += rotation.sin() * rand::thread_rng().gen_range(25.0..30.);
    }
}

const ASTEROID_LINE_THICKNESS: f32 = 2.;

struct Asteroid {
    position: Vec2,
    sides: u8,
    size: f32,
    rotation: f32,
}

impl Asteroid {
    fn new() -> Asteroid {
        Asteroid {
            position: Vec2::new(
                rand::thread_rng().gen_range(35.0..screen_width() - 35.),
                rand::thread_rng().gen_range(35.0..screen_height() - 35.),
            ),
            sides: 8,
            size: 100.,
            rotation: rand::thread_rng().gen_range(-360.0..360.),
        }
    }

    fn draw(&self) {
        draw_poly_lines(
            self.position.x,
            self.position.y,
            self.sides,
            self.size,
            self.rotation,
            ASTEROID_LINE_THICKNESS,
            BLACK,
        )
    }

    fn collided(&self, position: &Vec2) -> bool {
        if (self.position - *position).length() < self.size {
            return true;
        }
        false
    }

    fn mv(&mut self) {
        let rotation = self.rotation.to_radians();

        self.position.y += rotation.cos() * -4.;
        self.position.x += rotation.sin() * 4.;

        // wraping:
        if self.position.x > screen_width() {
            self.position.x = 0.;
        } else if self.position.x < 0. {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.;
        } else if self.position.y < 0. {
            self.position.y = screen_height();
        }
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    loop {
        if !play().await {
            break;
        }
    }
}

const FONT_SIZE: f32 = 30.;
async fn play() -> bool {
    let mut ship = Ship::new();
    let mut asteroids: Vec<_> = (0..10).map(|_| Asteroid::new()).collect();
    let mut did_win = true;

    loop {
        if is_key_down(KeyCode::Escape) {
            return false;
        }

        clear_background(LIGHTGRAY);

        if asteroids.is_empty() && did_win {
            let text = "You Win!. Press [enter] to play again.";
            let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                FONT_SIZE,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                return true;
            }
        } else if asteroids.is_empty() {
            let text = "Game Over. Press [enter] to play again.";
            let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                FONT_SIZE,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                return true;
            }
        }

        let mut indexes_to_remove = HashSet::new();
        for (i, asteroid) in asteroids.iter().enumerate() {
            for bullet in ship.bullets.iter() {
                if asteroid.collided(&bullet.position) {
                    indexes_to_remove.insert(i);
                }
            }

            if asteroid.collided(&ship.position) {
                did_win = false;
            }
        }

        if !did_win {
            asteroids = vec![];
        } else {
            let mut num_removed = 0;
            for i in indexes_to_remove {
                asteroids.remove(i - num_removed);
                num_removed += 1;
            }
        }

        ship.draw();
        ship.mv();
        asteroids.iter().for_each(|a| a.draw());
        asteroids.iter_mut().for_each(|a| a.mv());

        next_frame().await;
    }
}
