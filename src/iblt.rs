use crate::iblt_error::IBLTError;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct InvertibleBloomLookupTableNode {
    count: u32,
    key_sum: u32,
    value_sum: u32,
}

impl Default for InvertibleBloomLookupTableNode {
    fn default() -> InvertibleBloomLookupTableNode {
        InvertibleBloomLookupTableNode {
            count: 0,
            key_sum: 0,
            value_sum: 0,
        }
    }
}

type Node = InvertibleBloomLookupTableNode;
#[derive(Clone)]
pub struct InvertibleBloomLookupTable<T> {
    table: Vec<Node>,
    area_count: u8,
    hasher: T,
    postfix: Vec<u8>,
}

pub struct Output {
    key_sum: u32,
    value_sum: u32,
}

pub struct OutputList {
    key_pairs: Vec<Output>,
    complete_list: bool,
}
impl OutputList {
    fn new() -> OutputList {
        OutputList {
            key_pairs: Vec::new(),
            complete_list: true,
        }
    }
}

impl<T: Hasher + Default + Clone> InvertibleBloomLookupTable<T> {
    pub fn new(size: usize, area_count: u8) -> Option<InvertibleBloomLookupTable<T>> {
        if size == 0 || area_count <= 1 || size % (area_count as usize) != 0 {
            return None;
        }
        let mut postfix = Vec::with_capacity(area_count as usize);
        for i in 0..area_count {
            postfix.push(0x00 + i)
        }
        Some(InvertibleBloomLookupTable {
            table: vec![Node::default(); size],
            area_count,
            hasher: T::default(),
            postfix,
        })
    }

    pub fn hash(&mut self, i: u8, value: u32) -> Result<usize, IBLTError> {
        if i >= self.area_count {
            return Err(IBLTError::new("Index out of bounds"));
        }
        let area_size = self.table.len() / self.area_count as usize;
        value.hash(&mut self.hasher);
        let hash_value = self.hasher.finish();
        self.hasher = T::default();
        return Ok((hash_value % (area_size as u64)) as usize + i as usize * area_size);
    }

    pub fn insert(&mut self, x: u32, y: u32) -> Result<(), IBLTError> {
        for i in 0..self.area_count {
            let hash_value = self.hash(i, x+self.postfix[i as usize] as u32)?;
            self.table[hash_value].count += 1;
            self.table[hash_value].key_sum += x;
            self.table[hash_value].value_sum += y;
        }
        return Ok(());
    }

    pub fn get(&mut self, x: u32) -> Result<u32, IBLTError> {
        for i in 0..self.area_count {
            let hash_value = self.hash(i, x+self.postfix[i as usize] as u32)?;
            if self.table[hash_value].count == 0 {
                return Err(IBLTError::new("Error: Not Found"));
            } else if self.table[hash_value].count == 1 {
                if self.table[hash_value].key_sum == x {
                    return Ok(self.table[hash_value].value_sum);
                } else {
                    return Err(IBLTError::new("Error: Not Found"));
                }
            }
        }
        return Err(IBLTError::new("Error: Not Found"));
    }

