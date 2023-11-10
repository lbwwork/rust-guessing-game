use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字游戏！");
    //用于生成1到100的随机数
    let num = rand::thread_rng().gen_range(1..=100);
    //循环
    loop {
        println!("请输入您猜的数字！");
        let mut guess = String::new();
        //读取用户输入的内容 因为此方法是直接将输入的内容追加到参数guess后，所以每次循环都需要new guess对象
        io::stdin()
            .read_line(&mut guess)
            .expect("无法获取到输入的数据!");
        //将输入的内容转换为数字
        let guess: u32 = match guess.trim().parse(){
            //如果输入的内容都为数字可以转换为数字，走ok逻辑，直接返回数字
            Ok(num) => num,
            //如果输入的内容有转换不出去的，走判断逻辑
            Err(_) => {
                //判断输入的是否为quit,如果是，直接break出循环，结束程序
                let quit = "quit";
                match guess.trim().cmp(&quit) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        break
                    }
                    Ordering::Greater => {}
                }
                continue;
            }
        };
        println!("你输入的数字为:{guess}");
        match guess.cmp(&num) {
            Ordering::Less => {println!("太小了!")}
            Ordering::Equal => {
                println!("你猜对了！");
                break
            }
            Ordering::Greater => {println!("太大了!")}
        }
    }
}
