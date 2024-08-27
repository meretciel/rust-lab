use std::cell::{ RefCell};
use std::collections::VecDeque;
use std::fs::create_dir;
use std::rc::{Rc, Weak};


type NodePtr = Rc<RefCell<Node>>;


#[derive(Debug)]
struct Node {
    value: f64,
    partial_derivative: Option<f64>,
    parent_nodes: Vec<Weak<RefCell<Node>>>,
    child_nodes: Vec<Rc<RefCell<Node>>>,
    gradient_wrt_parents: Vec<f64>,
    is_ready_for_backpropagation: bool,
}


impl Node {
    fn create_var(value: f64) -> NodePtr {
        Rc::new(RefCell::new(Node{
            value,
            partial_derivative: None,
            parent_nodes: Vec::new(),
            child_nodes: Vec::new(),
            gradient_wrt_parents: Vec::new(),
            is_ready_for_backpropagation: false,
        }))
    }

    fn check_for_readiness(&mut self) {
        for child in self.child_nodes.iter() {
            if !child.borrow().is_ready_for_backpropagation {
                self.is_ready_for_backpropagation = false;
                break;
            }
        }
        self.is_ready_for_backpropagation = true;
    }
}

fn add(x: NodePtr, y: NodePtr) -> NodePtr {
    let node = Rc::new(RefCell::new(Node{
        value: x.borrow().value + y.borrow().value,
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x), Rc::downgrade(&y)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![1., 1.],
        is_ready_for_backpropagation: false,

    }));

    x.borrow_mut().child_nodes.push(node.clone());
    y.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn sub(x: NodePtr, y: NodePtr) -> NodePtr {
    let node = Rc::new(RefCell::new(Node{
        value: x.borrow().value - y.borrow().value,
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x), Rc::downgrade(&y)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![1., -1.],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());
    y.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn mul(x: NodePtr, y: NodePtr) -> NodePtr {
    let xv = x.borrow().value;
    let yv = y.borrow().value;

    let node = Rc::new(RefCell::new(Node{
        value: xv * yv,
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x), Rc::downgrade(&y)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![yv, xv],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());
    y.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn div(x: NodePtr, y: NodePtr) -> NodePtr {
    let xv = x.borrow().value;
    let yv = y.borrow().value;

    let node = Rc::new(RefCell::new(Node{
        value: xv / yv,
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x), Rc::downgrade(&y)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![1. / yv, -xv / (yv * yv)],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());
    y.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn ln(x: NodePtr) -> NodePtr {

    let node = Rc::new(RefCell::new(Node{
        value: f64::ln(x.borrow().value),
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![1. / x.borrow().value],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn exp(x: NodePtr) -> NodePtr {
    let v = f64::exp(x.borrow().value);
    let node = Rc::new(RefCell::new(Node{
        value: v,
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![v],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());

    return node;
}


fn sin(x: NodePtr) -> NodePtr {
    let v = x.borrow().value;
    let node = Rc::new(RefCell::new(Node{
        value: f64::sin(v),
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![f64::cos(v)],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn cos(x: NodePtr) -> NodePtr {
    let v = x.borrow().value;
    let node = Rc::new(RefCell::new(Node{
        value: f64::cos(v),
        partial_derivative: None,
        parent_nodes: vec![Rc::downgrade(&x)],
        child_nodes: Vec::new(),
        gradient_wrt_parents: vec![-f64::sin(v)],
        is_ready_for_backpropagation: false,
    }));

    x.borrow_mut().child_nodes.push(node.clone());

    return node;
}

fn backwards(root: NodePtr) {
    let mut queue: VecDeque<NodePtr> = VecDeque::new();

    queue.push_back(root);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        println!("Process {node:?}");

        let n = node.borrow().parent_nodes.len();
        let partial_derivative = node.borrow().partial_derivative.unwrap();
        for i in 0..n {
            let parent_node = node.borrow().parent_nodes[i].upgrade().unwrap();
            let gradient = node.borrow().gradient_wrt_parents[i];
            let parent_pd = parent_node.borrow().partial_derivative;

            match parent_pd {
                None =>
                    parent_node.borrow_mut().partial_derivative = Some(gradient * partial_derivative),
                Some(x) =>
                    parent_node.borrow_mut().partial_derivative = Some(x + gradient * partial_derivative),
            }

            parent_node.borrow_mut().check_for_readiness();
            println!("\tParent node after update: {parent_node:?}");
            if parent_node.borrow().is_ready_for_backpropagation {
                println!("\tAdd parent node to the queue.");
                queue.push_back(parent_node.clone())
            }
        }
    }
}


fn main() {
    // let x1 = Node::create_var(2.);
    // let x2 = Node::create_var(5.);
    // let v1 = ln(x1.clone());
    // let v2 = mul(x1.clone(), x2.clone());
    // let v3 = sin(x2.clone());
    // let v4 = add(v1.clone(), v2.clone());
    // let v5 = sub(v4, v3);
    // let y = v5.clone();
    //
    // y.borrow_mut().partial_derivative = Some(1.);
    // y.borrow_mut().is_ready_for_backpropagation = true;
    // backwards(y.clone());
    //
    // println!("{}", "-".repeat(80));
    // println!("output value: {}", y.borrow().value);
    // println!("partial derivative of x1: {:?}", x1.borrow().partial_derivative);
    // println!("partial derivative of x2: {:?}", x2.borrow().partial_derivative);
    // println!("{}", "-".repeat(80));
    //
    // println!("strong count of output variable: {}", Rc::strong_count(&v1));
    // std::mem::drop(x1);
    // std::mem::drop(x2);
    // println!("strong count of output variable after dropping variables: {}", Rc::strong_count(&v1));

    let x1 = Node::create_var(3.);
    let s = add(x1.clone(), x1.clone());
    let s = add(s.clone(), x1.clone());
    let s = add(s.clone(), x1.clone());
    s.borrow_mut().partial_derivative = Some(1.);
    s.borrow_mut().is_ready_for_backpropagation = true;
    backwards(s.clone());
    println!("output value: {}", s.borrow().value);
    println!("derivatives: {:?}", x1.borrow().partial_derivative);
}