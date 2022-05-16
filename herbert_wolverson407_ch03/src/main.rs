use bracket_lib::prelude::*;

#[derive(Copy, Clone)]
enum StartMode {
    New,
    Failed,
    Succeeded,
}

enum GameMode {
    Menu(StartMode),
    Playing,
    End(StartMode),
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn tick(&mut self, ctx: &mut BTerm) {
        if ctx.key == Some(VirtualKeyCode::Space) {
            self.velocity = -2.0;
        } else if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.x += 1;
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }

        self.render(ctx);
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn try_complete(&self, ctx: &BTerm) -> Option<StartMode> {
        let (w, h) = ctx.get_char_size();
        if self.y >= 0 && self.y as u32 >= h {
            Some(StartMode::Failed)
        } else if self.x >= 0 && self.x as u32 >= w {
            Some(StartMode::Succeeded)
        } else {
            None
        }
    }
}

struct State {
    player: Player,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(0, 0),
            mode: GameMode::Menu(StartMode::New),
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm, start_mode: StartMode) {
        ctx.cls();
        ctx.print_centered(
            5,
            match start_mode {
                StartMode::New => "Welcome to Flappy Dragon",
                StartMode::Failed => "You are dead!",
                StartMode::Succeeded => "You did it!",
            },
        );
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Q => ctx.quitting = true,
                VirtualKeyCode::P => self.restart(ctx),
                _ => {}
            }
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player = Player::new(0, 0);
        self.mode = GameMode::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.player.tick(ctx);
        if let Some(start_mode) = self.player.try_complete(ctx) {
            self.mode = GameMode::End(start_mode)
        }
    }

    fn ended(&mut self, _ctx: &mut BTerm, start_mode: StartMode) {
        self.mode = GameMode::Menu(start_mode)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu(start_mode) => self.main_menu(ctx, start_mode),
            GameMode::Playing => self.play(ctx),
            GameMode::End(start_mode) => self.ended(ctx, start_mode),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
