fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn main() {
    println!("测试简单语法函数");
    let x: i32 = 3;
    let y: i32 = 5;
    let z: i32 = add(x, y); // Remove the unused variable declaration and assignment
    println!("{} + {} = {}", x, y, z);
}
