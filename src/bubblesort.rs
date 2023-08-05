pub fn bubble_sort(values: &mut Vec<i32>) {
    loop {
        let mut swapped = false;
        for i in 1..values.len() {    
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }
        if ! swapped {
            break;
        }
    }
}
