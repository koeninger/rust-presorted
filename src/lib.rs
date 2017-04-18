#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]


/// something with a key
pub trait Keyed {
    type Key: Ord;

    fn key(&self) -> Self::Key;
}

/// associative binary operator
pub trait Semigroup {
    fn combine(&self, other: &Self) -> Self;
}

/// convenience functions for working with a presorted vec.
pub trait Presorted<T: Clone + Keyed + Semigroup> {
    /// insert item, maintaining order.  merge is probably faster for multiple items.
    fn put(&mut self, x: T);
    /// merge another Presorted vec, maintaining order
    fn merge(&mut self, other: Self);
    /// get item having matching key, assumes order has been maintained
    fn get_by_key(&self, key: &T::Key) -> Option<&T>;
}

impl<T: Clone + Keyed + Semigroup> Presorted<T> for Vec<T> {
    fn put(&mut self, x: T) {
        match self.binary_search_by_key(&x.key(), |y| y.key()) {
            Ok(i) => self[i] = self[i].combine(&x),
            Err(i) => self.insert(i, x),
        }
    }
    fn merge(&mut self, other: Self) {
        let mut i = 0;
        let mut j = 0;

        while i < self.len() && j < other.len() {
            if self[i].key() == other[j].key() {
                self[i] = self[i].combine(&other[j]);
                i += 1;
                j += 1;
            } else if self[i].key() < other[j].key() {
                i += 1;
            } else {
                self.insert(i, other[j].clone());
                i += 1;
                j += 1;
            }
        }

        while j < other.len() {
            self.push(other[j].clone());
            j += 1;
        }
    }
    fn get_by_key(&self, key: &T::Key) -> Option<&T> {
        match self.binary_search_by_key(key, |y| y.key()) {
            Ok(i) => self.get(i),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {

    use Presorted;
    use Keyed;
    use Semigroup;
    use quickcheck::{Arbitrary, Gen};
    use std::cmp::Ordering;

    #[derive(Debug, Copy, Clone)]
    struct Thing(i32, f32);

    impl PartialEq for Thing {
        fn eq(&self, other: &Thing) -> bool {
            self.key() == other.key()
        }
    }

    impl Eq for Thing {}

    impl Ord for Thing {
        fn cmp(&self, other: &Thing) -> Ordering {
            self.key().cmp(&other.key())
        }
    }

    impl PartialOrd for Thing {
        fn partial_cmp(&self, other: &Thing) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Keyed for Thing {
        type Key = i32;
        fn key(&self) -> Self::Key {
            self.0
        }
    }

    impl Semigroup for Thing {
        fn combine(&self, other: &Thing) -> Thing {
            Thing(self.0, self.1 + other.1)
        }
    }

    impl Arbitrary for Thing {
        fn arbitrary<G: Gen>(g: &mut G) -> Thing {
            Thing(g.gen::<i32>(), g.gen::<f32>())
        }
    }

    #[test]
    fn it_works() {
        let mut v = vec![Thing(1, 0.1), Thing(3, 0.3), Thing(5, 0.5)];
        v.put(Thing(4, 0.4));
        println!("{:?}", v);
        assert!(v == vec![Thing(1, 0.1), Thing(3, 0.3), Thing(4, 0.4), Thing(5, 0.5)]);
        v.put(Thing(4, 0.6));
        assert!(v == vec![Thing(1, 0.1), Thing(3, 0.3), Thing(4, 1.0), Thing(5, 0.5)]);

        assert!(v.get_by_key(&3) == Some(&Thing(3, 0.3)));

        let w = vec![Thing(1, 0.9), Thing(2, 0.2), Thing(6, 0.6)];
        v.merge(w);
        assert!(v ==
                vec![Thing(1, 1.0),
                     Thing(2, 0.2),
                     Thing(3, 0.3),
                     Thing(4, 1.0),
                     Thing(5, 0.5),
                     Thing(6, 0.6)]);
    }

    fn is_sorted<T: Ord>(v: &[T]) -> bool {
        let mut i = 1;
        while i < v.len() {
            if v[i] < v[i - 1] {
                return false;
            }
            i += 1;
        }
        return true;
    }

    #[quickcheck]
    fn merge_is_sorted(mut v: Vec<Thing>, mut w: Vec<Thing>) -> bool {
        v.sort();
        w.sort();
        v.merge(w);
        is_sorted(&v)
    }
}
