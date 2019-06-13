//! Disjoint-set data structure, known as union-find.

// TODO: Create `VecUnionFindSets`.

// Since Rust version of AtCoder is too outdated,
// `Rc`s cannot be compared by their pointer values.
// So, every node must have the cloned item for equality check.
// After language update of AtCoder, merge branch `disjoint-set-for-latest-rust`,
// and `T` does not require `Clone`.

// TODO: Show solution of ABC120 D and ABC126 E as examples
/// Disjoint-set data structure, known as union-find.
///
/// `HashUnionFindSets` uses `HashMap` internally to manage items.
/// Therefore, item type `T` must implement `Hash`.
///
/// Thanks to union-by-size and path-compression strategy,
/// average cost of each operation is so much low that
/// it can be regarded as constant time, although theoretically it is not constant.
#[snippet = "sets"]
pub struct HashUnionFindSets<T: Eq + std::hash::Hash + std::fmt::Debug> {
    // Maintaining `set_count` can be an unnecessary cost,
    // but that frees users from maintaining it
    // by checking the returned values for all `add` and `unite` operations.
    set_count: usize,
    items: std::collections::HashMap<T, UnionFindNode<T>>
}

#[snippet = "sets"]
#[derive(Clone)]
enum UnionFindNodeInner<T: Eq + std::hash::Hash> {
    Root {
        len: usize,
    },
    Child {
        parent: UnionFindNode<T>
    }
}

