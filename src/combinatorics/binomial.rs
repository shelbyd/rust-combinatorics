pub struct Choose<'t, T: 't> {
    list: &'t [T],
    k: usize,
    indices: Vec<usize>,
    first: bool,
}

impl<'t, T> Choose<'t, T> {
    fn new(list: &'t [T], k: usize) -> Choose<'t, T> {
        Choose {
            list: list,
            k: k,
            indices: (0..k).collect(),
            first: true,
        }
    }

    fn n(&self) -> usize {
        self.list.len()
    }

    fn from_indices(&self) -> Vec<&'t T> {
        self.indices.iter().map(|&i| &self.list[i]).collect()
    }

    fn increment_indices(&mut self) -> Option<()> {
        let to_increment = (0..self.k)
            .rev()
            .filter(|&i| {
                let max_allowed = self.n() - self.k + i;
                self.indices[i] < max_allowed
            })
            .next();
        let to_increment = match to_increment {
            None => return None,
            Some(i) => i,
        };

        self.indices[to_increment] += 1;

        for j in (to_increment + 1)..self.k {
            let previous_index = self.indices[j - 1];
            self.indices[j] = previous_index + 1;
        }

        Some(())
    }
}

impl<'t, T> Iterator for Choose<'t, T> {
    type Item = Vec<&'t T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n() < self.k {
            return None;
        }

        if !self.first {
            match self.increment_indices() {
                None => return None,
                Some(()) => {}
            };
        };

        self.first = false;

        Some(self.from_indices())
    }
}

pub trait Chooseable<'t, T> {
    fn choose(&'t self, k: usize) -> Choose<'t, T>;
}

impl<'t, T> Chooseable<'t, T> for [T] {
    fn choose(&'t self, k: usize) -> Choose<'t, T> {
        Choose::new(self, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_next(it: &mut Choose<usize>, items: Vec<usize>) {
        assert_eq!(it.next().unwrap(), items.iter().collect::<Vec<_>>());
    }

    #[test]
    fn it_returns_none_when_choosing_one_from_an_empty_list() {
        let vector: Vec<()> = vec![];
        let mut it = vector.choose(1);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn it_returns_the_empty_vector_when_choosing_zero_from_an_empty_list() {
        let vector: Vec<usize> = vec![];
        let mut it = vector.choose(0);
        assert_next(&mut it, vec![]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn it_returns_the_empty_vector_when_choosing_zero_from_a_list_with_elements() {
        let vector = vec![0, 1, 2];
        let mut it = vector.choose(0);
        assert_eq!(it.next().unwrap().len(), 0);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn it_returns_each_item_when_choosing_one_from_a_list_with_items() {
        let vector = vec![0, 1, 2];
        let mut it = vector.choose(1);

        assert_next(&mut it, vec![0]);
        assert_next(&mut it, vec![1]);
        assert_next(&mut it, vec![2]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn it_returns_each_item_combination_when_choosing_two_from_a_list_with_items() {
        let vector = vec![0, 1, 2];
        let mut it = vector.choose(2);

        assert_next(&mut it, vec![0, 1]);
        assert_next(&mut it, vec![0, 2]);
        assert_next(&mut it, vec![1, 2]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn it_returns_each_item_combination_when_choosing_two_from_a_list_with_five_items() {
        let vector = vec![0, 1, 2, 3, 4];
        let mut it = vector.choose(2);

        assert_next(&mut it, vec![0, 1]);
        assert_next(&mut it, vec![0, 2]);
        assert_next(&mut it, vec![0, 3]);
        assert_next(&mut it, vec![0, 4]);
        assert_next(&mut it, vec![1, 2]);
        assert_next(&mut it, vec![1, 3]);
        assert_next(&mut it, vec![1, 4]);
        assert_next(&mut it, vec![2, 3]);
        assert_next(&mut it, vec![2, 4]);
        assert_next(&mut it, vec![3, 4]);
        assert_eq!(it.next(), None);
    }
}
