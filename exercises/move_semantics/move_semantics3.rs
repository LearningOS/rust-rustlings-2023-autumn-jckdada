// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let vec1 = fill_vec(vec0.clone());

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let mut vec0 = vec0;
    vec0.push(22);
    vec0.push(44);
    vec0.push(66);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
