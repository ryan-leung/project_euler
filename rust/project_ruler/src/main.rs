fn main(){
    use std::io;
    let mut stdin = io::stdin();
    let mut line = String::new();
    let mut n : i32 = 0;
	while true {
		stdin.read_line(&mut line)
            .ok()
            .expect("Failed to read line");
        n = line.trim().parse::<i32>().unwrap();
        if n == 42 {break;}
        line.clear();
		println!("{}", n);
	}
}