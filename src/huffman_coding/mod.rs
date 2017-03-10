use std::cmp::Ordering;
use std::fmt;

use super::priority_queue::PriorityQueue;

struct BitWriter {
    buf: Vec<u8>,
    out_byte: u8,
    out_count: usize,
}

impl fmt::Debug for BitWriter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut byte = self.out_byte;
        let mut ans: Vec<bool> = Vec::new();
        while byte != 0 {
            if byte & 0x80 != 0 {
                ans.push(true);
            } else {
                ans.push(false);
            }
            byte <<= 1;
        }
        writeln!(f, "{:?}", ans)
    }
}

impl BitWriter {
    fn new() -> BitWriter {
        BitWriter {
            buf: Vec::new(),
            out_byte: 0u8,
            out_count: 0,
        }
    }

    fn write_bit(&mut self, bit: bool) {
        if self.out_count == 8 {
            self.buf.push(self.out_byte);
            self.out_byte = 0;
            self.out_count = 0;
        }
        self.out_byte = (self.out_byte << 1) | (if bit { 1 } else { 0 });
        self.out_count += 1;
    }

    fn flush(&mut self) {
        if self.out_count > 0 {
            if self.out_count < 8 {
                let diff = 8 - self.out_count;
                self.out_byte <<= diff;
            }
            self.buf.push(self.out_byte);
        }
    }

    fn data(&mut self) -> Vec<u8> {
        self.buf.drain(..).collect()
    }
}

struct BitReader<'a> {
    buf: &'a [u8],
    in_byte: u8,
    ptr: usize,
    in_count: usize,
}

impl<'a> BitReader<'a> {
    fn new(buf: &'a [u8]) -> BitReader {
        BitReader {
            buf: buf,
            in_byte: 0,
            in_count: 8,
            ptr: 0,
        }
    }

    fn read_bit(&mut self) -> bool {
        if self.in_count == 8 {
            self.in_byte = self.buf[self.ptr];
            self.ptr += 1;
            self.in_count = 0;
        }
        self.in_count += 1;

        let ret = !((self.in_byte & 0x80) == 0);
        self.in_byte <<= 1;
        ret
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Node {
    parent: i32,
    left: i32,
    right: i32,
    index: i32,
    count: usize,
}


impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        match self.count.cmp(&other.count) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            parent: -1,
            left: -1,
            right: -1,
            index: -1,
            count: 0,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("parent: {}, left: {}, right: {}, index: {}, count: {}",
                       self.parent, self.left, self.right, self.index, self.count))
    }
}

#[derive(Clone, Copy)]
pub struct Freq {
    byte: u8,
    count: usize,
}

impl fmt::Debug for Freq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.byte as char, self.count)
    }
}

impl Default for Freq {
    fn default() -> Freq {
        Freq {
            byte: 0,
            count: 0,
        }
    }
}

pub struct Huffman {
    root: i32,
    tree: Vec<Node>,    
}

impl fmt::Debug for Huffman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for node in &self.tree {
            if node.count > 0 {
                writeln!(f, "idx: {}, parent: {}, count: {}, left: {}, right: {}",
                         node.index, node.parent, node.count, node.left, node.right)?;
            }
        }
        writeln!(f, "\n")?;
        Ok(())
    }
}

impl Huffman {
    fn count_byte_freq(&mut self, data: &[u8]) {
        let _: Vec<_> = data.into_iter().map(|&byte| {
            let idx = byte as usize;
            self.tree[idx].count += 1;
            self.tree[idx].index = idx as i32;
        }).collect();

        // self.build_tree();
    }

    fn build_tree(&mut self) {
        let mut queue: PriorityQueue<Node> = PriorityQueue::new();
        let _ = self.tree.iter()
            .filter(|&node| node.count > 0)
            .map(|&node| {
                queue.enqueue(node)                    
            }).collect::<Vec<_>>();
        

        while queue.count() > 1 {
            let node1 = queue.dequeue().unwrap();
            let node2 = queue.dequeue().unwrap();

            let mut parent_node = Node::default();
            parent_node.count = node1.count + node2.count;
            parent_node.left = node1.index;
            parent_node.right = node2.index;
            parent_node.index = self.tree.len() as i32;
            self.tree.push(parent_node);

            self.tree[node1.index as usize].parent = parent_node.index;
            self.tree[node2.index as usize].parent = parent_node.index;

            queue.enqueue(parent_node);
        }

        let root_node = queue.dequeue().unwrap();
        self.root = root_node.index;
    }

