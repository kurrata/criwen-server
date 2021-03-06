use config::{Config};
use std::io::{Write};
use std::fs::File;
use std::path::{PathBuf};


#[derive(Debug, Deserialize, Serialize)]
struct Server {
    world_map_radius: u16,
    local_port: u16,
    public_port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    log_level: u64,
    server: Server,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            log_level: 1,
            server: Server {
                world_map_radius: 3,
                local_port: 6400,
                public_port: 6500,
            },
        }
    }
}

impl Settings {
    pub async fn new() -> Config {
        use config::{File};

        Settings::write_default_config().await;
        let base_path = Settings::config_dir();
        let mut s = Config::new();
        File::with_name(base_path.join("default").to_str().unwrap());
        s.merge(File::with_name(base_path.join("default").to_str().unwrap())).unwrap();
        s.merge(File::with_name(base_path.join("development").to_str().unwrap()).required(false)).unwrap();
        s
    }

    fn config_dir() -> PathBuf {
        let path = std::env::current_exe().unwrap();
        let base_path = std::path::Path::new(&path).parent().unwrap().join("config");
        std::fs::create_dir_all(base_path.clone()).unwrap();
        return base_path;
    }

    async fn write_default_config() {
        let base_path = Settings::config_dir();
        let s = toml::to_string(&Settings::default()).unwrap();
        let mut file = File::create(base_path.join("default.toml")).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }
}


