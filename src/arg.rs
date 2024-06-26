use std::collections::HashMap;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TUds {
    /// target socket
    ///
    /// 目标的uds文件
    // #[clap(&str)]
    #[arg(short, long, default_value = None)]
    pub target: Option<String>,

    /// listen socket
    ///
    /// 代理的本地端口
    #[arg(short, long, default_value = None)]
    pub listen: Option<String>,

    /// protocol
    ///
    /// 代理协议
    #[arg(short, long, default_value = None)]
    pub protocol: Option<String>,

    /// config file
    ///
    /// config file
    #[arg(short, long, default_value = None)]
    pub config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
    pub listen: String,
    pub target: String,
    pub protocol: String,

    pub timeout: Option<String>,
    pub http_header: Option<HashMap<String, String>>,
}


#[derive(Debug, Deserialize)]
pub struct Config {
    pub service: Vec<Service>,
    pub uri_mapping: Option<String>,
}