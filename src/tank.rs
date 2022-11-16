use notan::draw::*;
use notan::prelude::*;

use crate::molecule::Molecule;
use crate::molecule::MoleculeConfig;

pub struct Tank {
    pub height: f32,
    pub width: f32,
    pub wall: f32,
    pub l_mol: Vec<Molecule>,
    pub r_mol: Vec<Molecule>,
    pub l_collisions: Vec<u8>,
    pub r_collisions: Vec<u8>,
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
        let l_mol = Molecule::create_vec_mol(height, 0f32, wall, l_nb, &l);
        let r_mol = Molecule::create_vec_mol(height, wall, width, r_nb, &r);
        let l_collisions = vec![0; l_nb * (l_nb - 1) / 2];
        let r_collisions = vec![0; r_nb * (r_nb - 1) / 2];
        Tank {
            height,
            width,
            wall,
            l_mol,
            r_mol,
            l_collisions,
            r_collisions,
        }
    }

    pub fn collisions(&mut self) {
        let mut k = 0;
        for i in 0..(self.l_mol.len() - 1) {
            for j in (i + 1)..(self.l_mol.len()) {
                let mut iter_mut = self.l_mol.iter_mut();
                let mol_1 = iter_mut.nth(i).unwrap();
                let mol_2 = iter_mut.nth(j - i - 1).unwrap();
                let contact = mol_1.is_mols_touching(mol_2);
                if contact && self.l_collisions[k] == 0 {
                    mol_1.adjust_dir_after_collision(mol_2);
                    self.l_collisions[k] = 2;
                } else if !contact && self.l_collisions[k] > 0 {
                    self.l_collisions[k] -= 1;
                }
                k += 1;
            }
        }
        let mut k = 0;
        for i in 0..(self.r_mol.len() - 1) {
            for j in (i + 1)..(self.r_mol.len()) {
                let mut iter_mut = self.r_mol.iter_mut();
                let mol_1 = iter_mut.nth(i).unwrap();
                let mol_2 = iter_mut.nth(j - i - 1).unwrap();
                let contact = mol_1.is_mols_touching(mol_2);
                if contact && self.r_collisions[k] == 0 {
                    mol_1.adjust_dir_after_collision(mol_2);
                    self.r_collisions[k] = 5;
                } else if !contact && self.r_collisions[k] > 0 {
                    self.r_collisions[k] -= 1;
                }
                k += 1;
            }
        }
    }

    pub fn inverse_dir_mol(&mut self) {
        for mol in self.l_mol.iter_mut() {
            mol.inverse_dir(0.0, self.wall, self.height);
        }
        for mol in self.r_mol.iter_mut() {
            mol.inverse_dir(self.wall, self.width, self.height);
        }
    }

    pub fn update_mol_number(
        &mut self,
        new_l: usize,
        new_r: usize,
        l: &MoleculeConfig,
        r: &MoleculeConfig,
    ) {
        let difference = new_l as isize - self.l_mol.len() as isize;
        if difference > 0 {
            let nb_molecule = difference as usize;
            let new_mol = Molecule::create_vec_mol(self.height, 0.0, self.wall, nb_molecule, l);
            self.l_mol.extend(new_mol);
            self.l_collisions = vec![0; self.l_mol.len() * (self.l_mol.len() - 1) / 2];
        } else if difference < 0 {
            for _ in 0..(-difference as usize) {
                self.l_mol.pop();
            }
            self.l_collisions = vec![0; self.l_mol.len() * (self.l_mol.len() - 1) / 2];
        }

        let difference = new_r as isize - self.r_mol.len() as isize;
        if difference > 0 {
            let nb_molecule = difference as usize;
            let new_mol =
                Molecule::create_vec_mol(self.height, self.wall, self.width, nb_molecule, r);
            self.r_mol.extend(new_mol);
            self.r_collisions = vec![0; self.r_mol.len() * (self.r_mol.len() - 1) / 2];
        } else if difference < 0 {
            for _ in 0..(-difference as usize) {
                self.r_mol.pop();
            }
            self.r_collisions = vec![0; self.r_mol.len() * (self.r_mol.len() - 1) / 2];
        }
    }

    pub fn move_mol(&mut self) {
        for mol in self.l_mol.iter_mut() {
            mol.move_mol();
        }

        for mol in self.r_mol.iter_mut() {
            mol.move_mol();
        }
    }

    pub fn update(&mut self) {
        self.inverse_dir_mol();
        self.collisions();
        self.move_mol();
    }

    pub fn render(&self, gfx: &mut Graphics, draw: &mut Draw) {
        draw.clear(Color::WHITE);

        let twice_total = (2 * self.l_mol.len()) as f32;
        for (i, mol) in self.l_mol.iter().enumerate() {
            let other_colors = i as f32 / twice_total;
            draw.circle(mol.radius)
                .position(mol.x, mol.y)
                .color(Color::from_rgb(1.0, other_colors, other_colors));
        }

        let twice_total = (2 * self.r_mol.len()) as f32;
        for (i, mol) in self.r_mol.iter().enumerate() {
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
