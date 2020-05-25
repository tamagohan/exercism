pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub value: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(v: &str) -> Self {
                    Self {
                        value: v.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let hashed_attrs = attrs.iter().map(|(x, y)| (x.to_string(), y.to_string()));
                    self.attrs.extend(hashed_attrs);
                    self
                }

                pub fn get_attr(&self, value: &str) -> Option<&str> {
                    self.attrs.get(value).map(|v| v.as_str())
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let hashed_attrs = attrs.iter().map(|(x, y)| (x.to_string(), y.to_string()));
                    self.attrs.extend(hashed_attrs);
                    self
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            let hashed_attrs = attrs.iter().map(|(x, y)| (x.to_string(), y.to_string()));
            self.attrs.extend(hashed_attrs);
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn get_node(&self, value: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.value == value)
        }
    }
}
