use super::*;

////////////////////////////////////////////////////////////////////////////////

//ウィンドウの定義
pub static MAIN_WINDOW: Lazy<Option<Window>> = Lazy::new
(   ||
    {   let title = format!( "{APP_TITLE} v{APP_VER}" );
        let window = Window
        {   title,
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable: false,
            // fit_canvas_to_parent: true, //不具合が発生した場合コメントアウトする
            ..default()
        };

        Some ( window )
    }
);

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
const _CARGO_TOML_NAME: &str = env!( "CARGO_PKG_NAME"    );
const _CARGO_TOML_VER : &str = env!( "CARGO_PKG_VERSION" );

const APP_TITLE: &str = _CARGO_TOML_NAME; //アプリタイトル
const APP_VER  : &str = _CARGO_TOML_VER;  //アプリのバージョン

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Pixel)
pub const SCREEN_PIXELS_WIDTH : f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH  as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

////////////////////////////////////////////////////////////////////////////////

//単位Gridの縦横幅(Pixel)
const BASE_PIXELS: i32 = 8;
const SCALING: f32 = 4.0;
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * SCALING;

//GridのSize(縦横Pixel)
pub const SIZE_GRID: Vec2 = Vec2::new( PIXELS_PER_GRID, PIXELS_PER_GRID );

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Grid)
pub const SCREEN_GRIDS_WIDTH : i32 = 43; //memo: best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //memo: best 24

pub const SCREEN_GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const SCREEN_GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;

////////////////////////////////////////////////////////////////////////////////

//ログレベル
pub const LOG_LEVEL_DEV: &str = "warn,wgpu_hal=error"; //開発
pub const LOG_LEVEL_REL: &str = "error"; //リリース

////////////////////////////////////////////////////////////////////////////////

//Cameraのレンダリングの重なり
pub const ORDER_CAMERA2D_DEFAULT: isize = 2; //2Dが上
pub const ORDER_CAMERA3D_PLAYER : isize = 1; //Playerカメラ(Fpp&Tpp)
pub const ORDER_CAMERA3D_DEFAULT: isize = 0; //3Dが下

////////////////////////////////////////////////////////////////////////////////

//Cameraの背景色
pub const CAMERA2D_BGCOLOR: ClearColorConfig = CAMERA_BG_TRANSPARENCY;
pub const CAMERA3D_BGCOLOR: ClearColorConfig = CAMERA_BG_COLOR;

const CAMERA_BG_TRANSPARENCY: ClearColorConfig = ClearColorConfig::None;
const CAMERA_BG_COLOR       : ClearColorConfig = ClearColorConfig::Custom( BG_COLOR );
const BG_COLOR: Color = Color::rgb( 0.13, 0.13, 0.18 );

////////////////////////////////////////////////////////////////////////////////

//3Dライトの設定
pub const LIGHT3D_BRIGHTNESS : f32  = 15000.0; //明るさ
pub const LIGHT3D_TRANSLATION: Vec3 = Vec3::new( 30.0, 100.0, 40.0 ); //位置

////////////////////////////////////////////////////////////////////////////////

//assets（スプライト）
pub const ASSETS_SPRITE_DEBUG_GRID : &str = "sprites/debug_grid.png";
pub const ASSETS_SPRITE_BRICK_WALL : &str = "sprites/brick_wall.png";
pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "sprites/kani_DOTOWN.png";

//assets（フォント）
pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "fonts/Orbitron-Black.ttf";
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf";

//事前ロード対象
counted_array!
(   pub const PRELOAD_ASSETS: [ &str; _ ] =
    [   ASSETS_SPRITE_DEBUG_GRID,
        ASSETS_SPRITE_BRICK_WALL,
        ASSETS_SPRITE_KANI_DOTOWN,
        ASSETS_FONT_ORBITRON_BLACK,
        ASSETS_FONT_PRESSSTART2P_REGULAR,
    ]
);

////////////////////////////////////////////////////////////////////////////////

//スプライト重なり
pub const DEPTH_SPRITE_DEBUG_GRID : f32 = 999.0; //重なりの最大値
pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 900.0;
pub const DEPTH_SPRITE_GAME_FRAME : f32 = 800.0;

