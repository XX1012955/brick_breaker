use bevy::prelude::*;
use rand::Rng;

// 游戏常量
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 15.0;
const BALL_SIZE: f32 = 10.0;
const BRICK_WIDTH: f32 = 75.0;
const BRICK_HEIGHT: f32 = 20.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 400.0;

// 组件
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct Brick {
    points: u32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct Particle {
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Trail {
    positions: Vec<Vec3>,
    max_length: usize,
}

#[derive(Component)]
struct PaddleGlow;

#[derive(Component)]
struct ScorePopup {
    lifetime: f32,
    max_lifetime: f32,
    initial_y: f32,
}

#[derive(Component)]
struct GameOverModal;

#[derive(Component)]
struct ModalBackground;

#[derive(Resource, Default)]
struct GameState {
    score: u32,
    game_over: bool,
    restart_requested: bool,
    won: bool,
}

fn main() {
    println!("🧱 启动打砖块游戏...");
    run_brick_breaker();
}

fn run_brick_breaker() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "打砖块游戏".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_startup_system(setup)
        .add_system(paddle_movement)
        .add_system(move_ball)
        .add_system(check_collisions)
        .add_system(update_scoreboard)
        .add_system(handle_restart)
        .add_system(handle_input)
        .add_system(check_win_condition)
        .add_system(update_particles)
        .add_system(update_ball_trail)
        .add_system(animate_background)
        .add_system(animate_paddle_glow)
        .add_system(update_score_popups)
        .add_system(show_game_over_modal)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // 相机
    commands.spawn(Camera2dBundle::default());

    // 背景渐变效果
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.2),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        Background,
    ));

    // 挡板 - 美化版本
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.6, 1.0), // 更亮的蓝色
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 50.0, 0.0),
            ..default()
        },
        Paddle,
    ));

    // 挡板发光效果
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.6, 1.0, 0.3),
                custom_size: Some(Vec2::new(PADDLE_WIDTH + 10.0, PADDLE_HEIGHT + 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 50.0, -1.0),
            ..default()
        },
        PaddleGlow,
    ));

    // 球 - 美化版本
    let mut rng = rand::thread_rng();
    let direction_x = rng.gen_range(-0.8..0.8);
    let direction_y = 1.0; // 向上开始，这样更容易控制
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.9, 0.2), // 金黄色球
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // 从中心开始
            ..default()
        },
        Ball {
            velocity: Vec2::new(direction_x * BALL_SPEED, direction_y * BALL_SPEED),
        },
        Trail {
            positions: Vec::new(),
            max_length: 10,
        },
    ));

    // 球的发光效果
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1.0, 0.9, 0.2, 0.4),
            custom_size: Some(Vec2::new(BALL_SIZE + 8.0, BALL_SIZE + 8.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    // 砖块 - 美化版本
    let rows = 5;
    let cols = 8;
    let brick_spacing = 5.0;
    let total_width = cols as f32 * (BRICK_WIDTH + brick_spacing) - brick_spacing;
    let start_x = -total_width / 2.0 + BRICK_WIDTH / 2.0;
    let start_y = WINDOW_HEIGHT / 2.0 - 50.0;
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * (BRICK_WIDTH + brick_spacing);
            let y = start_y - row as f32 * (BRICK_HEIGHT + brick_spacing);
            
            // 根据行数选择不同的颜色主题
            let color = match row {
                0 => Color::rgb(1.0, 0.2, 0.2), // 红色
                1 => Color::rgb(1.0, 0.6, 0.2), // 橙色
                2 => Color::rgb(1.0, 1.0, 0.2), // 黄色
                3 => Color::rgb(0.2, 1.0, 0.2), // 绿色
                4 => Color::rgb(0.2, 0.6, 1.0), // 蓝色
                _ => Color::rgb(0.8, 0.2, 1.0), // 紫色
            };
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Brick {
                    points: (rows - row) as u32 * 10,
                },
            ));
            
            // 砖块边框效果
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.3),
                    custom_size: Some(Vec2::new(BRICK_WIDTH + 2.0, BRICK_HEIGHT + 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -0.1),
                ..default()
            });
        }
    }

    // 分数文本 - 美化版本
    commands.spawn((
        TextBundle::from_section(
            "🏆 分数: 0 | ← → 移动挡板 | ESC退出",
            TextStyle {
                font: Default::default(),
                font_size: 28.0,
                color: Color::rgb(1.0, 0.9, 0.2), // 金色文字
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreText,
    ));
    
    // 游戏说明文本 - 美化版本
    commands.spawn(TextBundle::from_section(
        "🎯 目标: 消除所有砖块！\n🎮 控制: ← → 移动挡板\n⚡ 测试: G键=游戏结束, W键=胜利",
        TextStyle {
            font: Default::default(),
            font_size: 18.0,
            color: Color::rgb(0.7, 0.9, 1.0), // 淡蓝色
        },
    ).with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(55.0),
            left: Val::Px(15.0),
            ..default()
        },
        ..default()
    }));

    // 游戏结束大字体文本（居中显示）- 美化版本
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "🎮 Playing... Score: 0",
                TextStyle {
                    font: Default::default(),
                    font_size: 28.0,
                    color: Color::rgb(0.2, 1.0, 0.4), // 亮绿色
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(180.0),
                    top: Val::Px(280.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        GameOverText,
    ));
}

