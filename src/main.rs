mod molecule;
mod tank;

use notan::draw::*;
use notan::prelude::*;
use notan_egui::{self, *};
use notan_extra::FpsLimit;
use tank::Tank;

#[derive(AppState)]
struct State {
    wall: f32,
    tank: Tank,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let wall = (gfx.size().0 / 2) as f32;
        let tank = Tank::new(gfx.size().1 as f32, gfx.size().0 as f32, wall, 100, 200);
        Self { wall, tank }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let fps_limit = FpsLimit::new(60).sleep(true);

    let windows = WindowConfig::new()
        .title("TP2 next generation")
        .resizable(true)
        .min_size(400, 300)
        // .vsync(true)
        .high_dpi(true);

    notan::init_with(State::new)
        .add_config(DrawConfig)
        .add_config(windows)
        .add_config(EguiConfig)
        .add_plugin(fps_limit)
        .draw(draw)
        .build()
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
        .show(ctx, |ui| draw_egui_ui(ui, state, gfx));
}

fn draw_egui_ui(ui: &mut Ui, state: &mut State, gfx: &mut Graphics) {
    let mut wall_position = state.wall;
    let mut left_molecules = state.tank.left_molecules.len();
    let mut right_molecules = state.tank.right_molecules.len();

    ui.label("Wall position");
    ui.add(
        Slider::new(
            &mut wall_position,
            100.0..=((gfx.size().0 / 100 * 100 - 100) as f32),
        )
        .clamp_to_range(false)
        .step_by(1.0),
    );

    ui.label("Left molecules");
    ui.add(Slider::new(&mut left_molecules, 1..=1000));

    ui.label("Right molecules");
    ui.add(Slider::new(&mut right_molecules, 1..=1000));

    state
        .tank
        .update_molecules_number(left_molecules, right_molecules);
    state.tank.wall = 0.9 * state.tank.wall + 0.1 * wall_position;
    state.wall = wall_position;
}
