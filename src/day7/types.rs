use std::rc::Rc;

trait DiskObject {
    fn get_name(&self) -> &str;
    fn get_parent(&self) -> Option<Rc<dyn DiskObject>>;
    fn calc_size(&self) -> u64;
}

pub mod directory {
    use std::rc::Rc;

    use super::DiskObject;
    use super::file::File;

    pub struct Directory {
        parent: Option<Rc<Directory>>,
        children: Vec<Rc<dyn DiskObject>>,
        name: String,
    }

    impl Directory {
        pub fn new(parent: Option<Rc<Directory>>, name: &str) -> Self {
            let parent = match parent {
                Some(parent) => Some(parent.clone()),
                None => None,
            };

            Self {
                parent,
                name: name.to_string(),
                children: vec![],
            }
        }

        pub fn add_file(&mut self, file: Rc<File>) -> &mut Self {
            self.children.push(file.clone());
            self
        }

        pub fn add_directory(self: Rc<Directory>, name: &str) {
            todo!()
        }

        pub fn add_parent(&mut self, directory: Rc<Directory>) {
            self.parent.replace(directory);
        }
    }

    impl Default for Directory {
        fn default() -> Self {
            Self {
                children: vec![],
                parent: None,
                name: "".to_string(),
            }
        }
    }

    impl DiskObject for Directory {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_parent(&self) -> Option<Rc<dyn DiskObject>> {
            self.parent.clone();
            todo!()
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

pub mod file {
    use std::rc::Rc;

    use super::directory::Directory;
    use super::DiskObject;

    pub struct File {
        parent: Rc<Directory>,
        size: u64,
        name: String,
    }

    impl DiskObject for File {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_parent(&self) -> Option<Rc<dyn DiskObject + 'static>> {
            Some(self.parent.clone())
        }

        fn calc_size(&self) -> u64 {
            self.size
        }
    }
}