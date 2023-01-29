fn main() {
    let mut sum : i64 = 0;
    
    let result = 'label_loop: loop {
        if sum > 100 {
            break sum * 2;
        }
        sum += 1;
    };
    
    println!("{}", sum);
    println!("{}", result);
}