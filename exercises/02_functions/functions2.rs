// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    for i in 0..num {
        let j = i + 1;
        println!("Ring! Call number {j}");
    }
}

fn main() {
    call_me(3);
}
