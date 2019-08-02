use crate::entities::player::player::Player;
use crate::entities::environment::level::Level;
use crate::entities::enemies::enemies::Enemy;
use crate::entities::enemies::ai::*;
use ggez::nalgebra as na;
use ggez::*;
use std::time::Duration;
//use rand::prelude::*;

use super::super::{CollideEntity, DrawableEntity};
use crate::ui::DmgText;

pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
    dmg_text: Vec<DmgText>,
    pub invulnerable: Duration,
	pub ai_type: AITypes,
}

impl Blob {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, ai_type: AITypes) -> Blob {
        let img = graphics::Image::new(ctx, "/blob.png").unwrap();
        let hb = img.dimensions();
        let dmg_text = Vec::new();

        Blob {
            x: xpos,
            y: ypos,
            hp: 10.0,
            atk: 3.0,
            def: 1.0,
            sprite: img,
            hitbox: hb,
            dmg_text,
            invulnerable: Duration::new(1u64, 0u32),
			ai_type: ai_type,
        }
    }

    pub fn take_dmg(&mut self, ctx: &mut Context, dmg_to_take: f32) {
        if !self.invulnerable() {
            self.hp -= dmg_to_take;
            self.invulnerable = Duration::new(0u64, 0u32);
            self.dmg_text.push(DmgText::new(ctx, self.x, self.y, dmg_to_take));
            // Check for death and maybe call a death function.
        }
    }

    // returns if blob should be able to take damage (time is 1/4 sec)
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }
}

impl DrawableEntity for Blob {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)?;

        self.dmg_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

impl CollideEntity for Blob {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}

impl Enemy for Blob {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level) {
        self.dmg_text.retain(|t| t.live());
        self.dmg_text.iter_mut().for_each(|t| t.update(delta));

        // cool down invulnerable of blob
        if self.invulnerable() {
            self.invulnerable += delta;
        }
        
		// Player's atk_box hits me
        if let Some(atk) = &player.atk_box {
            if self.collision(atk) {
                self.take_dmg(ctx, player.atk);
            }
        }
		//let mut t = AI::new(AITypes::MeleeDirect);
		//let mut s = Box::new(Blob::new(ctx, 250.0, 150.0, AITypes::MeleeDirect));
		//t.update(delta, s, player, level);
		//t.update(delta, self, player, level);
    }

    fn islive(&self) -> bool {
        self.hp > 0.0
    }
}