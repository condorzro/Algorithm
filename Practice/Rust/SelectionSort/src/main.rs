struct Executor {}

impl Executor {
    fn new() -> Self {
        Executor{}
    }
    fn selection_sort(self, arr:&mut [i32]) {
        for i in 0..arr.len()-1 {
            let mut min = i+1;
            for j in i+1..arr.len() {
                if arr[min] > arr[j] {
                    min = j
                }
            }
            if arr[i] > arr[min] {
                let temp = arr[min];
                arr[min] = arr[i];
                arr[i] = temp;
            }
        }
    }
}

fn main() {
    let executor = Executor::new();
    let mut arr = [77, 2, 17, 8, 1, 12, 5, 3, 7, 11, 25];
    print!("before sort : ");
    for num in &arr {
        print!("{} ", num);
    }
    executor.selection_sort(&mut arr);
    print!("\nafter sort  : ");
    for num in &arr {
        print!("{} ", num);
    }
}
