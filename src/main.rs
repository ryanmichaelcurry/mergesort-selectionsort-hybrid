use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

// Merge Sort Code from: https://dev.to/felixfaisal/implementing-merge-sort-in-rust-4ko8
fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let n1 = mid - left;
    let n2 = right - mid;
    let l1 = arr.clone();
    let r1 = arr.clone();
    let l = &l1[left..mid];
    let r = &r1[mid..right];
    /* Merge the temp arrays back into arr[l..r]*/
    let mut i = 0; // Initial index of first subarray
    let mut j = 0; // Initial index of second subarray
    let mut k = left; // Initial index of merged subarray
    while i < n1 && j < n2 {
        if l[i] < r[j] {
            arr[k] = l[i];
            i = i + 1;
        } else {
            arr[k] = r[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = l[i];
        i = i + 1;
        k = k + 1;
    }
    /* Copy the remaining elements of R[], if there
    are any */
    while j < n2 {
        arr[k] = r[j];
        j = j + 1;
        k = k + 1;
    }

    return arr;
}

fn merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    return arr;
}

fn selection_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    for i in 0..len {
        let mut min_index = i;

        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

fn hybrid_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - left <= 150 {
        // Crossover point
        selection_sort(&mut arr[left..right].to_vec());
    } else if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = hybrid_sort(arr, left, mid);
        arr = hybrid_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    return arr;
}

fn generate_and_sort_arrays(size: usize) -> (u128, u128, u128) {
    let mut rng = rand::thread_rng();
    let mut arr_merge = vec![];
    let mut arr_selection = vec![];
    let mut arr_hybrid = vec![];

    // Generate random arrays with 'size' elements
    for _ in 0..size {
        let num = rng.gen_range(1..100000); // Adjust the range as needed
        arr_merge.push(num);
        arr_selection.push(num);
        arr_hybrid.push(num);
    }

    // Measure the time taken to sort the arrays using merge sort
    let merge_start_time = Instant::now();
    merge_sort(arr_merge, 0, size);
    let merge_elapsed_time = merge_start_time.elapsed().as_nanos();

    // Measure the time taken to sort the arrays using selection sort
    let selection_start_time = Instant::now();
    selection_sort(&mut arr_selection);
    let selection_elapsed_time = selection_start_time.elapsed().as_nanos();

    // Measure the time taken to sort the arrays using my hybrid sort
    let hybrid_start_time = Instant::now();
    hybrid_sort(arr_hybrid, 0, size);
    let hybrid_elapsed_time = hybrid_start_time.elapsed().as_nanos();

    return (merge_elapsed_time, selection_elapsed_time, hybrid_elapsed_time);
}

fn main() {
    let array_sizes = vec![5, 10, 25, 50, 75, 100, 115, 125, 150, 175, 250, 500, 1000, 2500, 5000, 7500, 10000];
    let mut merge_results = String::new();
    let mut selection_results = String::new();
    let mut hybrid_results = String::new();

    for size in &array_sizes {
        let (merge_elapsed_time, selection_elapsed_time, hybrid_elapsed_time) = generate_and_sort_arrays(*size);
        println!("Size: {}, Hybrid Sort: {} nanoseconds, Merge Sort: {} nanoseconds, Selection Sort: {} nanoseconds", size, hybrid_elapsed_time, merge_elapsed_time, selection_elapsed_time);

        // Append the results to the respective strings
        merge_results.push_str(&format!("Size: {}, Time: {} nanoseconds\n", size, merge_elapsed_time));
        selection_results.push_str(&format!("Size: {}, Time: {} nanoseconds\n", size, selection_elapsed_time));
        hybrid_results.push_str(&format!("Size: {}, Time: {} nanoseconds\n", size, hybrid_elapsed_time));
    }

    // Save merge sort results to a file
    let mut merge_file = File::create("merge_sort_times.txt").expect("Unable to create merge sort file");
    write!(merge_file, "{}", merge_results).expect("Unable to write to merge sort file");

    // Save selection sort results to a file
    let mut selection_file = File::create("selection_sort_times.txt").expect("Unable to create selection sort file");
    write!(selection_file, "{}", selection_results).expect("Unable to write to selection sort file");

    // Save hybrid sort results to a file
    let mut hybrid_file = File::create("hybrid_sort_times.txt").expect("Unable to create hybrid sort file");
    write!(hybrid_file, "{}", hybrid_results).expect("Unable to write to selection sort file");
}
