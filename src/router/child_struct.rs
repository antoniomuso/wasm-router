#[derive(Debug)]
pub struct ByteIndexable<T> {
    childs: Vec<T>,
    map: [i32; 256],
}


impl <T> ByteIndexable <T>{
    pub fn new() -> Self {
        ByteIndexable {
            childs: vec![],
            map: [-1; 256]
        }
    } 

    pub fn add(&mut self, character: u8, node: T) {
        self.childs.push(node);
        self.map[character as usize] = (self.childs.len() - 1) as i32;

    }

    pub fn get_mut (&mut self, character: u8) -> Option<&mut T>{
        let index = self.map[character as usize];
        if index != -1 {
            return Some(&mut self.childs[index as usize]);
        }
        None
    }

    pub fn get (&self, character: u8) -> Option<& T>{
        let index = self.map[character as usize];
        if index != -1 {
            return Some(&self.childs[index as usize]);
        }
        None
    }

    pub fn contains_key (&self, character: u8) -> bool {
        let index = self.map[character as usize];
        index != -1 
    }
}