fn paddle_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    let mut transform = paddle_query.single_mut();
    let mut direction = 0.0;
    if keyboard.pressed(KeyCode::Left) { direction -= 1.0; }
    if keyboard.pressed(KeyCode::Right) { direction += 1.0; }

    let new_x = transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();
    transform.translation.x = new_x.clamp(
        -WINDOW_WIDTH / 2.0 + PADDLE_WIDTH / 2.0,
        WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0,
    );
}

fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.game_over { return; }

    let (mut transform, mut ball) = ball_query.single_mut();
    let delta = time.delta_seconds();
    transform.translation.x += ball.velocity.x * delta;
    transform.translation.y += ball.velocity.y * delta;

    // 左右边界反弹
    if transform.translation.x < -WINDOW_WIDTH/2.0 + BALL_SIZE/2.0 
        || transform.translation.x > WINDOW_WIDTH/2.0 - BALL_SIZE/2.0 {
        ball.velocity.x *= -1.0;
    }

    // 上边界反弹
    if transform.translation.y > WINDOW_HEIGHT/2.0 - BALL_SIZE/2.0 {
        ball.velocity.y *= -1.0;
    }

    // 下边界（游戏结束）
    if transform.translation.y < -WINDOW_HEIGHT/2.0 {
        game_state.game_over = true;
        println!("\n🚨🚨🚨 游戏结束！🚨🚨🚨");
        println!("💀 球掉出边界！");
        println!("🏆 最终分数: {}", game_state.score);
        println!("🎮 按 R 键重新开始，按 ESC 键退出");
        println!("================================\n");
    }
}

// 简化碰撞检测，避免复杂查询
fn check_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    brick_query: Query<(Entity, &Transform, &Brick), Without<Ball>>,
) {
    if game_state.game_over { return; }

    let (mut ball_transform, mut ball) = ball_query.single_mut();
    let ball_radius = BALL_SIZE / 2.0;

    // 挡板碰撞检测 - 球碰到挡板时反弹
    let paddle_transform = paddle_query.single();
    let paddle_half_w = PADDLE_WIDTH / 2.0;
    let paddle_half_h = PADDLE_HEIGHT / 2.0;
    
    // 检查球是否与挡板碰撞
    if (ball_transform.translation.x - paddle_transform.translation.x).abs() <= paddle_half_w + ball_radius
        && (ball_transform.translation.y - paddle_transform.translation.y).abs() <= paddle_half_h + ball_radius
        && ball.velocity.y < 0.0 {
        // 球碰到挡板，反弹
        ball.velocity.y *= -1.0;
        
        // 根据球碰到挡板的位置调整反弹角度
        let hit_pos = (ball_transform.translation.x - paddle_transform.translation.x) / paddle_half_w;
        ball.velocity.x = hit_pos * BALL_SPEED * 0.75;
        
        // 确保球不会卡在挡板里
        ball_transform.translation.y = paddle_transform.translation.y + paddle_half_h + ball_radius + 1.0;
    }

    // 砖块碰撞
    for (brick_entity, brick_transform, brick) in brick_query.iter() {
        let brick_half_w = BRICK_WIDTH / 2.0;
        let brick_half_h = BRICK_HEIGHT / 2.0;
        if (ball_transform.translation.x - brick_transform.translation.x).abs() <= brick_half_w + ball_radius
            && (ball_transform.translation.y - brick_transform.translation.y).abs() <= brick_half_h + ball_radius {
            
            // 创建粒子爆炸效果
            spawn_particles(&mut commands, brick_transform.translation, Color::rgb(1.0, 0.8, 0.2), 8);
            
            // 创建分数弹框
            spawn_score_popup(&mut commands, brick_transform.translation, brick.points);
            
            commands.entity(brick_entity).despawn();
            game_state.score += brick.points;
            ball.velocity.y *= -1.0; // 简化：只上下反弹
            break;
        }
    }
}

