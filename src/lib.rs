use std::process::Command;

#[derive(Debug,Eq)]
pub struct Package {
    pub name: String,
    pub version: String
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

pub fn install_package(package_name: &str) {
    Command::new("cinst")
        .args(&[package_name, "-y"])
        .output()
        .expect("failed to install package");
}

pub fn uninstall_package(package_name: &str) {
    Command::new("choco")
        .args(&["uninstall", package_name, "-y"])
        .output()
        .expect("failed to uninstall package");
}

pub fn get_installed_packages() -> Vec<Package> {
    let raw_string = get_package_list_string();
    parse_packages_from_list_string(&raw_string)
}

fn get_package_list_string() -> String {
    let output = Command::new("choco")
            .args(&["list", "-l", "-r"])
            .output()
            .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn parse_packages_from_list_string(list_str: &str) -> Vec<Package> {
    list_str.lines().into_iter()
    .map(|l| l.split("|").collect::<Vec<&str>>())
    .map(|vec| {
        assert_eq!(vec.len(), 2);
        Package {name: vec[0].to_string(), version: vec[1].to_string()}
    }).collect::<Vec<Package>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let result = parse_packages_from_list_string("");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_one() {
        let result = parse_packages_from_list_string("vlc|3.0.6");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Package {name: "vlc".to_string(), version: "3.0.6".to_string()})
    }
}