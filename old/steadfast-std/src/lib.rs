/// Several different methods for getting, or evaluating, uniqueness.
pub trait Uniq<T> {
    /// `uniq` returns a vector of unique values within itself as compared to
    /// the other vector which is provided as an input parameter.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,3,4,5,6].uniq( vec![1,2,5,7,9] );
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![3,4,6]
    /// ```
    fn uniq(&self, other: Self) -> Self;

    /// `unique` removes duplicates from within the vector and returns Self.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,1,3,2,3,4,5,6].unique();
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,2,3,4,5,6]
    /// ```
    fn unique(&self) -> Self;

    /// `is_unique` returns boolean value on whether all values within
    /// Self are unique.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,1,3,4,3,4,5,6].is_unique();
    /// ```
    ///
    /// # Output
    /// ```text
    /// false
    /// ```
    fn is_unique(&self) -> bool;

    /// `uniq_via` returns a vector of unique values within itself as compared to
    /// the other vector which is provided as an input parameter, as defined by a
    /// provided custom comparator.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,3,4,5,6].uniq_via( vec![1,2,5,7,9], |&l, r| l == r + 2 );
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,2,4,6]
    /// ```
    fn uniq_via<F: Fn(&T, &T) -> bool>(&self, other: Self, f: F) -> Self;

    /// `unique_via` removes duplicates, as defined by a provided custom comparator,
    /// from within the vector and returns Self.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1.0,2.0,1.4,3.3,2.1,3.5,4.6,5.2,6.2].unique_via( |l: &f64, r: &f64| l.floor() == r.floor() );
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1.0,2.0,3.3,4.6,5.2,6.2]
    /// ```
    fn unique_via<F: Fn(&T, &T) -> bool>(&self, f: F) -> Self;

    /// `is_unique_via` returns boolean value on whether all values within
    /// Self are unique, as defined by a provided custom comparator.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1.0,2.0,1.4,3.3,2.1,3.5,4.6,5.2,6.2].is_unique_via( |l: &f64, r: &f64| l.floor() == r.floor() );
    /// ```
    ///
    /// # Output
    /// ```text
    /// false
    /// ```
    fn is_unique_via<F: Fn(&T, &T) -> bool>(&self, f: F) -> bool;
}

impl<T: Clone + PartialEq> Uniq<T> for Vec<T> {
    fn uniq(&self, other: Vec<T>) -> Vec<T> {
        self.uniq_via(other, |lhs, rhs| lhs == rhs)
    }
    fn unique(&self) -> Vec<T> {
        self.unique_via(|lhs, rhs| lhs == rhs)
    }
    fn is_unique(&self) -> bool {
        self.is_unique_via(|lhs, rhs| lhs == rhs)
    }

    fn uniq_via<F: Fn(&T, &T) -> bool>(&self, other: Vec<T>, f: F) -> Vec<T> {
        let mut out = self.unique();
        for x in other.unique() {
            for y in (0..out.len()).rev() {
                if f(&x, &out[y]) {
                    out.remove(y);
                }
            }
        }
        out
    }
    fn unique_via<F: Fn(&T, &T) -> bool>(&self, f: F) -> Vec<T> {
        let mut a = self.clone();
        for x in (0..a.len()).rev() {
            for y in (x + 1..a.len()).rev() {
                if f(&a[x], &a[y]) {
                    a.remove(y);
                }
            }
        }
        a
    }
    fn is_unique_via<F: Fn(&T, &T) -> bool>(&self, f: F) -> bool {
        let mut a = true;
        for x in 0..self.len() {
            for y in x + 1..self.len() {
                if f(&self[x], &self[y]) {
                    a = false;
                    break;
                }
            }
        }
        a
    }
}

/// Set Intersection — Returns a new array containing elements common to the two arrays,
/// excluding any duplicates. The order is preserved from the original array.
pub trait Intersect<T> {
    /// # Example
    /// ```
    /// use array_tool::vec::Intersect;
    ///
    /// vec![1,1,3,5].intersect(vec![1,2,3]);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,3]
    /// ```
    fn intersect(&self, other: Self) -> Self;
    /// # Example
    /// ```
    /// # use std::ascii::AsciiExt;
    /// use array_tool::vec::Intersect;
    ///
    /// vec!['a','a','c','e'].intersect_if(vec!['A','B','C'], |l, r| l.eq_ignore_ascii_case(r));
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec!['a','c']
    /// ```
    fn intersect_if<F: Fn(&T, &T) -> bool>(&self, other: Self, validator: F) -> Self;
}
impl<T: PartialEq + Clone> Intersect<T> for Vec<T> {
    fn intersect(&self, other: Vec<T>) -> Vec<T> {
        self.intersect_if(other, |l, r| l == r)
    }
    fn intersect_if<F: Fn(&T, &T) -> bool>(&self, other: Self, validator: F) -> Self {
        let mut out = vec![];
        let a = self.unique();
        let length = other.len();
        for x in a {
            for y in 0..length {
                if validator(&x, &other[y]) {
                    out.push(x);
                    break;
                }
            }
        }
        out
    }
}

/// Create a `union` between two vectors.
/// Returns a new vector by joining with other, excluding any duplicates and preserving
/// the order from the original vector.
pub trait Union {
    /// # Example
    /// ```
    /// use array_tool::vec::Union;
    ///
    /// vec!["a","b","c"].union(vec!["c","d","a"]);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![ "a", "b", "c", "d" ]
    /// ```
    fn union(&self, other: Self) -> Self;
}
impl<T: PartialEq + Clone> Union for Vec<T> {
    fn union(&self, other: Vec<T>) -> Vec<T> {
        let mut stack = self.clone();
        for x in other {
            // don't use append method as it's destructive
            stack.push(x)
        }
        stack.unique()
    }
}
