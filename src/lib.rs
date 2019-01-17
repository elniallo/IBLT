use std::collections::hash_map::DefaultHasher;
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

struct InvertibleBloomLookupTable {
    table: Vec<Node>,
    area_count: u8,
}

struct Output {
    key_sum: u32,
    value_sum: u32
}

impl InvertibleBloomLookupTable {
    pub fn new(size: usize, area_count: u8) -> InvertibleBloomLookupTable {
        InvertibleBloomLookupTable {
            table: vec![Node::default(); size],
            area_count,
        }
    }

    pub fn hash(&self, i: u8, value: u32) -> usize {
        let area_size = 2_u32.pow(32) / (self.area_count as u32);
        let mut hasher_in = DefaultHasher::new();
        value.hash(&mut hasher_in);
        let hash_value = hasher_in.finish();
        return (hash_value % (area_size as u64)) as usize + i as usize * area_size as usize;
    }

    fn insert(&mut self, x: u32, y: u32) {
        for i in 0..self.area_count {
            let hash_value = self.hash(i, x);
            self.table[hash_value].count += 1;
            self.table[hash_value].key_sum += x;
            self.table[hash_value].value_sum += y;
        }
    }

    fn get(&self, x: u32) -> Option<u32> {
        for i in 0..self.area_count {
            if self.table[self.hash(i, x)].count == 0 {
                return None;
            } else if self.table[self.hash(i, x)].count == 1 {
                if self.table[self.hash(i, x)].key_sum == x {
                    return Some(self.table[self.hash(i, x)].value_sum);
                } else {
                    return None;
                }
            }
        }
        return None;
    }

    fn delete(&mut self, x: u32, y: u32) {
        for i in 0 .. self.area_count {
            let hash_value = self.hash(i, x);
            self.table[hash_value].count -= 1;
            self.table[hash_value].key_sum -= x;
            self.table[hash_value].value_sum -= y;
        }
    }

    fn list_entries(&mut self) -> Vec<Output> {
        let mut ret_val = Vec::<Output>::new();
        for i in 0 .. self.table.len() {
            if self.table[i].count == 1 {
                let key_sum = self.table[i].key_sum;
                let value_sum = self.table[i].value_sum;
                ret_val.push(Output{
                    key_sum ,
                    value_sum,
                });
                self.delete(key_sum, value_sum);
            }
        }
        return ret_val;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
