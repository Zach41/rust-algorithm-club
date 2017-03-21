use std::iter;
use std::fmt;
use std::ops;
use std::cmp::{max, min};

const N: usize = 64;
const ALLONE: u64 = !0u64;

#[derive(Clone, Eq, PartialEq)]
pub struct BitSet {
    pub size: usize,
    words: Vec<u64>,
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for word in &self.words {
            let mut cur = *word;
            let mut cnt = 0;
            while cnt < N {
                let c = if cur & 1 == 1 { "1" } else { "0" };
                try!(f.pad(c));
                cur >>= 1;
                cnt += 1;
            }
        }
        Ok(())
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl BitSet {
    pub fn new(size: usize) -> BitSet {
        let cnt = (size + N - 1) / N;
        BitSet {
            size: size,
            words: iter::repeat(0).map(|_| 0u64).
                take(cnt).collect::<Vec<_>>(),
        }
    }

    pub fn clear_all(&mut self) {
        for word in &mut self.words {
            *word = 0u64
        }
    }

    pub fn set_all(&mut self) {
        for word in &mut self.words {
            *word = ALLONE;
        }
        self.clear_unused();
    }

    pub fn set(&mut self, idx: usize) {
        let (idx, mask) = self.index_tuple(idx);
        self.words[idx] |= mask;
    }

    pub fn clear(&mut self, idx: usize) {
        let (idx, mask) = self.index_tuple(idx);
        self.words[idx] &= !mask;
    }

    // change 0 to 1 and 1 to 0, return the new value
    pub fn flip(&mut self, idx: usize) -> bool {
        let (idx, mask) = self.index_tuple(idx);
        self.words[idx] ^= mask;
        (self.words[idx] & mask) != 0
    }

    pub fn test(&self, idx: usize) -> bool {
        let (idx, mask) = self.index_tuple(idx);
        self.words[idx] & mask != 0
    }

    // returns the number of bits that are 1
    pub fn cardinality(&self) -> usize {
        let mut cnt = 0;
        for word in &self.words {
            let mut cur = *word;
            while cur != 0 {
                let y = cur & !(cur - 1);
                cur ^= y;
                cnt += 1;
            }
        }
        cnt
    }

    pub fn all1(&self) -> bool {
        let len = self.words.len();
        for i in 0..len - 1 {
            if self.words[i] != ALLONE {
                return false;
            }
        }
        self.words[len - 1] == self.last_mask()
    }

    pub fn all0(&self) -> bool {
        for word in &self.words {
            if *word != 0u64 {
                return false;
            }
        }
        true
    }

    pub fn any1(&self) -> bool {
        for word in &self.words {
            if *word != 0u64 {
                return true;
            }
        }
        false
    }
}

impl ops::Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, idx: usize) -> &bool {
        let ret = self.test(idx);
        unsafe { ::std::mem::transmute::<&bool, &bool>(&ret) }
    }
}

impl ops::BitAnd for BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: BitSet) -> BitSet {
        let size = max(self.size, rhs.size);
        let cnt = min(self.words.len(), self.words.len());
        let mut ret = BitSet::new(size);
        for i in 0..cnt {
            ret.words[i] = self.words[i] & rhs.words[i];
        }

        ret
    }
}

impl ops::BitAndAssign for BitSet {
    fn bitand_assign(&mut self, rhs: BitSet) {
        *self = self.clone() & rhs;
    }
}

impl ops::BitOr for BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: BitSet) -> BitSet {
        let mut ret = copy_larger(&self, &rhs);
        let cnt = min(self.words.len(), rhs.words.len());
        for i in 0..cnt {
            ret.words[i] = self.words[i] | rhs.words[i];
        }

        ret
    }
}

impl ops::BitOrAssign for BitSet {
    fn bitor_assign(&mut self, rhs: BitSet) {
        *self = self.clone() | rhs;
    }
}

impl ops::BitXor for BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: BitSet) -> BitSet {
        let mut ret = copy_larger(&self, &rhs);
        let cnt = min(self.words.len(), rhs.words.len());
        for i in 0..cnt {
            ret.words[i] = self.words[i] ^ rhs.words[i];
        }

        ret
    }
}

impl ops::BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, rhs: BitSet) {
        *self = self.clone() ^ rhs;
    }
}

impl ops::Not for BitSet {
    type Output = BitSet;
    fn not(self) -> BitSet {
        let mut ret = self.clone();
        for (idx, word) in self.words.iter().enumerate() {
            let w = *word;
            ret.words[idx] = !w;
        }
        ret.clear_unused();

        ret
    }
}

fn copy_larger(lhs: &BitSet, rhs: &BitSet) -> BitSet {
    if lhs.size > rhs.size {
        lhs.clone()
    } else {
        rhs.clone()
    }
}

impl BitSet {
    fn clear_unused(&mut self) {
        let len = self.words.len();
        self.words[len - 1] &= self.last_mask();
    }

    fn index_tuple(&self, idx: usize) -> (usize, u64) {
        let i = idx / N;
        let diff = 1u64 << (idx - i * N);
        (i, diff as u64)
    }

    fn last_mask(&self) -> u64 {
        let diff = self.words.len() * N - self.size;
        let mask = 1u64 << (N - 1 - diff);
        mask | (mask - 1)
    }
}


#[cfg(test)]
mod test;
