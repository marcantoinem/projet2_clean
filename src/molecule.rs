use std::ops::Range;

use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Molecule {
    pub x: f32,
    pub y: f32,
    dx: f32,
    dy: f32,
    pub radius: f32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MoleculeConfig {
    pub dx_range: Range<f32>,
    pub dy_range: Range<f32>,
    pub radius_range: Range<f32>,
}

impl MoleculeConfig {
    pub fn new(dx: f32, dy: f32, min_radius: f32, max_radius: f32) -> Self {
        MoleculeConfig {
            dx_range: (-dx..dx),
            dy_range: (-dy..dy),
            radius_range: (min_radius..max_radius),
        }
    }

    pub fn default() -> Self {
        MoleculeConfig {
            dx_range: (-4f32..4f32),
            dy_range: (-4f32..4f32),
            radius_range: (5f32..10f32),
        }
    }
}

impl Molecule {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32, radius: f32) -> Molecule {
        Self {
            x,
            y,
            dx,
            dy,
            radius,
        }
    }

    pub fn is_mols_touching(&self, mol: &Molecule) -> bool {
        let square_distance = (self.x - mol.x).powi(2) + (self.y - mol.y).powi(2);
        let sum_radius = self.radius + mol.radius;
        square_distance <= sum_radius.powi(2) + 5.0
    }

    pub fn move_mol(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn adjust_dir_after_collision(&mut self, mol: &mut Molecule) {
        let delta_x = mol.x - self.x;
        let mut delta_vx = 0.0;

        let delta_vy = if delta_x == 0.0 {
            mol.y - self.y
        } else {
            let r = (mol.y - self.y) / delta_x;
            delta_vx = (mol.dx - self.dx + (mol.dy - self.dy) * r) / (1.0 + r * r);
            r * delta_vx
        };
        self.dx += delta_vx;
        self.dy += delta_vy;
        mol.dx -= delta_vx;
        mol.dy -= delta_vy;
    }

    pub fn create_vec_mol(
        height: f32,
        xmin: f32,
        xmax: f32,
        nb_mol: usize,
        conf: &MoleculeConfig,
    ) -> Vec<Molecule> {
        let mut vec_mols = vec![];
        let mut rng = thread_rng();
        let radius_distribution = Uniform::from(conf.radius_range.clone());
        let dx_distribution = Uniform::from(conf.dx_range.clone());
        let dy_distribution = Uniform::from(conf.dy_range.clone());

        for _ in 0..nb_mol {
            let radius = radius_distribution.sample(&mut rng);
            let x = rng.gen_range((xmin + radius)..(xmax - radius));
            let y = rng.gen_range(radius..(height - radius));
            let dx = dx_distribution.sample(&mut rng);
            let dy = dy_distribution.sample(&mut rng);

            let mol = Molecule::new(x, y, dx, dy, radius);
            vec_mols.push(mol);
        }
        vec_mols
    }

    pub fn inverse_dir(&mut self, l_side: f32, r_side: f32, height: f32) {
        if self.x <= l_side + self.radius {
            self.x = l_side + self.radius;
            self.dx = -self.dx;
        } else if self.x + self.radius >= r_side {
            self.x = r_side - self.radius;
            self.dx = -self.dx;
        }

        if self.y + self.radius >= height {
            self.y = height - self.radius;
            self.dy = -self.dy;
        } else if self.y <= self.radius {
            self.y = self.radius;
            self.dy = -self.dy;
        }
    }
}
