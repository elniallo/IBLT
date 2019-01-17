use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

struct InvertibleBloomLookupTableNode{
    count: u32,
    key_sum: u32,
    value_sum: u32
}

type Node = InvertibleBloomLookupTableNode;

struct InvertibleBloomLookupTable {
    table: Vec<Node>,
    area_count: u8
}

impl InvertibleBloomLookupTable {
    pub fn new(size: usize, area_count: u8) -> InvertibleBloomLookupTable {
        let table = Vec::<Node>::with_capacity(size);
        InvertibleBloomLookupTable {
            table,
            area_count
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
            let hash_value = self.hash(i,x);
            self.table[hash_value].count += 1;
            self.table[hash_value].key_sum += x;
            self.table[hash_value].value_sum += y;
        }
    }

    fn get(&self, x:u32) -> Option<u32>{
        for i in 0..self.area_count {
            if self.table[self.hash(i,x)].count == 0 {
                return None;
            } else if self.table[self.hash(i,x)].count == 1 {
                if self.table[self.hash(i,x)].key_sum == x {
                    return Some(self.table[self.hash(i,x)].value_sum);
                } else {
                    return None;
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
