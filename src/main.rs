mod entity;
mod helper;
mod cmd;

use std::env;
use helper::*;
use crate::cmd::{init, remove, start, status, stop};

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
        "start" => start(args),
        "status" => status(args),
        "stop" => stop(args),
        "remove" | "rm" => remove(args),
        "version" => println!("{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE")),
        "help" => {
            print_start();
        }
        _ => {
            println!("未知操作 {}", action);
            print_common();
        }
    }
}