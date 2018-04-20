pub struct Choose<T: Clone> {
    vec: Vec<T>,
    k: usize,
    indices: Vec<usize>,
    first: bool,
}

impl<T: Clone> Choose<T> {
    fn new(vec: Vec<T>, k: usize) -> Choose<T> {
        Choose {
            vec: vec,
            k: k,
            indices: (0..k).collect(),
            first: true,
        }
    }

    fn n(&self) -> usize {
        self.vec.len()
    }
}

impl<T: Clone> Iterator for Choose<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.first {
            true => match self.n() < self.k {
                true => None,
                false => {
                    self.first = false;
                    Some(
                        self.indices
                            .iter()
                            .map(|&i| self.vec.get(i).unwrap().clone())
                            .collect(),
                    )
                }
            },
            false => {
                let first_thing = (0..self.k)
                    .rev()
                    .filter(|&_i| *self.indices.get(_i).unwrap() != _i + self.n() - self.k)
                    .take(1)
                    .collect::<Vec<usize>>();
                match first_thing.len() {
                    0 => None,
                    _ => {
                        let i = first_thing[0];
                        self.indices[i] += 1;
                        for j in (i + 1)..self.k {
                            self.indices[j] = self.indices[j - 1] + 1;
                        }
                        Some(
                            self.indices
                                .iter()
                                .map(|&i| self.vec.get(i).unwrap().clone())
                                .collect(),
                        )
                    }
                }
            }
        }
    }
}

pub trait Chooseable<T>
where
    T: Clone,
{
    fn choose(self, k: usize) -> Choose<T>;
}

impl<T: Copy> Chooseable<T> for Vec<T>
where
    T: Clone,
{
    fn choose(self, k: usize) -> Choose<T> {
        Choose::new(self, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_returns_none_when_choosing_one_from_an_empty_list() {
        let vector: Vec<()> = vec![];
        let mut it = vector.choose(1);
        assert!(it.next().is_none());
    }

    #[test]
    fn it_returns_the_empty_vector_when_choosing_zero_from_an_empty_list() {
        let vector: Vec<()> = vec![];
        let mut it = vector.choose(0);
        assert_eq!(it.next().unwrap(), vec![]);
        assert!(it.next().is_none());
    }

    #[test]
    fn it_returns_the_empty_vector_when_choosing_zero_from_a_list_with_elements() {
        let vector = vec![0, 1, 2];
        let mut it = vector.choose(0);
        assert_eq!(it.next().unwrap().len(), 0);
        assert!(it.next().is_none());
    }

    #[test]
    fn it_returns_each_item_when_choosing_one_from_a_list_with_items() {
        let vector = vec![0, 1, 2];
        let mut it = vector.choose(1);

        assert_eq!(it.next().unwrap(), vec![0]);
        assert_eq!(it.next().unwrap(), vec![1]);
        assert_eq!(it.next().unwrap(), vec![2]);
        assert!(it.next().is_none());
    }

    #[test]
    fn it_returns_each_item_combination_when_choosing_two_from_a_list_with_items() {
        let vector = vec![0, 1, 2];
        let mut it = vector.choose(2);

        assert_eq!(it.next().unwrap(), vec![0, 1]);
        assert_eq!(it.next().unwrap(), vec![0, 2]);
        assert_eq!(it.next().unwrap(), vec![1, 2]);
        assert!(it.next().is_none());
    }

    #[test]
    fn it_returns_each_item_combination_when_choosing_two_from_a_list_with_four_items() {
        let vector = vec![0, 1, 2, 3, 4];
        let mut it = vector.choose(2);

        assert_eq!(it.next().unwrap(), vec![0, 1]);
        assert_eq!(it.next().unwrap(), vec![0, 2]);
        assert_eq!(it.next().unwrap(), vec![0, 3]);
        assert_eq!(it.next().unwrap(), vec![0, 4]);
        assert_eq!(it.next().unwrap(), vec![1, 2]);
        assert_eq!(it.next().unwrap(), vec![1, 3]);
        assert_eq!(it.next().unwrap(), vec![1, 4]);
        assert_eq!(it.next().unwrap(), vec![2, 3]);
        assert_eq!(it.next().unwrap(), vec![2, 4]);
        assert_eq!(it.next().unwrap(), vec![3, 4]);
        assert!(it.next().is_none());
    }
}
