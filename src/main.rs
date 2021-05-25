use ggez::{Context, GameResult, conf, graphics, audio::{self, SoundSource,}, event::{self, KeyCode, KeyMods}};
use std::{env, path};
use rand::Rng;

const TETRIS_WIDTH : usize = 9;
const TETRIS_HEIGHT: usize = 16;
const TETRIS_SIZE  : f32   = 50.0;
const TIME_SPAN    : u32   = 30; // 60フレーム
const NONE_SCALE   : f32   = 0.8;
const TETRIS_RECT: graphics::Rect = graphics::Rect::new(50.0, 220.0, TETRIS_SIZE * TETRIS_WIDTH as f32, TETRIS_SIZE * TETRIS_HEIGHT as f32);
const WINDOW_RECT: graphics::Rect = graphics::Rect::new(0.0, 0.0, TETRIS_RECT.w + 100.0, TETRIS_RECT.h + 300.0);

// 5x5のブロックから情報を入れる
// 中央を(0,0) 左上(-2,-2) 右下(2,2)
const BLOCK_TYPE_MAX : u8    = 7;
const BLOCK_TYPE_I   : u8    = 0;
const BLOCK_NUM      : usize = 4;
const ROTATION_MAX   : usize = 4;
const BLOCK : [[[cgmath::Point2<i8>; BLOCK_NUM]; ROTATION_MAX]; BLOCK_TYPE_MAX as usize] = 
[
    // I
    [
        [cgmath::Point2::new(0, -2), cgmath::Point2::new(0, -1), cgmath::Point2::new(0, 0), cgmath::Point2::new(0, 1)],
        [cgmath::Point2::new(-2, 0), cgmath::Point2::new(-1, 0), cgmath::Point2::new(0, 0), cgmath::Point2::new(1, 0)],
        [cgmath::Point2::new(0, -1), cgmath::Point2::new(0,  0), cgmath::Point2::new(0, 1), cgmath::Point2::new(0, 2)],
        [cgmath::Point2::new(-1, 0), cgmath::Point2::new(0,  0), cgmath::Point2::new(1, 0), cgmath::Point2::new(2, 0)]
    ],
    // L
    [
        [cgmath::Point2::new( 0, -1), cgmath::Point2::new(0, 0), cgmath::Point2::new( 0,  1), cgmath::Point2::new( 1,  1)],
        [cgmath::Point2::new(-1,  0), cgmath::Point2::new(0, 0), cgmath::Point2::new( 1,  0), cgmath::Point2::new( 1, -1)],
        [cgmath::Point2::new( 0,  1), cgmath::Point2::new(0, 0), cgmath::Point2::new( 0, -1), cgmath::Point2::new(-1, -1)],
        [cgmath::Point2::new( 1,  0), cgmath::Point2::new(0, 0), cgmath::Point2::new(-1,  0), cgmath::Point2::new(-1,  1)]
    ],
    // 逆L
    [
        [cgmath::Point2::new( 0, -1), cgmath::Point2::new(0, 0), cgmath::Point2::new( 0,  1), cgmath::Point2::new(-1,  1)],
        [cgmath::Point2::new(-1,  0), cgmath::Point2::new(0, 0), cgmath::Point2::new( 1,  0), cgmath::Point2::new( 1,  1)],
        [cgmath::Point2::new( 0,  1), cgmath::Point2::new(0, 0), cgmath::Point2::new( 0, -1), cgmath::Point2::new( 1, -1)],
        [cgmath::Point2::new( 1,  0), cgmath::Point2::new(0, 0), cgmath::Point2::new(-1,  0), cgmath::Point2::new(-1, -1)]
    ],
    // Z
    [
        [cgmath::Point2::new(-1, -1), cgmath::Point2::new(0, -1), cgmath::Point2::new( 0, 0), cgmath::Point2::new( 1, 0)],
        [cgmath::Point2::new( 0, -1), cgmath::Point2::new(0,  0), cgmath::Point2::new(-1, 0), cgmath::Point2::new(-1, 1)],
        [cgmath::Point2::new(-1,  0), cgmath::Point2::new(0,  0), cgmath::Point2::new( 0, 1), cgmath::Point2::new( 1, 1)],
        [cgmath::Point2::new( 1, -1), cgmath::Point2::new(1,  0), cgmath::Point2::new( 0, 0), cgmath::Point2::new( 0, 1)]
    ],
    //逆Z
    [
        [cgmath::Point2::new( 1, -1), cgmath::Point2::new( 0,  0), cgmath::Point2::new( 0,-1), cgmath::Point2::new(-1, 0)],
        [cgmath::Point2::new(-1, -1), cgmath::Point2::new(-1,  0), cgmath::Point2::new( 0, 0), cgmath::Point2::new( 0, 1)],
        [cgmath::Point2::new( 1,  0), cgmath::Point2::new( 0,  1), cgmath::Point2::new( 0, 0), cgmath::Point2::new(-1, 1)],
        [cgmath::Point2::new( 0, -1), cgmath::Point2::new( 0,  0), cgmath::Point2::new( 1, 0), cgmath::Point2::new( 1, 1)]
    ],
    //凸
    [
        [cgmath::Point2::new( 0, -1), cgmath::Point2::new(-1,  0), cgmath::Point2::new(0, 0), cgmath::Point2::new( 1,  0)],
        [cgmath::Point2::new(-1,  0), cgmath::Point2::new( 0, -1), cgmath::Point2::new(0, 0), cgmath::Point2::new( 0,  1)],
        [cgmath::Point2::new( 1,  0), cgmath::Point2::new( 0,  1), cgmath::Point2::new(0, 0), cgmath::Point2::new(-1,  0)],
        [cgmath::Point2::new( 0,  1), cgmath::Point2::new( 1,  0), cgmath::Point2::new(0, 0), cgmath::Point2::new( 0, -1)]
    ],
    //四角
    [
        [cgmath::Point2::new(0, -1), cgmath::Point2::new(-1, -1), cgmath::Point2::new(-1, 0), cgmath::Point2::new(0, 0)],
        [cgmath::Point2::new(0, -1), cgmath::Point2::new(-1, -1), cgmath::Point2::new(-1, 0), cgmath::Point2::new(0, 0)],
        [cgmath::Point2::new(0, -1), cgmath::Point2::new(-1, -1), cgmath::Point2::new(-1, 0), cgmath::Point2::new(0, 0)],
        [cgmath::Point2::new(0, -1), cgmath::Point2::new(-1, -1), cgmath::Point2::new(-1, 0), cgmath::Point2::new(0, 0)],
    ],
];

