struct Executor {}

impl Executor {
    fn new() -> Self {
        Executor{}
    }
    fn insertion_sort(self, arr:&mut [i32]) {
        for i in 0..arr.len() {
            let temp = arr[i];
            let mut target = i;
            for j in arr.len()-i..arr.len() {
                if arr[arr.len()-j-1] > temp {
                    arr[target] = arr[arr.len()-j-1];
                    target = arr.len()-j-1;
                } else {
                    break;
                }
            }
            arr[target] = temp;
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
    executor.insertion_sort(&mut arr);
    print!("\nafter sort  : ");
    for num in &arr {
        print!("{} ", num);
    }
    // before sort : 77 2 17 8 1 12 5 3 7 11 25 
    // after sort  : 1 2 3 5 7 8 11 12 17 25 77
}
