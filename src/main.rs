use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 64.0;
const APPLE_SIZE: f32 = 60.0;
const APPLE_SPEED: f32 = 95.0;
const PLAYER_SPEED: f32 = 230.0;

#[derive(Clone)]
struct Apple {
    x: f32,
    y: f32,
}

struct Game {
    player_x: f32,
    player_y: f32,
    apples: Vec<Apple>,
    score: i32,
    game_over: bool,
    player_texture: Texture2D,
    ap: Texture2D,
    ba: Texture2D,
}

impl Game {
    async fn new() -> Self {
        
        let player_texture = load_texture("assets/b.png")
            .await
            .expect("Failed to load player texture!");
        
        let ap: macroquad::texture::Texture2D = load_texture("assets/a.png")
            .await
            .expect("Failed to load player texture!");
            
        let ba: macroquad::texture::Texture2D = load_texture("assets/ba.png")
            .await
            .expect("Failed to load back texture!");
        Self {
            player_x: screen_width() / 2.0 - PLAYER_SIZE / 2.0,
            player_y: screen_height() - 80.0,
            apples: Vec::new(),
            score: 0,
            game_over: false,
            player_texture,
            ap,
            ba,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        
        let mut dx = 0.0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dx -= 3.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dx += 3.0;
        }

        self.player_x += dx * PLAYER_SPEED * dt;
        self.player_x = self.player_x.clamp(0.0, screen_width() - PLAYER_SIZE);

        
        if rand::gen_range(0, 170) < 3 {
            self.apples.push(Apple {
                x: rand::gen_range(0.0, screen_width() - APPLE_SIZE),
                y: 0.0,
            });
        }

        
        for apple in &mut self.apples {
            apple.y += APPLE_SPEED * dt;
        }

        
        let mut new_apples = Vec::new();
        for apple in &self.apples {
            if apple.y + APPLE_SIZE > self.player_y
                && apple.y < self.player_y + PLAYER_SIZE
                && apple.x + APPLE_SIZE > self.player_x
                && apple.x < self.player_x + PLAYER_SIZE
            {
                self.score += 1;
                continue;
            }

            if apple.y > screen_height() {
                self.game_over = true;
            }

            new_apples.push(apple.clone());
        }
        self.apples = new_apples;
    }

    fn draw(&self) {
        draw_texture_ex(
            &self .ba,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            }
        );

        
        draw_texture_ex(
            &self.player_texture,
            self.player_x,
            self.player_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PLAYER_SIZE, 96.0)),
                ..Default::default()
            }
        );

        
        for apple in &self.apples {
            draw_texture_ex(
                &self .ap,
                apple.x,
                apple.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(APPLE_SIZE, APPLE_SIZE)),
                    ..Default::default()
                }
            );
        }

        
        draw_text(
            &format!("Score: {}", self.score),
            20.0,
            40.0,
            30.0,
            BLACK,
        );
        
        if self.game_over {
            draw_text(
                "Game Over! Press R to restart",
                screen_width() / 2.0 - 180.0,
                screen_height() / 2.0,
                30.0,
                BLACK,
            );
        }
    }

    fn reset(&mut self) {
        self.player_x = screen_width() / 2.0 - PLAYER_SIZE / 2.0;
        self.player_y = screen_height() - 80.0;
        self.apples.clear();
        self.score = 0;
        self.game_over = false;
    }
}

#[macroquad::main("hungry bird")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::R) {
            game.reset();
        }


        game.update(dt);
        game.draw();

        next_frame().await
    }
}