const BLOCK_STATE_NONE     : u8 = 0;
const BLOCK_STATE_GAMEOVER : u8 = BLOCK_TYPE_MAX + 1;
const BLOCK_STATE_MAX      : u8 = BLOCK_TYPE_MAX + 2; //NoneとGameover

enum KeyState{
    KeyChangeDown,
    KeyDown,
    KeyChangeUp,
    KeyUp,
}

impl PartialEq for KeyState{
    fn eq(&self, other: &Self) -> bool {
        let a = match self {
            KeyState::KeyChangeDown => 0,
            KeyState::KeyDown       => 1,
            KeyState::KeyChangeUp   => 2,
            KeyState::KeyUp         => 3,
        };

        let b = match other {
            KeyState::KeyChangeDown => 0,
            KeyState::KeyDown       => 1,
            KeyState::KeyChangeUp   => 2,
            KeyState::KeyUp         => 3,
        };

        a == b
    }
}

struct MainState{
    // 画像
    blockmesh       : [graphics::Mesh; BLOCK_STATE_MAX as usize],
    background      : graphics::Mesh,

    // ブロック情報
    blocks: [[u8; TETRIS_WIDTH]; TETRIS_HEIGHT],

    // プレイヤー情報
    control_point :cgmath::Point2<i8>,
    block_type    : u8,
    rotation      : usize,
    time          : u32,
    is_gameover   : bool,

    // bgm, se
    //bgm: audio::Source,
    //se : audio::Source,

    // キー入力
    up_state    : KeyState,
    right_state : KeyState,
    down_state  : KeyState,
    left_state  : KeyState,
    space_state : KeyState,
}

