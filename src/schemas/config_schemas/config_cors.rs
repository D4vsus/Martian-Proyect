struct ConfigCors{
    allow_origins: Vec<&str>,
    allow_origin_regex: Vec<&str>,
    allow_credentials: bool,
    allow_methods:Vec<&str>,
    allow_headers:Vec<&str>,
    max_age:Vec<u32>,
}