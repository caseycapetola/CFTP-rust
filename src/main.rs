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
    fn ising_calc(self, x: i32, y: i32, beta: f32) -> f64{
        // check neighbors and run Ising calculation
        let mut num_pos: i32 = 0;
        let mut num_neg: i32 = 0;
        
        // CHECK HORIZONTAL NEIGHBORS
        if y>0 {
            // if grid[x][y-1] == POS
            if self.elements[(x*y-1) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }
        if y < self.elements.len().try_into().unwrap() {
            // if grid[x][y+1] == POS
            if self.elements[(x*y+1) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }
        // CHECK VERTICAL NEIGHBORS
        if x>0 {
            // if grid[x-1][y] == POS
            if self.elements[((x-1)*y) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }
        if x < self.elements.len().try_into().unwrap() {
            // if grid[x+1][y] == POS
            if self.elements[((x+1)*y) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }

        let one = 1.0_f64;  // e^1
        let e = one.exp();

        return e.powf((beta * num_pos as f32).into())/(e.powf((beta * num_pos as f32).into()) + e.powf((beta * num_neg as f32).into()));
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
