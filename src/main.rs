mod molecule;
mod tank;

use molecule::MoleculeConfig;
use notan::draw::*;
use notan::prelude::*;
use notan_egui::{self, *};
use notan_extra::FpsLimit;
use tank::Tank;

#[derive(AppState)]
struct State {
    wall: f32,
    tank: Tank,
    l: MoleculeConfig,
    r: MoleculeConfig,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let wall = (gfx.size().0 / 2) as f32;
        let height = gfx.size().1 as f32;
        let width = gfx.size().0 as f32;
        let l = MoleculeConfig::default();
        let r = MoleculeConfig::default();
        let tank = Tank::new(height, width, wall, 100, 200, &l, &r);
        Self { wall, tank, r, l }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let fps_limit = FpsLimit::new(60).sleep(true);

    let windows = WindowConfig::new()
        .title("TP2 next generation")
        .resizable(true)
        .min_size(400, 300);

    notan::init_with(State::new)
        .add_config(DrawConfig)
        .add_config(windows)
        .add_config(EguiConfig)
        .add_plugin(fps_limit)
        .draw(draw)
        .update(update)
        .build()
}

fn update(app: &mut App) {
    // Closes the App pressing the Escape key.
    // On browsers the requestAnimationFrame will stop but the canvas will still be visible
    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let interface = plugins.egui(|ctx| {
        // Draw the EGUI Widget here
        draw_egui_widget(ctx, state, gfx);
    });

    state.tank.width = gfx.size().0 as f32;
    state.tank.height = gfx.size().1 as f32;

    // Move molecule
    state.tank.update();

    // Repaint the scene
    let mut scene = gfx.create_draw();
    state.tank.render(gfx, &mut scene);
    gfx.render(&interface);
}

// Creates a widget to change the properties
fn draw_egui_widget(ctx: &Context, state: &mut State, gfx: &mut Graphics) {
    Window::new("Simulation parameters")
        .default_width(400.0)
        .resizable(false)
        .show(ctx, |ui| draw_egui_ui(ui, state, gfx));
}

fn draw_egui_ui(ui: &mut Ui, state: &mut State, gfx: &mut Graphics) {
    let mut wall_position = state.wall;
    let mut left_molecules = state.tank.l_mol.len();
    let mut right_molecules = state.tank.r_mol.len();

    let mut l_dx = state.l.dx_range.end;
    let mut r_dx = state.r.dx_range.end;

    // Needed to keep the function from growing each frame
    let mut l_radius_average = (state.l.radius_range.start + state.l.radius_range.end - 9.0) / 2.0;
    let mut l_radius_variance =
        (state.l.radius_range.end - state.l.radius_range.start - 1.0) / l_radius_average * 50.0;
    let mut r_radius_average = (state.r.radius_range.start + state.r.radius_range.end - 9.0) / 2.0;
    let mut r_radius_variance =
        (state.r.radius_range.end - state.r.radius_range.start - 1.0) / r_radius_average * 50.0;

    let right_space = (gfx.size().0 / 10 * 10 - 100) as f32;

    ui.label("Wall position");
    ui.add(Slider::new(&mut wall_position, 100.0..=right_space));

    Grid::new("grid").show(ui, |ui| {
        ui.label("Left molecules number");
        ui.label("Right molecules number");
        ui.end_row();

        ui.add(Slider::new(&mut left_molecules, 1..=1000).logarithmic(true));
        ui.add(Slider::new(&mut right_molecules, 1..=1000).logarithmic(true));
        ui.end_row();
    });
    let mut default = false;
    ui.collapsing("Advanced settings for new molecules", |ui| {
        Grid::new("grid2").show(ui, |ui| {
            ui.label("Left speed");
            ui.label("Right speed");
            ui.end_row();

            ui.add(Slider::new(&mut l_dx, 1f32..=8f32));
            ui.add(Slider::new(&mut r_dx, 1f32..=8f32));
            ui.end_row();

            ui.label("Left average radius");
            ui.label("Right average radius");
            ui.end_row();

            ui.add(Slider::new(&mut l_radius_average, 4f32..=20f32));
            ui.add(Slider::new(&mut r_radius_average, 4f32..=20f32));
            ui.end_row();

            ui.label("Left radius variance");
            ui.label("Right radius variance");
            ui.end_row();

            ui.add(Slider::new(&mut l_radius_variance, 0f32..=100f32).suffix("%"));
            ui.add(Slider::new(&mut r_radius_variance, 0f32..=100f32).suffix("%"));
            ui.end_row();

            default = ui.add(Button::new("Default")).clicked();
        });
    });

    if state.tank.wall > right_space {
        wall_position = right_space - 10.0;
    }

    state.tank.wall = 0.9 * state.tank.wall + 0.1 * wall_position;
    state.wall = wall_position;

    if default {
        state.l = MoleculeConfig::default();
        state.r = MoleculeConfig::default();
    } else {
        // Avoid having excessively small particle and equal start and end.
        let min_radius = (1.0 - l_radius_variance / 100.0) * l_radius_average + 4.0;
        let max_radius = (1.0 + l_radius_variance / 100.0) * l_radius_average + 5.0;
        state.l = MoleculeConfig::new(l_dx, l_dx, min_radius, max_radius);

        let min_radius = (1.0 - r_radius_variance / 100.0) * r_radius_average + 4.0;
        let max_radius = (1.0 + r_radius_variance / 100.0) * r_radius_average + 5.0;
        state.r = MoleculeConfig::new(r_dx, r_dx, min_radius, max_radius);
    }

    if ui.add(Button::new("Reinitialize")).clicked() {
        state.tank = Tank::new(
            state.tank.height,
            state.tank.width,
            state.tank.wall,
            left_molecules,
            right_molecules,
            &state.l,
            &state.r,
        );
    } else {
        state
            .tank
            .update_mol_number(left_molecules, right_molecules, &state.l, &state.r);
    }
}