    fn restore_tree(&mut self, frequency_tbl: &[Freq]) {
        for freq in frequency_tbl.iter() {
            let byte = freq.byte;
            self.tree[byte as usize].index = byte as i32;
            self.tree[byte as usize].count = freq.count;            
        }
        self.build_tree();
    }

    fn traverse_tree(&self, writer: &mut BitWriter, node_idx: i32, child_idx: i32) {
        if self.tree[node_idx as usize].parent != -1 {
            let parent = self.tree[node_idx as usize].parent;            
            self.traverse_tree(writer, parent, node_idx);
        }
        if child_idx != -1 {
            if child_idx == self.tree[node_idx as usize].left {
                writer.write_bit(true);
            } else if child_idx == self.tree[node_idx as usize].right {
                writer.write_bit(false);
            }
        }
    }

    fn find_leaf_node(&self, reader: &mut BitReader, node_idx: i32) -> u8 {
        let mut idx = node_idx as usize;
        while self.tree[idx].right != -1 {
            if reader.read_bit() {
                idx = self.tree[idx].left as usize;
            } else {
                idx = self.tree[idx].right as usize;
            }            
        }
        idx as u8        
    }
}

impl Huffman {
    pub fn new() -> Huffman {
        Huffman {
            root: -1,
            tree: ::std::iter::repeat(0).map(|_| Node::default())
                .take(256).collect(),
        }
    }

    pub fn frequency_tbl(&self) -> Vec<Freq> {
        self.tree.iter()
            .take(256)
            .filter(|&node| node.count > 0)
            .map(|&node| Freq { byte: node.index as u8, count: node.count})
            .collect()
    }

    pub fn compress_data(&mut self, data: Vec<u8>) -> Vec<u8> {
        self.count_byte_freq(&data);
        self.build_tree();

        let mut writer = BitWriter::new();
        for byte in data {
            self.traverse_tree(&mut writer, byte as i32, -1);
        }
        writer.flush();
        writer.data()
    }

    pub fn decompress_data(&mut self, data: Vec<u8>, frequency_tbl: &[Freq]) -> Vec<u8> {
        self.restore_tree(frequency_tbl);

        let mut reader = BitReader::new(&data);
        let mut out_byte: Vec<u8> = Vec::new();
        let root = self.root;
        let byte_count = self.tree[root as usize].count;

        let mut i = 0;
        while i < byte_count {
            let b = self.find_leaf_node(&mut reader, root);
            out_byte.push(b);
            i += 1;
        }
        out_byte
    }
}

#[cfg(test)]
mod test {
    extern crate env_logger;
    
    use super::*;
    use std::iter;
    
    #[test]
    fn test_read_bits() {
        let data = vec![12u8];
        let mut reader = BitReader::new(&data);

        let results: Vec<_> = iter::repeat(0).map(|_| reader.read_bit()).take(8).collect();
        assert_eq!(results, vec![false, false, false, false, true, true, false, false]);
    }

    #[test]
    fn test_write_bits() {
        let mut writer = BitWriter::new();

        for _ in 0..4 {
            writer.write_bit(false);
        }
        for _ in 0..4 {
            writer.write_bit(true);
        }
        for _ in 0..4 {
            writer.write_bit(false);            
        }
        writer.write_bit(true);
        writer.write_bit(true);
        
        writer.flush();
        assert_eq!(writer.data(), vec![15u8, 12u8]);
    }

    #[test]
    fn test_huffman() {        
        let s = "so much words wow many compression";
        let mut huffman1 = Huffman::new();
        let compressed_data = huffman1.compress_data(s.to_owned().into_bytes());

        // assert!(false);
        let frequency_tbl = huffman1.frequency_tbl();

        let mut huffman2 = Huffman::new();
        let decompressed_data = huffman2.decompress_data(compressed_data, &frequency_tbl);

        assert_eq!(s, String::from_utf8(decompressed_data).unwrap());
    }
}

