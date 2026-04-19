use std::env;
//use is like importf
//std is rust standard library, or built in one
//env is the module name, same like pakcage in other langs


//pub keyword stand for public, like in java
// in java it will be like this
// public class Config {
//     public String port;
//     public String dbHost;
// }
//so struct here is an object blueprint
pub struct Config {
    pub port: String,

    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,

    pub redis_addr: String,
    pub redis_password: String,

    pub kafka_broker: String,

    pub otp_service_addr: String,
    pub file_service_addr: String,
}


// public class Config {
//     public static Config load() {
//            return config;
//     }
// }
//impl=implement
impl Config {

    pub fn load() -> Self {
    //-> arrow notaion like in JS and dart which means return
    //load is the function name
    //self means this, to this-self
    // so this load function will return Self, which is a struct, a config struct
    // since we specify that we will return Self, so prepare the self, remember that rust dont need return keyword
    // it it like we will return string, so prepare the string

        dotenv::dotenv().ok();
        //from dotenv namespace, call dotenv() method and then call .ok() method

        Self {
            port: env::var("PORT").unwrap_or("8080".to_string()),

            db_host: env::var("DB_HOST").unwrap(),
            db_port: env::var("DB_PORT").unwrap(),
            db_user: env::var("DB_USER").unwrap(),
            db_password: env::var("DB_PASSWORD").unwrap(),
            db_name: env::var("DB_NAME").unwrap(),

            redis_addr: env::var("REDIS_ADDR").unwrap(),
            redis_password: env::var("REDIS_PASSWORD").unwrap_or_default(),

            kafka_broker: env::var("KAFKA_BROKER").unwrap(),

            otp_service_addr: env::var("OTP_SERVICE_ADDR").unwrap(),
            file_service_addr: env::var("FILE_SERVICE_ADDR").unwrap(),
        }
    }

    pub fn database_url(&self) -> String {
    // & means reference, so we can read the value of self
    // & works like read-only permission
    // we read the some config in self, and return string, here we use arrow notation


        //format! is macro which wrap something like fmt.Sprintf or printf() in java
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name
        )
    }
}