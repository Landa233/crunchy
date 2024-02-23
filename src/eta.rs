use rand::Rng;

trait FieldData: Copy + Clone {
    type Marker: LatticeMarker;
    type IndexType: Copy + Clone;
    fn rebuild(node_index: Self::IndexType, grid: &mut Lattice<Self::Marker>);
}

trait UpdateField: FieldData {
    fn update(node_index: Self::IndexType, grid: &mut Lattice<Self::Marker>) -> Self;

    fn set(node_index: Self::IndexType, grid: &mut Lattice<Self::Marker>, field: Self);

    fn energy(node_index: Self::IndexType, grid: &Lattice<Self::Marker>) -> f32;

    fn init() -> impl Initializer<Self>;
}

struct Node<GraphConnections, Field: FieldData> {
    graph_connections: GraphConnections,
    data: Field,
}

struct EdgeConnections {
    vertices: [u32; 2],
}

struct VertexConnections {
    edges: [u32; 3],
}

type VertexNode<Field> = Node<VertexConnections, Field>;
type EdgeNode<Field> = Node<EdgeConnections, Field>;

trait LatticeMarker {
    type VertexData: FieldData;
    type EdgeData: FieldData;
}

struct Lattice<Marker: LatticeMarker> {
    vertices: Vec<VertexNode<<Marker as LatticeMarker>::VertexData>>,
    edges: Vec<EdgeNode<<Marker as LatticeMarker>::EdgeData>>,
}

struct MyLattice {}
impl LatticeMarker for MyLattice {
    type VertexData = Verty;
    type EdgeData = Edgy;
}

#[derive(Copy, Clone)]
struct Verty {}
#[derive(Copy, Clone)]
struct Edgy {}

impl FieldData for Verty {
    type Marker = MyLattice;
    type IndexType = u32;

    fn rebuild(node_index: u32, grid: &mut Lattice<Self::Marker>) {}
}

impl UpdateField for Verty {
    fn update(node_index: u32, grid: &mut Lattice<Self::Marker>) -> Self {
        let a = grid.vertices[node_index as usize].graph_connections.edges;

        for edge_index in a.iter() {
            <Self::Marker as LatticeMarker>::EdgeData::rebuild(*edge_index, grid);
        }

        todo!()
    }

    fn set(node_index: u32, grid: &mut Lattice<Self::Marker>, field: Self) {
        grid.vertices[node_index as usize].data = field;
    }

    fn energy(node_index: u32, grid: &Lattice<Self::Marker>) -> f32 {
        todo!()
    }

    fn init() -> impl Initializer<Self> {
        |x: u32| Verty {} // idk why todo!() doesn't work here
    }
}

impl FieldData for Edgy {
    type Marker = MyLattice;
    type IndexType = u32;

    fn rebuild(node_index: u32, grid: &mut Lattice<Self::Marker>) {}
}

struct Simulation<Marker: LatticeMarker> {
    lattice: Lattice<Marker>,
    rng_gen: rand::rngs::ThreadRng,
}

impl<LMarker: LatticeMarker> Simulation<LMarker> {
    fn metropolis_step<Field: UpdateField>(
        &mut self,
        new_field: Field,
        node_index: <Field as FieldData>::IndexType,
    ) where
        Field: FieldData<Marker = LMarker>,
    {
        let old_energy = Field::energy(node_index, &self.lattice);

        Field::set(node_index, &mut self.lattice, new_field);
        let old_field = Field::update(node_index, &mut self.lattice);
        let new_energy = Field::energy(node_index, &self.lattice);

        let delta_energy = new_energy - old_energy;

        if delta_energy < 0.0 {
            return;
        }

        let acceptance_probability = (-delta_energy).exp();

        let random_number: f32 = self.rng_gen.gen_range(0.0..1.0); // random number between 0 and 1

        if random_number < acceptance_probability {
            return;
        }

        Field::set(node_index, &mut self.lattice, old_field);
        Field::update(node_index, &mut self.lattice);
    }
}

fn foo<F, Field>(f: F)
where
    Field: FieldData,
    F: FnMut(<Field as FieldData>::IndexType) -> Field,
{
    // Function body
    todo!()
}

trait Initializer<Field: FieldData> {
    fn init(&mut self, index: <Field as FieldData>::IndexType) -> Field;
}

impl<Field: FieldData, F> Initializer<Field> for F
where
    F: FnMut(<Field as FieldData>::IndexType) -> Field,
{
    fn init(&mut self, index: <Field as FieldData>::IndexType) -> Field {
        self(index)
    }
}
