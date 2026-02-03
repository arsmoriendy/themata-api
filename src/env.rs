use std::env::var;
use std::sync::LazyLock;

pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| var("DATABASE_URL").unwrap());

macro_rules! ENV {
    ($name:ident) => {
        pub static $name: LazyLock<String> =
            LazyLock::new(|| var(format!("THEMATA_{}", stringify!($name))).unwrap());
    };
    ($($name:ident),+ $(,)?) => {
        $( ENV!($name); )+
    };
}

ENV![
    LISTEN_ADDR,
    REDIS_URL,
    GITHUB_CLIENT_ID,
    GITHUB_CLIENT_SECRET,
    GITHUB_APP_NAME,
    JWT_SECRET,
];

pub fn ensure_envs() {
    // INFO: add the env vars above here manually
    for v in [
        &LISTEN_ADDR,
        &DATABASE_URL,
        &REDIS_URL,
        &GITHUB_CLIENT_ID,
        &GITHUB_CLIENT_SECRET,
        &GITHUB_APP_NAME,
        &JWT_SECRET,
    ] {
        LazyLock::force(v);
    }
}
