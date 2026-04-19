// import java.util.Objects;

// public class Config {

//     public String port;

//     public String dbHost;
//     public String dbPort;
//     public String dbUser;
//     public String dbPassword;
//     public String dbName;

//     public String redisAddr;
//     public String redisPassword;

//     public String kafkaBroker;

//     public String otpServiceAddr;
//     public String fileServiceAddr;

//     public static Config load() {

//         Config config = new Config();

//         config.port = System.getenv().getOrDefault("PORT", "8080");

//         config.dbHost = requireEnv("DB_HOST");
//         config.dbPort = requireEnv("DB_PORT");
//         config.dbUser = requireEnv("DB_USER");
//         config.dbPassword = requireEnv("DB_PASSWORD");
//         config.dbName = requireEnv("DB_NAME");

//         config.redisAddr = requireEnv("REDIS_ADDR");
//         config.redisPassword = System.getenv().getOrDefault("REDIS_PASSWORD", "");

//         config.kafkaBroker = requireEnv("KAFKA_BROKER");

//         config.otpServiceAddr = requireEnv("OTP_SERVICE_ADDR");
//         config.fileServiceAddr = requireEnv("FILE_SERVICE_ADDR");

//         return config;
//     }

//     public String databaseUrl() {

//         return String.format(
//                 "postgres://%s:%s@%s:%s/%s",
//                 dbUser,
//                 dbPassword,
//                 dbHost,
//                 dbPort,
//                 dbName
//         );
//     }

//     private static String requireEnv(String key) {

//         String value = System.getenv(key);

//         if (value == null) {
//             throw new RuntimeException(key + " is required");
//         }

//         return value;
//     }
// }