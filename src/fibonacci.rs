pub fn fibonacci_f(n: u8){
    let prev = 0;
    let mut res: u8 = 0;
    let mut curr = 1;

    for i in 1..n {
        res += prev + curr;
        curr = res;
        
        if i == n {
            break;
        }

    }

    println!("{res}");
    
}