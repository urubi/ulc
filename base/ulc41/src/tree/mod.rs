use std::rc::Rc;

pub trait NodeExt<'a, T> {
    fn len(&self) -> usize;
    fn tag(&self) -> &str;
    fn slice(&self) -> &'a [T];
    fn collect(&self, tags: &[&str]) -> Vec<(Rc<String>, &'a [T])>;
}

//Rc<String>
#[derive(PartialEq)]
pub struct Leaf<'a, T: 'a> {
    slice: &'a [T],
    tag: Rc<String>
}
impl<'a, T: 'a> NodeExt<'a, T> for Leaf<'a, T>{
    fn len(&self) -> usize { self.slice.len() }
    fn tag(&self) -> &str { &self.tag }
    fn slice(&self) -> &'a [T] {
        &self.slice
    }
    fn collect(&self, tags: &[&str]) -> Vec<(Rc<String>, &'a [T])> {
        let mut out = vec![];
        // there is a bug in the docs, can't find equivilant to `if x in [x]`
        // TODO fix this
        for i in tags.iter() {
            if *i == *self.tag {
                out.push((self.tag.clone(), self.slice()));
            }
        }
        out
    }
}
impl<'a, T: 'a> Leaf<'a, T>{
    pub fn new(tag: Rc<String>, slice: &'a [T]) -> Node<'a, T> {
        Node::Leaf(Leaf {slice:slice, tag:tag} )
    }
}




// Note you can cache length upon adding them to the branch to avoid recursive 
// length calculation.
#[derive(PartialEq)]
pub struct Branch<'a, T: 'a> {
    root: &'a [T],
    nodes: Vec<Node<'a, T>>,
    tag: Rc<String>
}
impl<'a, T: 'a> NodeExt<'a, T> for Branch<'a, T> {
    fn len(&self) -> usize {
        let mut length: usize = 0;
        for n in self.nodes.iter() {
            length += n.len();
        }
        length
    }
    fn collect(&self, tags: &[&str]) -> Vec<(Rc<String>, &'a [T])> {
        let mut out = vec![];
        
        // if branch itself is wanted TODO same as for leaf
        for i in tags.iter() {
            if *i == *self.tag {
                out.push((self.tag.clone(), self.slice()));
            }
        }
        
        for n in self.nodes.iter() {
            out.push_all(&n.collect(tags));
        }
        out
    }
    fn tag(&self) -> &str { &self.tag }
    fn slice(&self) -> &'a [T] {
        &self.root[..self.len()]
    }
    
    
}
impl<'a, T: 'a> Branch<'a, T> {
    pub fn new(tag: Rc<String>, root: &'a [T]) -> Node<'a, T> {
        Node::Branch(Branch {nodes:vec![], tag:tag, root:root} )
    }
    pub fn get_nodes(&self) -> &Vec<Node<'a,T>> {
        &self.nodes
    }
    pub fn attach(&mut self, n: Node<'a, T>) {
        if self.len() + n.len() > self.root.len() { // WARNING: GROWTH HAZARD fix with struct member once stable (fix len() too)
            panic!("branch member node refrences outside the passed root");
        }
        self.nodes.push(n)
    }
}

#[derive(PartialEq)]
pub enum Node<'a, T: 'a> {
    Branch(Branch<'a, T>),
    Leaf(Leaf<'a, T>)
}
impl<'a, T: 'a> NodeExt<'a, T> for Node<'a, T> {
    fn len(&self) -> usize {
        match self {
            &Node::Branch(ref b) => b.len(),
            &Node::Leaf(ref l) => l.len()
        }
    }
    fn tag(&self) -> &str {
        match self {
            &Node::Branch(ref b) => b.tag(),
            &Node::Leaf(ref l) => l.tag()
        }    
    }
    fn slice(&self) -> &'a [T] {
        match self {
            &Node::Branch(ref b) => b.slice(),
            &Node::Leaf(ref l) => l.slice()
        }    
    }
    fn collect(&self, tags: &[&str]) -> Vec<(Rc<String>, &'a [T])> {
        match self {
            &Node::Branch(ref b) => b.collect(tags),
            &Node::Leaf(ref l) => l.collect(tags)
        }
    }
}
impl<'a, T: 'a> Node<'a, T> {
    pub fn attach(&mut self, n: Node<'a, T>) {
        match self {
            &mut Node::Branch(ref mut b) => b.attach(n),
            _ => panic!("Logical error in your code: attaching is only supported with Branch")
        }
    }
    
