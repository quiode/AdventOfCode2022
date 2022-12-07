trait DiskObject {
    fn get_name(&self) -> &str;
    fn get_parent(&self) -> Option<&dyn DiskObject>;
    fn calc_size(&self) -> u64;
}

pub mod Directory {
    use std::borrow::Borrow;
    use crate::day7::types::DiskObject;
    use crate::day7::types::File::File;

    pub struct Directory<'a> {
        parent: Option<Box<&'a Directory<'a>>>,
        children: Vec<Box<dyn DiskObject>>,
        name: String
    }

    impl <'a>Directory<'a> {
        pub fn new(parent: Option<&'a Directory<'_>>, name: &str) -> Self {
            let parent = match parent {
                Some(parent) => Some(Box::new(parent)),
                None => None,
            };

            Self {
                parent,
                name: name.to_string(),
                children: vec![]
            }
        }

        pub fn add_file(&mut self, file: File) -> &mut Self {
            self.children.push(Box::new(file));
            self
        }

        pub fn add_directory(&mut self, name: &str) -> &Directory {
            let directory = Directory::new(Some(&self), name);
            self.children.push(Box::new(directory));
            self.children.last().unwrap().
        }
    }

    impl <'a>Default for Directory<'a> {
        fn default() -> Self {
            Self {
                children: vec![],
                parent: None,
                name: "".to_string(),
            }
        }
    }

    impl <'a>DiskObject for Directory<'a> {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_parent(&self) -> Option<&dyn DiskObject> {
            return if let Some(parent) = &self.parent {
                let parent: &Directory = parent;
                Some(parent)
            } else {
                None
            }
        }

        fn calc_size(&self) -> u64 {
            let mut size = 0;
            for child in &self.children {
                size += child.calc_size();
            }

            size
        }
    }
}

pub mod File {
    use crate::day7::types::Directory::Directory;
    use crate::day7::types::DiskObject;

   pub struct File<'a> {
        parent: Directory<'a>,
        size: u64,
        name: String
    }

    impl <'a>DiskObject for File<'a> {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_parent(&self) -> Option<&dyn DiskObject> {
            Some(&self.parent)
        }

        fn calc_size(&self) -> u64 {
            self.size
        }
    }
}