impl MainState{
    fn new(ctx:&mut Context) -> GameResult<MainState>{
        let background = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), TETRIS_RECT, graphics::Color::new(1.0, 1.0, 1.0, 1.0)).unwrap();
        let rect = graphics::Rect::new(TETRIS_SIZE*(1.0 - NONE_SCALE)/2.0, TETRIS_SIZE*(1.0 - NONE_SCALE)/2.0, TETRIS_SIZE*0.8, TETRIS_SIZE*0.8);
        let blockmesh : [graphics::Mesh; BLOCK_STATE_MAX as usize] = [
            // None
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), rect, graphics::Color::new(0.2, 0.2, 0.2, 1.0)).unwrap(),
            // Red
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap(),
            // Green
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(0.0, 1.0, 0.0, 1.0)).unwrap(),
            // Blue
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(0.0, 0.0, 1.0, 1.0)).unwrap(),
            // Red
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap(),
            // Green
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(0.0, 1.0, 0.0, 1.0)).unwrap(),
            // Blue
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(0.0, 0.0, 1.0, 1.0)).unwrap(),
            // Blue
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(0.0, 0.0, 1.0, 1.0)).unwrap(),
            // Gray
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(0.4, 0.4, 0.4, 1.0)).unwrap(),
        ];
        let blocks = [[BLOCK_STATE_NONE; TETRIS_WIDTH]; TETRIS_HEIGHT];
        let block_type = rand::thread_rng().gen_range(0..BLOCK_TYPE_MAX);
        let control_point = MainState::new_control_point(block_type);

        //let bgm  = audio::Source::new(ctx, "/○○.mp3")?;
        //let se   = audio::Source::new(ctx, "/○○.mp3")?;

        Ok(
            MainState{
                blockmesh,
                background,
                blocks,
                control_point,
                block_type,
                rotation   :0,
                time       :0,
                is_gameover: false,

                //bgm,
                //se,

                up_state    : KeyState::KeyUp,
                right_state : KeyState::KeyUp,
                down_state  : KeyState::KeyUp,
                left_state  : KeyState::KeyUp,
                space_state : KeyState::KeyUp,
            }
        )
    }

    fn check_touch_block(&self, control_point :cgmath::Point2<i8>, rotation : usize) -> bool{
        for i in 0..BLOCK_NUM {
            let block = BLOCK[self.block_type as usize][rotation as usize][i];
            let pos = control_point;
            let pos = cgmath::Point2::new(pos.x + block.x, pos.y + block.y);
            if pos.x < 0 || pos.x >= TETRIS_WIDTH  as i8 { return true; }
            if pos.y >= TETRIS_HEIGHT as i8 { return true; }
            if pos.y < 0 { continue; }
            if self.blocks[pos.y as usize][pos.x as usize] != BLOCK_STATE_NONE { return true; }
        }
        false
    }

    fn put_block(&mut self){
        for i in 0..BLOCK_NUM {
            let block = BLOCK[self.block_type as usize][self.rotation][i];
            let pos = self.control_point;
            let pos = cgmath::Point2::new(pos.x + block.x, pos.y + block.y);
            if pos.y < 0 { continue; }
            self.blocks[pos.y as usize][pos.x as usize] = self.block_type + 1;
        }
    }

    pub fn new_control_point(block_type: u8) -> cgmath::Point2<i8>{
        let mut control_point = cgmath::Point2::new((TETRIS_WIDTH/2) as i8, 1);
        match block_type {
            BLOCK_TYPE_I => { control_point.y = 2; }
            _ => {}
        }
        control_point
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.is_gameover {
            return Ok(());
        }

        // 時間経過
        self.time += 1;
        if self.time >= TIME_SPAN || self.space_state == KeyState::KeyChangeDown{
            let mut control_point = self.control_point.clone();
            control_point.y += 1;
            self.time = 0;

            if self.space_state == KeyState::KeyChangeDown {
                while self.check_touch_block(control_point, self.rotation as usize) == false{
                    control_point.y += 1;
                }
                self.control_point = control_point;
                self.control_point.y -= 1;
            }

            // 当たり判定
            if self.check_touch_block(control_point, self.rotation as usize) {
                // ブロックの処理
                self.put_block();

                // ライン判定
                let mut erased = false;
                for y in 0..TETRIS_HEIGHT {
                    let mut erase = true;
                    for x in 0..TETRIS_WIDTH {
                        if self.blocks[y][x] == BLOCK_STATE_NONE{
                            erase = false;
                        }
                    }
                    // ライン削除
                    if erase {
                        for x in 0..TETRIS_WIDTH{
                            self.blocks[y][x] = BLOCK_STATE_NONE;
                        }
                        // ブロックを下げる処理
                        for i in 0..y {
                            for x in 0..TETRIS_WIDTH {
                                self.blocks[y-i][x] = self.blocks[y-i-1][x];
                            }
                        }
                        erased = true;
                    }
                }
                //if erased {
                //    let _ = self.se.play();
                //}

                // 新しいブロックの生成
                self.block_type    = rand::thread_rng().gen_range(0..BLOCK_TYPE_MAX);
                self.control_point = MainState::new_control_point(self.block_type);
                self.rotation      = 0;
                // ゲームオーバー判定
                if self.check_touch_block(self.control_point, self.rotation as usize) {
                    if self.blocks[self.control_point.y as usize][self.control_point.x as usize] == BLOCK_STATE_NONE {
                        self.put_block();
                    }

                    for y in 0..TETRIS_HEIGHT {
                        for x in 0..TETRIS_WIDTH {
                            if self.blocks[y][x] != BLOCK_STATE_NONE {
                                self.blocks[y][x] = BLOCK_STATE_GAMEOVER;
                            }
                        }
                    }
                    self.is_gameover = true;
                }
            } else {
                self.control_point = control_point;
            }
        }

        // 移動制御
        let mut control_point = self.control_point.clone();
        if self.right_state == KeyState::KeyChangeDown { control_point.x += 1; }
        if self.left_state  == KeyState::KeyChangeDown { control_point.x -= 1; }
        if self.check_touch_block(control_point, self.rotation as usize) == false {
            self.control_point = control_point;
        }

        let mut rotation:i8 = self.rotation.clone() as i8;
        if self.up_state    == KeyState::KeyChangeDown { rotation -= 1; }
        if self.down_state  == KeyState::KeyChangeDown { rotation += 1; }
        rotation = (rotation + ROTATION_MAX as i8) % ROTATION_MAX as i8;
        if self.check_touch_block(self.control_point, rotation as usize) == false {
            self.rotation = rotation as usize;
        }
        debug_assert!(self.rotation < ROTATION_MAX);

        // キーステートの更新
        if self.up_state    == KeyState::KeyChangeDown { self.up_state    = KeyState::KeyDown; }
        if self.right_state == KeyState::KeyChangeDown { self.right_state = KeyState::KeyDown; }
        if self.down_state  == KeyState::KeyChangeDown { self.down_state  = KeyState::KeyDown; }
        if self.left_state  == KeyState::KeyChangeDown { self.left_state  = KeyState::KeyDown; }
        if self.space_state == KeyState::KeyChangeDown { self.space_state = KeyState::KeyDown; }

        if self.up_state    == KeyState::KeyChangeUp { self.up_state    = KeyState::KeyUp; }
        if self.right_state == KeyState::KeyChangeUp { self.right_state = KeyState::KeyUp; }
        if self.down_state  == KeyState::KeyChangeUp { self.down_state  = KeyState::KeyUp; }
        if self.left_state  == KeyState::KeyChangeUp { self.left_state  = KeyState::KeyUp; }
        if self.space_state == KeyState::KeyChangeUp { self.space_state = KeyState::KeyUp; }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, graphics::BLACK);

        // 背景描画
        graphics::draw(ctx, &self.background, (cgmath::Point2::new(0.0, 0.0),))?;
        // ブロック描画
        for y in 0..TETRIS_HEIGHT {
            for x in 0..TETRIS_WIDTH {
                let pos = cgmath::Point2::new(x as f32 * TETRIS_SIZE, y as f32 * TETRIS_SIZE);
                let pos = cgmath::Point2::new(pos.x + TETRIS_RECT.x, pos.y + TETRIS_RECT.y);
                let state = self.blocks[y][x] as usize;
                graphics::draw(ctx, &self.blockmesh[state], (pos,))?;
            }
        }

        // コントロール中のブロックの描画
        if self.is_gameover == false {
            for i in 0..BLOCK_NUM {
                let block = BLOCK[self.block_type as usize][self.rotation as usize][i];
                let pos = self.control_point;
                let pos = cgmath::Point2::new(pos.x + block.x, pos.y + block.y);
                let pos = cgmath::Point2::new(pos.x as f32 * TETRIS_SIZE, pos.y as f32 * TETRIS_SIZE);
                let pos = cgmath::Point2::new(pos.x + TETRIS_RECT.x, pos.y + TETRIS_RECT.y);
                graphics::draw(ctx, &self.blockmesh[(self.block_type + 1) as usize], (pos,))?;
            }
        }


        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool){
        match keycode{
            KeyCode::A     => { if self.left_state   != KeyState::KeyDown { self.left_state   = KeyState::KeyChangeDown; }}
            KeyCode::D     => { if self.right_state  != KeyState::KeyDown { self.right_state  = KeyState::KeyChangeDown; }}
            KeyCode::S     => { if self.down_state   != KeyState::KeyDown { self.down_state   = KeyState::KeyChangeDown; }}
            KeyCode::W     => { if self.up_state     != KeyState::KeyDown { self.up_state     = KeyState::KeyChangeDown; }}
            KeyCode::Space => { if self.space_state     != KeyState::KeyDown { self.space_state     = KeyState::KeyChangeDown; }}
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods){
        match keycode{
            KeyCode::A => { self.left_state   = KeyState::KeyChangeUp; }
            KeyCode::D => { self.right_state  = KeyState::KeyChangeUp; }
            KeyCode::S => { self.down_state   = KeyState::KeyChangeUp; }
            KeyCode::W => { self.up_state     = KeyState::KeyChangeUp; }
            KeyCode::Space => { self.space_state     = KeyState::KeyChangeUp; }
            _ => {}
        }
    }

}

pub fn main() -> GameResult {
    // リソースの読み込み
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR"){
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let setup = conf::WindowSetup {
        title         : String::from("Tetris"),
        samples       : conf::NumSamples::One,
        icon          : String::from(""),
        vsync         : true,
        srgb          : true,
    };
    let mode = conf::WindowMode::default().dimensions(WINDOW_RECT.w, WINDOW_RECT.h);
    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir).window_setup(setup).window_mode(mode);
    let (ctx, events_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx).unwrap();

    // bgm
    //&state.bgm.set_repeat(true);
    //&state.bgm.play_detached();

    // ゲーム開始
    event::run(ctx, events_loop, state)
}
