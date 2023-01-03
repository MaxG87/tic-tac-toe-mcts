mod referee;

fn main() {
    let mut foo = vec![1, 2, 3];
    let _immut_ref = &foo;
    let mut _mut_ref = &mut foo;
    let mut _mut_ref2 = &mut foo;
    println!("Hello, world!");
}
