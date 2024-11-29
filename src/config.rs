pub mod config {
    use std::{fs::{self}, path::Path};

    #[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
    pub struct Server {
        pub name: String,
        pub address: String,
        pub port: String
    }

    #[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
    pub struct Config {
        pub servers: Vec<Server>
    }

    impl Config {

        pub fn initialize(&mut self) {
            // Check if a file exists
            let path = Path::new("Config.toml");
            if !path.exists() {
                self.servers = Vec::new();
                self.write_config_file(path);
            } else {
                // if a file exists then make sure that it is valid
                let contents = fs::read_to_string(path).unwrap();
                if toml::from_str::<Config>(&contents).is_err() {
                    self.servers = Vec::new();
                    self.write_config_file(path);
                }
            }
            
            let read_obj: Config = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
            self.servers = read_obj.servers;
        }

        fn write_config_file(&self, path: &Path) {
            fs::write(path, toml::to_string(&self).unwrap()).unwrap();
        }

        pub fn add_new_server(&mut self, new_server: Server) {
            self.servers.push(new_server);
            self.write_config_file(Path::new("Config.toml"));
        }

    }
}
