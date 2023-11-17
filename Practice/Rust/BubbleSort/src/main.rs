struct Executor {}

impl Executor {
    pub fn new() -> Self {
        Executor{}
    }
    pub fn bubble_sort(self, arr:&mut Vec<i32>) {
        for i in 0..arr.len()-1 {
            for j in 0..arr.len()-i-1 {
                if (*arr)[j] > (*arr)[j+1] {
                    let temp = (*arr)[j];
                    (*arr)[j] = (*arr)[j+1];
                    (*arr)[j+1] = temp;
                }
            }
        }
    }
}

fn main() {
    let executor = Executor::new();
    let mut v1 = vec![2, 17, 8, 12, 5, 3, 7, 11];
    print!("before sort : ");
    for n in &v1 {
        print!("{} ", n);
    }
    println!();
    executor.bubble_sort(&mut v1);
    print!("after sort : ");
    for n in &v1 {
        print!("{} ", n);
    }
}
