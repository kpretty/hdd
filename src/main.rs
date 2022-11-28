mod entity;
mod helper;
mod cmd;

use std::env;
use helper::*;
use crate::cmd::init;

fn main() {
    // 获取命令行参数
    let mut args: Vec<String> = env::args().collect();
    // ./hdd init dev -nn 1 -dn 3 -rm 1 -nm 3 -2nn 1 -jh 1
    // ["./hdd", "init", "dev", "-nn", "1", "-dn", "3", "-rm", "1", "-nm", "3", "-2nn", "1", "-jh", "1"]
    // 第一个参数为脚本名 不要
    args.remove(0);
    if args.len() <= 0 {
        print_common();
        return;
    }
    // ["init", "dev", "-nn", "1", "-dn", "3", "-rm", "1", "-nm", "3", "-2nn", "1", "-jh", "1"]
    // 获取 action
    let action: String = args.remove(0);
    match action.trim() {
        "init" => init(args),
        "list" | "ls" => {}
        "start" => {}
        "stop" => {}
        "remove" | "rm" => {}
        "version" => {}
        "help" => {
            print_start();
        }
        _ => {
            println!("未知操作 {}", action);
            print_common();
        }
    }
}