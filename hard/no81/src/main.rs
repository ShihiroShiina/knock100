use std::io;

fn main() {
    println!("数字を３つ入力して下さい");
    let mut arr = Vec::new();

    for i in 0..3 {
        println!("{}つ目: ", i+1);
        let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("Failed to read line");
        
        let num: i32 = num.trim().parse().unwrap();
        arr.push(num);
    }

    arr.sort();
    println!("中間値は {} です", arr[1]);
}
