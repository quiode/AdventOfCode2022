use std::{ str::FromStr, convert::Infallible, fmt::Debug };

use serde::{ Serialize };

// thanks to https://stackoverflow.com/a/36168919/13556449
#[derive(Debug)]
pub struct FileZipper {
    node: File,
    parent: Option<Box<FileZipper>>,
    index_in_parent: usize,
}

impl FileZipper {
    fn child(mut self, index: usize) -> FileZipper {
        // Remove the specified child from the node's children.
        // A NodeZipper shouldn't let its users inspect its parent,
        // since we mutate the parents
        // to move the focused nodes out of their list of children.
        // We use swap_remove() for efficiency.
        let child = self.node.children.swap_remove(index);

        // Return a new NodeZipper focused on the specified child.
        FileZipper {
            node: child,
            parent: Some(Box::new(self)),
            index_in_parent: index,
        }
    }

    fn parent(self) -> FileZipper {
        // Destructure this NodeZipper
        let FileZipper { node, parent, index_in_parent } = self;

        // Destructure the parent NodeZipper
        let FileZipper {
            node: mut parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        } = *parent.unwrap();

        // Insert the node of this NodeZipper back in its parent.
        // Since we used swap_remove() to remove the child,
        // we need to do the opposite of that.
        parent_node.children.push(node);
        let len = parent_node.children.len();
        parent_node.children.swap(index_in_parent, len - 1);

        // Return a new NodeZipper focused on the parent.
        FileZipper {
            node: parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        }
    }

    pub fn finish(mut self) -> File {
        while let Some(_) = self.parent {
            self = self.parent();
        }

        self.node
    }

    pub fn borrow_file(&self) -> &File {
        &self.node
    }

    pub fn borrow_file_mut(&mut self) -> &mut File {
        &mut self.node
    }

    pub fn cd(self, nav_option: NavOption) -> FileZipper {
        match nav_option {
            NavOption::ROOT => self.finish().zipper(),
            NavOption::PARENT => self.parent(),
            NavOption::CHILD(name) => {
                let index = self.node.children
                    .binary_search_by(|val| val.get_name().cmp(&name))
                    .expect("Path nod found!");
                self.child(index)
            }
        }
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct File {
    file_type: FileType,
    children: Vec<File>,
    size: u64,
    name: String,
}

impl File {
    pub fn new(file_type: FileType, size: u64, name: &str) -> Self {
        Self { file_type, children: vec![], size, name: name.to_string() }
    }

    pub fn zipper(self) -> FileZipper {
        FileZipper { node: self, parent: None, index_in_parent: 0 }
    }

    pub fn get_type(&self) -> FileType {
        self.file_type
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_child(&mut self, child: File) {
        let already_exists = self.children.binary_search_by(|c| c.name.cmp(&child.name)).is_ok();

        if !already_exists {
            self.children.push(child);
        } else {
            println!("Found duplicate: {}", child.name);
        }
    }

    pub fn calculate_size(&self) -> u64 {
        let mut size = 0;

        match self.file_type {
            FileType::File => {
                size = self.size;
            }
            FileType::Directory => {
                for child in &self.children {
                    size += child.calculate_size();
                }
            }
        }

        size
    }

    /// finds all dirs that are smaller than `cap` and returns the total
    pub fn smallest_dirs_total(&self, cap: u64) -> u64 {
        let mut total = 0;

        if self.file_type == FileType::Directory {
            let size = self.calculate_size();
            if size <= cap {
                total += size;
            }
            for child in &self.children {
                total += child.smallest_dirs_total(cap);
            }
        }

        total
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum FileType {
    File,
    #[default]
    Directory,
}

#[derive(Debug)]
pub enum NavOption {
    ROOT,
    PARENT,
    CHILD(String),
}

impl FromStr for NavOption {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/" => Ok(Self::ROOT),
            ".." => Ok(Self::PARENT),
            s => Ok(Self::CHILD(s.to_string())),
        }
    }
}