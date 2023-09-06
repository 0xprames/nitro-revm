use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct ServerArgs {
    pub port: u32,
}

impl ServerArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(ServerArgs {
            port: parse_port(args)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientArgs {
    pub cid: u32,
    pub port: u32,
    pub key_id: String,
}

impl ClientArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(ClientArgs {
            cid: parse_cid_client(args)?,
            port: parse_port(args)?,
            key_id: parse_key_id(args)?,
        })
    }
}

fn parse_cid_client(args: &ArgMatches) -> Result<u32, String> {
    let cid = args.value_of("cid").ok_or("Could not find cid argument")?;
    cid.parse()
        .map_err(|_err| "cid is not a number".to_string())
}

fn parse_port(args: &ArgMatches) -> Result<u32, String> {
    let port = args
        .value_of("port")
        .ok_or("Could not find port argument")?;
    port.parse()
        .map_err(|_err| "port is not a number".to_string())
}

fn parse_key_id(args: &ArgMatches) -> Result<String, String> {
    let key_id = args
        .value_of("key_id")
        .ok_or("Could not find key_id argument")?;
    key_id
        .parse()
        .map_err(|_err| "key_id is not a string".to_string())
}
