// ----------------------------------------------->
// init

use std::path::Path;
use std::process;

// 直接接管传入参数的所有权
// note:接不接管所有权都一样
pub fn init(mut args: Vec<String>) {
    // 第一个参数为stack名
    let stack = args.remove(0);
    // step-1 检查stack是否存在
    stack_exist(&stack);
    // step-2 校验参数

    // step-3 生成docker-compose文件
}

#[allow(deprecated)]
fn stack_exist(stack: &String) {
    let path = std::env::home_dir().unwrap().join(Path::new(".hdd"));
    // 校验项目根目录是否存在
    if !path.exists() {
        // notice: create_dir_all 会产生所有权的移交，注意使用借用
        println!("初始化项目空间:{:?}", path);
        std::fs::create_dir_all(&path).unwrap();
    }
    // 拼接stack路径
    let stack_path = path.join(Path::new(stack));
    if stack_path.exists() {
        // 文件夹存在，则无法执行init操作，停止程序
        println!("stack:{}已经存在", stack);
        process::exit(1);
    } else {
        // 不存在则创建
        std::fs::create_dir_all(&stack_path).unwrap();
    }
}
// <-----------------------------------------------