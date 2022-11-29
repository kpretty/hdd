// ----------------------------------------------->
// init

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, io, process};
use std::ffi::OsString;
use std::fs::read_dir;
use std::io::ErrorKind;
use crate::helper::print_red;

// 直接接管传入参数的所有权
// note:接不接管所有权都一样
pub fn init(mut args: Vec<String>) {
    // 第一个参数为stack名
    let stack = args.remove(0);
    // 修改：应该先校验参数，参数没问题再去创建相对应的文件夹
    // step-1 校验参数
    let args = check_args(args);
    // step-2 检查stack是否存在
    let stack_path = stack_exist(&stack);
    // step-3 生成docker-compose文件
    build_compose(stack_path, args)
}

#[allow(deprecated)]
/// 检查 $HOME/.hdd/{stack} 是否存在
/// 1. 检查项目空间是否已创建 $HOME/.hdd，不存在则创建
/// 2. 检查stack是否已创建，存在停止运行(stack重名)，不存在创建
fn stack_exist(stack: &String) -> PathBuf {
    let path = env::home_dir().unwrap().join(Path::new(".hdd"));
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
        println!("创建stack：{}，本地路径：{:?}", stack, stack_path);
        std::fs::create_dir_all(&stack_path).unwrap();
        // 拷贝文件夹
        for dir in vec!["init", "env"] {
            let src = get_project_root().unwrap().join(Path::new(dir));
            let dest = stack_path.join(Path::new(dir));
            copy_dir(&src, &dest)
        }
    }
    stack_path
}

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo =
            read_dir(p)?
                .into_iter()
                .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
}

// 封装递归拷贝文件逻辑
fn copy_dir(src: &PathBuf, dest: &PathBuf) {
    // 创建必要的文件夹
    std::fs::create_dir_all(&dest).unwrap();
    // 递归复制文件
    for entry in src.read_dir().expect(format!("目录或文件不存在:{:?}", src).trim()) {
        let entry = entry.unwrap().path();
        if entry.is_file() {
            println!("拷贝依赖文件：{:?} -> {:?}", &entry, &dest);
            std::fs::copy(&entry, &dest.join(&entry.file_name().unwrap())).unwrap();
        }
    }
}

/// 校验参数
/// 1. 校验参数是否对齐，即参数个数必须是偶数
/// 2. -nn和-rm至少存在一个，且个数必须为1(HA暂时没有做，hdfs的ha初始化比较复杂，后续再考虑)
/// 3. -2nn和-jh 可有可无，不做校验
/// 4. 所有的value必须>=0(u32 默认保证了)
fn check_args(args: Vec<String>) -> HashMap<String, u32> {
    if args.len() % 2 != 0 {
        // 参数不对齐
        println!("参数不对齐，请检查参数：{:?}", args);
        process::exit(1);
    }
    let mut param: HashMap<String, u32> = HashMap::new();
    let mut index = 0;
    loop {
        if index >= args.len() {
            break;
        }
        let key = &args[index];
        let value: u32 = args[index + 1].parse().expect("节点个数存在非正整数");
        param.insert(key.to_owned(), value);
        index += 2;
    }
    // step-1 nn和rm至少要有一个
    if !param.contains_key("-nn") && !param.contains_key("-rm") {
        print_red("namenode或resourcemanager需要至少存在一个 ".to_string());
        process::exit(1);
    }
    // step-2 有worker节点但没有master节点
    if (!param.contains_key("-nn") && *param.get("-dn").unwrap() > 0) || (!param.contains_key("-rm") && *param.get("-nm").unwrap() > 0) {
        print_red("worker节点缺少master节点管理 ".to_string());
        process::exit(1);
    }
    // step-3 检查高可用
    if (param.contains_key("-nn") && *param.get("-nn").unwrap() > 1) ||
        (!param.contains_key("-rm") && *param.get("-rm").unwrap() > 1) ||
        (!param.contains_key("-jh") && *param.get("-jh").unwrap() > 1) ||
        (!param.contains_key("-2nn") && *param.get("-2nn").unwrap() > 1) {
        print_red("当前版本暂不支持HA，请确保namenode|resourcemanager|jobhistory|secondarynamenode个数为1".to_string());
        process::exit(1);
    }
    // ...
    param
}

fn build_compose(stack: PathBuf, param: HashMap<String, u32>) {
    use crate::entity::{Server, InnerServer};
    let mut services: HashMap<String, InnerServer> = HashMap::new();
    // namenode
    match param.get("-nn") {
        None => {}
        Some(_) => {
            let nn = InnerServer {
                env_file: vec![],
                image: "hdd/hadoop-base".to_string(),
                hostname: "namenode".to_string(),
                container_name: "namenode".to_string(),
                volumes: vec!["9870:9870".to_string()],
                ports: vec!["9870:9870".to_string()],
                command: vec!["sh".to_string(), "/run-server.sh".to_string(), "nn".to_string()],
            };
            services.insert("namenode".to_string(), nn);
        }
    }
    // datanode
    match param.get("-dn") {
        None => {}
        Some(value) => {
            for i in 0..*value {
                let nn = InnerServer {
                    env_file: vec![],
                    image: "".to_string(),
                    hostname: "".to_string(),
                    container_name: "".to_string(),
                    volumes: vec![],
                    ports: vec![],
                    command: vec![],
                };
                services.insert(format!("{}-{}", "namenode", i).to_string(), nn);
            }
        }
    }
    // secondarynamenode
    match param.get("-2nn") {
        None => {}
        Some(_) => {
            let nn = InnerServer {
                env_file: vec![],
                image: "".to_string(),
                hostname: "".to_string(),
                container_name: "".to_string(),
                volumes: vec![],
                ports: vec![],
                command: vec![],
            };
            services.insert("namenode".to_string(), nn);
        }
    }
    // resourcemanager
    match param.get("-rm") {
        None => {}
        Some(_) => {
            let nn = InnerServer {
                env_file: vec![],
                image: "".to_string(),
                hostname: "".to_string(),
                container_name: "".to_string(),
                volumes: vec![],
                ports: vec![],
                command: vec![],
            };
            services.insert("namenode".to_string(), nn);
        }
    }
    // nodemanager
    match param.get("-nm") {
        None => {}
        Some(_) => {
            let nn = InnerServer {
                env_file: vec![],
                image: "".to_string(),
                hostname: "".to_string(),
                container_name: "".to_string(),
                volumes: vec![],
                ports: vec![],
                command: vec![],
            };
            services.insert("namenode".to_string(), nn);
        }
    }
    // jobhistory
    match param.get("-jh") {
        None => {}
        Some(_) => {
            let nn = InnerServer {
                env_file: vec![],
                image: "".to_string(),
                hostname: "".to_string(),
                container_name: "".to_string(),
                volumes: vec![],
                ports: vec![],
                command: vec![],
            };
            services.insert("namenode".to_string(), nn);
        }
    }
    let _server = Server {
        version: "3.0".to_string(),
        services,
    };
}
// <-----------------------------------------------