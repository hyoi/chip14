use super::*;

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの構造
// (Clear box)        ：親 A
//  ├─(Clear box)     ：中間親 B
//  │　├─PlayerFigure ：キャラクターの姿
//  |　└─FppCamera    ：一人称視点カメラ
//  └─TppCamera       ：三人称視点カメラ

//PlayerのComponent
#[derive( Component, Default )]
pub struct Player
{   position : IVec2,    //位置
    direction: News,     //向き
    in_action: InAction, //行動の種類
}

#[derive( Default, PartialEq )]
enum InAction
{   #[default] Stop,
    TurnRight, TurnLeft, //左右回転
    Forward, Backward,   //前進後退
}

impl Player
{   fn is_stop( &self ) -> bool
    {   self.in_action == InAction::Stop
    }
    fn is_turn( &self ) -> bool
    {   self.in_action == InAction::TurnRight || self.in_action == InAction::TurnLeft
    }
    fn is_move( &self ) -> bool
    {   self.in_action == InAction::Forward || self.in_action == InAction::Backward
    }
}

//Player Figure のComponent
#[derive( Component )] pub struct FigureHead;

//カメラのComponent
#[derive( Component )] pub struct FppCamera; //一人称視点カメラ
#[derive( Component )] pub struct TppCamera; //三人称視点カメラ

////////////////////////////////////////////////////////////////////////////////

//Playerの姿のspawn用メソッド
trait SpawnTrait1
{   fn spawn_figure
    (   &mut self,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>
    );
}
impl SpawnTrait1 for &mut ChildBuilder<'_, '_, '_>
{   //Playerの姿をspawnする
    fn spawn_figure
    (   &mut self,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    )
    {   self.spawn( PbrBundle::default() )
        .insert( meshes.add( shape::UVSphere { radius: 0.4, ..default() }.into() ) )
        .insert( materials.add( Color::DARK_GRAY.into() ) )
        .insert( Transform::from_translation( Vec3::ZERO ) )
        ;
        self.spawn( PbrBundle::default() )
        .insert( meshes.add( shape::UVSphere { radius: 0.395, ..default() }.into() ) )
        .insert( materials.add( Color::YELLOW.into() ) )
        .insert( Transform::from_translation( Vec3::NEG_Z * 0.01 ) )
        ;
    }
}

//3Dカメラspawn用メソッド
trait SpawnTrait2<T>
{   fn spawn_camera3d( &mut self, component: T, is_active: bool, position: Vec3, target: Vec3 );
}
impl<T: Component> SpawnTrait2<T> for &mut ChildBuilder<'_, '_, '_>
{   //3Dカメラをspawnする
    fn spawn_camera3d( &mut self, component: T, is_active: bool, position: Vec3, target: Vec3 )
    {   let viewport = Some
        (   camera::Viewport
            {   physical_position: SCREEN_FRAME.zero.as_uvec2(),
                physical_size    : SCREEN_FRAME.size.as_uvec2(),
                ..default()
            }
        );

        self.spawn( ( Camera3dBundle::default(), component ) )
        .insert( Camera { order: ORDER_CAMERA3D_PLAYER, viewport, is_active, ..default() } )
        .insert( Camera3d { clear_color: CAMERA3D_BGCOLOR, ..default() } )
        .insert( Transform::from_translation( position ).looking_at( target, Vec3::Y ) );
    }
}

////////////////////////////////////////////////////////////////////////////////