////////////////////////////////////////////////////////////////////////////////

//極座標カメラの設定
pub const ORBIT_CAMERA_INIT_R    : f32 = 5.0;      //初期値
pub const ORBIT_CAMERA_INIT_THETA: f32 = PI * 0.6; //初期値(ラジアン) 1.0:天頂、0.5:真横、0.0:真下
pub const ORBIT_CAMERA_INIT_PHI  : f32 = PI * 1.8; //初期値(ラジアン) 6時方向が0.0で反時計回り

pub const ORBIT_CAMERA_MAX_R    : f32 = 10.0;      //rの最大値
pub const ORBIT_CAMERA_MIN_R    : f32 = 1.0;       //rの最小値
pub const ORBIT_CAMERA_MAX_THETA: f32 = PI * 0.99; //Θの最大値(ラジアン)
pub const ORBIT_CAMERA_MIN_THETA: f32 = PI * 0.51; //Θの最小値(ラジアン)

//極座標カメラ操作時のマウスの感度調整
pub const MOUSE_WHEEL_Y_COEF : f32 = 0.1;
pub const MOUSE_MOTION_Y_COEF: f32 = 0.01;
pub const MOUSE_MOTION_X_COEF: f32 = 0.01;

////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
pub const SCREEN_FRAME_WALL_CHAR: char = '#';
pub static SCREEN_FRAME: Lazy<ScreenFrame> = Lazy::new
(   ||
    {   let design = vec!
        [  //0123456789 123456789 123456789 123456789 12
            "###########################################", //0
            "#..............................############", //1
            "#..............................############", //2
            "#..............................############", //3
            "#..............................############", //4
            "#..............................############", //5
            "#..............................############", //6
            "#..............................############", //7
            "#..............................############", //8
            "#..............................############", //9
            "#..............................############", //10
            "#..............................############", //11
            "#..............................############", //12
            "#..............................############", //13
            "#..............................############", //14
            "#..............................############", //15
            "#..............................############", //16
            "#..............................############", //17
            "#..............................############", //18
            "#..............................############", //19
            "#..............................############", //20
            "#..............................############", //21
            "###########################################", //22
            "                                           ", //23
        ]; //0123456789 123456789 123456789 123456789 12

        //表示エリア(viewport)の設定
        let adjust = PIXELS_PER_GRID / 2.0;
        let zero = Vec2::new(  1.0,  1.0 ) * PIXELS_PER_GRID - adjust;
        let size = Vec2::new( 30.0, 21.0 ) * PIXELS_PER_GRID + adjust * 2.0;

        ScreenFrame { design, zero, size,}
    }
);

////////////////////////////////////////////////////////////////////////////////

//マップ縦横幅(Grid)
pub const MAP_GRIDS_WIDTH : i32 = 51;
pub const MAP_GRIDS_HEIGHT: i32 = 51;

//マップのレンジ（外壁含む）
pub const MAP_GRIDS_X_RANGE: Range<i32> = 0..MAP_GRIDS_WIDTH;
pub const MAP_GRIDS_Y_RANGE: Range<i32> = 0..MAP_GRIDS_HEIGHT;

//外壁を含まないレンジ
pub const MAP_GRIDS_X_RANGE_INNER: Range<i32> = 1..MAP_GRIDS_WIDTH  - 1;
pub const MAP_GRIDS_Y_RANGE_INNER: Range<i32> = 1..MAP_GRIDS_HEIGHT - 1;

////////////////////////////////////////////////////////////////////////////////

//四方の配列
pub const NEWS: [ News; 4 ] = [ News::South, News::East, News::West, News::North ];

////////////////////////////////////////////////////////////////////////////////

//Playerの設定値
pub const PLAYER_TURN_COEF: f32 = 3.5;
pub const PLAYER_MOVE_COEF: f32 = 3.5;

pub const UNIT_TURN: f32 = FRAC_PI_2;
pub const UNIT_MOVE: f32 = 1.0;

////////////////////////////////////////////////////////////////////////////////

//End of code.