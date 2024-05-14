fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用本地dfget_config/daemon.json 比对文件内容 /etc/docker/daemon.json
    println!("比对daemon.json");
    let daemon_json = std::fs::read_to_string("./dfget_config/daemon.json")?;
    let docker_daemon_json = match std::fs::read_to_string("/etc/docker/daemon.json") {
        Ok(json) => json,
        Err(_) => "".to_string()
    };
    if daemon_json != docker_daemon_json {
        std::fs::write("/etc/docker/daemon.json", daemon_json)?;
        wei_run::command("systemctl", vec!["restart", "docker"])?;
    }

    // 使用本地dfget_config/http-proxy.conf 比对文件内容 /etc/systemd/system/docker.service.d/http-proxy.conf
    println!("比对http-proxy.conf");
    let http_proxy_conf = std::fs::read_to_string("./dfget_config/http-proxy.conf")?;
    let docker_http_proxy_conf = match std::fs::read_to_string("/etc/systemd/system/docker.service.d/http-proxy.conf") {
        Ok(conf) => conf,
        Err(_) => "".to_string()
    };
    if http_proxy_conf != docker_http_proxy_conf {
        // 创建目录/etc/systemd/system/docker.service.d/
        println!("创建目录/etc/systemd/system/docker.service.d/");
        std::fs::create_dir_all("/etc/systemd/system/docker.service.d")?;
        std::fs::write("/etc/systemd/system/docker.service.d/http-proxy.conf", http_proxy_conf)?;
        wei_run::command("systemctl", vec!["daemon-reload"])?;
        wei_run::command("systemctl", vec!["restart", "docker"])?;
    }

    let ip = match get_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("获取IP失败: {}", e);
            "".to_string()
        }
    };

    // 读取dfget.yaml.template文件，替换　{{advertiseIP}}　为本机IP
    let dfget_yaml_template = std::fs::read_to_string("./dfget_config/dfget.yaml.template")?;
    let dfget_yaml = dfget_yaml_template.replace("{{advertiseIP}}", &ip);
    std::fs::write("./dfget_config/dfget.yaml", dfget_yaml)?;

    wei_run::command("./dfget", vec!["daemon", "--config", "./dfget_config/dfget.yaml"])?;

    Ok(())
}


fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    // 使用 ureq 访问 https://www.ip.cn/api/index?ip&type=0 获取本机IP
    let data = ureq::get("https://www.ip.cn/api/index?ip&type=0").call()?.into_string()?;
    println!("获取IP: {}", data);
    let json: serde_json::Value = serde_json::from_str(&data)?;
    // 如果存在ip字段则返回
    if let Some(ip) = json.get("ip") {
        return Ok(ip.as_str().unwrap().to_string());
    }
    // 如果不存在则返回错误
    Err("获取IP失败".into())
}