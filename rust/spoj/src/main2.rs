// n [the number of multiplications ≤ 1000]
// l1 l2 [numbers to multiply (at most 10000 decimal digits each)]

fn main(){
    use std::io;
    let mut stdin = io::stdin();
    let mut line = String::new();
    let mut n : i32 = 0;
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    stdin.read_line(&mut line)
            .ok()
            .expect("Failed to read line");
    n = line.trim().parse::<i32>().unwrap();
    line.clear();

	for i in 0..n {
		stdin.read_line(&mut line)
            .ok()
            .expect("Failed to read line");
        let l: Vec<&str> = line.split(" ").collect();
        x = l[0].trim().parse::<i32>().unwrap();
        y = l[1].trim().parse::<i32>().unwrap();
        line.clear();
		println!("{}", x * y);
	}
}