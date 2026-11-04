use std::path::PathBuf;

pub struct WeaponConfig {
    pub name: &'static str,
    pub ammo_capacity: u32,
    pub fire_rate_rpm: u32,
    pub model_path: PathBuf,
    pub texture_path: PathBuf,
}

impl WeaponConfig {
    pub fn new_mac10() -> Self {
        Self {
            name: "MAC-10",
            ammo_capacity: 32,
            fire_rate_rpm: 1090,
            model_path: PathBuf::from("assets/models/mac-10.gltf"),
            texture_path: PathBuf::from("assets/textures/mac10.png"),
        }
    }
}

pub struct Player {
    pub active_weapon: Option<WeaponConfig>,
}

impl Player {
    pub fn new() -> Self {
        Self { active_weapon: None }
    }

    pub fn give_weapon_to_hands(&mut self, weapon: WeaponConfig) {
        println!("[Инвентарь] Предмет '{}' успешно выдан игроку!", weapon.name);
        self.active_weapon = Some(weapon);
    }

    pub fn check_hands(&self) -> bool {
        match &self.active_weapon {
            Some(w) => {
                println!("[Руки] В руках у игрока: {}", w.name);
                true
            }
            None => {
                println!("[Руки] Оружие в руках не обнаружено (пусто).");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equip_mac10() {
        let mut player = Player::new();
        let mac10 = WeaponConfig::new_mac10();

        assert!(!player.check_hands());
        player.give_weapon_to_hands(mac10);
        assert!(player.check_hands());
    }
}