use core::fmt;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

/// OrdMap implements some conveniences like Clone and PartialEq for BTreeMap so that we can compare
/// XML Documents.
#[derive(Default)]
pub(crate) struct OrdMap(BTreeMap<String, String>);

impl Clone for OrdMap {
    fn clone(&self) -> Self {
        let mut result = BTreeMap::new();
        for (k, v) in self.0.iter() {
            result.insert(k.clone(), v.clone());
        }
        Self(result)
    }
}

impl PartialEq for OrdMap {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (k, v) in self.0.iter() {
            if let Some(other_v) = other.0.get(k) {
                if other_v != v {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl Eq for OrdMap {}

impl OrdMap {
    /// Return the inner BTreeMap as immutable.
    pub(crate) fn map(&self) -> &BTreeMap<String, String> {
        &self.0
    }

    /// Return the inner BTreeMap as mutable.
    pub(crate) fn mut_map(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.0
    }

    fn size_le(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
    }

    fn size_gt(&self, other: &Self) -> bool {
        self.0.len() > other.0.len()
    }
}

impl PartialOrd for OrdMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less || ordering == Ordering::Equal;
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater || ordering == Ordering::Equal;
        }
        false
    }
}

impl Ord for OrdMap {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.size_le(other) {
            return Ordering::Less;
        } else if self.size_gt(other) {
            return Ordering::Greater;
        }
        for (k, my_val) in self.0.iter() {
            let get_opt = other.0.get(k.as_str());
            match get_opt {
                None => {
                    return Ordering::Greater;
                }
                Some(other_val) if my_val < other_val => {
                    return Ordering::Less;
                }
                Some(other_val) if my_val > other_val => {
                    return Ordering::Greater;
                }
                Some(_) => {}
            }
        }
        Ordering::Equal
    }
}

impl fmt::Debug for OrdMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for OrdMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.0.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::OrdMap;

    /// Although this test does nothing but explore the behavior of a built-in data structure, it's
    /// here to demonstrate the desired characteristic of that data structure. We want attributes to
    /// serialize in the order that they were inserted.
    #[test]
    fn map_insertion_order() {
        let mut a = OrdMap(BTreeMap::new());
        a.mut_map().insert("0".to_string(), String::new());
        a.mut_map().insert("1".to_string(), String::new());
        a.mut_map().insert("2".to_string(), String::new());
        a.mut_map().insert("3".to_string(), String::new());
        a.mut_map().insert("4".to_string(), String::new());
        let entries = a.map().keys();
        for (i, item) in entries.enumerate() {
            assert_eq!(format!("{i}"), item.to_owned());
        }
    }

    /// This test demonstrates that two maps containing the same entries are equal irrespective of
    /// insertion order.
    #[test]
    fn map_equality() {
        let mut a = OrdMap(BTreeMap::new());
        a.mut_map().insert("0".to_string(), "a".to_string());
        a.mut_map().insert("1".to_string(), "b".to_string());
        a.mut_map().insert("2".to_string(), "c".to_string());
        a.mut_map().insert("3".to_string(), "d".to_string());
        a.mut_map().insert("4".to_string(), "e".to_string());
        let mut b = OrdMap(BTreeMap::new());
        b.mut_map().insert("4".to_string(), "e".to_string());
        b.mut_map().insert("3".to_string(), "d".to_string());
        b.mut_map().insert("1".to_string(), "b".to_string());
        b.mut_map().insert("2".to_string(), "c".to_string());
        b.mut_map().insert("0".to_string(), "a".to_string());
        assert_eq!(a, b);
    }
}