    pub fn delete(&mut self, x: u32, y: u32) -> Result<(), IBLTError> {
        let mut matched = false;
        for i in 0..self.area_count {
            let hash_value = self.hash(i, x+self.postfix[i as usize] as u32)?;
            if self.table[hash_value].count == 0 {
                continue;
            }
            self.table[hash_value].count -= 1;
            self.table[hash_value].key_sum -= x;
            self.table[hash_value].value_sum -= y;
            matched = true;
        }
        if !matched {
            return Err(IBLTError::new("Error: Not Found"));
        }
        Ok(())
    }
    /// Returns the list of entries from the Table, the output list contains a boolean indicating a successful run or partial success.
    pub fn list_entries(&self) -> Result<OutputList, IBLTError> {
        let mut ret_val = OutputList::new();
        let mut table = self.clone();
        for i in 0..table.table.len() {
            if table.table[i].count == 1 {
                let key_sum = table.table[i].key_sum;
                let value_sum = table.table[i].value_sum;
                ret_val.key_pairs.push(Output { key_sum, value_sum });
                table.delete(key_sum, value_sum)?;
            } else if ret_val.complete_list && table.table[i].count > 1 {
                ret_val.complete_list = false;
            }
        }
        return Ok(ret_val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn constructor_with_zero_size_fails() {
        assert!(InvertibleBloomLookupTable::<DefaultHasher>::new(0, 1).is_none());
    }

    #[test]
    fn constructor_with_zero_or_one_area_count_fails() {
        assert!(InvertibleBloomLookupTable::<DefaultHasher>::new(1, 0).is_none());
        assert!(InvertibleBloomLookupTable::<DefaultHasher>::new(1, 1).is_none());
    }

    #[test]
    fn constructor_with_size_not_divisible_with_area_count_fails() {
        assert!(InvertibleBloomLookupTable::<DefaultHasher>::new(3, 2).is_none());
        assert!(InvertibleBloomLookupTable::<DefaultHasher>::new(5, 3).is_none());
    }

    #[test]
    fn constructor() {
        for i in 2..4 {
            assert!(InvertibleBloomLookupTable::<DefaultHasher>::new(i * i, i as u8).is_some());
        }
    }

    #[test]
    fn too_high_hash_index() {
        let area_count = 16;
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, area_count).unwrap();
        assert!(table.hash(16, 17).is_err());
        assert!(table.hash(17, 17).is_err());
    }

    #[test]
    fn hash_in_area() {
        for i in 0..7 {
            let area_count = 2u8.pow(i + 1) as u8;
            let area_size = 256 / area_count as u32;
            let mut table =
                InvertibleBloomLookupTable::<DefaultHasher>::new(256, area_count).unwrap();
            let value = i * i;
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            assert_eq!(
                table.hash(i as u8, (i * i) as u32).unwrap(),
                (hasher.finish() % area_size as u64 + i as u64 * area_size as u64) as usize
            );
            assert!(
                table.hash(i as u8, (i * i) as u32).unwrap()
                    < ((i as usize + 1) * (area_size as usize))
            );
            assert!(
                table.hash(i as u8, (i * i) as u32).unwrap()
                    >= ((i as usize) * (area_size as usize))
            );
        }
    }

    #[test]
    fn try_to_get_a_value_from_empty_table() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.get(3).is_err());
    }

    #[test]
    fn insert_and_get_the_value() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.insert(3, 5).is_ok());
        assert_eq!(table.get(3).ok().unwrap(), 5);
    }

    #[test]
    fn try_to_remove_a_value_from_an_empty_table() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.delete(3, 5).is_err());
    }

    #[test]
    fn insert_remove_and_get_the_value() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.insert(3, 5).is_ok());
        assert!(table.delete(3, 5).is_ok());
        assert!(table.get(3).is_err());
    }

    #[test]
    fn insert_one_items_and_get_list_entries() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.insert(4, 6).is_ok());
        let results = table.list_entries().ok().unwrap();
        assert_eq!(results.key_pairs.len(), 1);
        for output in results.key_pairs {
            if output.key_sum == 4 {
                assert_eq!(output.value_sum, 6);
            }
        }
    }

    #[test]
    fn insert_two_items_and_get_list_entries() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.insert(4, 6).is_ok());
        assert!(table.insert(5, 7).is_ok());
        let results = table.list_entries().ok().unwrap();
        assert_eq!(results.key_pairs.len(), 2);
        for output in results.key_pairs {
            if output.key_sum == 4 {
                assert_eq!(output.value_sum, 6);
            } else if output.key_sum == 5 {
                assert_eq!(output.value_sum, 7);
            }
        }
    }

    #[test]
    fn insert_three_items_and_get_list_entries() {
        let mut table = InvertibleBloomLookupTable::<DefaultHasher>::new(256, 8).unwrap();
        assert!(table.insert(3, 5).is_ok());
        assert!(table.insert(4, 6).is_ok());
        assert!(table.insert(5, 7).is_ok());
        let results = table.list_entries().ok().unwrap();
        assert_eq!(results.key_pairs.len(), 3);
        for output in results.key_pairs {
            if output.key_sum == 3 {
                assert_eq!(output.value_sum, 5);
            } else if output.key_sum == 4 {
                assert_eq!(output.value_sum, 6);
            } else if output.key_sum == 5 {
                assert_eq!(output.value_sum, 7);
            }
        }
    }
}
