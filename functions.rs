fn main() {
	for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    //another_func(2);
	//measurements(5, 'c');
	//let x = five();
    //println!("The value of x = {x}");
    //println!("The value of 5 + 1: {}", plus_one(5));
}


fn another_func(x: i32){
	println!("the value of x is {x}");
}

fn measurements(length: i32, unit_label: char){
	println!("The measurement is: {length}{unit_label}");
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
