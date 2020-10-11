/// default compare function
fn compare(a: i32, b: i32) -> i32 {
    a - b
}

/// Internal function for heap to fix itself to conform to heap definition.
/// Precondition: all elements below 'i' are in heap order.
fn sink(a: &mut [i32], i: usize, n: usize) {
    let mut root = i; // Get the root.
    let mut placed = false; // By default, 2 comparing elements are not placed in heap order.
    let mut child = 2 * root + 1; // Get the left child.
    while !placed && child < n {
        if child < n - 1 && compare(a[child], a[(child + 1)]) < 0 {
            // Right child exists and is greater.
            child += 1;
        }
        if compare(a[root], a[child]) >= 0 {
            // if 'child' is lesser than 'root' - it is already placed.
            placed = true;
        } else {
            // if 'child' is greater than 'root' - swap them.
            a.swap(root, child);
        }
        root = child;
        child = 2 * root + 1; // Re-inserting default value.
    }
}

/// Heapsort
pub fn heapsort(a: &mut [i32]) {
    let mut n = a.len();
    let mut i = ((n - 1) / 2) as i32;
    // first phase: building a valid max-heap.
    while i > -1 {
        println!("{:?}", a);
        sink(a, i as usize, n);
        i -= 1;
    }
    // second phase: sorting.
    // Iteratively sift down unsorted part (the heap).
    println!("Sorting:");
    while n > 0 {
        println!("{:?}", a);
        a.swap(0, n - 1);
        n -= 1;
        sink(a, 0, n);
    }
}

#[cfg(test)]
mod tests {

    use super::heapsort;

    #[test]
    /// Heapsort test.
    /// Test is passed if the unsorted array is correctly sorted.
    fn heapsort_test() {
        let mut array = [77, 89, 74, 68, 70, 49, 5, 62, 51];
        heapsort(&mut array);
        assert_eq!(array, [5, 49, 51, 62, 68, 70, 74, 77, 89]); // if the right statement is equal to the left statement - test passes
    }
}
