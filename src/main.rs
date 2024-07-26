// use rand::Rng;

// static int POS = 1
// let POS: bool = true;
// let NEG: bool = false;

struct Row {
    elements: Vec<bool>
}

impl Row {
    fn init_grid(&mut self, init_type: bool, size: i32) {
        // loop for "size" times
        for _ in 0..size {
            self.elements.push(init_type)
        }    
    }
}

fn main() {
    println!("Hello, world!");
    // 0 -> negative spin; 1 -> positive spin
    // 2D Array -> 
    let mut pos_grid: Row = Row{
        elements: Vec::new()
    };
    pos_grid.init_grid(true, 10);
    println!("{:#?}", pos_grid.elements);
}
