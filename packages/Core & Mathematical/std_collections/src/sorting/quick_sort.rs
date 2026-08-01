// packages/Core & Mathematical/std_collections/src/sorting/quick_sort.rs

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quick_sort(left);
    quick_sort(&mut right[1..]); // Skip the pivot
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 12; // Simple pivot selection
    arr.swap(pivot_index, len - 1);
    
    let mut store_index = 0;
    for i in 0..len - 1 {
        if arr[i] <= arr[len - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, len - 1);
    store_index
}