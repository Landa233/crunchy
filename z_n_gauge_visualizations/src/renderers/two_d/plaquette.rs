use bytemuck::{Pod, Zeroable};
use crunchy::crarray::shape::Shape;
use iris::{
    byteable::Byteable, renderer::index_renderer::IndexRenderer, shapes::Rect,
    vertex::vertex::Vertex, Camera, Screen,
};
use std::ops::Add;

use super::grid::GridSettings;

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Color {
    rgba: [f32; 4],
}
impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            rgba: [
                self.rgba[0] + rhs.rgba[0],
                self.rgba[1] + rhs.rgba[1],
                self.rgba[2] + rhs.rgba[2],
                self.rgba[3] + rhs.rgba[3],
            ],
        }
    }
}

impl Byteable for Color {}

pub struct PlaquetteRenderer {
    pub renderer: IndexRenderer<Vertex<2, Color>>,
    pub vertices: Vec<Vertex<2, Color>>,
    pub shape: Shape<2>,
}

impl PlaquetteRenderer {
    pub fn new(
        screen: &Screen,
        camera: &Camera,
        shape: Shape<2>,
        grid_settings: GridSettings,
    ) -> Self {
        let vertex_layout = vec![
            // Position
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            // Color
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: wgpu::VertexFormat::Float32x2.size(),
                shader_location: 1,
            },
        ];

        let mut rectangles = vec![];

        let dim = shape.dim;
        let spacing = grid_settings.spacing;

        for i in 0..dim[0] {
            for j in 0..dim[1] {
                let rect = Rect::new(
                    Rect::from_pixel(
                        [i as f32 * spacing, j as f32 * spacing],
                        [spacing, spacing],
                        screen,
                    ),
                    Rect::uniform(Color {
                        rgba: [0.0, 1.0, 0.0, 1.0],
                    }),
                );
                rectangles.push(rect);
            }
        }

        let vertices = Rect::into_vertices(&rectangles);
        let indicies = Rect::index_buffer(&rectangles);

        let renderer = IndexRenderer::new(
            screen,
            &vec![&camera.create_camera_bind_group(0, 0)],
            "src/renderers/shaders/colored_2d_quad.wgsl",
            &vertices,
            vertex_layout,
            &indicies,
        );

        Self {
            renderer,
            vertices,
            shape,
        }
    }

    pub fn update(&mut self, screen: &Screen, coord: [usize; 2], color: [f32; 4]) {
        let flat_index = coord[0] + coord[1] * self.shape.dim[0];
        for i in 4 * flat_index..(4 * flat_index + 4) {
            let vertex = &mut self.vertices[i];
            vertex.info.rgba = color;
        }

        self.renderer.upload_vertex_slice(
            screen,
            &self.vertices[(4 * flat_index)..((4 * flat_index) + 4)],
            4 * flat_index,
        );
    }
}
