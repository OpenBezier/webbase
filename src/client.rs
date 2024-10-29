use dashmap::DashMap;
use reqwest;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Clone)]
pub struct UniClient {
    pub server: String,        // Http的URL地址
    pub secure_server: String, // Https的URL地址
    pub ws_server: String,     // ws的URL地址
    pub wss_server: String,    // wss的URL地址
    pub secure_mode: bool,
    pub need_login_data: bool,

    pub client: reqwest::Client,         // HTTP的Client请求客户端
    pub config: DashMap<String, String>, // 其他可用的配置信息
    pub user: Option<Value>,             // 保存的是登录后的登录信息
}

impl UniClient {
    pub fn get_http_url(&self) -> String {
        if self.secure_mode {
            self.secure_server.clone()
        } else {
            self.server.clone()
        }
    }

    pub fn get_websocket_url(&self) -> String {
        if self.secure_mode {
            self.wss_server.clone()
        } else {
            self.ws_server.clone()
        }
    }
}

static CLIENT: OnceLock<UniClient> = OnceLock::<UniClient>::new();

pub fn get_client() -> &'static UniClient {
    CLIENT.get().unwrap()
}

pub fn check_client() {
    let client = get_client();
    if client.need_login_data && client.user.is_none() {
        println!("user access token is invalid, please login firstly");
        std::process::exit(1);
    }
}

pub fn init_client(
    server: String,
    cache_dir_name: &str,
    secure_mode: bool,
    need_login_data: bool,
) -> &'static UniClient {
    CLIENT.get_or_init(|| {
        let client = reqwest::Client::builder()
            .http1_only()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(5))
            // .tcp_keepalive(std::time::Duration::from_secs(120))
            .build()
            .unwrap();
        let config = DashMap::<String, String>::default();

        let user = if need_login_data {
            let mut datapath = directories::UserDirs::new()
                .unwrap()
                .home_dir()
                .to_path_buf();
            datapath.push(cache_dir_name);
            std::fs::create_dir_all(&datapath).unwrap();
            let mut cachefile = datapath.clone();
            cachefile.push("config.json");
            config.insert("datapath".into(), datapath.to_string_lossy().to_string());
            config.insert("cachefile".into(), cachefile.to_string_lossy().to_string());

            let user = match read_client_access_token(cachefile) {
                Ok(user) => Some(user),
                Err(_e) => None,
            };
            user
        } else {
            None
        };

        let client: UniClient = UniClient {
            client,
            server: format!("http://{}", server),
            secure_server: format!("https://{}", server),
            ws_server: format!("ws://{}", server),
            wss_server: format!("wss://{}", server),
            need_login_data,
            config,
            user: user,
            secure_mode,
        };
        client
    })
}

pub fn read_client_access_token(config_file: PathBuf) -> anyhow::Result<Value> {
    let config_str = std::fs::read_to_string(config_file)?;
    let config = serde_json::from_str(&config_str)?;
    Ok(config)
}
