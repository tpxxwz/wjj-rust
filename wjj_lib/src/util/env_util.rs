use std::env;

type EnvKey = &'static str;

const PG_PASSWD_ENV_KEY: EnvKey = "a";
const MYSQL_PASSWD_ENV_KEY: EnvKey = "a";

pub fn get_env(env_key: EnvKey) {
    env::var(env_key.to_string());
}
