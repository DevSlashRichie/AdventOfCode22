

pub fn quicksort<T: Eq + PartialEq + Clone + PartialOrd>(arr: &mut Vec<T>, low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high, &|a, b| {a <= b});
        quicksort(arr, low, p-1);
        quicksort(arr, p+1, high);
    }
}
fn partition<T: Clone, F: Fn(&T, &T) -> bool>(arr: &mut Vec<T>, low: usize, high: usize, f: &F) -> usize {
    let pivot = match arr.get(high) {
        Some(v) => {v.clone()}
        _ => {panic!("Array index {:?} out of bounds", high)}
    };
    let mut i = low;
    for j in low..high-1 {
        match arr.to_vec().get(j) {
            Some(v) => {
                if f(v, &pivot) {
                    &arr.swap(i, j);
                    i += 1;
                }
            }
            _ => {panic!("Array index {:?} for j out of bounds", j)}
        }
    }
    &arr.swap(i, high);
    i
}
