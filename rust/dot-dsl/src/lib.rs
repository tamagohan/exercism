pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq)]
            pub struct Node<'a> {
                pub value: &'a str,
                attrs: HashMap<String, String>,
            }

            impl<'a> Node<'a> {
                pub fn new(v: &'a str) -> Self {
                    Self {
                        value: v,
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

            #[derive(Debug, PartialEq)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Self {
                        from: from,
                        to: to,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &'a [(&str, &str)]) -> Self {
                    let hashed_attrs = attrs.iter().cloned().collect::<HashMap<&str, &str>>();
                    self.attrs.extend(hashed_attrs);
                    self
                }
            }
        }
    }

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &'a [(&str, &str)]) -> Self {
            let hashed_attrs = attrs.iter().map(|(x, y)| (x.to_string(), y.to_string()));
            self.attrs.extend(hashed_attrs);
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.iter().collect::<Vec<&Node>>().extend(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.iter().collect::<Vec<&Edge>>().extend(edges);
            self
        }

        pub fn get_node(&self, value: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.value == value)
        }
    }
}
