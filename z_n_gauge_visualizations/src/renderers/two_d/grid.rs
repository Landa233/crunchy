use crunchy::crarray::shape::Shape;
use iris::{Camera, Screen};

use crate::simulation::simulation::Simulation2D;

use super::plaquette::PlaquetteRenderer;

#[derive(Debug, Clone)]
pub struct GridSettings {
    pub spacing: f32,
    pub colors: Vec<[f32; 4]>,
}

pub struct GridRenderer {
    pub plaquette_renderer: PlaquetteRenderer,

    pub grid_settings: GridSettings,
}

impl GridRenderer {
    pub fn new<const ZORDER: usize>(
        screen: &Screen,
        camera: &Camera,
        shape: Shape<2>,
        grid_settings: GridSettings,
        sim: &Simulation2D<ZORDER>,
    ) -> Self {
        let plaquette_renderer =
            PlaquetteRenderer::new(screen, camera, shape, grid_settings.clone());
        Self {
            plaquette_renderer,
            grid_settings,
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.plaquette_renderer.renderer.render(render_pass);
    }
}
