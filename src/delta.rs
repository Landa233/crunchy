struct Node<GraphConnections, Field: FieldData> {
    graph_connections: GraphConnections,
    data: Field,
}

trait FieldData {
    fn rebuild<Marker: LatticeMarker>(node_index: u32, grid: &Lattice<Marker>) -> Self;
}

impl FieldData for () {
    fn rebuild<Marker: LatticeMarker>(node_index: u32, grid: &Lattice<Marker>) -> Self {}
}

struct EdgeConnections {
    vertices: [u32; 2],
}

trait Energy: FieldData {
    fn energy(&self) -> f64;
}

struct VertexConnections {
    edges: [u32; 3],
}

type VertexNode<Field> = Node<VertexConnections, Field>;
type EdgeNode<Field> = Node<EdgeConnections, Field>;

trait Nody {
    fn update<Marker: LatticeMarker>(node_index: u32, grid: &mut Lattice<Marker>);
}

impl<Field: FieldData> Nody for VertexNode<Field> {
    fn update<Marker: LatticeMarker>(node_index: u32, grid: &mut Lattice<Marker>) {
        for edge_index in grid.vertices[node_index as usize]
            .graph_connections
            .edges
            .iter()
        {
            let edge = <Marker as LatticeMarker>::EdgeData::rebuild(*edge_index, grid);
            grid.edges[*edge_index as usize].data = edge;
        }
    }
}

trait LatticeMarker {
    type VertexData: FieldData;
    type EdgeData: FieldData;
}

struct Lattice<Marker: LatticeMarker> {
    vertices: Vec<VertexNode<<Marker as LatticeMarker>::VertexData>>,
    edges: Vec<EdgeNode<<Marker as LatticeMarker>::EdgeData>>,
}
