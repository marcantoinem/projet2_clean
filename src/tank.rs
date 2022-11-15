use notan::draw::*;
use notan::prelude::*;

use crate::molecule::Molecule;
use crate::molecule::MoleculeConfig;

pub struct Tank {
    pub height: f32,
    pub width: f32,
    pub wall: f32,
    pub left_molecules: Vec<Molecule>,
    pub right_molecules: Vec<Molecule>,
    pub left_collisions: Vec<u8>,
    pub right_collisions: Vec<u8>,
}

impl Tank {
    pub fn new(
        height: f32,
        width: f32,
        wall: f32,
        l_nb: usize,
        r_nb: usize,
        l: &MoleculeConfig,
        r: &MoleculeConfig,
    ) -> Tank {
        let left_molecules = Molecule::create_vec_molecules(height, 0f32, wall, l_nb, &r);
        let right_molecules = Molecule::create_vec_molecules(height, wall, width, r_nb, &l);
        let left_collisions = vec![0; l_nb * (l_nb - 1) / 2];
        let right_collisions = vec![0; r_nb * (r_nb - 1) / 2];
        Tank {
            height,
            width,
            wall,
            left_molecules,
            right_molecules,
            left_collisions,
            right_collisions,
        }
    }

    pub fn collisions(&mut self) {
        let mut k = 0;
        for i in 0..(self.left_molecules.len() - 1) {
            for j in (i + 1)..(self.left_molecules.len()) {
                let mut iter_mut = self.left_molecules.iter_mut();
                let mol_1 = iter_mut.nth(i).unwrap();
                let mol_2 = iter_mut.nth(j - i - 1).unwrap();
                let contact = mol_1.is_molecules_touching(mol_2);
                if contact && self.left_collisions[k] == 0 {
                    mol_1.adjust_dir_after_collision(mol_2);
                    self.left_collisions[k] = 2;
                } else if !contact && self.left_collisions[k] > 0 {
                    self.left_collisions[k] -= 1;
                }
                k += 1;
            }
        }
        let mut k = 0;
        for i in 0..(self.right_molecules.len() - 1) {
            for j in (i + 1)..(self.right_molecules.len()) {
                let mut iter_mut = self.right_molecules.iter_mut();
                let mol_1 = iter_mut.nth(i).unwrap();
                let mol_2 = iter_mut.nth(j - i - 1).unwrap();
                let contact = mol_1.is_molecules_touching(mol_2);
                if contact && self.right_collisions[k] == 0 {
                    mol_1.adjust_dir_after_collision(mol_2);
                    self.right_collisions[k] = 5;
                } else if !contact && self.right_collisions[k] > 0 {
                    self.right_collisions[k] -= 1;
                }
                k += 1;
            }
        }
    }

    pub fn inverse_dir_molecules(&mut self) {
        for mol in self.left_molecules.iter_mut() {
            mol.inverse_dir(0.0, self.wall, self.height);
        }
        for mol in self.right_molecules.iter_mut() {
            mol.inverse_dir(self.wall, self.width, self.height);
        }
    }

    pub fn update_molecules_number(
        &mut self,
        new_right: usize,
        new_left: usize,
        r: &mut MoleculeConfig,
        l: &mut MoleculeConfig,
    ) {
        let difference = new_left as isize - self.left_molecules.len() as isize;
        if difference > 0 {
            let nb_molecule = difference as usize;
            let new_molecules =
                Molecule::create_vec_molecules(self.height, 0.0, self.wall, nb_molecule, r);
            self.left_molecules.extend(new_molecules);
            self.left_collisions =
                vec![0; self.left_molecules.len() * (self.left_molecules.len() - 1) / 2];
        } else if difference < 0 {
            for _ in 0..(-difference as usize) {
                self.left_molecules.pop();
            }
            self.left_collisions =
                vec![0; self.left_molecules.len() * (self.left_molecules.len() - 1) / 2];
        }

        let difference = new_right as isize - self.right_molecules.len() as isize;
        if difference > 0 {
            let nb_molecule = difference as usize;
            let new_molecules =
                Molecule::create_vec_molecules(self.height, self.wall, self.width, nb_molecule, l);
            self.right_molecules.extend(new_molecules);
            self.right_collisions =
                vec![0; self.right_molecules.len() * (self.right_molecules.len() - 1) / 2];
        } else if difference < 0 {
            for _ in 0..(-difference as usize) {
                self.right_molecules.pop();
            }
            self.right_collisions =
                vec![0; self.right_molecules.len() * (self.right_molecules.len() - 1) / 2];
        }
    }

    pub fn move_molecules(&mut self) {
        for mol in self.left_molecules.iter_mut() {
            mol.move_molecule();
        }

        for mol in self.right_molecules.iter_mut() {
            mol.move_molecule();
        }
    }

    pub fn update(&mut self) {
        self.inverse_dir_molecules();
        self.collisions();
        self.move_molecules();
    }

    pub fn render(&self, gfx: &mut Graphics, draw: &mut Draw) {
        draw.clear(Color::WHITE);

        let twice_total = (2 * self.left_molecules.len()) as f32;
        for (i, mol) in self.left_molecules.iter().enumerate() {
            let other_colors = i as f32 / twice_total;
            draw.circle(mol.radius)
                .position(mol.x, mol.y)
                .color(Color::from_rgb(1.0, other_colors, other_colors));
        }

        let twice_total = (2 * self.right_molecules.len()) as f32;
        for (i, mol) in self.right_molecules.iter().enumerate() {
            let other_colors = i as f32 / twice_total;
            draw.circle(mol.radius)
                .position(mol.x, mol.y)
                .color(Color::from_rgb(other_colors, other_colors, 1.0));
        }

        draw.line((self.wall, 0f32), (self.wall, self.height))
            .color(Color::BLUE)
            .width(5f32);

        gfx.render(draw);
    }
}