fn update_scoreboard(
    game_state: Res<GameState>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<GameOverText>)>,
    mut game_over_query: Query<&mut Text, (With<GameOverText>, Without<ScoreText>)>,
) {
    if game_state.is_changed() {
        // 更新顶部分数文本
        let mut score_text = score_query.single_mut();
        score_text.sections[0].value = if game_state.won {
            format!("🎉 恭喜通关! 🏆 分数: {} | 按R重新开始 | 按ESC退出", game_state.score)
        } else if game_state.game_over {
            format!("💀 游戏结束! 🏆 分数: {} | 按R重新开始 | 按ESC退出", game_state.score)
        } else {
            format!("🏆 分数: {} | ← → 移动挡板 | ESC退出", game_state.score)
        };
        
        // 根据状态改变文字颜色
        score_text.sections[0].style.color = if game_state.won {
            Color::rgb(0.2, 1.0, 0.2) // 绿色胜利
        } else if game_state.game_over {
            Color::rgb(1.0, 0.3, 0.3) // 红色失败
        } else {
            Color::rgb(1.0, 0.9, 0.2) // 金色正常
        };

        // 更新中央游戏结束文本 - 使用更大更明显的显示
        for mut game_over_text in game_over_query.iter_mut() {
            if game_state.won {
                game_over_text.sections[0].value = format!("🎉✨ VICTORY! ✨🎉\n🏆 Final Score: {}\n🎮 Press R to Restart", game_state.score);
                game_over_text.sections[0].style.color = Color::rgb(1.0, 0.8, 0.2);
                game_over_text.sections[0].style.font_size = 52.0;
            } else if game_state.game_over {
                game_over_text.sections[0].value = format!("💀⚡ GAME OVER ⚡💀\n🏆 Final Score: {}\n🎮 Press R to Restart", game_state.score);
                game_over_text.sections[0].style.color = Color::rgb(1.0, 0.2, 0.2);
                game_over_text.sections[0].style.font_size = 52.0;
            } else {
                game_over_text.sections[0].value = format!("🎮 Playing... 🏆 Score: {}", game_state.score);
                game_over_text.sections[0].style.color = Color::rgb(0.2, 1.0, 0.4);
                game_over_text.sections[0].style.font_size = 28.0;
            }
        }
    }
}

fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    if keyboard.just_pressed(KeyCode::R) && (game_state.game_over || game_state.won) {
        game_state.restart_requested = true;
    }
    
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit);
    }
    
    // 测试用：按G键触发游戏结束，按W键触发胜利
    if keyboard.just_pressed(KeyCode::G) && !game_state.game_over {
        game_state.game_over = true;
        game_state.score += 100; // 添加一些分数用于测试
        println!("\n🚨🚨🚨 测试游戏结束！🚨🚨🚨");
        println!("🎮 手动触发游戏结束");
        println!("🏆 最终分数: {}", game_state.score);
        println!("🎮 按 R 键重新开始，按 ESC 键退出");
        println!("================================\n");
    }
    
    if keyboard.just_pressed(KeyCode::W) && !game_state.game_over {
        game_state.won = true;
        game_state.game_over = true;
        game_state.score += 500; // 胜利奖励分数
        println!("\n🎉🎉🎉 恭喜胜利！🎉🎉🎉");
        println!("🏆 你赢了！所有砖块已消除！");
        println!("🏆 最终分数: {}", game_state.score);
        println!("🎮 按 R 键重新开始，按 ESC 键退出");
        println!("================================\n");
    }
}

