use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, io, process};
use std::ffi::OsString;
use std::fs::{File, read_dir};
use std::io::{ErrorKind, Write};
use crate::helper::print_red;

// ----------------------------------------------->
// init
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
    build_compose(&stack_path, args)
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

// 构建docker-compose文件
fn build_compose(stack: &PathBuf, param: HashMap<String, u32>) {
    use crate::entity::{Server, InnerServer};
    // 固定参数
    let image = "hdd/hadoop-base".to_string();
    let volume_path = stack.join(Path::new("init"));
    let volumes = vec![
        format!("{}:{}", volume_path.join("core-site.xml").into_os_string().into_string().unwrap(), "/opt/hadoop/etc/hadoop/core-site.xml"),
        format!("{}:{}", volume_path.join("hdfs-site.xml").into_os_string().into_string().unwrap(), "/opt/hadoop/etc/hadoop/hdfs-site.xml"),
        format!("{}:{}", volume_path.join("yarn-site.xml").into_os_string().into_string().unwrap(), "/opt/hadoop/etc/hadoop/yarn-site.xml"),
        format!("{}:{}", volume_path.join("mapred-site.xml").into_os_string().into_string().unwrap(), "/opt/hadoop/etc/hadoop/mapred-site.xml"),
        format!("{}:{}", volume_path.join("capacity-scheduler.xml").into_os_string().into_string().unwrap(), "/opt/hadoop/etc/hadoop/capacity-scheduler.xml"),
    ];
    let env_path = stack.join("env");
    let env_hdfs = vec![env_path.join("hdd-hdfs.env").into_os_string().into_string().unwrap()];
    let env_yarn = vec![env_path.join("hdd-yarn.env").into_os_string().into_string().unwrap()];
    let mut env_file = env_hdfs.to_owned();
    env_file.append(&mut env_yarn.to_owned());
    let base_command = vec!["sh".to_string(), "/run-server.sh".to_string()];
    // end
    let mut services: HashMap<String, InnerServer> = HashMap::new();
    // namenode
    match param.get("-nn") {
        None => {}
        Some(_) => {
            let mut command = base_command.to_owned();
            command.push("nn".to_string());
            let nn = InnerServer {
                env_file: env_hdfs.to_owned(),
                image: image.to_owned(),
                hostname: "namenode".to_string(),
                container_name: "namenode".to_string(),
                volumes: volumes.to_owned(),
                ports: vec!["9870:9870".to_string()],
                command,
            };
            services.insert("namenode".to_string(), nn);
        }
    }
    // datanode
    match param.get("-dn") {
        None => {}
        Some(value) => {
            let mut command = base_command.to_owned();
            command.push("dn".to_string());
            for i in 0..*value {
                let name = format!("{}-{}", "datanode", i).to_string();
                let dn = InnerServer {
                    env_file: env_hdfs.to_owned(),
                    image: image.to_owned(),
                    hostname: name.to_owned(),
                    container_name: name.to_owned(),
                    volumes: volumes.to_owned(),
                    ports: vec![],
                    command: command.to_owned(),
                };
                services.insert(name, dn);
            }
        }
    }
    // secondarynamenode
    match param.get("-2nn") {
        None => {}
        Some(_) => {
            let mut command = base_command.to_owned();
            command.push("2nn".to_string());
            let snn = InnerServer {
                env_file: env_file.to_owned(),
                image: image.to_owned(),
                hostname: "secondarynamenode".to_string(),
                container_name: "secondarynamenode".to_string(),
                volumes: volumes.to_owned(),
                ports: vec![],
                command,
            };
            services.insert("secondarynamenode".to_string(), snn);
        }
    }
    // resourcemanager
    match param.get("-rm") {
        None => {}
        Some(_) => {
            let mut command = base_command.to_owned();
            command.push("rm".to_string());
            let rm = InnerServer {
                env_file: env_yarn.to_owned(),
                image: image.to_owned(),
                hostname: "resourcemanager".to_string(),
                container_name: "resourcemanager".to_string(),
                volumes: volumes.to_owned(),
                ports: vec!["8088:8088".to_string()],
                command,
            };
            services.insert("resourcemanager".to_string(), rm);
        }
    }
    // nodemanager
    match param.get("-nm") {
        None => {}
        Some(value) => {
            let mut command = base_command.to_owned();
            command.push("nm".to_string());
            for i in 0..*value {
                let name = format!("{}-{}", "nodemanager", i).to_string();
                let nm = InnerServer {
                    env_file: env_yarn.to_owned(),
                    image: image.to_owned(),
                    hostname: name.to_owned(),
                    container_name: name.to_owned(),
                    volumes: volumes.to_owned(),
                    ports: vec![],
                    command: command.to_owned(),
                };
                services.insert(name.to_owned(), nm);
            }
        }
    }
    // jobhistory
    match param.get("-jh") {
        None => {}
        Some(_) => {
            let mut command = base_command.to_owned();
            command.push("jh".to_string());
            let jh = InnerServer {
                env_file: env_file.to_owned(),
                image: image.to_owned(),
                hostname: "jobhistory".to_string(),
                container_name: "jobhistory".to_string(),
                volumes: volumes.to_owned(),
                ports: vec![],
                command,
            };
            services.insert("jobhistory".to_string(), jh);
        }
    }
    let server = Server {
        version: "3.0".to_string(),
        services,
    };
    let result = serde_yaml::to_string(&server).unwrap();
    let mut file = File::create(stack.join("docker-compose.yml")).unwrap();
    file.write_all((&result).as_ref()).unwrap();
}

// 获取项目路径
fn get_project_root() -> io::Result<PathBuf> {
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
// <-----------------------------------------------