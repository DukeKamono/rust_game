use ggez::*;

pub struct Wall {
    pub x: f32,
    pub y: f32,
    pub hitbox: graphics::Rect,
    pub shape: graphics::Mesh,
}

impl Wall {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Wall {
        // radius of circle
        let r = 50f32;
        // create hit box
        let hb = graphics::Rect::new(0.0, 0.0, r * 2.0, r * 2.0);
        // create mesh
        let circle = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(r, r),
                r,
                1.0,
                graphics::WHITE,
            )
            .rectangle(graphics::DrawMode::stroke(1.0), hb.clone(), graphics::WHITE)
            .build(ctx)
            .unwrap();

        Wall {
            x: xpos,
            y: ypos,
            shape: circle,
            hitbox: hb,
        }
    }

    pub fn hit_box(&self) -> graphics::Rect {
       let mut r = self.hitbox.clone();
       r.x = self.x;
       r.y = self.y;
       r
    }
    
    // MAYBE if it was a moving wall?
    //pub fn collide(&self, other: &super::Player) -> bool {
    //    self.hit_box().overlaps(&other.hit_box())
    //}

    pub fn draw(&self, ctx: &mut Context) {
        let dp = graphics::DrawParam::default().dest(nalgebra::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.shape, dp).expect("Error drawing Wall");
    }
}