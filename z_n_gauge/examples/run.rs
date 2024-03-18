#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_macros,
    incomplete_features
)]
#![feature(generic_const_exprs)]

use std::{f32::consts::PI, fmt, fs::File, ops::Deref, thread};

use crunchy::{
    crarray::shape::Shape,
    lattice::{
        ind::Ind,
        lattice::{EmptyField, Field, Lattice, LatticeTypes, SimParameter, UpdateField},
    },
    math_utils::binomial_coefficient,
};
use ndarray::{Array, IxDyn};
use rand::Rng;

#[derive(Debug, Clone, Copy, Default)]
struct EdgeField<const NDIM: usize, const ZORDER: usize> {
    phase: usize,
}

impl<const NDIM: usize, const ZORDER: usize> Field for EdgeField<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    type IndexType = [usize; NDIM + 1];
    type GridType = Lattice<NDIM, ZNLatticeTypes<NDIM, ZORDER>>;
}

impl<const NDIM: usize, const ZORDER: usize> UpdateField for EdgeField<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    fn update(node_index: Self::IndexType, grid: &mut Self::GridType, new_field: Self) -> Self {
        let old_field = grid.edges[node_index].data;
        grid.edges[node_index].data = new_field;

        let faces = grid.edges[node_index].graph_connections.faces;
        for plaquette_index in faces.iter() {
            PlaquetteField::<NDIM, ZORDER>::rebuild(*plaquette_index, grid);
        }

        let cubes = grid.edges[node_index].graph_connections.cubes;
        for cube_index in cubes.iter() {
            MonopoleField::<NDIM, ZORDER>::rebuild(*cube_index, grid);
        }

        old_field
    }

    fn energy(node_index: Self::IndexType, grid: &mut Self::GridType) -> f32 {
        let mut action = 0.;
        let ZNParameters {
            beta,
            cosines,
            lambda,
        } = grid.sim_parameters;

        for plaquette_id in grid.edges[node_index].graph_connections.faces.iter() {
            action += beta * (1. - 1. * cosines[grid.faces[*plaquette_id].data.holonomy])
        }

        for cube_id in grid.edges[node_index].graph_connections.cubes.iter() {
            action += lambda * grid.cubes[*cube_id].data.charge.abs() as f32;
        }

        action
    }

    fn init() -> impl FnMut([usize; NDIM + 1]) -> Self
    where
        [(); NDIM + 1]:,
    {
        |node_index: Self::IndexType| EdgeField::<NDIM, ZORDER> { phase: 0 }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct PlaquetteField<const NDIM: usize, const ZORDER: usize> {
    holonomy: usize,
    dirac_string: isize,
}

impl<const NDIM: usize, const ZORDER: usize> fmt::Display for PlaquetteField<NDIM, ZORDER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlaquetteField: {}", self.holonomy)
    }
}

impl<const NDIM: usize, const ZORDER: usize> Field for PlaquetteField<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    type IndexType = [usize; NDIM + 1];
    type GridType = Lattice<NDIM, ZNLatticeTypes<NDIM, ZORDER>>;

    fn rebuild(node_index: Self::IndexType, grid: &mut Self::GridType) {
        let edges = grid.faces[node_index].graph_connections.edges;

        let mut holonomy = 2 * ZORDER;
        holonomy += grid.edges[edges[0]].data.phase;
        holonomy -= grid.edges[edges[1]].data.phase;
        holonomy += grid.edges[edges[2]].data.phase;
        holonomy -= grid.edges[edges[3]].data.phase;

        grid.faces[node_index].data.holonomy = holonomy % ZORDER;

        let dirac_string = calculate_dirac_string::<NDIM, ZORDER>(node_index, grid);
        grid.faces[node_index].data.dirac_string = dirac_string;
    }
}

fn calculate_dirac_string<const NDIM: usize, const ZORDER: usize>(
    face_index: [usize; NDIM + 1],
    grid: &Lattice<NDIM, ZNLatticeTypes<NDIM, ZORDER>>,
) -> isize
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    fn shift<const ZORDER: usize>(phase: usize) -> isize {
        if phase <= ZORDER / 2 {
            return phase as isize;
        }
        return phase as isize - ZORDER as isize;
    }

    let mut windings = 0;
    let edges = grid.faces[face_index].graph_connections.edges;
    windings += shift::<ZORDER>(grid.edges[edges[0]].data.phase);
    windings -= shift::<ZORDER>(grid.edges[edges[1]].data.phase);
    windings += shift::<ZORDER>(grid.edges[edges[2]].data.phase);
    windings -= shift::<ZORDER>(grid.edges[edges[3]].data.phase);

    if ZORDER != 3 {
        panic!("Only implemented for Z3")
    }

    let mut res = 0;
    // For ZORDER > 3, windings of 2?
    if windings < -(ZORDER as isize) / 2 {
        res = -1;
    }
    if windings > ZORDER as isize / 2 {
        res = 1;
    }

    res
}

