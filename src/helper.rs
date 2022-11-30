// ----------------------------------------------->
// 控制台颜色打印
const REST: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

#[allow(dead_code)]
pub enum Color {
    RED,
    GREEN,
    YELLOW,
    BLUE,
}

#[allow(dead_code)]
fn print_color(color_type: &Color, message: &str) {
    match color_type {
        Color::RED => println!("{}{}{}", RED, message, REST),
        Color::GREEN => println!("{}{}{}", GREEN, message, REST),
        Color::YELLOW => println!("{}{}{}", YELLOW, message, REST),
        Color::BLUE => println!("{}{}{}", BLUE, message, REST),
    }
}

#[allow(dead_code)]
pub(crate) fn print_red(message: String) {
    print!("{}{}{}", RED, message, REST)
}

#[allow(dead_code)]
fn print_green(message: String) {
    print!("{}{}{}", GREEN, message, REST)
}

#[allow(dead_code)]
pub(crate) fn print_yellow(message: String) {
    print!("{}{}{}", YELLOW, message, REST)
}

#[allow(dead_code)]
fn print_blue(message: String) {
    print!("{}{}{}", BLUE, message, REST)
}
// <-----------------------------------------------


// ----------------------------------------------->
// 产品相关信息
pub fn print_logo() {
    let logo_up: String = format!("{}{}{}{}",
                                  "      ___           ___           ___     \n",
                                  "     /\\__\\         /\\  \\         /\\  \\\n",
                                  "    /:/  /        /::\\  \\       /::\\  \\\n",
                                  "   /:/__/        /:/\\:\\  \\     /:/\\:\\  \\ \n"
    );

    let logo_mid: String = format!("{}{}{}{}",
                                   "  /::\\  \\ ___   /:/  \\:\\__\\   /:/  \\:\\__\\\n",
                                   " /:/\\:\\  /\\__\\ /:/__/ \\:|__| /:/__/ \\:|__|\n",
                                   " \\/__\\:\\/:/  / \\:\\  \\ /:/  / \\:\\  \\ /:/  /\n",
                                   "      \\::/  /   \\:\\  /:/  /   \\:\\  /:/  /  \n");

    let logo_down: String = format!("{}{}{}",
                                    "      /:/  /     \\:\\/:/  /     \\:\\/:/  /\n",
                                    "     /:/  /       \\::/__/       \\::/__/\n",
                                    "     \\/__/         ~~            ~~\n");
    print_red(logo_up);
    print_yellow(logo_mid);
    print_green(logo_down);
}

pub fn print_desc() {
    let hdd_desc = "HDD CLI is a developer tool used to manage local development stacks\n\n\
    This tool automates creation of stacks with many infrastructure components which\n\
    would otherwise be a time consuming manual task. It also wraps docker compose\n\
    commands to manage the lifecycle of stacks.\n\n";
    println!("{}", hdd_desc);
}

pub fn print_start() {
    let hdd_start = "To get started run: hdd init\n\n\
    Usage:\n  hdd [command]\n\n\
    Available Commands:\n \
      help        帮助命令\n \
      info        查看stack详细信息[未完成]\n \
      init        初始化一个stack\n \
      list        查看所有stack\n \
      logs        查看某个stack日志信息[未完成]\n \
      ls          查看所有stack\n \
      remove      移除stack\n \
      start       启动stack\n \
      stop        停止stack\n \
      status      查看stack状态 \
      version     打印版本信息\n";
    println!("{}", hdd_start);
}

pub fn print_common() {
    print_logo();
    print_desc();
    print_start();
}
// <-----------------------------------------------