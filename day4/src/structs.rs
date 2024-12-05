#[derive(Debug)]
pub struct LineIterator<'a, T> {
    line: &'a Line<'a, T>,
    index: usize,
}

impl<'a, T> LineIterator<'a, T> {
    fn new(line: &'a Line<'a, T>) -> Self {
        Self { line, index: 0 }
    }
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.line.data.len() {
            let out = Some(self.line.data[self.index]);
            self.index += 1;

            out
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Line<'a, T> {
    data: Vec<&'a T>,
}

impl<'a, T> Line<'a, T> {
    fn new(data: Vec<&'a T>) -> Self {
        Self { data }
    }

    pub fn iter(&'a self) -> LineIterator<'a, T> {
        LineIterator::new(self)
    }

    fn get(&'a self, index: usize) -> &'a T {
        self.data.get(index).unwrap()
    }

    pub fn len(&'a self) -> usize {
        self.data.len()
    }
}

#[derive(Debug)]
pub struct Matrix<T: Default + Clone> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<'a, T: Default + Clone> Matrix<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            data: vec![T::default(); rows * columns],
            rows,
            columns,
        }
    }

    pub fn with_data(mut self, data: Vec<T>) -> Self {
        self.data = data;
        self
    }

    pub fn row_count(&self) -> usize {
        self.rows
    }

    pub fn column_count(&self) -> usize {
        self.rows
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn row(&'a self, row_index: usize) -> Option<Line<'a, T>> {
        if row_index >= self.rows {
            None
        } else {
            let start = row_index * self.columns;
            let a: Vec<&T> = self.data.iter().skip(start).take(self.columns).collect();

            Some(Line::new(a))
        }
    }

    pub fn column(&'a self, column_index: usize) -> Option<Line<'a, T>> {
        if column_index >= self.columns {
            None
        } else {
            let data: Vec<&T> = self
                .data
                .iter()
                .enumerate()
                .filter_map(|e| {
                    if e.0 % self.columns == column_index {
                        Some(e.1)
                    } else {
                        None
                    }
                })
                .collect();

            Some(Line::new(data))
        }
    }

    pub fn left_diagonal(&'a self, diagonal_index: usize) -> Option<Line<'a, T>> {
        let starting_index = if diagonal_index <= self.columns - 1 {
            self.columns - diagonal_index - 1
        } else {
            self.columns * self.columns.abs_diff(diagonal_index + 1)
        };

        self.left_diagonal_at_index(starting_index)
    }

    pub fn left_diagonal_at_index(&'a self, starting_index: usize) -> Option<Line<'a, T>> {
        let next_index = |i: usize| i + self.columns + 1;

        if starting_index > self.data.len() {
            None
        } else {
            let diagonal_length = self.columns - starting_index % self.columns;
            let mut current_index = starting_index;

            let mut data: Vec<&T> = Vec::new();

            for _ in 0..diagonal_length {
                if current_index < self.data.len() {
                    data.push(self.data.get(current_index).unwrap());
                }
                current_index = next_index(current_index)
            }

            Some(Line::new(data))
        }
    }

    pub fn right_diagonal(&'a self, diagonal_index: usize) -> Option<Line<'a, T>> {
        let starting_index = if diagonal_index <= self.columns - 1 {
            diagonal_index
        } else {
            (self.columns - 1) + ((diagonal_index - (self.columns - 1)) * self.columns)
        };

        self.right_diagonal_at_index(starting_index)
    }

    pub fn right_diagonal_at_index(&'a self, starting_index: usize) -> Option<Line<'a, T>> {
        let next_index = |i: usize| i + self.columns - 1;

        if starting_index > self.data.len() {
            None
        } else {
            let diagonal_length =
                (self.columns + 1) - (self.columns - starting_index % self.columns);
            let mut current_index = starting_index;

            let mut data: Vec<&T> = Vec::new();

            for _ in 0..diagonal_length {
                if current_index < self.data.len() {
                    data.push(self.data.get(current_index).unwrap());
                }
                current_index = next_index(current_index)
            }

            Some(Line::new(data))
        }
    }
}
