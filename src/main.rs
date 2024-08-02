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

    fn ising_calc(&mut self, x: i32, y: i32, beta: f32, arr_size: i32) -> f64{
        // check neighbors and run Ising calculation
        let mut num_pos: i32 = 0;
        let mut num_neg: i32 = 0;
        
        // CHECK HORIZONTAL NEIGHBORS
        if y>0 {
            // if grid[x][y-1] == POS
            // println!("current index: {}", x*y-1);
            // println!("value: {}", self.elements[(arr_size*x+y-1) as usize]);
            if self.elements[(arr_size*x+y-1) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }
        if y < (arr_size - 1) {
            // if grid[x][y+1] == POS
            if self.elements[(arr_size*x+y+1) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }
        // CHECK VERTICAL NEIGHBORS
        if x>0 {
            // if grid[x-1][y] == POS
            if self.elements[(arr_size*(x-1)+y) as usize] == true {
                num_pos += 1;
            }
            else {
                num_neg += 1;
            }
        }
        if x < (arr_size - 1) {
            // if grid[x+1][y] == POS
            if self.elements[(arr_size*(x+1)+y) as usize] == true {
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
    let beta: f32 = 0.1;
    let mut x: i32;
    let mut y: i32;

    let mut pos_grid: Row = Row{
        elements: Vec::new()
    };

    pos_grid.init_grid(true, arr_size*arr_size);
    // println!("{:#?}", pos_grid.elements);

    let mut neg_grid: Row = Row {
        elements: Vec::new()
    };
    neg_grid.init_grid(false, arr_size*arr_size);
    let mut diff: i32 = arr_size*arr_size;

    let mut flag: f64;
    let mut calc1: f64;
    let mut calc2: f64;
    let mut num_steps: i32 = 0;
    while diff > 0 {
        flag = rng.gen::<f64>();
        x = rng.gen_range(0..(arr_size));
        y = rng.gen_range(0..(arr_size));

        // println!("x: {}; y: {}", x, y);

        calc1 = pos_grid.ising_calc(x, y, beta, arr_size);
        calc2 = neg_grid.ising_calc(x, y, beta, arr_size);
        // diff += pos_grid.couple_step(neg_grid, rand_num, pos_grid.ising_calc(x, y, beta), neg_grid.ising_calc(x, y, beta), x, y);
    
        if flag <= calc1 && flag <= calc2 {
            if pos_grid.elements[(arr_size*x+y) as usize] == neg_grid.elements[(arr_size*x+y) as usize] {
                pos_grid.elements[(arr_size*x+y) as usize] = true;
                neg_grid.elements[(arr_size*x+y) as usize] = true;
            }
            else {
                pos_grid.elements[(arr_size*x+y) as usize] = true;
                neg_grid.elements[(arr_size*x+y) as usize] = true;
                diff -= 1;
            }
        }
        else if flag > calc1 && flag > calc2 {
            if pos_grid.elements[(arr_size*x+y) as usize] == neg_grid.elements[(arr_size*x+y) as usize] {
                pos_grid.elements[(arr_size*x+y) as usize] = false;
                neg_grid.elements[(arr_size*x+y) as usize] = false;
            }
            else {
                pos_grid.elements[(arr_size*x+y) as usize] = false;
                neg_grid.elements[(arr_size*x+y) as usize] = false;
                diff -= 1;
            }
        }
        else if flag <= calc1 && flag > calc2 {
            if pos_grid.elements[(arr_size*x+y) as usize] == neg_grid.elements[(arr_size*x+y) as usize] {
                pos_grid.elements[(arr_size*x+y) as usize] = true;
                neg_grid.elements[(arr_size*x+y) as usize] = false;
                diff += 1;
            }
            else {
                pos_grid.elements[(arr_size*x+y) as usize] = true;
                neg_grid.elements[(arr_size*x+y) as usize] = false;
            }
        }
        else {
            if pos_grid.elements[(arr_size*x+y) as usize] == neg_grid.elements[(arr_size*x+y) as usize] {
                pos_grid.elements[(arr_size*x+y) as usize] = false;
                neg_grid.elements[(arr_size*x+y) as usize] = true;
                diff += 1;
            }
            else {
                pos_grid.elements[(arr_size*x+y) as usize] = false;
                neg_grid.elements[(arr_size*x+y) as usize] = true;
            }
        }
        num_steps += 1;
        // println!("diff: {}", diff);
    }

    println!("{}", num_steps);

}
