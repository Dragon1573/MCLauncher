use clap::{Parser, Subcommand};
use launcher::config::{MCMirror, RuntimeConfig, VersionManifestJson, VersionType};
use launcher::install::install_mc;
use launcher::runtime::gameruntime;
use log::error;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,

    #[command(subcommand)]
    List(VersionType),

    Account {
        name: String,
    },

    Build {
        version: Option<String>,
    },

    Run,

    #[command(subcommand)]
    SetMirror(Mirrors),
}

#[derive(Subcommand, Debug)]
enum Mirrors {
    Official,
    BMCLAPI,
}

fn handle_args() -> anyhow::Result<()> {
    let args = Args::parse();
    let config_path:&Path = Path::new("config.toml");
    let config = fs::read_to_string("config.toml")?;
    let mut config: RuntimeConfig = toml::from_str(&config)?;
    match args.command {
        Command::Init => {
            let normal_config = RuntimeConfig {
                max_memory_size: 5000,
                window_weight: 854,
                window_height: 480,
                user_name: "no_name".to_string(),
                user_type: "offline".to_string(),
                game_dir: std::env::current_dir()?.to_str().unwrap().to_string() + "/",
                game_version: "no_game_version".to_string(),
                java_path: "/usr/bin/java".to_string(),
                mirror: MCMirror {
                    version_manifest: "https://launchermeta.mojang.com/".to_string(),
                    assets: "https://resources.download.minecraft.net/".to_string(),
                    client: "https://launcher.mojang.com/".to_string(),
                    libraries: "https://libraries.minecraft.net/".to_string(),
                },
            };
            fs::write(config_path, toml::to_string_pretty(&normal_config)?)?;
            println!("Initialized empty game direction");
        }
        Command::List(_type) => {
            let list = VersionManifestJson::new(&config)?.version_list(_type);
            println!("{:?}", list);
        }
        Command::Account { name: _name } => {
            config.user_name = _name;
            fs::write(config_path, toml::to_string_pretty(&config)?)?;
        }
        Command::Build { version: None } => {
            install_mc(&config)?;
        }
        Command::Build {
            version: Some(_version),
        } => {
            config.game_version = _version.clone();
            fs::write(config_path, toml::to_string_pretty(&config)?)?;
            println!("Set version to {}", _version);
            install_mc(&config)?;
        }
        Command::Run => {
            gameruntime(config)?;
        }
        Command::SetMirror(Mirrors::Official) => {
            config.mirror = MCMirror {
                version_manifest: "https://launchermeta.mojang.com/".to_string(),
                assets: "https://resources.download.minecraft.net/".to_string(),
                client: "https://launcher.mojang.com/".to_string(),
                libraries: "https://libraries.minecraft.net/".to_string(),
            };
            fs::write(config_path, toml::to_string_pretty(&config)?)?;
            println!("Set official mirror");
        }

        Command::SetMirror(Mirrors::BMCLAPI) => {
            config.mirror = MCMirror {
                version_manifest: "https://bmclapi2.bangbang93.com/".to_string(),
                assets: "https://bmclapi2.bangbang93.com/assets/".to_string(),
                client: "https://bmclapi2.bangbang93.com/".to_string(),
                libraries: "https://bmclapi2.bangbang93.com/maven/".to_string(),
            };
            fs::write(config_path, toml::to_string_pretty(&config)?)?;
            println!("Set BMCLAPI mirror");
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(e) = handle_args() {
        error!("{:#?}", e);
    }
}
