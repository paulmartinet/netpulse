use clap::{Arg, Command};
use tokio::main;
use tokio::process::Command as TokioCommand;
use trust_dns_resolver::{TokioAsyncResolver, config::*};
use std::net::IpAddr;

#[main]
async fn main() {
    let matches = Command::new("NetPulse")
        .version("0.1.0")
        .about("Outil de diagnostic réseau : DNS, Ping, Traceroute")
        .arg(Arg::new("domain")
            .help("Le domaine ou l'adresse IP à tester")
            .required(true)
            .index(1))
        .arg(Arg::new("dns")
            .long("dns")
            .help("Spécifie le serveur DNS à utiliser")
            .num_args(1)) 
        .arg(Arg::new("ping")
            .long("ping")
            .help("Active le ping pour tester la latence")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("traceroute")
            .long("traceroute")
            .help("Active le traceroute pour tracer le chemin réseau")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let domain = matches.get_one::<String>("domain").expect("Argument 'domain' requis");
    let default_dns = "8.8.8.8".to_string();
    let dns_server = matches.get_one::<String>("dns").unwrap_or(&default_dns);
    let ping_enabled = matches.get_flag("ping");
    let traceroute_enabled = matches.get_flag("traceroute");

    if matches.contains_id("dns") {
        resolve_dns(domain, dns_server).await;
    }
    if ping_enabled {
        ping(domain).await;
    }
    if traceroute_enabled {
        traceroute(domain).await;
    }
}

async fn resolve_dns(domain: &str, dns_server: &str) {
    println!("Résolution DNS pour {} avec serveur DNS {}...", domain, dns_server);

    let resolver_config = ResolverConfig::from_parts(
        None,
        vec![],
        NameServerConfigGroup::from_ips_clear(
            &[dns_server.parse::<IpAddr>().expect("Serveur DNS invalide")],
            53,
            true,
        ),
    );

    let resolver_options = ResolverOpts::default();

    let resolver = TokioAsyncResolver::tokio(resolver_config, resolver_options)
        .expect("Erreur lors de la création du résolveur");

    match resolver.lookup_ip(domain).await {
        Ok(lookup_result) => {
            for ip in lookup_result.iter() {
                println!("Adresse IP pour {}: {}", domain, ip);
            }
        }
        Err(e) => println!("Erreur de résolution DNS : {}", e),
    }
}

async fn ping(domain: &str) {
    println!("Envoi de ping à {}...", domain);

    let output = if cfg!(target_os = "windows") {
        TokioCommand::new("ping")
            .args(&["-n", "4", domain]) 
            .output()
            .await
            .expect("Erreur lors de l'exécution du ping")
    } else {
        TokioCommand::new("ping")
            .args(&["-c", "4", domain]) 
            .output()
            .await
            .expect("Erreur lors de l'exécution du ping")
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

async fn traceroute(domain: &str) {
    println!("Traceroute vers {} :", domain);

    let output = if cfg!(target_os = "windows") {
        TokioCommand::new("cmd")
            .args(&["/C", "tracert", domain])
            .output()
            .await
            .expect("Erreur lors de l'exécution de tracert")
    } else {
        TokioCommand::new("traceroute")
            .arg(domain)
            .output()
            .await
            .expect("Erreur lors de l'exécution de traceroute")
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