//Playerの3Dオブジェクトをspawnする
pub fn spawn_entity
(   que_player: Query<Entity, With<Player>>,
    map: Res<map::Map>,
    mut orbit_camera: ResMut<OrbitCamera>,
    mut cmds: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   //既存のPlayerがあれば削除する
    que_player.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //Playerの設定
    let sides = map.get_sides_space( map.start );
    let side = sides[ rand::thread_rng().gen_range( 0..sides.len() ) ];
    let direction = side;
    let player = Player { position: map.start, direction, ..default() };

    let player_position  = player.position.to_3dxz();
    let player_direction = player.direction.to_quat();

    //一人称視点カメラの設定
    let is_active = true;

    //三人称視点カメラ(極座標カメラ)の設定
    *orbit_camera = OrbitCamera { is_active: ! is_active, ..default() };
    let orbit_position = orbit_camera.orbit.to_vec3();

    //透明な箱をspawnし、それを親にして中に子をspawnする
    cmds.spawn( ( PbrBundle::default(), player ) )
    .insert( materials.add( Color::NONE.into() ) ) //透明
    .insert( Transform::from_translation( player_position ) ) //位置
    .with_children
    (   | mut cmds |
        {   //透明な箱をspawnし、それを親にして中に子をspawnする
            cmds.spawn( ( PbrBundle::default(), FigureHead ) )
            .insert( materials.add( Color::NONE.into() ) ) //透明
            .insert( Transform::from_rotation( player_direction ) ) //向き
            .with_children
            (   | mut cmds |
                {   //Playerの姿をspawnする
                    cmds.spawn_figure( meshes, materials );

                    //一人称視点カメラをspawnする
                    //一人称視点カメラはPlayerの中心(Vec3::ZERO)にあり正面を向いている
                    let target = Vec3::NEG_Z; //正面をNEG_Z(News::North)に固定する
                    let position = Vec3::Z * 0.478; //画角を稼ぐため背面方向へカメラを少し引く
                    cmds.spawn_camera3d( FppCamera, is_active, position, target );
                }
            );

            //三人称視点カメラをspawnする（極座標カメラ）
            let is_active = orbit_camera.is_active; //一人称視点カメラと反対の状態にする
            let target = Vec3::ZERO; //注視点はPlayer自身なのでVec3::ZERO
            cmds.spawn_camera3d( TppCamera, is_active, orbit_position, target );
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//キー入力によって自機の位置と向きを更新する
pub fn catch_input_keyboard
(   mut que_player: Query<&mut Player>,
    map: Res<map::Map>,
    orbit_camera: Res<OrbitCamera>,
    inkey: Res<Input<KeyCode>>,
)
{   //三人称視点カメラがアクティブなら、入力を受け付けない
    if orbit_camera.is_active { return }

    //Playerが停止していない場合、入力を受け付けない
    let Ok ( mut player ) = que_player.get_single_mut() else { return };
    if ! player.is_stop() { return }

    //自機の位置と向きを更新する
    for keycode in inkey.get_just_pressed()
    {   match keycode
        {   KeyCode::Right =>
            {   player.direction = player.direction.turn_right();
                player.in_action = InAction::TurnRight;
            }
            KeyCode::Left =>
            {   player.direction = player.direction.turn_left();
                player.in_action = InAction::TurnLeft;
            }
            KeyCode::Up =>
            {   let front = player.direction;
                if map.is_space( player.position + front )
                {   player.position += front;
                    player.in_action = InAction::Forward;
                }
            }
            KeyCode::Down =>
            {   let back = player.direction.back();
                if map.is_space( player.position + back )
                {   player.position += back;
                    player.in_action = InAction::Backward;
                }
            }
            _ => (),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//プレイヤーを左右旋回する
pub fn rotate_player
(   mut que_player: Query<&mut Player>,
    mut que_figure: Query<&mut Transform, With<FigureHead>>,
    time: Res<Time>,
    mut radian: Local<f32>,
)
{   let Ok ( mut player    ) = que_player.get_single_mut() else { return };
    let Ok ( mut transform ) = que_figure.get_single_mut() else { return };

    if ! player.is_turn() { return } //左右旋回でないなら

    //微小時間の回転角度
    let delta_radian = UNIT_TURN * time.delta().as_secs_f32() * PLAYER_TURN_COEF;
    *radian += delta_radian; //累積を保存する

    //累積が1単位を超えたら
    if *radian >= UNIT_TURN
    {   //向きをピッタリにする
        *transform = Transform::from_rotation( player.direction.to_quat() );

        //情報更新する
        player.in_action = InAction::Stop;
        *radian = 0.0;
    }
    else
    {   //左右旋回する（中間アニメーション）
        let delta_quat = Quat::from_rotation_y( delta_radian );
        match player.in_action
        {   InAction::TurnRight => transform.rotation *= delta_quat.inverse(),
            InAction::TurnLeft  => transform.rotation *= delta_quat,
            _ => (),
        }
    }
}

//プレイヤーを前進後退させる
pub fn move_player
(   mut que_player: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    mut distance: Local<f32>,
)
{   let Ok ( ( mut transform, mut player ) ) = que_player.get_single_mut() else { return };

    if ! player.is_move() { return } //前進後退でないなら

    //微小時間の移動距離
    let delta = UNIT_MOVE * time.delta().as_secs_f32() * PLAYER_MOVE_COEF;
    *distance += delta; //累積を保存する

    //累積が1単位を超えたら
    if *distance >= UNIT_MOVE
    {   //位置をピッタリにする
        *transform = Transform::from_translation( player.position.to_3dxz() );

        //情報更新する
        player.in_action = InAction::Stop;
        *distance = 0.0;
    }
    else
    {   //前進後退する（中間アニメーション）
        let vec3 = delta * match player.direction
        {   News::North => Vec3::NEG_Z,
            News::East  => Vec3::X,
            News::West  => Vec3::NEG_X,
            News::South => Vec3::Z,
        };

        match player.in_action
        {   InAction::Forward  => transform.translation += vec3,
            InAction::Backward => transform.translation -= vec3,
            _ => (),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.