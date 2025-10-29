use std::io;
use rand::Rng;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    health: i32,
    attack: i32,
    gold: i32,
}

#[derive(Debug, Clone)]
struct Monster {
    name: String,
    health: i32,
    attack: i32,
    gold_reward: i32,
}

pub fn play_adventure_game() {
    println!("🗡️  欢迎来到文字冒险游戏！");
    println!("请输入你的角色名字:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("读取输入失败");
    let name = name.trim().to_string();
    
    let mut player = Player {
        name: name.clone(),
        health: 100,
        attack: 20,
        gold: 50,
    };
    
    println!("\n🎭 欢迎，勇敢的冒险者 {}！", player.name);
    println!("你的初始状态：生命值: {}, 攻击力: {}, 金币: {}", 
             player.health, player.attack, player.gold);
    
    let mut level = 1;
    
    loop {
        println!("\n🏰 === 第{}关 ===", level);
        show_menu();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");
        
        match choice.trim() {
            "1" => {
                if battle(&mut player, level) {
                    level += 1;
                    if level > 5 {
                        println!("\n🏆 恭喜！你已经通关了所有关卡！");
                        println!("🌟 {} 成为了传说中的英雄！", player.name);
                        break;
                    }
                } else {
                    println!("\n💀 游戏结束！{} 在冒险中倒下了...", player.name);
                    break;
                }
            }
            "2" => shop(&mut player),
            "3" => rest(&mut player),
            "4" => show_status(&player),
            "5" => {
                println!("👋 {} 结束了这次冒险。再见！", player.name);
                break;
            }
            _ => println!("❌ 无效选择，请重新输入！"),
        }
    }
}

fn show_menu() {
    println!("\n📋 请选择你的行动:");
    println!("1. 🗡️  进入战斗");
    println!("2. 🏪 访问商店");
    println!("3. 😴 休息恢复");
    println!("4. 📊 查看状态");
    println!("5. 🚪 退出游戏");
}

fn battle(player: &mut Player, level: i32) -> bool {
    let monsters = vec![
        Monster { name: "史莱姆".to_string(), health: 30, attack: 8, gold_reward: 20 },
        Monster { name: "哥布林".to_string(), health: 50, attack: 12, gold_reward: 35 },
        Monster { name: "骷髅战士".to_string(), health: 70, attack: 18, gold_reward: 50 },
        Monster { name: "兽人".to_string(), health: 90, attack: 25, gold_reward: 75 },
        Monster { name: "巨龙".to_string(), health: 150, attack: 35, gold_reward: 200 },
    ];
    
    let monster_index = (level - 1).min(monsters.len() as i32 - 1) as usize;
    let mut monster = monsters[monster_index].clone();
    
    // 根据等级调整怪物强度
    monster.health += (level - 1) * 10;
    monster.attack += (level - 1) * 3;
    monster.gold_reward += (level - 1) * 10;
    
    println!("\n⚔️  一只 {} 出现了！", monster.name);
    println!("怪物状态 - 生命值: {}, 攻击力: {}", monster.health, monster.attack);
    
    while player.health > 0 && monster.health > 0 {
        println!("\n🎯 选择你的行动:");
        println!("1. 🗡️  攻击");
        println!("2. 🛡️  防御");
        println!("3. 🏃 逃跑");
        
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("读取输入失败");
        
        match action.trim() {
            "1" => {
                // 玩家攻击
                let damage = rand::thread_rng().gen_range(player.attack - 5..=player.attack + 5);
                monster.health -= damage;
                println!("💥 你对 {} 造成了 {} 点伤害！", monster.name, damage);
                
                if monster.health <= 0 {
                    println!("🎉 你击败了 {}！", monster.name);
                    player.gold += monster.gold_reward;
                    println!("💰 获得了 {} 金币！", monster.gold_reward);
                    return true;
                }
            }
            "2" => {
                println!("🛡️  你进入了防御姿态！");
                // 防御减少伤害
            }
            "3" => {
                if rand::thread_rng().gen_bool(0.7) {
                    println!("🏃 你成功逃跑了！");
                    return true;
                } else {
                    println!("❌ 逃跑失败！");
                }
            }
            _ => {
                println!("❌ 无效行动！");
                continue;
            }
        }
        
        // 怪物攻击
        if monster.health > 0 {
            let monster_damage = rand::thread_rng().gen_range(monster.attack - 3..=monster.attack + 3);
            let actual_damage = if action.trim() == "2" { 
                monster_damage / 2 // 防御减半伤害
            } else { 
                monster_damage 
            };
            
            player.health -= actual_damage;
            println!("💢 {} 对你造成了 {} 点伤害！", monster.name, actual_damage);
            
            if player.health <= 0 {
                return false;
            }
        }
        
        println!("📊 你的生命值: {} | {} 的生命值: {}", 
                 player.health, monster.name, monster.health);
    }
    
    false
}

fn shop(player: &mut Player) {
    println!("\n🏪 === 神秘商店 ===");
    println!("商人: 欢迎光临！看看我的商品吧！");
    println!("你的金币: {}", player.gold);
    println!("\n📦 商品列表:");
    println!("1. 🧪 生命药水 (+30 生命值) - 30 金币");
    println!("2. ⚔️  铁剑 (+10 攻击力) - 50 金币");
    println!("3. 🛡️  铁盾 (+20 生命值上限) - 40 金币");
    println!("4. 🚪 离开商店");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("读取输入失败");
    
    match choice.trim() {
        "1" => {
            if player.gold >= 30 {
                player.gold -= 30;
                player.health += 30;
                if player.health > 100 { player.health = 100; }
                println!("✅ 购买成功！生命值恢复了30点！");
            } else {
                println!("❌ 金币不足！");
            }
        }
        "2" => {
            if player.gold >= 50 {
                player.gold -= 50;
                player.attack += 10;
                println!("✅ 购买成功！攻击力增加了10点！");
            } else {
                println!("❌ 金币不足！");
            }
        }
        "3" => {
            if player.gold >= 40 {
                player.gold -= 40;
                player.health += 20;
                println!("✅ 购买成功！生命值上限增加了20点！");
            } else {
                println!("❌ 金币不足！");
            }
        }
        "4" => println!("👋 下次再来！"),
        _ => println!("❌ 无效选择！"),
    }
}

fn rest(player: &mut Player) {
    println!("\n😴 你在旅店休息了一晚...");
    let heal_amount = rand::thread_rng().gen_range(20..=40);
    player.health += heal_amount;
    if player.health > 100 { player.health = 100; }
    println!("🌅 休息后恢复了 {} 点生命值！", heal_amount);
}

fn show_status(player: &Player) {
    println!("\n📊 === {} 的状态 ===", player.name);
    println!("❤️  生命值: {}", player.health);
    println!("⚔️  攻击力: {}", player.attack);
    println!("💰 金币: {}", player.gold);
}