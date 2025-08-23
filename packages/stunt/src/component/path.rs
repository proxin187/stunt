//! A [`Path`] describes a path from root to a node.


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct PathNode {
    index: usize,
    name: String,
}

impl PathNode {
    pub(crate) fn new(index: usize, name: String) -> PathNode {
        PathNode {
            index,
            name,
        }
    }
}

/// Describes a path from root to a node. This is used to build an XPath query during
/// reconciliation.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct Path {
    nodes: Vec<PathNode>,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let xpath = self.nodes.iter()
            .map(|node| format!("/*[{}]", node.index + 1))
            .collect::<String>();

        f.write_str(&xpath)
    }
}

impl Path {
    pub(crate) fn new() -> Path {
        Path {
            nodes: Vec::new(),
        }
    }

    pub(crate) fn concat(mut self, node: PathNode) -> Path {
        self.nodes.push(node);

        Path {
            nodes: self.nodes,
        }
    }
}


