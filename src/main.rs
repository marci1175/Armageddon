use macroquad::{prelude::*, ui::InputHandler};

static GAMESPEED: i32 = 1;
static TIMERESOLUTION: f32 = 1.;
struct BackgroundAnimation {
    bg: Texture2D,
    y1: f32,
    y2: f32,
}

impl BackgroundAnimation {
    fn new() -> Self {
        Self { bg: Texture2D::from_file_with_format(include_bytes!("../assets/bg.png"), Some(ImageFormat::Png)), y1: 0., y2: -1000. }
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

struct Spaceship {
    texture: Texture2D,
    x: f32,
    y: f32,
    children: Vec<Rocket>,
}

impl Spaceship {
    fn new() -> Self {
        Self { texture: Texture2D::from_file_with_format(include_bytes!("../assets/sor.png"), Some(ImageFormat::Png)), x: 0., y: 0., children: Vec::new() }
    }

    fn draw(&mut self) -> &mut Self {
        draw_texture(&self.texture, self.x, self.y, Color::new(255., 255., 255., 255.));

        self
    }

    fn movement(&mut self) -> &mut Self {
        if is_key_down(KeyCode::Left) {
            self.x -= (GAMESPEED * 3) as f32;
        }

        if is_key_down(KeyCode::Right) {
            self.x += (GAMESPEED * 3) as f32;
        }

        if is_key_down(KeyCode::Up) {
            self.y -= (GAMESPEED * 3) as f32;
        }

        if is_key_down(KeyCode::Down) {
            self.y += (GAMESPEED * 3) as f32;
        }

        self
    }

    //Only used to push children items
    fn shoot(&mut self, display: &Image) -> &mut Self {

        if is_key_down(KeyCode::Space) {
            self.children.push(Rocket::new(self.x, self.y, display))
        }

        self
    }

    fn children_lifetime(&mut self) -> &mut Self {
        for (index, child) in self.children.clone().iter().enumerate() {
            if 0. - child.texture.height() > child.y {
                self.children.remove(index);
            }
        }
        
        self
    }
}

#[derive(Clone)]
struct Rocket {
    texture: Texture2D,
    x: f32,
    y: f32,
    rocket_liftime: f32,
}

impl Rocket {
    fn new(x: f32, y: f32, display: &Image) -> Self {
        Self {
            texture: Texture2D::from_file_with_format(include_bytes!("../assets/rocket.png"), Some(ImageFormat::Png)),
            x,
            y,
            rocket_liftime: display.height() as f32
        }
    }

    fn draw(&mut self) -> &mut Self {
        draw_texture(&self.texture, self.x, self.y, Color::new(255., 255., 255., 255.));

        self
    }

    fn animate(&mut self) -> &mut Self {
        self.y -= (6 * GAMESPEED) as f32;

        self
    }

    
}


#[macroquad::main("Spacebang")]
async fn main() {
    let screen_data = get_screen_data();
    
    let mut bg = BackgroundAnimation::new();
    
    let mut ship = Spaceship::new();


    //Main loop
    loop {
        bg.draw().animate(&screen_data);

        //spaceship
        ship.draw().movement().children_lifetime();

        //Check for shot
        ship.shoot(&screen_data);

        //Draw rockets
        for rocket in &mut ship.children {
            rocket.draw().animate();
        }

        //Draw debug
        #[cfg(debug_assertions)]
        {  
            draw_text(&format!("[DEBUG]"), 100., 70., 30., Color::new(255., 255., 255., 255.));
            draw_text(&format!("Ship: Children count: {}", ship.children.len()), 100., 100., 30., Color::new(255., 255., 255., 255.))
        }

        //Call on loop end
        next_frame().await
    }

}
