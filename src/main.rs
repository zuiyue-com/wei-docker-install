// [完成] 编写基础的UI框架
// 确认只存在一个程序进程，没有其它ai.exe

// 判断当前文件目录下面是否有docker_install.exe 和 ubuntu_install.tar 和 wsl_update.msi，其中任一个文件
// 如果有则把当前目录保存到AppData\\Local\\Ai的配置env文件，把当前程序重命名为ai-x86_64-pc-windows-msvc.exe
// 执行完毕之后重新运行ai-x86_64-pc-windows-msvc.exe

// 第一步，检测当前ai-x86_64-pc-windows-msvc.exe，则自动把自己复制进去AppData\\Local\\Ai目录，并创建桌面图标，以及自动启动

// 第二步，每次开机检查一下是否存在docker，如果存在就不执行下面的

// 第三步，进行安装流程

// 提升为管理员权限，自己注册成服务
// 隐藏自己的黑色框框
// 检测 wsl --set-default-version 2
// 以管理员模式运行powershell
// 安装 wsl 虚拟机开启
// 重启机器
// 安装wsl_update_x64.msi
// 设定盘符 用户选择
// 安装 wsl ubuntu
// 安装 wsl docker
// 拉取镜像 stable diffusion 或者本地安装 default.tar.gz

use std::thread;
use std::time::Duration;
use colored::*;


#[macro_use]
extern crate wei_log;


// rust把宏从journal引入到main.rs,也相当于把info给了全局
// 在install/windows.rs里面只需要引入 use crate::info即可
// use crate::info;

fn main() {
    println!("Hello, World");
}

