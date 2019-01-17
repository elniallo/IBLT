

struct InvertibleBloomLookupTableNode{
    count: u32,
    keySum: u32,
    valueSum: u32
}

type Node = InvertibleBloomLookupTableNode;
type HashFunction = Fn(u32) -> u32;

struct InvertibleBloomLookupTable {
    table: Vec<Node>,
    hashes: Vec<HashFunction>
}

impl InvertibleBloomLookupTable {
    fn new(size: usize, hashes: &Vec<HashFunction>) -> InvertibleBloomLookupTable {
        let table = Vec::<Node>::with_capacity(size);
        let hashes = hashes.clone();
        InvertibleBloomLookupTable {
            table,
            hashes
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
