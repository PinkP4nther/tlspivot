use clap::{
    App,
    Arg,
    ArgMatches
};
use sslrelay::*;

struct PivotOpts {
    listen_host: String,
    listen_port: String,
    remote_host: String,
    remote_port: String,
    lmode: TCPDataType,
    rmode: TCPDataType,
}

fn main() {

    #[derive(Clone)]
    struct HandleObj;
    impl HandlerCallbacks for HandleObj{}

    let certificate_embed = include_bytes!("pivot.crt");
    let private_key_embed = include_bytes!("pivot.key");

    let opts = parse_args();

    let tls_conf = TLSConfig::DATA {
        certificate: certificate_embed.to_vec(),
        private_key: private_key_embed.to_vec(),
    };

    let relay_config = RelayConfig {
        downstream_data_type: opts.lmode,
        upstream_data_type: opts.rmode,
        bind_host: opts.listen_host,
        bind_port: opts.listen_port,
        remote_host: opts.remote_host,
        remote_port: opts.remote_port,
        tls_config: tls_conf,
    };

    let mut relay = SSLRelay::new(HandleObj, relay_config);
    relay.start();
}

fn parse_args() -> PivotOpts {

    let arg_options = get_args();

    let lhost = arg_options.value_of("listen-host").unwrap();
    let lport = arg_options.value_of("listen-port").unwrap();
    let rhost = arg_options.value_of("remote-host").unwrap();
    let rport = arg_options.value_of("remote-port").unwrap();

    let lmode_unparsed: i64 = arg_options.value_of("lmode").unwrap().parse().unwrap();
    let rmode_unparsed: i64 = arg_options.value_of("rmode").unwrap().parse().unwrap();

    let lmode: TCPDataType;
    let rmode: TCPDataType;

    if lmode_unparsed == 0 {
        lmode = TCPDataType::RAW;
    } else if lmode_unparsed == 1 {
        lmode = TCPDataType::TLS;
    } else {
        println!("[!] --lmode must be 0(raw) or 1(tls).");
        std::process::exit(-1);
    }

    if rmode_unparsed == 0 {
        rmode = TCPDataType::RAW;
    } else if rmode_unparsed == 1 {
        rmode = TCPDataType::TLS;
    } else {
        println!("[!] --rmode must be 0(raw) or 1(tls).");
        std::process::exit(-1);
    }

    PivotOpts {
        listen_host: lhost.to_string(),
        listen_port: lport.to_string(),
        remote_host: rhost.to_string(),
        remote_port: rport.to_string(),
        lmode,
        rmode,
    }
}

fn get_args<'arg_matches>() -> ArgMatches<'arg_matches> {
    App::new("TLS Pivot")
        .version("0.1.0")
        .author("PinkP4nther")
        .about("A binary for pivoting with TLS capabilities.")
        .arg(Arg::with_name("listen-host")
            .long("listen-host")
            .value_name("0.0.0.0")
            .help("IP/Host to bind the listener to.")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("listen-port")
            .long("listen-port")
            .value_name("443")
            .help("Port to bind the listener to.")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("remote-host")
            .long("remote-host")
            .value_name("172.16.0.1")
            .help("IP/Host to connect to.")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("remote-port")
            .long("remote-port")
            .value_name("443")
            .help("Port to connect to.")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("lmode")
            .long("lmode")
            .help("Sets the listening protocol (0=raw/1=tls).")
            .value_name("1")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("rmode")
            .long("rmode")
            .help("Sets the listening protocol (0=raw/1=tls).")
            .value_name("1")
            .required(true)
            .takes_value(true)
        ).get_matches()
}