use glam::{U16Vec2, Vec2};

pub struct RayCasting {
    pub world: Vec<Vec<u8>>,
    pub pos: Vec2,
    pub dir: Vec2,
    pub plane: Vec2,
}

impl RayCasting {
    pub fn lines(&mut self, w: u16, h: u16) -> Vec<(U16Vec2, U16Vec2, u8, bool)> {
        let mut lines = Vec::<(U16Vec2, U16Vec2, u8, bool)>::new();
        for x in 0..w {
            let ray_dir = self.dir + self.plane * Vec2::splat(2.0 * x as f32 / w as f32 - 1.0);

            let mut map_x = self.pos.x as usize;
            let mut map_y = self.pos.y as usize;

            let mut side_dist = Vec2::ZERO;

            let delta_dist_x = if ray_dir.x == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir.x).abs()
            };
            let delta_dist_y = if ray_dir.y == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir.y).abs()
            };

            let step_x: i8;
            let step_y: i8;

            let mut hit: usize = 0;
            let mut side: usize = 0;

            if ray_dir.x < 0.0 {
                step_x = -1;
                side_dist.x = (self.pos.x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist.x = (map_x as f32 + 1.0 - self.pos.x) * delta_dist_x;
            }

            if ray_dir.y < 0.0 {
                step_y = -1;
                side_dist.y = (self.pos.y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist.y = (map_y as f32 + 1.0 - self.pos.y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist_x;
                    map_x = (map_x as i8 + step_x) as usize;
                    side = 0;
                } else {
                    side_dist.y += delta_dist_y;
                    map_y = (map_y as i8 + step_y) as usize;
                    side = 1;
                }

                if self.world[map_y][map_x] > 0 {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                side_dist.x - delta_dist_x
            } else {
                side_dist.y - delta_dist_y
            };

            let line_height = h as f32 / perp_wall_dist;
            let mut draw_start = -line_height / 2.0 + h as f32 / 2.0;
            if draw_start < 0.0 {
                draw_start = 0.0;
            }
            let mut draw_end = line_height / 2.0 + h as f32 / 2.0;
            if draw_end >= h as f32 {
                draw_end = h as f32 - 1.0;
            }

            lines.push((
                U16Vec2::new(x, draw_start as u16),
                U16Vec2::new(x, draw_end as u16),
                self.world[map_y][map_x],
                side == 1,
            ));
        }
        lines
    }

    pub fn transform_cam(&mut self, move_factor: f32, rotation_factor: f32) {
        if move_factor != 0.0 {
            if self.world[self.pos.y as usize][(self.pos + self.dir * move_factor).x as usize] == 0
            {
                self.pos.x += (self.dir * move_factor).x;
            }
            if self.world[(self.pos + self.dir * move_factor).y as usize][self.pos.x as usize] == 0
            {
                self.pos.y += (self.dir * move_factor).y;
            }
        }

        // rotation matrix
        //[ cos(a) -sin(a) ]
        //[ sin(a)  cos(a) ]
        if rotation_factor != 0.0 {
            let old_dir_x = self.dir.x;
            self.dir.x = self.dir.x * rotation_factor.cos() - self.dir.y * rotation_factor.sin();
            self.dir.y = old_dir_x * rotation_factor.sin() + self.dir.y * rotation_factor.cos();
            let old_plane_x = self.plane.x;
            self.plane.x =
                self.plane.x * rotation_factor.cos() - self.plane.y * rotation_factor.sin();
            self.plane.y =
                old_plane_x * rotation_factor.sin() + self.plane.y * rotation_factor.cos();
        }
    }
}
