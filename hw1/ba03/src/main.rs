use std::env;

fn main() {
    let mut input_args: Vec<String> = env::args().skip(1).collect();

    //input_args.sort();

    //insertion_sort(&mut input_args);
    quick_sort(&mut input_args);

    println!("Sorted arguments: {:?}", input_args);
}


fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {        
        let mut j = i - 1;
        
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}


fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);

    
    let (left, right) = arr.split_at_mut(pivot_index);
    
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    
    let pivot_index = arr.len() / 2;
    let last_idx = arr.len() - 1;
    arr.swap(pivot_index, last_idx);
    let mut i = 0;
    for j in 0..last_idx {
        if arr[j] <= arr[last_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, last_idx);
    i
}