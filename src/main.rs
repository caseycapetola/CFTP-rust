use rand::Rng;

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
    let mut rng = rand::thread_rng();
    let arr_size: i32 = 10;
    let beta: f32 = 0.5;

    let mut pos_grid: Row = Row{
        elements: Vec::new()
    };

    pos_grid.init_grid(true, arr_size*arr_size);
    println!("{:#?}", pos_grid.elements);

    let mut neg_grid: Row = Row {
        elements: Vec::new()
    };
    neg_grid.init_grid(false, arr_size*arr_size);
    let mut diff: i32 = arr_size*arr_size;

    let mut rand_num: f32;
    while diff > 0 {
        rand_num = rng.gen::<f32>();
    }


}
