use std::thread;
use std::time::Duration;
use std::process::Command;

pub fn install() {
    admin();

    if !read_json("hyper") {
        hyper();
    }

    if !read_json("wsl") {
        wsl();
    }

    if !read_json("first_reboot") {
        first_reboot();
    }

    if !read_json("wsl2") {
        wsl2();
    }

    if !read_json("wsl_update") {
        wsl_update();
    }

    if !read_json("ubuntu") {
        ubuntu();
    }
}

pub fn check() -> f32 {
    // 检查是否安装了wsl2
    // 检查是否安装了ubuntu
    // 检查是否安装了docker
    // 检查是否安装了docker-compose

    let mut count_success = 0;
    let mut count_all = 0;

    count_all += 1;
    if read_json("hyper") {
        count_success += 1;
    }

    count_all += 1;
    if read_json("wsl") {
        count_success += 1;
    }

    count_all += 1;
    if read_json("first_reboot") {
        count_success += 1;
    }

    count_all += 1;
    if read_json("wsl2") {
        count_success += 1;
    }

    count_all += 1;
    if read_json("wsl_update") {
        count_success += 1;
    }

    count_all += 1;
    if read_json("ubuntu") {
        count_success += 1;
    }

    count_success as f32 / count_all as f32
}

fn hyper() {
    info!("设置hyper");

    let output = Command::new("powershell")
    .arg("bcdedit /set hypervisorlaunchtype auto").output().unwrap();
    let (output_str, _, _) = encoding_rs::GB18030.decode(&output.stdout);
    let data = output_str.to_string();

    info!(" {}", data);
    if !data.contains("操作成功完成") {
        failed("403");
    }
    success("hyper");
}

fn wsl2() {
    info!(" 设置默认wsl2:");
    let data = shell("wsl --set-default-version 2");
    if !data.contains("操作成功完成") {
        failed("403");
    }
    success("wsl2");
}

fn first_reboot() {
    info!("准备第一次重启:");
    
    match Command::new("shutdown").arg("/r").arg("/t").arg("5").spawn() {
        Ok(_) => {success("first_reboot");},
        Err(err) => {failed(&err.to_string());},
    };
}


fn wsl_update() {
    info!("更新wsl_update:");

    shell("msiexec -i docker\\wsl_update.msi /quiet /l out.txt");

    let mut file = File::open("out.txt").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let (data, _, _) = encoding_rs::UTF_16LE.decode(&bytes);
    fs::remove_file("out.txt").unwrap();

    if data.contains("成功地完成了配置") || 
       data.contains("已重新配置产品") ||
       data.contains("安装成功") ||
       data.contains("成功")   
    {
        success("wsl_update");
    } else {
        failed(&data);
    }
}


fn wsl() {
    info!("初始化wsl:");

    let output = Command::new("powershell")
    .arg("dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart").output().unwrap();
    let (output_str, _, _) = encoding_rs::GB18030.decode(&output.stdout);
    let data = output_str.to_string();

    info!(" {}", data);

    if !data.contains("100.0%") {
        failed(&data);
    }

    let output = Command::new("powershell")
    .arg("dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart").output().unwrap();
    let (output_str, _, _) = encoding_rs::GB18030.decode(&output.stdout);
    let data = output_str.to_string();

    info!(" {}", data);
    
    if !data.contains("100.0%") {
        failed(&data);
    }

    success("wsl");
}


use serde_json::{json};
fn write_json(key: &str, value: bool) {
    // 获取 home 目录
    let home_dir = wei_env::home_dir().unwrap();

    // 创建 AppData\\Local\\Ai 文件夹
    let dir_path = Path::new(&home_dir);
    fs::create_dir_all(&dir_path).unwrap();

    // 拼接文件路径
    let file_path = dir_path.join("docker.dat");

    if !file_path.exists() {
        let default_json = r#"{"key": "value"}"#;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path).unwrap();
        file.write_all(default_json.as_bytes()).unwrap();
    }

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
     let dir_path = Path::new(&home_dir);
     fs::create_dir_all(&dir_path).unwrap();
 
     // 拼接文件路径
     let file_path = dir_path.join("docker.dat");
 
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
     let data = data.get(key).and_then(Value::as_bool).unwrap_or(false);

     info!("read json: {} {}", key, data);
     data
}

fn success(data: &str) {
    write_json(data, true);
    info!(" {}", "成功");
}

fn failed(err: &str) {
    let data = format!("失败，原因：{}",err);
    info!(" {}", data);
    // thread::sleep(Duration::from_secs(10));
    std::process::exit(1);
}

pub fn admin() {
    if !is_elevated() {
        if !relaunch_elevated() {
            info!(" 请使用管理员模式重新运行！");
            thread::sleep(Duration::from_secs(10));
        };

        std::process::exit(0);
    }
}


