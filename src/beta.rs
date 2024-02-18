#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_macros,
    incomplete_features
)]
#![feature(generic_const_exprs, generic_arg_infer)]

// struct A<B, C> {
//     b: PhantomData<B>,
//     c: PhantomData<C>,
// }

// trait Foo {
//     type Edge;
//     type Node;

//     type Bar = A<Self::Edge, Self::Node>;
// }

// struct Node<GraphConnections, NodeData> {
//     graph_connections: GraphConnections,
//     node_data: PhantomData<NodeData>,
// }

// impl<GraphConnections, NodeData> Node<GraphConnections, NodeData> {
//     fn baar(&self) {}
// }

// struct EdgeConnections {
//     plaquettes: (),
//     cubes: (),
// }

// type EdgeNode<NodeData> = Node<EdgeConnections, NodeData>;

// fn foo<NodeData>(a: EdgeNode<NodeData>) {
//     a.graph_connections.plaquettes;

//     a.baar();
// }

// fn a(v: &mut Vec<i32>) {
//     v.push(1);
//     b(v);
//     v.push(2);
// }

// fn b(v: &mut Vec<i32>) {}

trait GridMarker {
    type TypeA: Node;
    type TypeB: Node;
}

trait Node {
    // type NodeIndex;
    type G: GridMarker;

    fn update(node_index: u32, grid: &mut Grid<Self::G>);

    fn recalculate(node_index: u32, grid: &Grid<Self::G>) -> Self;
}

struct MyGrid {}

impl GridMarker for MyGrid {
    type TypeA = ExampleA;
    type TypeB = ExampleB;
}

struct ExampleA {}
impl Node for ExampleA {
    type G = MyGrid;

    fn update(node_index: u32, grid: &mut Grid<Self::G>) {
        for i in 0..10 {
            grid.b[i] = ExampleB::recalculate(i as u32, grid);
        }
    }

    fn recalculate(node_index: u32, grid: &Grid<Self::G>) -> Self {
        todo!()
    }
}

struct ExampleB {}
impl Node for ExampleB {
    type G = MyGrid;

    fn update(node_index: u32, grid: &mut Grid<Self::G>) {
        todo!()
    }

    fn recalculate(node_index: u32, grid: &Grid<Self::G>) -> Self {
        todo!()
    }
}

struct Grid<G: GridMarker> {
    a: Vec<<G as GridMarker>::TypeA>,
    b: Vec<<G as GridMarker>::TypeB>,
}

// fn foo<NodeType: Node>(grid: &mut Grid<NodeType::TypeA, NodeType::TypeB>) {
//     for i in 0..10 {
//         NodeType::update(i, grid);
//     }
// }

fn main() {
    let mut grid: Grid<MyGrid> = Grid {
        a: vec![ExampleA {}, ExampleA {}],
        b: vec![ExampleB {}, ExampleB {}],
    };
}

// Connectivity grid
// recalculate node from grid and index
// propagate update to neighbors in custom defined way
// update<Edge>(node_index: [usize;3], grid: &mut Grid) {}

//

trait GridNew {
    fn update_vertex();

    fn update_edge();

    fn update_face();

    fn update_cube();
}