#[derive(Debug, Clone, Copy, Default)]
struct MonopoleField<const NDIM: usize, const ZORDER: usize> {
    charge: isize,
}

impl<const NDIM: usize, const ZORDER: usize> Field for MonopoleField<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    type IndexType = [usize; NDIM + 1];
    type GridType = Lattice<NDIM, ZNLatticeTypes<NDIM, ZORDER>>;

    fn rebuild(node_index: Self::IndexType, grid: &mut Self::GridType) {
        let faces = grid.cubes[node_index].graph_connections.faces;

        let mut charge = 0;
        let mut alternator = 1;
        for face_index in faces.iter() {
            charge += alternator * grid.faces[*face_index].data.dirac_string;
            alternator *= -1;
        }

        grid.cubes[node_index].data.charge = charge;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ZNLatticeTypes<const NDIM: usize, const ZORDER: usize> {}

impl<const NDIM: usize, const ZORDER: usize> LatticeTypes for ZNLatticeTypes<NDIM, ZORDER>
where
    [(); NDIM + 1]:,
    [(); NDIM * 2]:,
    [(); 2 * (NDIM - 1)]:,
    [(); 4 * binomial_coefficient(NDIM, 2)]:,
    [(); 4 * binomial_coefficient(NDIM - 1, 2)]:,
    [(); 2 * (NDIM - 2)]:,
    [(); 8 * binomial_coefficient(NDIM, 3)]:,
{
    const NDIM: usize = NDIM;

    type VertexType = EmptyField<ZNLatticeTypes<NDIM, ZORDER>>;
    type EdgeType = EdgeField<NDIM, ZORDER>;
    type FaceType = PlaquetteField<NDIM, ZORDER>;
    type CubeType = MonopoleField<NDIM, ZORDER>;

    type SimParameterType = ZNParameters<ZORDER>;
}

#[derive(Debug, Clone, Copy)]
struct ZNParameters<const ZORDER: usize> {
    beta: f32,
    cosines: [f32; ZORDER],
    lambda: f32,
}

fn generate_cosine<const ZORDER: usize>() -> [f32; ZORDER] {
    let pi = std::f32::consts::PI;
    let mut res = [0.0; ZORDER];

    // Generate perfectly symmetric cosine, otherwise this lead to symmetry breaking
    for i in 0..(ZORDER / 2 + 1) {
        res[i] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        if i != 0 {
            res[ZORDER - i] = (i as f32 * 2.0 * pi / ZORDER as f32).cos();
        }
    }

    res
}

impl<const ZORDER: usize> SimParameter for ZNParameters<ZORDER> {}

fn record_polyakov_loops<const ZORDER: usize>(
    sim: &mut Simulation<ZORDER>,
    complex_roots: [[f32; 2]; ZORDER],
    recordings: usize,
) -> Array<[f32; 2], IxDyn> {
    let grid_shape = sim.shape;
    let [x_dim, y_dim, z_dim, t_dim] = grid_shape.dim;

    let res_shape = [recordings, x_dim, y_dim, z_dim];
    let zeros = vec![[0., 0.]; res_shape.iter().product()];

    let mut polyakov_recordings: Array<[f32; 2], IxDyn> =
        Array::from_shape_vec(res_shape, zeros).unwrap().into_dyn();

    for i in 0..recordings {
        for x in 0..x_dim {
            for y in 0..y_dim {
                for z in 0..z_dim {
                    let mut singe_loop = 0;
                    for t in 0..t_dim {
                        let a = sim.edges[[3, x, y, z, t]];
                        singe_loop += a.data.phase;
                    }
                    singe_loop %= ZORDER;
                    polyakov_recordings[[i, x, y, z]] = complex_roots[singe_loop];
                }
            }
        }
        sim.sweep();
    }

    return polyakov_recordings;
}

fn polyakov_record_range<const ZORDER: usize>(
    beta_slice: &[f32],
    recordings: usize,
    grid_shape: Shape<4>,
    thermalization_steps: usize,
    number_of_threads: usize,
    lambda: f32,
) -> Vec<(f32, Array<[f32; 2], IxDyn>)> {
    let mut complex_roots = [[0.0, 0.0]; ZORDER];
    for i in 0..ZORDER {
        let angle = 2.0 * PI * i as f32 / ZORDER as f32;
        complex_roots[i] = [angle.cos(), angle.sin()];
    }

    let mut res = vec![];

    for (i, beta) in beta_slice.iter().enumerate() {
        println!("{:?}", i * number_of_threads);

        let sim_pars = ZNParameters {
            beta: *beta,
            cosines: generate_cosine::<ZORDER>(),
            lambda,
        };

        let lattice = Lattice::<4, ZNLatticeTypes<4, ZORDER>>::new(grid_shape, sim_pars);

        let mut sim: Simulation<ZORDER> = Simulation { sim: lattice };
        let a = record_polyakov_loops::<ZORDER>(&mut sim, complex_roots, recordings);

        res.push((*beta, a));
    }
    return res;
}

#[derive(Debug, Clone, Copy)]
pub struct PolyakovParameters {
    pub beta_range: [f32; 2],
    pub steps: usize,
    pub number_of_threads: usize,
    pub recordings: usize,
    pub grid_shape: Shape<4>,
    pub thermalization_steps: usize,
    pub lambda: f32,
}

pub fn split_beta_range(
    start_beta: f32,
    end_beta: f32,
    steps: usize,
    number_threads: usize,
) -> Vec<Vec<f32>> {
    let beta_step = (end_beta - start_beta) / (steps - 1) as f32;

    let steps_per_thread = steps / number_threads;

    let mut rest = steps % number_threads;

    let mut steps_list = vec![];

    for _ in 0..number_threads {
        let overflow = if rest != 0 {
            rest -= 1;
            1
        } else {
            0
        };
        steps_list.push(steps_per_thread + overflow)
    }

    let mut accumulator = start_beta;

    let mut betas = vec![];

    for i in 0..number_threads {
        let mut temp = vec![];
        for j in 0..steps_list[i] {
            temp.push(accumulator + j as f32 * beta_step)
        }
        betas.push(temp);
        accumulator += steps_list[i] as f32 * beta_step;
    }

    return betas;
}

fn polyakov_record_range_threaded<const ZORDER: usize>(
    parameter: PolyakovParameters,
) -> Vec<(f32, Array<[f32; 2], IxDyn>)> {
    let PolyakovParameters {
        beta_range,
        steps,
        number_of_threads,
        recordings,
        grid_shape,
        thermalization_steps,
        lambda,
    } = parameter;
    let [start_beta, end_beta] = beta_range;

    let betas = split_beta_range(start_beta, end_beta, steps, number_of_threads);

    let mut thread_pool = vec![];

    for i in 0..number_of_threads {
        let beta_i = betas[i].to_vec();
        thread_pool.push(thread::spawn(move || {
            let partial_data = polyakov_record_range::<ZORDER>(
                &beta_i,
                recordings,
                grid_shape,
                thermalization_steps,
                number_of_threads,
                lambda,
            );

            return partial_data;
        }));
    }

    let mut res = vec![];

    for thread in thread_pool {
        let mut partial_data = thread.join().unwrap();
        res.append(&mut partial_data);
    }

    return res;
}

struct Simulation<const ZORDER: usize> {
    sim: Lattice<4, ZNLatticeTypes<4, ZORDER>>,
}

impl<const ZORDER: usize> Simulation<ZORDER> {
    fn new(shape: Shape<4>, sim_parameters: ZNParameters<ZORDER>) -> Self {
        let sim = Lattice::<4, ZNLatticeTypes<4, ZORDER>>::new(shape, sim_parameters);

        Simulation { sim }
    }

    fn sweep(&mut self) {
        let edge_shape = self.edges.shape();
        let mut rng_gen = rand::thread_rng();

        for edge_index in self.sim.edges.shape().iter() {
            let new_edge = EdgeField {
                phase: rng_gen.gen_range(0..ZORDER),
            };
            EdgeField::metropolis_step(edge_index, &mut self.sim, new_edge, &mut rng_gen)
        }
    }
}

impl<const ZORDER: usize> Deref for Simulation<ZORDER> {
    type Target = Lattice<4, ZNLatticeTypes<4, ZORDER>>;
    fn deref(&self) -> &Self::Target {
        &self.sim
    }
}

use byteorder::{LittleEndian, WriteBytesExt};
use npyz::{
    npz, AutoSerialize, DType, Field as NpyField, NpyWriter, Serialize, TypeWrite, WriterBuilder,
};
use zip::write::FileOptions;

pub struct Loop {
    pub beta: f32,
    pub data: Array<[f32; 2], IxDyn>,
}

impl Serialize for Loop {
    type TypeWriter = LoopWriter;

    fn writer(_dtype: &npyz::DType) -> Result<Self::TypeWriter, npyz::DTypeError> {
        Ok(LoopWriter {})
    }
}

pub struct LoopWriter {}

impl TypeWrite for LoopWriter {
    type Value = Loop;

    fn write_one<W: std::io::Write>(
        &self,
        mut writer: W,
        value: &Self::Value,
    ) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_f32::<LittleEndian>(value.beta)?;

        let ptr = value.data.as_ptr();
        let len = value.data.len();
        let raw: &[u8] = unsafe {
            std::slice::from_raw_parts(ptr as *const u8, len * std::mem::size_of::<[f32; 2]>())
        };
        writer.write_all(raw)?;

        Ok(())
    }
}

