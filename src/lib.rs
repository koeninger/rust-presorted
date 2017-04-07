pub trait Keyed {
    type Key: Ord;

    fn key(&self) -> Self::Key;
}

pub trait Semigroup {
    fn combine(&self, other: &Self) -> Self;
}

pub trait PreSorted<T: Keyed + Semigroup> {
    fn add(&mut self, x: T);
    fn get_by_key(&self, key: &T::Key) -> Option<&T>;
}

impl<T: Keyed + Semigroup> PreSorted<T> for Vec<T> {
    fn add(&mut self, x: T) {
        match self.binary_search_by_key(&x.key(), |y| y.key()) {
            Ok(i) => self[i] = self[i].combine(&x),
            Err(i) => self.insert(i, x)
        }
    }
    fn get_by_key(&self, key: &T::Key) -> Option<&T> {
        match self.binary_search_by_key(key, |y| y.key()) {
            Ok(i) => self.get(i),
            Err(_) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use PreSorted;
    use Keyed;
    use Semigroup;
    
    #[derive(Debug)]
    struct Thing(i32,f32);

    impl PartialEq for Thing {
        fn eq(&self, other: &Thing) -> bool {
            self.key() == other.key()
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
    #[test]
    fn it_works() {
        let mut v = vec![Thing(1, 0.1), Thing(3, 0.3), Thing(5, 0.5)];
        v.add(Thing(4, 0.4));
        println!("{:?}", v);
        assert!(v == vec![Thing(1, 0.1), Thing(3, 0.3), Thing(4, 0.4), Thing(5, 0.5 )]);
        v.add(Thing(4, 0.6));
        assert!(v == vec![Thing(1, 0.1), Thing(3, 0.3), Thing(4, 1.0), Thing(5, 0.5 )]);
        assert!(v.get_by_key(&3) == Some(&Thing(3, 0.3)));
    }
}
