#![allow(non_snake_case)]
use macroquad::{audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound}, prelude::*};

//Default values
static SHIP_HEALTH: f32 = 3.;
static ENEMY_HEALTH: f32 = 100.;
static GAMESPEED: f32 = 1.;
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

    fn animate(&mut self) -> &mut Self {
        self.y1 += GAMESPEED * TIMERESOLUTION;
        self.y2 += GAMESPEED * TIMERESOLUTION;

        if self.y1 > screen_height() {
            self.y1 = -self.bg.height();
        }

        if self.y2 > screen_height() {
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
    life: f32,
}

impl Spaceship {
    fn new(x: f32, y: f32) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/sor.png"),
            Some(ImageFormat::Png),
        );
        Self {
            texture: texture.clone(),
            children: Vec::new(),
            //Spawn to the middle of the window
            hitbox: Hitbox::new(x - texture.width() / 2., y, texture),
            life: SHIP_HEALTH,
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

        //Draw health bar
        draw_line(550., 550., 550. + (self.life * 150. / SHIP_HEALTH), 550., 30., RED);
        //Draw health text
        draw_text(&format!("Health: {}", self.life), 550., 560., 30., WHITE);
        self
    }

    fn movement(&mut self) -> &mut Self {
        if is_key_down(KeyCode::Left) {
            self.hitbox.x -= GAMESPEED * 3.;
        }

        if is_key_down(KeyCode::Right) {
            self.hitbox.x += GAMESPEED * 3.;
        }

        if is_key_down(KeyCode::Up) {
            self.hitbox.y -= GAMESPEED * 3.;
        }

        if is_key_down(KeyCode::Down) {
            self.hitbox.y += GAMESPEED * 3.;
        }

        //Restrict movement
        self.hitbox.x = self.hitbox.x.clamp(
            0. - self.texture.width() / 3.,
            screen_width() as f32 - self.texture.width() / 1.5,
        );
        self.hitbox.y = self.hitbox.y.clamp(
            0. - self.texture.height() / 11.,
            screen_height() as f32 - self.texture.height(),
        );

        self
    }

    //Only used to push children items
    fn shoot(&mut self) -> &mut Self {
        if is_key_pressed(KeyCode::Space) {
            self.children.push(Rocket::new(
                self.hitbox.x,
                self.hitbox.y,
                screen_height(),
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
    fn new(x: f32, y: f32, display_height: f32, angle: f32) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/rocket.png"),
            Some(ImageFormat::Png),
        );
        Self {
            texture: texture.clone(),
            hitbox: Hitbox::new(x, y, texture),
            angle: angle,
            rocket_liftime: display_height,
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

        //Only works in debug mode!!!
        self.hitbox.draw();

        self
    }

    fn animate(&mut self) -> &mut Self {
        //Calculate angle too
        self.hitbox.y -= 6. * GAMESPEED * self.angle.to_radians().cos();

        self.hitbox.x += 6. * GAMESPEED * self.angle.to_radians().sin();

        self
    }
}

struct Enemy {
    texture: Texture2D,
    hitbox: Hitbox,
    angle: f32,
    life: f32,
    children: Vec<Rocket>,
    rocket_cooldown: std::time::Instant,

    movement: MovementState,
}

impl Enemy {
    fn new(x: f32, y: f32) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/cigan.png"),
            Some(ImageFormat::Png),
        );
        Self {
            texture: texture.clone(),
            hitbox: Hitbox::new(x - texture.width() / 2., y, texture),
            angle: 0.,
            life: ENEMY_HEALTH,
            children: Vec::new(),
            rocket_cooldown: std::time::Instant::now(),

            movement: MovementState { right: true, up: false, step: 1. },
        }
    }

    fn draw(&mut self) -> &mut Self {
        draw_texture_ex(
            &self.texture,
            self.hitbox.x,
            self.hitbox.y,
            Color::new(255., 255., 255., 255.),
            DrawTextureParams {
                rotation: (self.angle  + 90.).to_radians(),
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
                + self.life * self.hitbox.texture.width() / 100.,
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

    fn movement(&mut self, ship_pos: &Hitbox) -> &mut Self {
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
            screen_width() as f32 - self.texture.width() / 1.5,
        );
        self.hitbox.y = self.hitbox.y.clamp(
            0. - self.texture.height() / 11.,
            screen_height() as f32 - self.texture.height(),
        );

        //Implement bot movement
        if self.movement.up {
            self.hitbox.y += self.movement.step;
        } else if !self.movement.up {
            self.hitbox.y -= self.movement.step;
        }

        if self.movement.right {
            self.hitbox.x -= self.movement.step;
        } else if !self.movement.right {
            self.hitbox.x += self.movement.step;
        }

        if self.rocket_cooldown.elapsed() > std::time::Duration::from_secs_f32(3. / (GAMESPEED * TIMERESOLUTION)) {
            self.children.push(Rocket::new(
                self.hitbox.x,
                self.hitbox.y,
                screen_height(),
                self.angle - 90.,
            ));

            //Reset timer
            self.rocket_cooldown = std::time::Instant::now();
            self.movement.step = rand::gen_range(2., 5.);

            
        }

        if (rand::gen_range(0., 100.) % 3.) == 0. {
            self.movement.up = !self.movement.up;
        }

        if (rand::gen_range(0., 100.) % 2.) == 0. {
            self.movement.right = !self.movement.right;
        }

        //Restrict movement
        self.hitbox.x = self.hitbox.x.clamp(
            0. - self.texture.width() / 3.,
            screen_width() as f32 - self.texture.width() / 1.5,
        );
        self.hitbox.y = self.hitbox.y.clamp(
            0. - self.texture.height() / 11.,
            screen_height() as f32 - self.texture.height(),
        );

        //Implement bounciness
        if self.hitbox.x == 0. - self.texture.width() / 3. {
            self.movement.right = !self.movement.right;
        } else if self.hitbox.x == screen_width() as f32 - self.texture.width() / 1.5 {
            self.movement.right = !self.movement.right;
        }

        if self.hitbox.y == 0. - self.texture.height() / 11. {
            self.movement.up = !self.movement.up;
        } else if self.hitbox.y == screen_height() as f32 - self.texture.height() {
            self.movement.up = !self.movement.up;
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
struct MovementState {
    right: bool,
    up: bool,

    step: f32,
}

#[macroquad::main("Armageddon: A végső leszámolás")]
async fn main() {
    
    let mut bg = BackgroundAnimation::new();

    let mut ship = Spaceship::new(screen_width() as f32 / 2., screen_height() as f32);

    let mut enemy = Enemy::new(screen_width() as f32 / 2., 0.);

    //Main loop
    loop {
        bg.draw().animate();

        //spaceship
        ship.draw().movement().children_lifetime();

        //Check for shot
        ship.shoot();

        enemy
            .draw()
            .movement(&ship.hitbox)
            .children_lifetime();

        //Draw debug
        #[cfg(debug_assertions)]
        {
            let font_size = 20.;
            let debug = vec![
                format!("[DEBUG]"),
                format!("Fps: {}", get_fps()),
                format!("Ship: Children count: {}", ship.children.len()),
                format!("Ship: Life count: {}", ship.life),
                format!("Enemy: Hp count: {}", enemy.life),
                format!("Enemy: Children count: {}", enemy.children.len()),
                format!("Enemy: target angle: {}", enemy.angle),
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
                enemy.life -= 1.;

                //Remove rockets which hit the enemy
                if let Some(_) = ship.children.get(index) {
                    ship.children.remove(index);
                }
            }
        }
        for (index, child) in enemy.children.clone().iter().enumerate() {
            if child.hitbox == ship.hitbox {
                //Remove rockets which hit the enemy
                ship.life -= 1.;

                enemy.children.remove(index);
            }
        }

        //Check life
        if enemy.life <= 0. {
            match &load_sound_from_bytes(include_bytes!("../assets/win.ogg")).await {
                Ok(sound_bytes) => {
                    play_sound(sound_bytes, PlaySoundParams { looped: false, volume: 100.});
                }
                Err(err) => {}
            }
            loop {
                
                draw_text(r#""Ide bejönnének a magyar gárdák szétbombáznánk őket." -Leonidas"#, screen_width() / 8., screen_height() / 2., 25., WHITE);
                
                next_frame().await
            }
        }  

        if ship.life <= 0. {
            match &load_sound_from_bytes(include_bytes!("../assets/lose.ogg")).await {
                Ok(sound_bytes) => {
                    play_sound(sound_bytes, PlaySoundParams { looped: false, volume: 100.});
                }
                Err(err) => {}
            }
            loop {
                
                draw_text(r#"Szétbombáztak a magyar gárdák"#, screen_width() / 8., screen_height() / 2., 25., WHITE);
                
                next_frame().await
            }
        }  

        //Call on loop end
        next_frame().await
    }
}
