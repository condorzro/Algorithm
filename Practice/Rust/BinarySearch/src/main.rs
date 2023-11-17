struct Executor {}

impl Executor {
    fn new() -> Self {
        Executor{}
    }
    fn binary_search(self, arr:&[i32], target:&i32) -> bool {
        if arr.len() < 1 {
            false
        } else {
            let midx = (arr.len() as f32/2.0_f32) as usize;
            let middle = arr[midx];
            if *target == middle {
                true
            } else if arr.len() == 1 { 
                false
            } else if *target > middle {
                self.binary_search(&arr[midx+1..], target)
            } else if *target < middle {
                self.binary_search(&arr[..midx], target)
            } else {
                false
            }
        }
    }    
}

fn main() {
    let executor = Executor::new();
    let target = 6_i32;
    let mut arr = vec![2, 17, 8, 12, 5, 3, 7, 11];
    arr.sort();
    for n in &arr {
        print!("{} ", n);
    }
    println!("\nfind {}", target);
    let result = executor.binary_search(&arr, &target);
    println!("result is {}", result);
}
