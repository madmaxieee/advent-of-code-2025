pub struct CombinationIter<'a, T> {
    items: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T> CombinationIter<'a, T> {
    pub fn new(items: &'a [T], k: usize) -> Self {
        if k > items.len() || k == 0 {
            return Self {
                items,
                indices: vec![],
                done: true,
            };
        }
        Self {
            items,
            indices: (0..k).collect(), // Initial indices: 0, 1, 2...
            done: false,
        }
    }
}

impl<'a, T> Iterator for CombinationIter<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // 1. Create the result from current indices
        let result: Vec<T> = self
            .indices
            .iter()
            .map(|&i| self.items[i].clone())
            .collect();

        // 2. Update indices for the next iteration (Lexicographic order)
        let k = self.indices.len();
        let n = self.items.len();
        let mut i = k;

        // Find the rightmost index that can be incremented
        while i > 0 {
            i -= 1;
            if self.indices[i] < n - (k - i) {
                self.indices[i] += 1;
                // Reset subsequent indices
                for j in (i + 1)..k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(result);
            }
        }

        self.done = true;
        Some(result)
    }
}
