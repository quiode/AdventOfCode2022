pub struct Forest {
    trees: Vec<Tree>,
    /// width, height
    dimensions: (u32, u32),
}

impl Forest {
    pub fn new() -> Self {
        Self { trees: vec![], dimensions: (0, 0) }
    }

    pub fn parse_string(&mut self, string: &str) {
        let mut row_count = 1;
        let mut index = 0;
        for line in string.lines() {
            row_count += 1;
            for str_height in line.chars() {
                index += 1;
                let height = str_height.to_digit(10).unwrap();
                self.trees.push(Tree { index: index, visible: None, height: height });
            }
        }
        self.dimensions = (row_count, ((index as u32) + 1) / row_count);
    }

    pub fn navigate(&self, direction: Direction, tree: &Tree) -> &Tree {
        match direction {
            Direction::TOP => todo!(),
            Direction::LEFT => {
                if tree.index == 0 {
                    return self.trees.last().unwrap();
                } else {
                    return &self.trees[tree.index - 1];
                }
            }
            Direction::BOTTOM => todo!(),
            Direction::RIGHT => {
                if tree.index >= self.trees.len() - 2 {
                    return &self.trees[0];
                } else {
                    return &self.trees[tree.index + 1];
                }
            }
        }
    }
}

pub struct Tree {
    index: usize,
    visible: Option<bool>,
    height: u32,
}

pub enum Direction {
    TOP,
    LEFT,
    BOTTOM,
    RIGHT,
}