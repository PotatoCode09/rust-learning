fn main() {
    let x = 6;
    println!("the {x}th fibonacci sequence is: {}", fibo(x));    
}

fn fibo(num: i32) -> i32 {
    let mut temp1 = 0;
    let mut temp2 = 1;

    for _ in 0..num {
        let next = temp1 + temp2;
        temp1 = temp2;
        temp2 = next;
    }
    
    temp1
}
