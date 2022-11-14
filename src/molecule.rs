use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};

#[derive(PartialEq, Debug)]
pub struct Molecule {
    pub x: f32,
    pub y: f32,
    dx: f32,
    dy: f32,
    pub radius: f32,
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

    pub fn is_molecules_touching(&self, mol: &Molecule) -> bool {
        let square_distance = (self.x - mol.x).powi(2) + (self.y - mol.y).powi(2);
        let sum_radius = self.radius + mol.radius;
        square_distance <= sum_radius.powi(2) + 5.0
    }

    pub fn move_molecule(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn ajust_dir_after_collision(&mut self, mol: &mut Molecule) {
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

    pub fn create_vec_molecules(
        height: f32,
        xmin: f32,
        xmax: f32,
        nb_molecules: usize,
    ) -> Vec<Molecule> {
        let mut vec_molecules = vec![];
        let mut rng = thread_rng();
        let radius_distribution = Uniform::from(5f32..10f32);
        let dx_distribution = Uniform::from(-3f32..3f32);
        let dy_distribution = Uniform::from(-3f32..3f32);

        for _ in 0..nb_molecules {
            let radius = radius_distribution.sample(&mut rng);
            let x = rng.gen_range((xmin + radius)..(xmax - radius));
            let y = rng.gen_range(radius..(height - radius));
            let dx = dx_distribution.sample(&mut rng);
            let dy = dy_distribution.sample(&mut rng);

            let molecule = Molecule::new(x, y, dx, dy, radius);
            vec_molecules.push(molecule);
        }
        vec_molecules
    }

    pub fn inverse_dir(&mut self, left_side: f32, right_side: f32, height: f32) {
        if self.x <= left_side + self.radius {
            self.x = left_side + self.radius;
            self.dx = -self.dx;
        } else if self.x + self.radius >= right_side {
            self.x = right_side - self.radius;
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
