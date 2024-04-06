fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_run::kill("dfget")?;

    // 使用本地dfget_config/daemon.json 比对文件内容 /etc/docker/daemon.json
    println!("比对daemon.json");
    let daemon_json = std::fs::read_to_string("./dfget_config/daemon.json")?;
    let docker_daemon_json = std::fs::read_to_string("/etc/docker/daemon.json")?;
    if daemon_json != docker_daemon_json {
        std::fs::write("/etc/docker/daemon.json", daemon_json)?;
        wei_run::command("systemctl", vec!["restart", "docker"])?;
    }

    // 使用本地dfget_config/http-proxy.conf 比对文件内容 /etc/systemd/system/docker.service.d/http-proxy.conf
    println!("比对http-proxy.conf");
    let http_proxy_conf = std::fs::read_to_string("./dfget_config/http-proxy.conf")?;
    let docker_http_proxy_conf = std::fs::read_to_string("/etc/systemd/system/docker.service.d/http-proxy.conf")?;
    if http_proxy_conf != docker_http_proxy_conf {
        // 创建目录/etc/systemd/system/docker.service.d/
        println!("创建目录/etc/systemd/system/docker.service.d/");
        std::fs::create_dir_all("/etc/systemd/system/docker.service.d")?;
        std::fs::write("/etc/systemd/system/docker.service.d/http-proxy.conf", http_proxy_conf)?;
        wei_run::command("systemctl", vec!["daemon-reload"])?;
        wei_run::command("systemctl", vec!["restart", "docker"])?;
    }

    wei_run::command("./dfget", vec!["daemon", "--config", "./dfget_config/dfget.yaml"])?;

    Ok(())
}
