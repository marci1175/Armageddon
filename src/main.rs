use std::{any::Any, f32::consts::PI, mem::Discriminant, thread::sleep, time::Duration};

use macroquad::{miniquad::window::set_window_size, prelude::*, ui::InputHandler};

static GAMESPEED: i32 = 1;
static TIMERESOLUTION: f32 = 1.;
struct BackgroundAnimation {
    bg: Texture2D,
    y1: f32,
    y2: f32,
}

impl BackgroundAnimation {
    fn new() -> Self {
        Self {
            bg: Texture2D::from_file_with_format(
                include_bytes!("../assets/bg.png"),
                Some(ImageFormat::Png),
            ),
            y1: 0.,
            y2: -1000.,
        }
    }

    fn draw(&mut self) -> &mut Self {
        //Draw first tile
        draw_texture(&self.bg, 0., self.y2, Color::new(255., 255., 255., 255.));

        //Draw second tile
        draw_texture(&self.bg, 0., self.y1, Color::new(255., 255., 255., 255.));

        self
    }

    fn animate(&mut self, display: &Image) -> &mut Self {
        self.y1 += GAMESPEED as f32 * TIMERESOLUTION;
        self.y2 += GAMESPEED as f32 * TIMERESOLUTION;

        if self.y1 as usize > display.height() {
            self.y1 = -self.bg.height();
        }

        if self.y2 as usize > display.height() {
            self.y2 = -self.bg.height();
        }

        self
    }
}

#[derive(Clone)]
struct Hitbox {
    x: f32,
    y: f32,
    texture: Texture2D,
}

impl Hitbox {
    fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        Self { x, y, texture }
    }

    fn draw(&self) {
        //Draw hitbox
        #[cfg(debug_assertions)]
        {
            draw_line(
                self.x,
                self.y,
                self.x + self.texture.width(),
                self.y,
                2.,
                Color::from_rgba(255, 0, 0, 255),
            );
            draw_line(
                self.x + self.texture.width(),
                self.y,
                self.x + self.texture.width(),
                self.y + self.texture.height(),
                2.,
                Color::from_rgba(255, 0, 0, 255),
            );
            draw_line(
                self.x + self.texture.width(),
                self.y + self.texture.height(),
                self.x,
                self.y + self.texture.height(),
                2.,
                Color::from_rgba(255, 0, 0, 255),
            );
            draw_line(
                self.x,
                self.y + self.texture.height(),
                self.x,
                self.y,
                2.,
                Color::from_rgba(255, 0, 0, 255),
            );
        }
    }
}

impl PartialEq for Hitbox {
    fn eq(&self, other: &Self) -> bool {
        //Check for x
        if self.x < other.x + other.texture.width() && other.x <= self.x + self.texture.width() {
            //Check for y
            if self.y < other.y + other.texture.height()
                && other.y <= self.y + self.texture.height()
            {
                return true;
            }
        }
        false
    }
}

struct Spaceship {
    texture: Texture2D,
    hitbox: Hitbox,
    children: Vec<Rocket>,
}

impl Spaceship {
    fn new() -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/sor.png"),
            Some(ImageFormat::Png),
        );
        Self {
            texture: texture.clone(),
            children: Vec::new(),
            hitbox: Hitbox::new(0., 0., texture),
        }
    }

    fn draw(&mut self) -> &mut Self {
        draw_texture(
            &self.texture,
            self.hitbox.x,
            self.hitbox.y,
            Color::new(255., 255., 255., 255.),
        );

        //Draw owned children
        for rocket in &mut self.children {
            rocket.draw().animate();
        }

        //Only works in debug mode!!!!
        self.hitbox.draw();

        self
    }

    fn movement(&mut self, display: &Image) -> &mut Self {
        if is_key_down(KeyCode::Left) {
            self.hitbox.x -= (GAMESPEED * 3) as f32;
        }

        if is_key_down(KeyCode::Right) {
            self.hitbox.x += (GAMESPEED * 3) as f32;
        }

        if is_key_down(KeyCode::Up) {
            self.hitbox.y -= (GAMESPEED * 3) as f32;
        }

        if is_key_down(KeyCode::Down) {
            self.hitbox.y += (GAMESPEED * 3) as f32;
        }

        //Restrict movement
        self.hitbox.x = self.hitbox.x.clamp(
            0. - self.texture.width() / 3.,
            display.width() as f32 - self.texture.width() / 1.5,
        );
        self.hitbox.y = self.hitbox.y.clamp(
            0. - self.texture.height() / 11.,
            display.height() as f32 - self.texture.height(),
        );

        self
    }

    //Only used to push children items
    fn shoot(&mut self, display: &Image) -> &mut Self {
        if is_key_down(KeyCode::Space) {
            self.children.push(Rocket::new(
                self.hitbox.x,
                self.hitbox.y,
                display,
                0.,
            ));
        }

        self
    }

    fn children_lifetime(&mut self) -> &mut Self {
        for (index, child) in self.children.clone().iter().enumerate() {
            if 0. - child.texture.height() > child.hitbox.y || child.rocket_liftime < child.hitbox.y
            {
                self.children.remove(index);
            }
        }

        self
    }
}


#[derive(Clone)]
struct Rocket {
    texture: Texture2D,
    hitbox: Hitbox,
    angle: f32,
    rocket_liftime: f32,
}