pub fn create_dtype(shape: &[usize]) -> DType {
    let inner = Box::new(DType::Array(
        2,
        Box::new(DType::Plain("<f4".parse().unwrap())),
    ));

    let mut dtype = inner;
    for dim in shape.iter().rev() {
        println!("{:?}", dim);
        dtype = Box::new(DType::Array(*dim as u64, dtype))
    }

    let dtype = DType::Record(vec![
        NpyField {
            name: "beta".to_string(),
            dtype: DType::Plain("<f4".parse().unwrap()),
        },
        NpyField {
            name: "data".to_string(),
            dtype: *dtype,
        },
    ]);

    return dtype;
}

#[derive(npyz::Serialize, npyz::Deserialize, npyz::AutoSerialize, Debug, PartialEq, Clone)]
pub struct MetaData {
    pub z_order: u32,
}

pub fn create_writer<'a, W: std::io::Write, T: ?Sized + Serialize>(
    struc_array_shape: &[u64],
    dtype: DType,
    writer: &'a mut W,
) -> NpyWriter<T, &'a mut W> {
    npyz::WriteOptions::<T>::new()
        .dtype(dtype)
        .shape(&struc_array_shape)
        .writer(writer)
        .begin_nd()
        .unwrap()
}

fn main() {
    // const LATTICEDIM: usize = 6;
    // let shape = Shape::new([LATTICEDIM, LATTICEDIM, LATTICEDIM, LATTICEDIM]);
    // const ZORDER: usize = 3;

    // let mut sim = Simulation::<ZORDER>::new(
    //     shape,
    //     ZNParameters {
    //         beta: 0.2,
    //         cosines: generate_cosine::<ZORDER>(),
    //     },
    // );

    // for _ in 0..3 {
    //     sim.sweep();
    // }

    const LATTICEDIM: usize = 6;
    let shape = Shape::new([LATTICEDIM, LATTICEDIM, LATTICEDIM, LATTICEDIM]);
    const ZORDER: usize = 3;

    let polyakov_parameters = PolyakovParameters {
        beta_range: [0.49, 0.56],
        steps: 48,
        number_of_threads: 8,
        recordings: 10000,
        grid_shape: shape,
        thermalization_steps: 0,
        lambda: 1.0,
    };

    let res = polyakov_record_range_threaded::<ZORDER>(polyakov_parameters);

    let configurations = res
        .into_iter()
        .map(|x| Loop {
            beta: x.0,
            data: x.1,
        })
        .collect::<Vec<Loop>>();

    let file = File::create("z_n_gauge/data_analysis/temp/transfer_zip.npz").unwrap();

    // Write Configurations
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default().large_file(true);
    zip.start_file(npz::file_name_from_array_name("loops"), options)
        .unwrap();
    let dtype_configs = create_dtype(&configurations[0].data.shape());
    let mut writer =
        create_writer::<_, Loop>(&[configurations.len() as u64], dtype_configs, &mut zip);
    writer.extend(configurations).unwrap();
    writer.finish().unwrap();

    // Write Metadata
    zip.start_file(
        npz::file_name_from_array_name("meta_data"),
        Default::default(),
    )
    .unwrap();
    let dtype_meta_data = <MetaData as AutoSerialize>::default_dtype();
    let mut writer = create_writer::<_, MetaData>(&[1], dtype_meta_data, &mut zip);
    writer
        .extend(vec![MetaData {
            z_order: ZORDER as u32,
        }])
        .unwrap();
    writer.finish().unwrap();

    zip.finish().unwrap();
}
