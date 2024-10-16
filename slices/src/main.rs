fn main() {
    
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
    
    // The reference to the slice below is called a fat pointer.
    let sv: &[i32] = &v;
    println!("{:?}", sv);
}
