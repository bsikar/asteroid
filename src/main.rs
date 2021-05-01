use macroquad::prelude::*;

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

    fn draw(self) {
        draw_circle(self.position.x, self.position.y, BULLET_SIZE, BLACK);
    }

    fn update(&mut self) {
        // spacing of bullets
        let rotation = self.rotation.to_radians();
        self.position.y += rotation.cos() * rand::gen_range(-30., -25.);
        self.position.x += rotation.sin() * rand::gen_range(25., 30.);

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

#[allow(dead_code)]
struct Asteroid {
    position: Vec2,
    rotation: f32,
    sides: u8,
    collided: bool,
}

impl Asteroid {}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut ship = Ship::new();

    loop {
        clear_background(LIGHTGRAY);

        ship.draw();
        ship.mv();

        next_frame().await;
    }
}
