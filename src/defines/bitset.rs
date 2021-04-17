use std::ops::Index;
const BITMASK: u8 = 0xFF;
const SINGLE_BITMASK: u8 = 0x80;

const VECSIZE: usize = 8;

#[derive(Clone)]
pub struct BitSet {
    bits: Vec<u8>,
    size: usize,
}


impl BitSet {
    pub fn new() ->BitSet {
        BitSet::with_size(0)
    }

    pub fn with_size(size: usize) -> BitSet {
        BitSet {
            bits: vec![0; BitSet::rounding_up_div(size)],
            size: size,
        }
    }

    pub fn push(&mut self, bit: bool) {
        self.size += 1;
        let mut new_bit: u8 = 0;
        if bit {
            new_bit = 1;
        }

        if BitSet::rounding_up_div(self.size) > self.bits.len() {
            self.bits.push(0);
        }

        *self.bits.last_mut().unwrap() |= new_bit << (VECSIZE - (self.size % VECSIZE))
    }

    pub fn at(&mut self, index: usize) -> u8 {
        assert!(index < self.size);
        self.get_current_bit_vector(index) & (SINGLE_BITMASK >> (index % VECSIZE))
    }

    fn rounding_up_div(value: usize) -> usize{
        let mut result = value / 8;
        if value % 8 == 0 {
            result += 1;
        } 

        result
    }

    fn get_current_bit_vector(&self, index: usize) -> &u8{
        &self.bits[BitSet::rounding_up_div(index)]
    }


}