    pub fn get_node(&self, tag: &str) -> Option<&Node<'a, T>> {
        if self.tag() == tag {
            return Some(self);
        } 
        match self {
            &Node::Branch(ref b) => {
                for i in b.get_nodes().iter() {
                    match i.get_node(tag) {
                        Some(n) => return Some(n),
                        _ => ()
                    };
                }
            },
            _ => ()
        };
        None
        
    }
}


macro_rules! as_branch{
    ($node: expr) => (
        if let &Node::Branch(ref b) = $node {
            b
        }
        else {panic!("not a branch");}
    )
}

macro_rules! as_leaf{
    ($node: expr) => (
        if let &Node::Leaf(ref l) = $node {
            l
        }
        else {panic!("not a branch");}
    )
}

#[test]
fn test_tree() {
    macro_rules! r {
        ($e: expr) => (
            Rc::new($e.to_string())
        )
    }
    
    let array = b"Hi:::0123456789";
    let mut tree = Branch::new(r!("tree"), array);
    
    tree.attach(Leaf::new(r!("greeting"), &array[..2]));
    tree.attach(Leaf::new(r!("collon"), &array[2..5]));
    
    assert!(tree.len() == 5);
    assert!(as_branch!(&tree).get_nodes().len() == 2);
    
    let mut sub = Branch::new(r!("two-part"), &array[5..]);
    sub.attach(Leaf::new(r!("one to five"), &array[5..10]));
    sub.attach(Leaf::new(r!("rest"), &array[10..]));
    tree.attach(sub);
        
    
        
    assert!(tree.tag() == "tree");
    assert!(tree.len() == 15);
    assert!(tree.slice() == array);
    
    assert!(as_branch!(&tree).get_nodes().len() == 3);
    
    
    assert!(as_leaf!(&as_branch!(&tree).get_nodes()[0]).slice() == b"Hi");
    assert!(as_leaf!(&as_branch!(&tree).get_nodes()[0]).tag() == "greeting");
    assert!(as_leaf!(&as_branch!(&tree).get_nodes()[1]).slice() == b":::");
    assert!(as_leaf!(&as_branch!(&tree).get_nodes()[1]).tag() == "collon");
    
    assert!(as_branch!(&as_branch!(&tree).get_nodes()[2]).tag() == "two-part");
    assert!(as_branch!(&as_branch!(&tree).get_nodes()[2]).len() == 10);
    assert!(as_branch!(&as_branch!(&tree).get_nodes()[2]).slice() == b"0123456789");
    assert!(as_branch!(&as_branch!(&tree).get_nodes()[2]).get_nodes().len() == 2);
    assert!(as_leaf!(&as_branch!(&as_branch!(&tree).get_nodes()[2]).get_nodes()[0]).slice() == b"01234");
    assert!(as_leaf!(&as_branch!(&as_branch!(&tree).get_nodes()[2]).get_nodes()[0]).tag() == "one to five");
    assert!(as_leaf!(&as_branch!(&as_branch!(&tree).get_nodes()[2]).get_nodes()[1]).slice() == b"56789");
    assert!(as_leaf!(&as_branch!(&as_branch!(&tree).get_nodes()[2]).get_nodes()[1]).tag() == "rest");
 
    assert!(tree.get_node("one to five").unwrap().slice() == b"01234");
    assert!(tree.get_node("two-part").unwrap().slice() == b"0123456789");
    assert!(tree.get_node("tree").unwrap().slice() == array);
    
    //panic!("{:?}", tree.collect(&["one to five", "two-part"]));
}
