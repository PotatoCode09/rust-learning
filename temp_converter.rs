    fn main(){
    let temp_cel = 32.0;
    let temp_fahr = 25.0;

    println!("{temp_fahr} degrees fahrenheit is {:.2} degrees celsius", fahr_to_cel(temp_fahr));

    println!("{temp_cel} degrees celsius is {:.2} fahrenheit", cel_to_fahr(temp_cel));
}

fn cel_to_fahr(cel: f64) -> f64 {
    cel * (9.0/5.0) + 32.0
}

fn fahr_to_cel(fahr: f64) -> f64 {
    (fahr - 32.0) * 5.0/9.0
}
