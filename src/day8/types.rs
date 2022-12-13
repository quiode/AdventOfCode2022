use std::fmt::Debug;
use strum::IntoEnumIterator;
use crate::shared::Direction;

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
                self.trees.push(Tree {
                    index: index,
                    visible: None,
                    height: height,
                    scenic_score: None,
                });
                index += 1;
            }
            row_count += 1;
        }
        self.dimensions = (row_count, index / row_count);
    }

    fn navigate<'a>(
        direction: Direction,
        tree: &'a Tree,
        trees: &'a Vec<Tree>,
        dimensions: (usize, usize)
    ) -> &'a Tree {
        match direction {
            Direction::UP => {
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
            Direction::DOWN => {
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

    /// gets all trees in the same row/column in the specified direction
    fn get_every<'a>(
        direction: Direction,
        tree: &'a Tree,
        trees: &'a Vec<Tree>,
        dimensions: (usize, usize)
    ) -> Vec<&'a Tree> {
        let mut output_trees: Vec<&Tree> = vec![];
        let column = Self::calc_column(tree, dimensions);
        let row = Self::calc_row(tree, dimensions);

        match direction {
            Direction::UP => {
                for i in (0..row).rev() {
                    output_trees.push(&trees[Self::get_index(column, i, dimensions)]);
                }
            }
            Direction::LEFT => {
                for i in (0..column).rev() {
                    output_trees.push(&trees[Self::get_index(i, row, dimensions)]);
                }
            }
            Direction::DOWN => {
                for i in row + 1..dimensions.1 {
                    output_trees.push(&trees[Self::get_index(column, i, dimensions)]);
                }
            }
            Direction::RIGHT => {
                for i in column + 1..dimensions.0 {
                    output_trees.push(&trees[Self::get_index(i, row, dimensions)]);
                }
            }
        }

        output_trees
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

        // calculate for each tree which has not been set
        let trees = self.trees.clone();
        for tree in self.trees.iter_mut() {
            if tree.visible.is_none() {
                let is_visible =
                    Self::get_every(Direction::LEFT, tree, &trees, self.dimensions)
                        .iter()
                        .all(|t| t.height < tree.height) ||
                    Self::get_every(Direction::UP, tree, &trees, self.dimensions)
                        .iter()
                        .all(|t| t.height < tree.height) ||
                    Self::get_every(Direction::RIGHT, tree, &trees, self.dimensions)
                        .iter()
                        .all(|t| t.height < tree.height) ||
                    Self::get_every(Direction::DOWN, tree, &trees, self.dimensions)
                        .iter()
                        .all(|t| t.height < tree.height);

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

    /// calculates in which row the tree is
    fn calc_row<'a>(tree: &'a Tree, dimensions: (usize, usize)) -> usize {
        tree.index / dimensions.0
    }

    /// calculates in which column the tree is
    fn calc_column<'a>(tree: &'a Tree, dimensions: (usize, usize)) -> usize {
        tree.index % dimensions.0
    }

    /// gets the index of the tree in trees with `dimensions` in `row` and `column`
    fn get_index(column: usize, row: usize, dimensions: (usize, usize)) -> usize {
        row * dimensions.0 + column
    }

    /// calculates the scenic score of the `tree`
    fn calc_scenic_score<'a>(
        tree: &'a Tree,
        trees: &'a Vec<Tree>,
        dimensions: (usize, usize)
    ) -> usize {
        // scenic score = amount of trees visible in each direction
        let mut score = 1;

        let mut direction_trees = Vec::with_capacity(4);

        for direction in Direction::iter() {
            // push all trees for each direction
            direction_trees.push(Self::get_every(direction, tree, trees, dimensions));
        }

        for direction_tree in direction_trees {
            let mut direction_score = 0;
            for s_tree in direction_tree {
                direction_score += 1;
                if s_tree.height >= tree.height {
                    break;
                }
            }
            score *= direction_score;
        }

        score
    }

    /// calculates the highest scenic score possible
    pub fn find_highest_scenic_score(&mut self) -> usize {
        if self.trees.len() == 0 {
            return 0;
        }

        let trees_clone = self.trees.clone();

        for tree in &mut self.trees {
            if tree.scenic_score.is_none() {
                tree.scenic_score = Some(
                    Self::calc_scenic_score(&tree, &trees_clone, self.dimensions)
                );
            }
        }

        self.trees
            .iter()
            .max_by(|x, y| x.scenic_score.unwrap_or(0).cmp(&y.scenic_score.unwrap_or(0)))
            .unwrap()
            .scenic_score.unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    index: usize,
    height: u32,
    visible: Option<bool>,
    scenic_score: Option<usize>,
}