pub async fn init() {

    // 编写 daemon.exe 守护进程，做为开机启动，接收到退出信息之后也跟着退出，
    // 根据 AppData\\Local\\Ai 里面的文件 start.dat 里面的值如果为1代表开启，daemon.exe会一起打开程序，如果为 0 代表关闭
    // ai.exe会把daemon.exe 注册成为服务。下次开机的时候会自动开启
    // 第一次启动的时候检测要安装图标到桌面，要把daemon.exe复制进AppData\\Local\\Ai.AppData\\Local\\Ai要移动到AppData里面
    // daemon.exe 开机启动，或者做为服务来

    // 第一步，检测当前ai-x86_64-pc-windows-msvc.exe，则自动把自己复制进去AppData\\Local\\Ai目录，并创建桌面图标，以及自动启动
    init_first();

    init_color();
println!("{}", "
 ╔═╗  ╦ ╦  ╦  ╦ ╦  ╦ ╦  ╔═╗  ╔═╗  ╔═╗  ╔╦╗
 ╔═╝  ║ ║  ║  ╚╦╝  ║ ║  ║╣   ║    ║ ║  ║║║
 ╚═╝  ╚═╝  ╩   ╩   ╚═╝  ╚═╝  ╚═╝  ╚═╝  ╩ ╩".green());
println!("{}", "
 欢迎使用我们的AI镜像一键安装包软件。我们的目标是为研究者和开发者提供
 最快捷，最方便的AI环境部署体验。".cyan());
println!("{}", "
 该软件包含以下特性:".red());
println!("{}", "
  1. 大模型支持: 我们提供最新的大模型，无论是在深度学习，机器学习，
     还是其他AI相关领域，我们都努力提供最新的模型支持。".green());
println!("{}", "
  2. 快速下载: 我们理解时间的重要性，特别是在AI开发中。因此，我们的
     软件提供快速的下载功能，让你能够在最短的时间内开始工作。".blue());
println!("{}", "
  3. 一键安装: 我们知道环境部署可能会非常复杂和耗时，因此我们的软件
     提供一键安装功能。只需一个按键，你就能获得一个完全配置好的，准
     备就绪的AI环境。".yellow());
println!("{}", "
  4. 适合所有用户: 无论你是一个经验丰富的研究者，还是一个AI初学者，
     我们的软件都能满足你的需求。我们的软件易于使用，同时也提供了强
     大的功能和灵活的配置选项。".magenta());
println!("{}", "
 我们坚信，这个软件将极大地简化你的AI开发工作，并加速你的研究和开发进
 度。感谢你选择我们的AI镜像一键安装包软件，期待能在你的AI之旅中提供帮助。".cyan());
println!("");
    println!(" 开始初始化系统...");

    if read_json("init_ubuntu") && read_json("init_docker") {
        for _ in 0..5 {

            let start_docker_bool = start_docker().await;

            if start_docker_bool {
                if !is_debug() {
                    hide();
                }
                return;
            }

            thread::sleep(Duration::from_secs(30));
        }
    }

    init_admin();

    if !read_json("init_wsl") {
        init_wsl();
    }

    if !read_json("init_first_reboot") {
        init_first_reboot();
    }

    if !read_json("init_wsl2") {
        init_wsl2();
    }

    if !read_json("init_wsl_update") {
        init_wsl_update();
    }

    let mut _drive_letter = "".to_string();

    // todo 如果检测到没有开启虚拟技术，则提示他进入bios去设置
    // 还需要做不同的bios 对应的教程

    // cd C:\Users\Wei\source\ai\
    // wsl --import Ubuntu-ai c:\Ubuntu-ai .\ubuntu.tar --version 2

    if !read_json("init_ubuntu") {
        if _drive_letter == "" {
            _drive_letter = init_drive_letter();
        }
        init_ubuntu(_drive_letter.clone());
    }

    if !read_json("init_docker") {
        if _drive_letter == "" {
            _drive_letter = init_drive_letter();
        }
        init_docker(_drive_letter.clone());

        for _ in 0..5 {
            let start_docker_bool = start_docker().await;
            if start_docker_bool {
                if !is_debug() {
                    hide();
                }
                return;
            }
            thread::sleep(Duration::from_secs(30));
        }
        
    }
    
    if !is_debug() {
        hide();
    }
}

pub fn start_daemon() {
    let dir = dirs::data_local_dir().unwrap().join("Ai");
    let exe_path = dir.join("daemon.exe");

    if !is_process_running("daemon.exe") {
        if exe_path.exists() {
            Command::new("powershell")
            .args(&["/C", "start", &format!("\"{}\"", exe_path.display())])
            .spawn().unwrap();
        }
    }
}

pub fn stop_daemon() {
    Command::new("taskkill")
    .arg("/F")
    .arg("/IM")
    .arg("daemon.exe")
    .output().unwrap();
}

async fn start_docker() -> bool {
    info_print!(" 正在启动Docker:     ");
    std::io::stdout().flush().unwrap();

    // 从注册表获取 docker 位置，把安装流程重置一下
    let docker_exe_path = docker_start_exe();
    if docker_exe_path == "" {
        return false;
    }
    // 判断 docker 执行文件是否存在，把安装流程重置一下
    let path = Path::new(&docker_exe_path);
    if !path.is_file() {
        write_json("init_ubuntu", true);
        write_json("init_docker", true);
        println!(" {}", "失败，原因：docker启动程序不存在".to_string().red());
        return false;
    }

    // 判断 进程里面是否存在 docker desktop.exe 如果进程里面没有则 启动 docker desktop.exe
    if !is_process_running("Docker Desktop.exe") {
        let status = Command::new("powershell")
        .arg("/c")
        .arg(&format!("& '{}'", docker_exe_path))
        .status().unwrap();

        if !status.success() {
            info!(" {}", "失败，原因：docker执行失败".to_string().red());
            return false;
        }
    }

    // 判断 docker images 是否能执行成功，如果能执行成功就返回 true
    if let Err(err) = crate::docker::container_exists("_test_docker_").await {
        info!("{}", err);
        return false;
    }

    success("start_docker");
    return true;
}

use sysinfo::{ProcessExt, System, SystemExt};

pub fn is_process_running(process_name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    let processes = sys.processes();
    for (_pid, proc) in processes {
        if proc.name().to_lowercase() == process_name.to_lowercase() {
            return true;
        }
    }
    false
}

pub fn write_start_dat(value: &str) {
    let dir = dirs::data_local_dir().unwrap().join("Ai");
    let file_path = dir.join("start.dat");
    std::fs::create_dir_all(&dir).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(value.as_bytes()).unwrap();
}

fn docker_start_exe() -> String {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let docker = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Docker Desktop");

    if let Ok(docker) = docker {
        let installation_path: Result<String, _> = docker.get_value("InstallLocation");

        if let Ok(installation_path) = installation_path {
            return format!("{}\\Docker Desktop.exe",installation_path);
        }
    }
    "".to_string()
}

use winapi::um::consoleapi::SetConsoleMode;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
use winapi::um::consoleapi::GetConsoleMode;
fn init_color() {
    unsafe {
        let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_out.is_null() {
            error!("failed to get standard output handle");
            panic!("failed to get standard output handle");
        }

        let mut dw_mode: u32 = 0;
        if GetConsoleMode(h_out, &mut dw_mode) == 0 {
            error!("failed to get console mode");
            panic!("failed to get console mode");
        }

        dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if SetConsoleMode(h_out, dw_mode) == 0 {
            error!("failed to set console mode");
            panic!("failed to set console mode");
        }
    }
}

fn init_first() {
    let current_dir = env::current_dir().unwrap();

    let docker_install_path = current_dir.join("docker_install.exe");
    let ubuntu_install_path = current_dir.join("ubuntu_install.tar");
    let wsl_update_path = current_dir.join("wsl_update.msi");

    if (docker_install_path.exists() || ubuntu_install_path.exists() || wsl_update_path.exists())
        && !env::current_exe().unwrap().to_string_lossy().contains("ai-x86_64-pc-windows-msvc.exe") 
        && !is_debug() {

        info!(" 安装文件到AppData...");
        // 获取 home 目录
        let home_dir = wei_env::home_dir().unwrap();

        // 创建 AppData\\Local\\Ai 文件夹
        let dir_path = Path::new(&home_dir).join("AppData\\Local\\Ai");
        fs::create_dir_all(&dir_path).unwrap();

        // 拼接文件路径
        let config_file_path = dir_path.join("install.dat");

        // 将当前目录保存到配置文件中
        let install_dir = format!("{}", current_dir.to_string_lossy());
        fs::write(config_file_path, install_dir).unwrap();

        let current_exe = env::current_exe().unwrap();
        let exe_dir = Path::new(&current_exe).parent().unwrap();

        let target_path = dir_path.join("ai-x86_64-pc-windows-msvc.exe");
        let cmd = format!("Copy-Item \"{}\" -Destination \"{}\"", current_exe.to_string_lossy(), target_path.to_string_lossy());
        shell(&cmd);

        let new_daemon_target_path = dir_path.join("daemon.exe");
        let source_path = exe_dir.join("daemon.exe");
        let cmd = format!("Copy-Item \"{}\" -Destination \"{}\"", source_path.to_string_lossy(), new_daemon_target_path.to_string_lossy());
        shell(&cmd);

        let new_target_path = dir_path.join("res");
        let source_path = exe_dir.join("res");
        if !new_target_path.exists() {
            let cmd = format!("Copy-Item \"{}\" -Destination \"{}\" -Recurse", source_path.to_string_lossy(), new_target_path.to_string_lossy());
            shell(&cmd);
        }

        let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap();
        // Replace with the actual path to your executable.
        key.set_value("AI", &format!("{}",target_path.to_string_lossy())).unwrap();
    
        create_link();

        // 执行新的 daemon.exe
        Command::new("powershell")
        .args(&["/C", "start", &format!("\"{}\"", new_daemon_target_path.display())])
        .spawn().unwrap();
        // timeout /T 10 && start cmd /C "C:\Users\Wei\AppData\Local\Ai\ai-x86_64-pc-windows-msvc.exe"

        // 程序执行完毕后终止当前进程
        std::process::exit(0);
    }
}

fn uninstall_tips(tip: &str) -> bool {
    let data = format!(" 准备重新安装 {}，请确认 ? [y/n]", tip);
    info!("{}",data.red());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            return true;
        }
        "n" | "no" => {
            return false;
            // 这里放置你想要在用户回答 "no" 时执行的代码
        }
        _ => {
            info!(" 无效输入，请输入 'y' 或者 'n'。");
            // 你也可以选择在这里递归地再次调用 main()，以便在输入无效时让用户重新输入
        }
    }

    false
}

fn init_docker(drive_letter: String) {

    if !uninstall_tips("docker，这将会删除所有镜像和容器数据") {
        success("init_docker");
        return;
    }

    info_print!(" 正在安装docker:     ");
    io::stdout().flush().unwrap();

    shell("wsl --unregister docker-desktop-data");
    shell("wsl --unregister docker-desktop");

    // 删除 docker_new_dir
    // 判断 docker_vhdx_path 是否存在
    // 如果 存在 就把 docker_vhdx_path里面的数据移入到 docker_new_dir
    // unregister docker-desktop-data
    // unregister docker-desktop
    // 删除 docker_vhdx_path
    // 使用 wsl 导入 dock_new_dir 里面的数据

    let userprofile = env::var("USERPROFILE").unwrap();
    let docker_install_path = format!("{}\\AppData\\Local\\Docker", userprofile);
    let docker_install_wsl_path = format!("{}\\wsl", docker_install_path);
    let docker_new_dir = format!("{}:\\Docker",drive_letter);

    Command::new("taskkill").arg("/F").arg("/IM").arg("docker.exe").output().unwrap();

    let _ = fs::remove_dir_all(&docker_install_path);
    let _ = fs::remove_dir_all(&docker_new_dir);
    let _ = fs::remove_dir_all(&docker_install_wsl_path);

    // info!("docker_install_path:{}",docker_install_path);
    // info!("docker_new_dir:{}",docker_new_dir);
    // info!("docker_install_wsl_path:{}",docker_install_wsl_path);
    // info!("{}",&format!("mklink /J \"{}\" \"{}\"", docker_install_wsl_path, docker_new_dir));

    fs::create_dir_all(&docker_install_path).unwrap();
    fs::create_dir_all(&docker_new_dir).unwrap();

    Command::new("powershell").arg("/c")
    .arg(&format!("New-Item -ItemType SymbolicLink -Path \"{}\" -Target \"{}\"", docker_install_wsl_path, docker_new_dir))
    //.arg("start").arg("-await")
    //.arg(&format!("mklink /J \"{}\" \"{}\"", docker_install_wsl_path, docker_new_dir))
    .output().unwrap();
    
    let home_dir = wei_env::home_dir().unwrap();
    let install_dat_path = Path::new(&home_dir).join("AppData\\Local\\Ai\\install.dat");
    let install_path = read_to_string(&install_dat_path).unwrap();
    let docker_exe = format!("{}\\docker_install.exe", install_path);

    let output = Command::new("powershell")
    .args(&["/C", "start", "-wait", &format!("\"{}\"", docker_exe)]).output().unwrap();
    //.arg("/c").arg(&format!("\"{}\"", docker_exe)).output().unwrap();

    

    let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);
    if res == "" {
        success("init_docker");
    } else {
        failed(&res);
    }
}

fn init_ubuntu(drive_letter: String) {
    if !uninstall_tips("ubuntu，这将会删除所有ubuntu数据") {
        success("init_ubuntu");
        return;
    }

    info_print!(" 正在安装ubuntu:     ");
    io::stdout().flush().unwrap(); 

    // 获取 home 目录
    let home_dir = wei_env::home_dir().unwrap();
    let install_dat_path = Path::new(&home_dir).join("AppData\\Local\\Ai\\install.dat");
    let install_path = read_to_string(&install_dat_path).unwrap();
    
    shell("wsl --unregister Ubuntu");

    let dir_path = format!("{}:\\Ubuntu", drive_letter);
    let _ = fs::remove_dir_all(&dir_path);
    fs::create_dir_all(&dir_path).unwrap();

    // let cmd = format!("wsl --import Ai {}:\\Ai .\\ubuntu.tar --version 2", drive_letter);
    let data = shell(&format!("tar -xvf \"{}\\ubuntu_install.tar\" -C \"{}\"", install_path, dir_path));
    // 请启用虚拟机平台 Windows 功能并确保在 BIOS 中启用虚拟化。有关信息，请访问 https://aka.ms/wsl2-install
    if data.contains("请启用虚拟机平台") {
        failed("请重启电脑进入bios, 开启Virtualization Technology（虚拟化技术）, 或者参考网站教程：https://www.zuiyue.com/helpdoc.html");
    }

    if data == "" {
        let ubuntu_exe = format!("{}\\ubuntu.exe", dir_path);
        let output = Command::new("powershell")
        .args(&["/C", "start", "-wait", &format!("\"{}\"", ubuntu_exe)])
        .output().unwrap();
        
        if output.status.success() {
            success("init_ubuntu");
        } else {
            failed(&data);
        }
    }
}


fn init_drive_letter() -> String {
    let mut drive_letter = String::new();

    let mut drives = String::new();
    for letter in b'A'..=b'Z' {
        let drive = format!("{}:", letter as char);
        if fs::read_dir(&drive).is_ok() {
            drives.push_str(&format!("{},", letter as char));
        }
    }

    if !drives.is_empty() {
        drives.truncate(drives.len() - 1); // Remove the last delimiter
    }

    // drives.replace(":", "");


    loop {
        info_print!(" 请输入数据存储的盘符({}): ",drives);
        io::stdout().flush().unwrap();  // Ensure the prompt is immediately displayed

        drive_letter.clear();
        io::stdin().read_line(&mut drive_letter).unwrap();

        let drive_letter = drive_letter.trim().to_uppercase();
        let path = format!("{}:/", drive_letter);

        if fs::metadata(&path).is_ok() {
            return drive_letter;
        } else {
            info!("  磁盘 {} 不存在，请重新尝试输入正确的盘符", drive_letter);
        }
    }
}

fn init_wsl_update() {
    info_print!(" 更新wsl_update:     ");
    std::io::stdout().flush().unwrap();
    
    let home_dir = wei_env::home_dir().unwrap();
    let install_dat_path = Path::new(&home_dir).join("AppData\\Local\\Ai\\install.dat");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&install_dat_path)
        .unwrap();
    
    let mut file_content = String::new();
    let install_path = file.read_to_string(&mut file_content).unwrap();
    
    shell(&format!("msiexec -i \"{}\\wsl_update.msi\" /quiet /l out.txt", install_path));

    let mut file = File::open("out.txt").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let (data, _, _) = encoding_rs::UTF_16LE.decode(&bytes);
    fs::remove_file("out.txt").unwrap();

    if data.contains("成功地完成了配置") || data.contains("已重新配置产品") {
        success("init_wsl_update");
    } else {
        failed(&data);
    }
}

fn success(data: &str) {
    write_json(data, true);
    info!(" {}", "成功".green());
}

fn failed(err: &str) {
    let data = format!("失败，原因：{}，10秒后退出...",err);
    info!(" {}", data.red());
    thread::sleep(Duration::from_secs(10));
    std::process::exit(1);
}

fn init_wsl2() {
    info_print!(" 设置默认wsl2:       ");
    std::io::stdout().flush().unwrap();
    let data = shell("wsl --set-default-version 2");
    if !data.contains("操作成功完成") {
        failed("403");
    }
    success("init_wsl2");
}

fn init_first_reboot() {
    info_print!(" 准备第一次重启:      ");
    std::io::stdout().flush().unwrap();
    match Command::new("shutdown").arg("/r").arg("/t").arg("0").spawn() {
        Ok(_) => {success("init_first_reboot");},
        Err(err) => {failed(&err.to_string());},
    };
}

fn init_wsl() {
    info_print!(" 初始化wsl:          ");
    std::io::stdout().flush().unwrap();

    let output = Command::new("powershell")
    .arg("dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart").output().unwrap();
    let (output_str, _, _) = encoding_rs::GB18030.decode(&output.stdout);
    let data = output_str.to_string();

    if !data.contains("100.0%") {
        failed(&data);
    }

    let output = Command::new("powershell")
    .arg("dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart").output().unwrap();
    let (output_str, _, _) = encoding_rs::GB18030.decode(&output.stdout);
    let data = output_str.to_string();
    
    if !data.contains("100.0%") {
        failed(&data);
    }

    success("init_wsl");
}

use serde_json::{json};
fn write_json(key: &str, value: bool) {
    // 获取 home 目录
    let home_dir = wei_env::home_dir().unwrap();

    // 创建 AppData\\Local\\Ai 文件夹
    let dir_path = Path::new(&home_dir).join("AppData\\Local\\Ai");
    fs::create_dir_all(&dir_path).unwrap();

    // 拼接文件路径
    let file_path = dir_path.join("init.dat");

    // 读取现有的 JSON
    let mut data = match fs::read_to_string(&file_path) {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(_) => json!({}),
    };

    // 更新或插入新的键值对
    data[key] = Value::Bool(value);

    // 打开或创建文件
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path).unwrap();

    // 序列化 JSON 并写入到文件
    let json_string = serde_json::to_string(&data).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}

use std::fs::{self, OpenOptions, read_to_string};
use std::path::Path;
use serde_json::Value;
use std::io::prelude::*;
fn read_json(key: &str) -> bool {
     // 获取 home 目录
     let home_dir = wei_env::home_dir().unwrap();

     // 创建 AppData\\Local\\Ai 文件夹
     let dir_path = Path::new(&home_dir).join("AppData\\Local\\Ai");
     fs::create_dir_all(&dir_path).unwrap();
 
     // 拼接文件路径
     let file_path = dir_path.join("init.dat");
 
     // 如果文件不存在，就创建文件
     OpenOptions::new()
         .write(true)
         .create(true)
         .open(&file_path).unwrap();
 
     // 读取文件内容
     let content = read_to_string(&file_path).unwrap();
 
     // 如果文件内容为空，就创建一个默认的 json
     if content.is_empty() {
         let default_json = r#"{"key": "value"}"#;
         let mut file = OpenOptions::new()
             .write(true)
             .open(&file_path).unwrap();
         file.write_all(default_json.as_bytes()).unwrap();
     } 
 
     // 解析 JSON
     let data: Value = serde_json::from_str(&content).unwrap();
 
     // 检查是否存在关键字，并且其值是否为 true
     data.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn init_admin() {
    if !is_elevated() {
        if !relaunch_elevated() {
            info!(" 请使用管理员模式重新运行！");
            thread::sleep(Duration::from_secs(10));
        };

        std::process::exit(0);
    }
}

use std::io;
use std::fs::File;
fn shell(cmd: &str) -> String {
    let output = Command::new("powershell")
        .arg("/c").arg(cmd).output().unwrap();

    let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);

    res.to_string()
}

use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::TokenElevation;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::{DWORD};
use winapi::shared::ntdef::HANDLE;
use winapi::um::winnt::TOKEN_QUERY;

pub fn is_elevated() -> bool {
    unsafe {
        let mut token_handle: HANDLE = null_mut();
        let current_process_handle = winapi::um::processthreadsapi::GetCurrentProcess();

        if OpenProcessToken(current_process_handle, TOKEN_QUERY, &mut token_handle) == 0 {
            panic!("OpenProcessToken failed");
        }

        let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length: DWORD = 0;

        if GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as DWORD,
            &mut return_length,
        ) == 0
        {
            CloseHandle(token_handle);
            panic!("GetTokenInformation failed");
        }

        CloseHandle(token_handle);
        elevation.TokenIsElevated != 0
    }
}


use std::process::Command;
use winapi::um::shellapi::{ShellExecuteW};
use winapi::um::winuser::{SW_SHOW};
use std::os::windows::prelude::*;
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::iter::once;
// use std::os::raw::c_void;

pub fn relaunch_elevated() -> bool {
    let mut command = Command::new(&format!("{}", std::env::current_exe().unwrap().display()));
    let command = command.args(std::env::args().skip(1));

    let command_str = format!("{:?}", command);
    let command_str = OsStr::new(&command_str)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();
    let operation = OsStr::new("runas")
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    let result = unsafe {
        ShellExecuteW(
            null_mut(),
            operation.as_ptr(),
            command_str.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOW,
        )
    };

    if (result as isize) <= 32 {
        // panic!("{}", format!("ShellExecuteW failed: {}", result as isize).into())
        return false;
    }
    true
}



use std::ptr;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

fn hide() {
    let window = unsafe { GetConsoleWindow() };
    if window != ptr::null_mut() {
        unsafe {
            // hide the console window
            ShowWindow(window, SW_HIDE);
        }
    }
}

// fn show() {
//     let window = unsafe { GetConsoleWindow() };

//     if window != ptr::null_mut() {
//         // 再次显示窗口
//         unsafe { ShowWindow(window, SW_SHOW); }
//     }
// }

use std::env;
fn is_debug() -> bool {
    let home_dir = wei_env::home_dir().unwrap();
    if std::path::Path::new(&home_dir).join("AppData\\Local\\Ai\\debug.dat").exists() {
        return true;
    }

    return false;
}


// pub fn is_single_instance(app_name: &str) -> bool {
//     // use single_instance::SingleInstance;
//     // let instance = SingleInstance::new(app_name).unwrap();
//     // instance.is_single()

//     if let Ok(guard) = SingleInstance::new("my_unique_app_id") {
//         return true;
//     }
//     false;
// }

use std::mem::zeroed;
use winapi::shared::ntdef::NTSTATUS;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::winnt::PRTL_OSVERSIONINFOW;
use winapi::um::winnt::RTL_OSVERSIONINFOW;
use winapi::um::libloaderapi::LoadLibraryW;

fn _version() {
    type RtlGetVersionFn = unsafe extern "system" fn(PRTL_OSVERSIONINFOW) -> NTSTATUS;

    unsafe {
        let hlib = LoadLibraryW("ntdll.dll\0".encode_utf16().collect::<Vec<u16>>().as_ptr());
        let fptr = GetProcAddress(hlib, "RtlGetVersion\0".as_ptr() as *const i8);
        let rtl_get_version: RtlGetVersionFn = std::mem::transmute(fptr);

        let mut os_version_info: RTL_OSVERSIONINFOW = zeroed();
        os_version_info.dwOSVersionInfoSize = std::mem::size_of::<RTL_OSVERSIONINFOW>() as u32;

        let _ = rtl_get_version(&mut os_version_info);

        info!("Windows version {}.{}.{}",
                 os_version_info.dwMajorVersion,
                 os_version_info.dwMinorVersion,
                 os_version_info.dwBuildNumber);
    }
}


fn create_link() {
    let home_dir = wei_env::home_dir().unwrap();
    let dir_path = Path::new(&home_dir).join("AppData\\Local\\Ai");
    let target = dir_path.join("ai-x86_64-pc-windows-msvc.exe");
    let lnk = std::path::Path::new(&home_dir).join("Desktop\\Ai.lnk");

    let sl = mslnk::ShellLink::new(&target).unwrap();

    match mslnk::ShellLink::new(&target) {
        Ok(_) => {
            match sl.create_lnk(&lnk) {
                Ok(_) => {},
                Err(err) => info!(" 创建图标错误：{}", err)
            }
        },
        Err(err) => info!(" 创建图标错误：{}", err)
    }
}


use winapi::ctypes::c_int;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId};
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::LPARAM;


unsafe extern "system" fn _enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> c_int {
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, &mut pid);

    if pid == lparam as u32 {
        ShowWindow(hwnd, SW_SHOW);
        return 0;
    }

    return 1;
}

unsafe extern "system" fn _enum_windows_wrapper(hwnd: HWND, lparam: LPARAM) -> c_int {
    _enum_windows_callback(hwnd, lparam)
}

pub fn _show_hidden_window_by_pid(pid: u32) {
    unsafe {
        EnumWindows(Some(_enum_windows_wrapper), pid as isize);
    }
}