pub fn is_elevated() -> bool {
    use winapi::um::winnt::TOKEN_ELEVATION;
    use winapi::um::winnt::TokenElevation;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::handleapi::CloseHandle;
    use winapi::shared::minwindef::{DWORD};
    use winapi::shared::ntdef::HANDLE;
    use winapi::um::winnt::TOKEN_QUERY;
    use std::ptr::null_mut;
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


pub fn relaunch_elevated() -> bool {
    use winapi::um::shellapi::{ShellExecuteW};
    use winapi::um::winuser::{SW_SHOW};
    use std::os::windows::prelude::*;
    use std::ptr::null_mut;
    use std::ffi::OsStr;
    use std::iter::once;

    let exe_path = std::env::current_exe().unwrap();
    let args: Vec<_> = std::env::args().skip(1).collect();
    let args = args.join(" ");

    let exe_path_str = format!("{}", exe_path.display());
    let exe_path_str = OsStr::new(&exe_path_str)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();
    let operation = OsStr::new("runas")
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();
    let args = OsStr::new(&args)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    let result = unsafe {
        ShellExecuteW(
            null_mut(),
            operation.as_ptr(),
            exe_path_str.as_ptr(),
            args.as_ptr(),
            null_mut(),
            SW_SHOW,
        )
    };

    if (result as isize) <= 32 {
        return false;
    }
    true
}


fn ubuntu() {
    info!("正在安装ubuntu:");
    
    shell("wsl --shutdown");
    shell("wsl --unregister wei-ubuntu");
    shell("wsl --import wei-ubuntu docker/wei-ubuntu docker/Ubuntu.tar.gz --version 2");

    let data = shell("wsl -l -v");
    info!(" {}", data);
    // 请启用虚拟机平台 Windows 功能并确保在 BIOS 中启用虚拟化。有关信息，请访问 https://aka.ms/wsl2-install
    if data.contains("请启用虚拟机平台") {
        message("提示", "请重启电脑进入bios, 开启Virtualization Technology（虚拟化技术）");
        failed("请重启电脑进入bios, 开启Virtualization Technology（虚拟化技术）, 或者参考网站教程：https://www.zuiyue.com/helpdoc.html");
    }

    if data.contains("wei-ubuntu") {
        shell("wsl --set-default wei-ubuntu");
        success("ubuntu");
    }

    failed(&data);
}


fn _docker() {
    info!(" 正在安装docker:");

    shell("wsl --unregister docker-desktop-data");
    shell("wsl --unregister docker-desktop");

    // 删除 docker_new_dir
    // 判断 docker_vhdx_path 是否存在
    // 如果 存在 就把 docker_vhdx_path里面的数据移入到 docker_new_dir
    // unregister docker-desktop-data
    // unregister docker-desktop
    // 删除 docker_vhdx_path
    // 使用 wsl 导入 dock_new_dir 里面的数据

    let userprofile = std::env::var("USERPROFILE").unwrap();
    let docker_install_path = format!("{}\\AppData\\Local\\Docker", userprofile);
    let docker_install_wsl_path = format!("{}\\wsl", docker_install_path);
    // 获取当前目录
    let current_dir = std::env::current_dir().unwrap();

    let docker_new_dir = format!("{}/docker/Docker", current_dir.display());

    Command::new("taskkill").arg("/F").arg("/IM").arg("docker.exe").output().unwrap();

    let _ = fs::remove_dir_all(&docker_install_path);
    let _ = fs::remove_dir_all(&docker_new_dir);
    let _ = fs::remove_dir_all(&docker_install_wsl_path);

    info!("docker_install_path:{}",docker_install_path);
    info!("docker_new_dir:{}",docker_new_dir);
    info!("docker_install_wsl_path:{}",docker_install_wsl_path);
    info!("{}",&format!("mklink /J \"{}\" \"{}\"", docker_install_wsl_path, docker_new_dir));

    fs::create_dir_all(&docker_install_path).unwrap();
    fs::create_dir_all(&docker_new_dir).unwrap();

    Command::new("powershell").arg("/c")
    .arg(&format!("New-Item -ItemType SymbolicLink -Path \"{}\" -Target \"{}\"", docker_install_wsl_path, docker_new_dir))
    //.arg("start").arg("-await")
    //.arg(&format!("mklink /J \"{}\" \"{}\"", docker_install_wsl_path, docker_new_dir))
    .output().unwrap();
    
    let docker_exe = format!("./docker/docker.exe");

    let output = Command::new("powershell")
    .args(&["/C", "start", "-wait", &format!("\"{}\"", docker_exe)]).output().unwrap();

    let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);
    if res == "" {
        success("docker");
    } else {
        failed(&res);
    }
}

use std::fs::File;
fn shell(cmd: &str) -> String {
    let output = Command::new("powershell")
        .arg("/c").arg(cmd).output().unwrap();

    let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);

    res.to_string()
}


#[cfg(target_os = "windows")]
fn message(title: &str, text: &str) {
    use winapi::um::winuser::{MessageBoxW, MB_OK};
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    use std::iter::once;
    use std::ptr::null_mut;

    let title: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let text: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    
    unsafe {
        MessageBoxW(null_mut(), text.as_ptr(), title.as_ptr(), MB_OK);
    }
}