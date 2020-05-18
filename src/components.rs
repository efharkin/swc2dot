use std::rc::Rc;

pub struct Vertex {
    data: SWCCompartment,
    parent: Rc<Vertex>,
    children: Vec<Rc<Vertex>>
}