#[snippet = "sets"]
#[derive(Clone)]
struct UnionFindNode<T: Eq + std::hash::Hash>(
    std::rc::Rc<std::cell::RefCell<UnionFindNodeInner<T>>>, T
);

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash + Clone> UnionFindNode<T> {
    fn new(item: T) -> UnionFindNode<T> {
        UnionFindNode(std::rc::Rc::new(std::cell::RefCell::new(
            UnionFindNodeInner::Root { len: 1 }
        )), item)
    }
}

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash> std::cmp::PartialEq for UnionFindNode<T> {
    fn eq(&self, other: &UnionFindNode<T>) -> bool {
        self.1 == other.1
    }
}

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash> std::cmp::Eq for UnionFindNode<T> {}

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash> std::hash::Hash for UnionFindNode<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash + std::fmt::Debug + Clone> HashUnionFindSets<T> {
    /// Creates empty sets.
    pub fn new() -> HashUnionFindSets<T> {
        HashUnionFindSets {
            set_count: 0,
            items: std::collections::HashMap::new()
        }
    }

    fn error_msg(items: &[&T]) -> String {
        assert!(items.len() == 1 || items.len() == 2);
        if items.len() == 1 {
            format!("no set contains {:?}", items[0])
        } else {
            format!("no set contains {:?} and no set contains {:?}", items[0], items[1])
        }
    }

    /// Adds a singleton set composed of only `item`.
    ///
    /// If a set containing `item` already exists, the sets don't change.
    /// In the case, returns `false`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::collections::sets::*;
    /// let mut sets = HashUnionFindSets::new();
    /// assert!(sets.add(1));
    /// assert!(!sets.add(1));
    /// assert_eq!(sets.items_len(), 1);
    /// ```
    pub fn add(&mut self, item: T) -> bool {
        if self.items.contains_key(&item) {
            false
        } else {
            self.set_count += 1;
            self.items.insert(item.clone(), UnionFindNode::new(item));
            true
        }
    }

    /// Returns how many items are contained by all the sets.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::collections::sets::*;
    /// let mut sets: HashUnionFindSets<i32> = vec![1, 2].into_iter().collect();
    /// assert_eq!(sets.items_len(), 2);
    /// sets.unite(&1, &2);
    /// assert_eq!(sets.items_len(), 2);
    /// ```
    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    fn find(&self, item: &T) -> Option<(UnionFindNode<T>, usize)> {
        fn go<T: Eq + std::hash::Hash + Clone>(node: UnionFindNode<T>) -> (UnionFindNode<T>, usize) {
            let inner = node.0.as_ref().clone().into_inner();
            match inner {
                UnionFindNodeInner::Root { len } => (node, len),
                UnionFindNodeInner::Child { parent } => {
                    let (root, len) = go(parent);
                    let mut borrowed = node.0.borrow_mut();
                    *borrowed = UnionFindNodeInner::Child { parent: root.clone() };
                    (root, len)
                }
            }
        }

        self.items.get(item).cloned().map(go)
    }

    /// Returns how many sets `self` contains.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::collections::sets::*;
    /// let mut sets: HashUnionFindSets<i32> = vec![1, 2].into_iter().collect();
    /// assert_eq!(sets.count(), 2);
    /// sets.unite(&1, &2);
    /// assert_eq!(sets.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.set_count
    }

    /// Returns how many items `self` contains by the set which has `item`.
    ///
    /// If no set contains `item`, returns `Err` with an error message.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::collections::sets::*;
    /// let mut sets: HashUnionFindSets<i32> = vec![1, 2].into_iter().collect();
    ///
    /// assert_eq!(sets.len_of(&1), Ok(1));
    /// sets.unite(&1, &2);
    /// assert_eq!(sets.len_of(&1), Ok(2));
    ///
    /// assert!(sets.len_of(&3).is_err());
    /// ```
    pub fn len_of(&self, item: &T) -> Result<usize, String> {
        self.find(item).map(|(_, len)| len).ok_or_else(|| {
            HashUnionFindSets::error_msg(&[item])
        })
    }

    /// Returns if two sets containing `item1` and `item2` are the same one.
    ///
    /// If no set contains `item1` or `item2`, returns `Err` with an error message.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate atcoder_snippets;
    /// # use atcoder_snippets::collections::sets::*;
    /// let mut sets: HashUnionFindSets<i32> = vec![1, 2].into_iter().collect();
    ///
    /// assert_eq!(sets.set_eq(&1, &2), Ok(false));
    /// sets.unite(&1, &2);
    /// assert_eq!(sets.set_eq(&1, &2), Ok(true));
    ///
    /// assert!(sets.set_eq(&1, &3).is_err());
    /// assert!(sets.set_eq(&3, &4).is_err());
    /// ```
    pub fn set_eq(&self, item1: &T, item2: &T) -> Result<bool, String> {
        match (self.find(item1), self.find(item2)) {
            (Some((root1, _)), Some((root2, _))) => Ok(root1 == root2),
            (Some(_), None) => Err(HashUnionFindSets::error_msg(&[item2])),
            (None, Some(_)) => Err(HashUnionFindSets::error_msg(&[item1])),
            (None, None) => Err(HashUnionFindSets::error_msg(&[item1, item2])),
        }
    }

    /// Merges two sets, set containing `item1` and set containing `item2`.
    ///
    /// If the two sets are same (already merged ones), do nothing and returns `Ok(false)`.
    ///
    /// If no set contains `item1` or `item2`, returns `Err` with an error message.
    pub fn unite(&mut self, item1: &T, item2: &T) -> Result<bool, String> {
        match (self.find(item1), self.find(item2)) {
            (Some((root1, len1)), Some((root2, len2))) => {
                if root1 == root2 {
                    Ok(false)
                } else {
                    self.set_count -= 1;
                    let (mut root, mut child, root_node) = if len1 < len2 {
                        (root2.0.borrow_mut(), root1.0.borrow_mut(), &root2)
                    } else {
                        (root1.0.borrow_mut(), root2.0.borrow_mut(), &root1)
                    };
                    *root = UnionFindNodeInner::Root { len: len1 + len2 };
                    *child = UnionFindNodeInner::Child { parent: root_node.clone() };
                    Ok(true)
                }
            },
            (Some(_), None) => Err(HashUnionFindSets::error_msg(&[item2])),
            (None, Some(_)) => Err(HashUnionFindSets::error_msg(&[item1])),
            (None, None) => Err(HashUnionFindSets::error_msg(&[item1, item2]))
        }
    }
}

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash + std::fmt::Debug + Clone> std::fmt::Debug for HashUnionFindSets<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::collections::{HashMap, HashSet};

        let mut root_to_set = HashMap::new();
        for item in self.items.keys() {
            let root = self.find(item);
            let set = root_to_set.entry(root).or_insert(HashSet::new());
            set.insert(item);
        }

        let sets: Vec<HashSet<&T>> = root_to_set.into_iter().map(|(_, v)| v).collect();
        if sets.len() == 0 {
            write!(f, "{{}}")
        } else {
            write!(f, "{{{:?}", sets[0])?;
            for set in &sets[1..] {
                write!(f, ", {:?}", set)?;
            }
            write!(f, "}}")
        }
    }
}

