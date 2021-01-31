fn main() {
    println!("パスカルのなんとか");
    let h = 15;
    // 段データを保管する
    let mut vec = Vec::with_capacity(h);

    for i in 0..h { 
        let mut row: Vec<i32> = Vec::with_capacity(i+1);
        for j in 0..i+1 {
            // 最終列の場合は1を追加して終了
            if j == i {
                row.push(1);
                println!("{} ", 1);
                vec.push(row);
                break;
            }
            // 最初列の場合をプッシュする
            if j == 0 {
                row.push(1);
                print!("{} ", 1);
            }
            // それ以外
            else{
                let num = vec[i-1][j] + vec[i-1][j-1];
                row.push(num);
                print!("{} ", num);
            }
        }
    }
}