fn handle_restart(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut paddle_query: Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
    brick_query: Query<Entity, With<Brick>>,
    modal_query: Query<Entity, With<GameOverModal>>,
    bg_query: Query<Entity, With<ModalBackground>>,
    popup_query: Query<Entity, With<ScorePopup>>,
) {
    if !game_state.restart_requested {
        return;
    }
    
    // 清理所有UI元素
    for entity in modal_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in bg_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in popup_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // 重置游戏状态
    game_state.score = 0;
    game_state.game_over = false;
    game_state.won = false;
    game_state.restart_requested = false;
    
    // 重置球的位置和速度
    if let Ok((mut ball_transform, mut ball)) = ball_query.get_single_mut() {
        ball_transform.translation = Vec3::new(0.0, -100.0, 0.0);
        let mut rng = rand::thread_rng();
        let direction_x = rng.gen_range(-0.8..0.8);
        ball.velocity = Vec2::new(direction_x * BALL_SPEED, -BALL_SPEED);
    }
    
    // 重置挡板位置
    if let Ok(mut paddle_transform) = paddle_query.get_single_mut() {
        paddle_transform.translation.x = 0.0;
    }
    
    // 删除所有现有砖块
    for brick_entity in brick_query.iter() {
        commands.entity(brick_entity).despawn();
    }
    
    // 重新生成砖块
    let rows = 5;
    let cols = 8;
    let brick_spacing = 5.0;
    let total_width = cols as f32 * (BRICK_WIDTH + brick_spacing) - brick_spacing;
    let start_x = -total_width / 2.0 + BRICK_WIDTH / 2.0;
    let start_y = WINDOW_HEIGHT / 2.0 - 50.0;
    
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * (BRICK_WIDTH + brick_spacing);
            let y = start_y - row as f32 * (BRICK_HEIGHT + brick_spacing);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::hsl(row as f32 * 60.0, 0.8, 0.5),
                        custom_size: Some(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Brick {
                    points: (rows - row) as u32 * 10,
                },
            ));
        }
    }
}

fn check_win_condition(
    mut game_state: ResMut<GameState>,
    brick_query: Query<&Brick>,
) {
    if game_state.game_over || game_state.won {
        return;
    }
    
    // 检查是否还有砖块
    if brick_query.is_empty() {
        game_state.won = true;
        game_state.game_over = true; // 游戏结束，但是胜利状态
        println!("\n🎉🎉🎉 完美通关！🎉🎉🎉");
        println!("🏆 恭喜！所有砖块已消除！");
        println!("🏆 最终分数: {}", game_state.score);
        println!("🎮 按 R 键重新开始，按 ESC 键退出");
        println!("================================\n");
    }
}

// 粒子效果系统
fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &mut Particle, &mut Sprite)>,
) {
    for (entity, mut transform, mut particle, mut sprite) in particle_query.iter_mut() {
        // 更新粒子位置
        transform.translation.x += particle.velocity.x * time.delta_seconds();
        transform.translation.y += particle.velocity.y * time.delta_seconds();
        
        // 更新生命周期
        particle.lifetime -= time.delta_seconds();
        
        // 根据生命周期调整透明度
        let alpha = particle.lifetime / particle.max_lifetime;
        sprite.color.set_a(alpha);
        
        // 移除过期的粒子
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// 球轨迹效果系统（优化版本）
fn update_ball_trail(
    mut ball_query: Query<(&Transform, &mut Trail), With<Ball>>,
    time: Res<Time>,
) {
    // 简化轨迹系统，避免创建太多实体
    for (transform, mut trail) in ball_query.iter_mut() {
        // 每隔一定时间才添加轨迹点
        if time.elapsed_seconds() % 0.1 < time.delta_seconds() {
            trail.positions.push(transform.translation);
            
            // 限制轨迹长度
            if trail.positions.len() > trail.max_length {
                trail.positions.remove(0);
            }
        }
    }
}

// 背景动画系统
fn animate_background(
    time: Res<Time>,
    mut bg_query: Query<&mut Sprite, With<Background>>,
) {
    for mut sprite in bg_query.iter_mut() {
        let time_factor = time.elapsed_seconds() * 0.5;
        let r = 0.1 + (time_factor.sin() * 0.05).abs();
        let g = 0.1 + (time_factor.cos() * 0.05).abs();
        let b = 0.2 + ((time_factor * 1.5).sin() * 0.1).abs();
        sprite.color = Color::rgb(r, g, b);
    }
}

// 创建粒子爆炸效果
fn spawn_particles(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    count: usize,
) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        let velocity = Vec2::new(
            rng.gen_range(-200.0..200.0),
            rng.gen_range(-200.0..200.0),
        );
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(3.0, 3.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            Particle {
                velocity,
                lifetime: rng.gen_range(0.5..1.5),
                max_lifetime: 1.0,
            },
        ));
    }
}