#[snippet = "sets"]
impl<T: Eq + std::hash::Hash + std::fmt::Debug + Clone> std::iter::FromIterator<T>
    for HashUnionFindSets<T>
{
    /// Creates sets of singletons from an iterator.
    ///
    /// If `iter` has duplicated elements, only the first one is added.
    ///
    /// # Example
    ///
    /// ```
    /// #[macro_use] extern crate atcoder_snippets;
    /// use atcoder_snippets::collections::sets::*;
    /// let sets: HashUnionFindSets<i32> = vec![1, 2, 3, 1].into_iter().collect();
    /// assert_eq!(sets.items_len(), 3);
    /// ```
    ///
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> HashUnionFindSets<T> {
        let items = iter.into_iter()
            .map(|x| (x.clone(), UnionFindNode::new(x)))
            .collect::<std::collections::HashMap<_, _>>();
        HashUnionFindSets {
            set_count: items.len(),
            items: items
        }
    }
}

/*
#[snippet = "sets"]
impl<T: Eq + std::hash::Hash + std::fmt::Debug> IntoIterator for HashUnionFindSets<T> {
    type Item = HashSet<T>;
    type IntoIter = std::collections::hash_map::Values<>;

    fn into_iter(self) -> Self::IntoIter {
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_eq() {
        let mut sets: HashUnionFindSets<i32> = (0..20).collect();

        // unite in sequential order
        for i in 0..9 {
            sets.unite(&i, &(i+1)).unwrap();
        }

        for i in 0..10 {
            for j in 0..10 {
                assert!(sets.set_eq(&i, &j).unwrap());
            }
        }
        for i in 0..10 {
            for j in 10..20 {
                assert!(!sets.set_eq(&i, &j).unwrap());
            }
        }

        // unite in random order
        sets.unite(&10, &11).unwrap();
        sets.unite(&12, &13).unwrap();
        sets.unite(&10, &12).unwrap();

        sets.unite(&14, &15).unwrap();
        sets.unite(&16, &17).unwrap();
        sets.unite(&17, &18).unwrap();
        sets.unite(&14, &17).unwrap();

        sets.unite(&10, &14).unwrap();
        sets.unite(&10, &19).unwrap();

        for i in 10..20 {
            for j in 10..20 {
                assert!(sets.set_eq(&i, &j).unwrap());
            }
        }
        for i in 0..10 {
            for j in 10..20 {
                assert!(!sets.set_eq(&i, &j).unwrap());
            }
        }
    }

    #[test]
    fn test_count() {
        let mut sets = HashUnionFindSets::new();
        assert_eq!(sets.count(), 0);

        sets.add(0);
        assert_eq!(sets.count(), 1);
        sets.add(1);
        assert_eq!(sets.count(), 2);
        sets.add(2);
        assert_eq!(sets.count(), 3);
        sets.add(3);
        assert_eq!(sets.count(), 4);
        sets.add(4);
        assert_eq!(sets.count(), 5);
        sets.add(5);
        assert_eq!(sets.count(), 6);

        sets.add(0);
        assert_eq!(sets.count(), 6);

        sets.unite(&0, &1).unwrap();
        assert_eq!(sets.count(), 5);
        sets.unite(&2, &3).unwrap();
        assert_eq!(sets.count(), 4);
        sets.unite(&3, &4).unwrap();
        assert_eq!(sets.count(), 3);
        sets.unite(&0, &2).unwrap();
        assert_eq!(sets.count(), 2);

        sets.unite(&1, &3).unwrap();
        assert_eq!(sets.count(), 2);

        sets.add(6);
        assert_eq!(sets.count(), 3);
    }

    #[test]
    fn test_count_from_iterator() {
        let sets: HashUnionFindSets<i32> = (0..20).collect();
        assert_eq!(sets.count(), 20);
    }
}
