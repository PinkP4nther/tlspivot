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

    let certificate_embed = b"-----BEGIN CERTIFICATE-----\nMIIDgzCCAmugAwIBAgIUQWH/ROrduvZahG3NArLP+BrjOaYwDQYJKoZIhvcNAQELBQAwUTELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDEKMAgGA1UEAwwBKjAeFw0yMTEwMTcwMzQyMTdaFw0yMTExMTYwMzQyMTdaMFExCzAJBgNVBAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5ldCBXaWRnaXRzIFB0eSBMdGQxCjAIBgNVBAMMASowggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCkwKU3JEzwIrlHWfQLCdtRe21TP4SJvsYRm5bjilglsj/LJ22zh6FXr8kdP0rsC0V/vJMcQEjlcVbLapSqfMOxbBKTlWwIp750BKRvZKotV3zVNyt6q0hiaZQLG9pq0P5dfbEFkgS70f+N/MfYSDcylX9kDAMVXZQZ6zzgzAx16GCR2i9/uP5JIUOStDuFJQMYxCHrKrmEkNKq0m+Kcao8p1+URrVfAegSNCDWZVaQwMXhqNDMLnjOwd6OA9LzojRLjqkMUXTZxnAwrKExgnPyjxK+SCxUW+U0nN6AK5cJdJ0wuQvWaD++UVf6jFI+mwggVLJi5uca+ZbNxMI6/A/1AgMBAAGjUzBRMB0GA1UdDgQWBBR8nJK7CM9ycK0uJuXXtAuIBlrFsTAfBgNVHSMEGDAWgBR8nJK7CM9ycK0uJuXXtAuIBlrFsTAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQAZKLCerRj/VfaTjmoeG8ZZXPPi2oT5DM6s8tyo2mirPcbsBTpI3++8qE+pYGOzlhxsYQchTdy4AfaEZGvcbcameYgO5fUmlJFwRkvAbUK522fu/uoz4UGpNNYKQt6veaYePpTU7QTRYX53tUZBHPw9CEKZN0pQu4oKAKTI5bt5ppDOWSJ5VymIDNwOk/14QZPGNYhs14m9CnEys6w8ztypm14Krmne7dVwKyjl9UBbj/RoZOU5qwdgEo6QD5qUNb1R3i5O2y3+lurpUdSaE5GMy5B2CXFbSKhUcz2ijxLnH5sz1DWlfCth70H/2afUHjlegNxLAUP0BzTgnQ6u2P7q\n-----END CERTIFICATE-----";
    let private_key_embed = b"-----BEGIN ENCRYPTED PRIVATE KEY-----\nMIIFHDBOBgkqhkiG9w0BBQ0wQTApBgkqhkiG9w0BBQwwHAQIIQxEU29HnaUCAggAMAwGCCqGSIb3DQIJBQAwFAYIKoZIhvcNAwcECAJGx8k28IprBIIEyBMhlN3sw7qQCEHRuRY9Y/HDgF2HSMFKEK/I7uh2L5q4iiYb+wdREdbYQP1yLnYPvvVQYJ8hDwjQztQgTBcatiHQewHow+CwCJFOs5Uz5RbjVZdM9TbevsK4v9PO44iPuUgcEYP7S9vt2NAsoo6y8I7WLD+NMonWRugM5nWy4thM4GkWTa8lB1fatU4yPSqAHJwE1TROs4XE1uRBwWFqLde2tqN3CKSGOLXYCkwyoadjoewMxNyrswUdfn6fXDRuk/8fgLi3y6hbzviulB4K8oD3RMqjueeRGt8rU1+bU67I854HaoScQn/JIDnmv50mtI9svO0NubYoyA+pNdmNLgBDIGkxCI3TBYvsN1ft4E5P/6Ug/Oz/h1RMU/HTnA3KUblVvXiMkX3scVC0FlH3M6F42jtESd5m4iOlaAwuWWZbmBWLjDMgg1tJnXu0cDT3NnVRQ3FiBDuU3s03f3ZwIYId3JPQBjtyXSEpK1koZPQKlkFsTKP1OZrqlRy+A2pUiVWdFbUGre0Bu09NXYjZP5IEEUVdKXMZOU5yW+YgySsb+GG/eS4fb52mjMPWuYN2E8TZy1qBCcmsZrHM2HvIUrrHRsQZqRA4nvhcJ7B6uOIPW984oJAHFKvByYzB9JyVqsMxi88mBTCEUMKkC7bOPIWGVQgDplFMrQzlMLzyk7jfeT4W4SqErjds6ovg5DLsCVMWk0E6YH0J3jFRNDUXr9gipl2ji1fwwMLxASdVlJySlTlqLQTxyQ9fOHCYET2PbSboYdhODkAawfPtugnCtB1Lqtk8s7HjoBko/m0nn2ZN/BMA59xSVgjR0dxssPNUgO4c7oaYX4BExRlgSRvYoFvQrWH5MInn8FX9xTCDA/SER0M6i32T9YYD3S9/t15ANKZBE60zNWkeMR4dlBJQnm583JGzRUQmKpDoFv8dC7xFYPrtZoY+bZDynLFX4acRpS6t4xmEgiV911cbtQSyzyH8hJoawYsSQ9+VPGoemMCg1oNkKBvyD48yjHzGI+Rziku0+O2Gw0zDm9bpmPm88f3TTyNDwDsyQme8uciECvoirif3CbpnZgc4CjPqef0/9nbRa8QFhaUvbyZMok4pz4CQDCNiy6SWZ4+coZiy7hUPgxft5dyrovN4ri1VxnzXkX7KUKwfNajJhyE+pIDzO/QOTTTewQI0LWuxIrqjKczbjCs5RNKja1m99Jqq6L9e+RKqZBND3Rf2sbVKHGsftiX77O/BQ2EljDXFtkQqNAK+ShHemDlEXHhfr6IAq+0CMwKI8Dq9wdb38CpIA2JYz24Ne/95RmGu5yDHCzKOEuHauAMYfhTZueSfVrcdW70MzpSIoLIq+dyo3ygnEmdAup4QxhxMh7rhjjfTg2iATt0bkutqF1PcSnVr90I14zYLQYeLRyyFNi4GdStdmiOqPhBgAaiYE+1FGN5Xj50YuWitEBNW+MVLj12VQRYI2hFrLAf+ONlLAG41QfVKuvb2KmswhyHUUTlifg5g/v4dCTQ3/cJbFsKr2ooqzXYUubU8ZynRmI5rNiICjDCVHCiMQTQLK+gHleNqfPo21av9la7uSbhr+gYQDozKZoD+yXZPZEaTenEGvnZsqe8DuOl98YGD9aCW9vqw8Q==\n-----END ENCRYPTED PRIVATE KEY-----";

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