impl Rocket {
    fn new(x: f32, y: f32, display: &Image, angle: f32) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/rocket.png"),
            Some(ImageFormat::Png),
        );
        Self {
            texture: texture.clone(),
            hitbox: Hitbox::new(x, y, texture),
            angle: angle,
            rocket_liftime: display.height() as f32,
        }
    }

    fn draw(&mut self) -> &mut Self {
        draw_texture_ex(
            &self.texture,
            self.hitbox.x,
            self.hitbox.y,
            Color::new(255., 255., 255., 255.),
            DrawTextureParams {
                rotation: (self.angle as f32).to_radians(),
                ..Default::default()
            },
        );

        //Only works in debug mode!!!
        self.hitbox.draw();

        self
    }

    fn animate(&mut self) -> &mut Self {
        //Calculate angle too
        self.hitbox.y -= (6 * GAMESPEED) as f32 * (self.angle as f32).to_radians().cos();

        self.hitbox.x += (6 * GAMESPEED) as f32 * (self.angle as f32).to_radians().sin();

        self
    }
}

struct Enemy {
    texture: Texture2D,
    hitbox: Hitbox,
    angle: f32,
    life: u8,
    children: Vec<Rocket>,
    rocket_cooldown: std::time::Instant,
}

impl Enemy {
    fn new() -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/cigan.png"),
            Some(ImageFormat::Png),
        );
        Self {
            texture: texture.clone(),
            hitbox: Hitbox::new(250., 250., texture),
            angle: 0.,
            life: 100,
            children: Vec::new(),
            rocket_cooldown: std::time::Instant::now(),
        }
    }

    fn draw(&mut self) -> &mut Self {
        draw_texture_ex(
            &self.texture,
            self.hitbox.x,
            self.hitbox.y,
            Color::new(255., 255., 255., 255.),
            DrawTextureParams {
                rotation: self.angle.to_radians(),
                ..Default::default()
            },
        );

        //Draw owned children
        for rocket in &mut self.children {
            rocket.draw().animate();
        }

        //Draw healthbar
        draw_line(
            self.hitbox.x,
            self.hitbox.y + self.hitbox.texture.height(),
            self.hitbox.x
                + self.life as f32 * (self.hitbox.texture.width() / (self.hitbox.x + 100.)),
            self.hitbox.y + self.hitbox.texture.height(),
            10.,
            Color::from_rgba(255, 0, 0, 255),
        );

        //Show health
        draw_text(
            &self.life.to_string(),
            self.hitbox.x,
            self.hitbox.y + self.hitbox.texture.height(),
            30.,
            Color::from_rgba(255, 255, 255, 255),
        );

        //Draw hitbox
        self.hitbox.draw();

        self
    }

    fn movement(&mut self, ship_pos: &Hitbox, display: &Image) -> &mut Self {
        self.angle = {
            let x_diff = self.hitbox.x - ship_pos.x;
            let y_diff = self.hitbox.y - ship_pos.y;

            let rad = y_diff.atan2(x_diff);

            rad
        }
        .to_degrees();

        //Restrict movement
        self.hitbox.x = self.hitbox.x.clamp(
            0. - self.texture.width() / 3.,
            display.width() as f32 - self.texture.width() / 1.5,
        );
        self.hitbox.y = self.hitbox.y.clamp(
            0. - self.texture.height() / 11.,
            display.height() as f32 - self.texture.height(),
        );

        if self.rocket_cooldown.elapsed() > std::time::Duration::from_secs(5) {
            self.children.push(Rocket::new(
                self.hitbox.x,
                self.hitbox.y,
                display,
                self.angle - 90.,
            ));

            //Reset timer
            self.rocket_cooldown = std::time::Instant::now();
        }

        self
    }

    fn children_lifetime(&mut self) -> &mut Self {
        for (index, child) in self.children.clone().iter().enumerate() {
            if 0. - child.texture.height() > child.hitbox.y || child.rocket_liftime < child.hitbox.y
            {
                self.children.remove(index);
            }
        }

        self
    }
}

#[macroquad::main("Armageddon: A végső leszámolás")]
async fn main() {
    let screen_data = get_screen_data();

    let mut bg = BackgroundAnimation::new();

    let mut ship = Spaceship::new();

    let mut enemy = Enemy::new();

    //Main loop
    loop {
        bg.draw().animate(&screen_data);

        //spaceship
        ship.draw().movement(&screen_data).children_lifetime();

        //Check for shot
        ship.shoot(&screen_data);

        enemy
            .draw()
            .movement(&ship.hitbox, &screen_data)
            .children_lifetime();

        //Draw debug
        #[cfg(debug_assertions)]
        {
            let font_size = 20.;
            let debug = vec![
                format!("[DEBUG]"),
                format!("Ship: Children count: {}", ship.children.len()),
                format!("Enemy: Hp count: {}", enemy.life),
                format!("Fps: {}", get_fps()),
                format!("Enemy: Children count: {}", enemy.children.len()),
                format!("Enemy: angle target: {}", enemy.angle),
            ];

            for (index, debug_item) in debug.iter().enumerate() {
                draw_text(
                    &debug_item,
                    0.,
                    30. + index as f32 * font_size,
                    font_size,
                    Color::new(255., 255., 255., 255.),
                )
            }
        }

        //Check for collision
        for (index, child) in ship.children.clone().iter().enumerate() {
            if child.hitbox == enemy.hitbox {
                enemy.life -= 1;

                //Remove rockets which hit the enemy
                ship.children.remove(index);
            }
        }

        //Call on loop end
        next_frame().await
    }
}
