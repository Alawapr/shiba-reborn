use std::{io::Read, path::Path, process::Command};

pub fn get_os_string() -> String {
    #[cfg(target_os = "linux")]
    {
        if Path::new("/.dockerenv").exists() {
            return "Docker Container".to_string();
        }

        if let Ok(mut file) = std::fs::File::open("/proc/sys/kernel/osrelease") {
            let mut buf = String::new();
            if file.read_to_string(&mut buf).is_ok()
                // Depending on the WSl version, "microsoft" may start with uppercase or lowercase
                && (buf.contains("microsoft") || buf.contains("Microsoft"))
            {
                return "Windows Subsystem for Linux".to_string();
            }
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg("grep ID /etc/os-release | awk -F= \'$1==\"ID\" {print}\'")
            .output()
            .expect("failed to execute process");

        let os_string = String::from_utf8_lossy(&output.stdout);
        let (_, distro) = os_string
            .split_once('\n')
            .expect("Couldn't split OS string")
            .0
            .split_once('=')
            .expect("Couldn't split OS string");

        distro.to_string()
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("wmic")
            .arg("os")
            .arg("get")
            .arg("Caption")
            .output()
            .expect("failed to execute process");

        let cow_string = String::from_utf8_lossy(&output.stdout);
        let string = cow_string.replace("Caption", "").replace("Microsoft", "");
        string.trim().to_owned()
    }

    #[cfg(target_os = "freebsd")]
    {
        "FreeBSD".to_string()
    }
}

pub fn get_version_string() -> String {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg("git rev-parse --short main")
            .output()
            .expect("failed to execute process");

        let os_string = String::from_utf8_lossy(&output.stdout);

        os_string.trim().to_owned()
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--short")
            .arg("main")
            .output()
            .expect("failed to execute process");

        let os_string = String::from_utf8_lossy(&output.stdout);

        os_string.trim().to_owned()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        "Unknown".to_string()
    }
}
