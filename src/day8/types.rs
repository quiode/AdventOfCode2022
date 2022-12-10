use std::{ fmt::Debug, borrow::BorrowMut };

#[derive(Debug)]
pub struct Forest {
    trees: Vec<Tree>,
    /// width, height
    dimensions: (usize, usize),
}

impl Forest {
    pub fn new() -> Self {
        Self { trees: vec![], dimensions: (0, 0) }
    }

    /// parses the string
    /// ##### works
    pub fn parse_string(&mut self, string: &str) {
        let mut row_count = 0;
        let mut index = 0;
        for line in string.lines() {
            for str_height in line.chars() {
                let height = str_height.to_digit(10).unwrap();
                self.trees.push(Tree { index: index, visible: None, height: height });
                index += 1;
            }
            row_count += 1;
        }
        self.dimensions = (row_count, index / row_count);
    }

    pub fn navigate<'a>(
        direction: Direction,
        tree: &'a Tree,
        trees: &'a Vec<Tree>,
        dimensions: &'a (usize, usize)
    ) -> &'a Tree {
        match direction {
            Direction::TOP => {
                let top_index = tree.index - dimensions.0;
                &trees[top_index]
            }
            Direction::LEFT => {
                if tree.index == 0 {
                    return trees.last().unwrap();
                } else {
                    return &trees[tree.index - 1];
                }
            }
            Direction::BOTTOM => {
                let top_index = tree.index + dimensions.0;
                if top_index >= trees.len() {
                    &trees[trees.len() - top_index]
                } else {
                    &trees[top_index]
                }
            }
            Direction::RIGHT => {
                if tree.index >= trees.len() - 2 {
                    return &trees[0];
                } else {
                    return &trees[tree.index + 1];
                }
            }
        }
    }

    // sets trees visible status
    pub fn set_visible(&mut self) {
        // every tree on the edge is visible
        // works
        for top_row in 0..self.dimensions.0 {
            self.trees[top_row].visible = Some(true);
        }
        for bottom_row in self.trees.len() - self.dimensions.0..self.trees.len() {
            self.trees[bottom_row].visible = Some(true);
        }
        for left_row in (0..self.trees.len()).step_by(self.dimensions.0) {
            self.trees[left_row].visible = Some(true);
        }
        for right_row in (self.dimensions.0 - 1..self.trees.len()).step_by(self.dimensions.0) {
            self.trees[right_row].visible = Some(true);
        }

        // calculate for each tree
        let trees = self.trees.clone();
        for tree in self.trees.iter_mut() {
            if tree.visible.is_none() {
                let surroundings = [
                    Forest::navigate(Direction::TOP, &tree, &trees, &self.dimensions),
                    Forest::navigate(Direction::RIGHT, &tree, &trees, &self.dimensions),
                    Forest::navigate(Direction::BOTTOM, &tree, &trees, &self.dimensions),
                    Forest::navigate(Direction::LEFT, &tree, &trees, &self.dimensions),
                ];

                let is_visible = surroundings
                    .iter()
                    .any(
                        |surrounding|
                            surrounding.height < tree.height &&
                            surrounding.visible.is_some_and(|v| v)
                    );

                tree.visible = Some(is_visible);
            }
        }
    }

    // counts how many trees are visible
    pub fn calc_visible(&self) -> u32 {
        let mut visible = 0;

        for tree in &self.trees {
            if tree.visible.is_some_and(|v| v) {
                visible += 1;
            }
        }

        visible
    }

    /// if `visible_or_height` is true, display visible (`V` = visible, `N` = invisible, `X` = not set) else display height
    /// ##### works
    pub fn get_forest(&self, visible_or_height: bool) -> String {
        let mut output = String::with_capacity(self.trees.len() + self.dimensions.1);

        for row in 0..self.dimensions.1 {
            for tree in self.dimensions.0 * row..self.dimensions.0 * (row + 1) {
                let text = if visible_or_height {
                    if let Some(is_visible) = self.trees[tree].visible {
                        if is_visible { "V".to_string() } else { "N".to_string() }
                    } else {
                        "X".to_string()
                    }
                } else {
                    let height = self.trees[tree].height;
                    height.to_string()
                };

                output.push_str(&text);
                output.push(' ');
            }
            output.push('\n');
        }

        output
    }
}

#[derive(Debug, Clone)]
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