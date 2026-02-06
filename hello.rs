fn main() {
	
	//doesnt work because it doesnt work like let, const are always immutable
	const X: u32  = 5;
	{
		X = X * 2;
		println!("Value of x in the inner scope: {X}");
	}
	println!("Value of x in the inner scope: {X}");
	
	//shadowing is nice to have to remove redundancies in the program
	let x = 3;
	{
		let x = x * 2;
		println!("Value of x in the inner scope: {x}");
	}
	println!("Value of x in the outer scope: {x}");


	
		
	

}
