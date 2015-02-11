fn linear_search(items: &[i32], target: &i32) -> i32 {

    for (idx, item) in items.iter().enumerate() {
        if item == target {
            return idx as i32;
        }
    }

    -1
}

fn main() {
    let items: [i32; 6] = [1, 3, 5, 7, 12, 454];
    let target: i32 = 12;

    println!("Result: {}", linear_search(&items, &target));
}
