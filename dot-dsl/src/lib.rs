pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            #[derive(Debug, PartialEq, Eq)]
            pub struct Node<'a> {
                pub name: &'a str,
                pub attrs: Vec<(&'a str, &'a str)>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Node {
                        name,
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for (attr_name, attr_val) in attrs {
                        self.attrs.push((attr_name, attr_val));
                    }
                    self
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    for (name, val) in self.attrs.iter() {
                        if name == &attr_name {
                            return Some(val);
                        }
                    }
                    None
                }
            }
        }

        pub mod edge {
            #[derive(Debug, PartialEq, Eq)]
            pub struct Edge<'a> {
                pub name: (&'a str, &'a str),
                pub attrs: Vec<(&'a str, &'a str)>,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        name: (from, to),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for (attr_name, attr_val) in attrs {
                        self.attrs.push((attr_name, attr_val));
                    }
                    self
                }

                pub fn attr(&self, attr_name: &'a str) -> Option<&str> {
                    for (name, val) in self.attrs.iter() {
                        if &attr_name == name {
                            return Some(val);
                        }
                    }
                    None
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Self {
            for node in nodes {
                let new = Node::new(&node.name).with_attrs(&node.attrs);
                self.nodes.push(new);
            }
            self
        }

        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Self {
            for edge in edges {
                let new = Edge::new(&edge.name.0, &edge.name.1).with_attrs(&edge.attrs);
                self.edges.push(new);
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            for (attr_name, attr_val) in attrs {
                self.attrs
                    .insert(attr_name.to_string(), attr_val.to_string());
            }
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            for node in self.nodes.iter() {
                if node.name == node_name {
                    return Some(node);
                }
            }
            None
        }
    }
}