// 挡板发光动画系统
fn animate_paddle_glow(
    time: Res<Time>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut glow_query: Query<(&mut Transform, &mut Sprite), (With<PaddleGlow>, Without<Paddle>)>,
) {
    if let Ok(paddle_transform) = paddle_query.get_single() {
        for (mut glow_transform, mut glow_sprite) in glow_query.iter_mut() {
            // 跟随挡板位置
            glow_transform.translation.x = paddle_transform.translation.x;
            
            // 脉冲发光效果
            let pulse = (time.elapsed_seconds() * 3.0).sin() * 0.1 + 0.3;
            glow_sprite.color.set_a(pulse);
            
            // 大小变化
            let scale = 1.0 + (time.elapsed_seconds() * 2.0).sin() * 0.1;
            glow_transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}

// 创建分数弹框
fn spawn_score_popup(
    commands: &mut Commands,
    position: Vec3,
    points: u32,
) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                format!("+{}", points),
                TextStyle {
                    font: Default::default(),
                    font_size: 32.0,
                    color: Color::rgb(1.0, 0.8, 0.2), // 金色
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(position.x + WINDOW_WIDTH / 2.0),
                    top: Val::Px(WINDOW_HEIGHT / 2.0 - position.y),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        ScorePopup {
            lifetime: 2.0,
            max_lifetime: 2.0,
            initial_y: WINDOW_HEIGHT / 2.0 - position.y,
        },
    ));
}

// 更新分数弹框系统
fn update_score_popups(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_query: Query<(Entity, &mut ScorePopup, &mut Style, &mut Text)>,
) {
    for (entity, mut popup, mut style, mut text) in popup_query.iter_mut() {
        // 更新生命周期
        popup.lifetime -= time.delta_seconds();
        
        // 向上移动
        if let Val::Px(ref mut top) = style.position.top {
            *top = popup.initial_y - (popup.max_lifetime - popup.lifetime) * 50.0;
        }
        
        // 淡出效果
        let alpha = popup.lifetime / popup.max_lifetime;
        text.sections[0].style.color.set_a(alpha);
        
        // 移除过期的弹框
        if popup.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// 显示游戏结束模态框（简化版本）
fn show_game_over_modal(
    mut commands: Commands,
    game_state: Res<GameState>,
    modal_query: Query<Entity, With<GameOverModal>>,
) {
    // 如果游戏结束且还没有显示模态框
    if (game_state.game_over || game_state.won) && modal_query.is_empty() {
        // 创建简单的游戏结束弹框
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    if game_state.won {
                        format!("🎉 恭喜胜利！ 🎉\n🏆 最终分数: {}\n🎮 按 R 键重新开始", game_state.score)
                    } else {
                        format!("💀 游戏结束 💀\n🏆 最终分数: {}\n🎮 按 R 键重新开始", game_state.score)
                    },
                    TextStyle {
                        font: Default::default(),
                        font_size: 32.0,
                        color: if game_state.won { 
                            Color::rgb(0.2, 1.0, 0.2) 
                        } else { 
                            Color::rgb(1.0, 0.3, 0.3) 
                        },
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(150.0),
                        top: Val::Px(200.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            GameOverModal,
        ));
        
        // 创建半透明背景
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.7),
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 5.0), // 在游戏元素之上
                ..default()
            },
            ModalBackground,
        ));
    }
    
    // 如果游戏重新开始，移除模态框
    if !game_state.game_over && !game_state.won {
        for entity in modal_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}