use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

struct Ship {
    position: Vec2,
    rotation: f32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rotation: 0.,
        }
    }

    fn draw(&self) {
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
    }

    fn mv(&mut self) {
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            let rotation = self.rotation.to_radians();

            self.position.y += rotation.cos() * -3.;
            self.position.x += rotation.sin() * 3.;
        }

        if is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
            // This will shoot in the future
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rotation -= 5.;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
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
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut ship = Ship::new();

    loop {
        clear_background(LIGHTGRAY);

        ship.draw();
        ship.mv();

        next_frame().await
    }
}
