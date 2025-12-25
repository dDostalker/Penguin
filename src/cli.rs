use std::path::PathBuf;

use crate::{
    i18n,
    tools_api::{
        FileInfo, HashInfo,
        calc::{calc_md5, calc_sha1},
        read_file::ResourceTree,
        serde_pe::save_to_file,
    },
};
use clap::{Parser, Subcommand, ValueEnum};

const ABOUT: &str = r"
  _____                       _
 |  __ \                     (_)
 | |__) |__ _ __   __ _ _   _ _ _ __
 |  ___/ _ \ '_ \ / _` | | | | | '_ \
 | |  |  __/ | | | (_| | |_| | | | | |
 |_|   \___|_| |_|\__, |\__,_|_|_| |_|
                   __/ |
                  |___/
";

#[derive(Parser)]
#[command(version, about = ABOUT)]
pub struct Cli {
    pub file_path: String,
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Serde PE to toml or json
    Serde {
        #[arg(short, long)]
        #[arg(default_value = "./")]
        output: String,
        #[arg(short, long)]
        #[arg(default_value = "json")]
        ftype: FileType,
    },
    /// Print PE info to console
    Info {},
    /// Extract resource from PE
    Resource {
        #[arg(short, long)]
        #[arg(default_value = "./")]
        output: String,
    },
}
#[derive(ValueEnum, Clone)]
enum FileType {
    Toml,
    Json,
}

impl Cli {
    pub fn execute(&self) {
        let file_path = PathBuf::from(self.file_path.clone());
        let mut file_manager = FileInfo::new(file_path).expect("Failed to create file manager");
        match &self.command {
            CliCommand::Serde { output, ftype } => {
                let file_type = match ftype {
                    FileType::Json => "json",
                    FileType::Toml => "toml",
                };
                let mut file_path = PathBuf::from(output);
                file_path.push(format!("{}.{}", file_manager.file_name, file_type));
                save_to_file(&mut file_manager, &file_path, file_type).expect(i18n::SAVE_FAILED);
                println!("Success: {}", i18n::SAVE_SUCCESS);
            }
            CliCommand::Info {} => {
                println!("Info: ");
                println!("File Name: {}", file_manager.file_name);
                println!("File Path: {}", file_manager.file_path.display());
                println!("File Size: {}B", file_manager.file_size);
                if file_manager.file_hash.is_none() {
                    file_manager.file_hash = Some(HashInfo {
                        md5: calc_md5(&file_manager.file_path),
                        sha1: calc_sha1(&file_manager.file_path),
                        path: file_manager.file_path.clone(),
                    });
                }
                println!(
                    "File Hash: {}",
                    file_manager
                        .file_hash
                        .as_ref()
                        .expect("File hash is not found")
                        .md5
                );
                println!(
                    "File Hash: {}",
                    file_manager
                        .file_hash
                        .as_ref()
                        .expect("File hash is not found")
                        .sha1
                );
                println!("File Is 64 Bit: {}", file_manager.is_64_bit);
                println!("File DOS Header Magic: {}", file_manager.dos_head.e_magic);
            }
            CliCommand::Resource { output } => {
                let mut file = std::fs::File::open(file_manager.file_path.clone())
                    .expect("Failed to open file");
                let resource_tree = match ResourceTree::get_resource_tree(
                    &mut file,
                    file_manager
                        .data_directory
                        .get_resource_directory_address()
                        .unwrap(),
                    &*file_manager.nt_head,
                    &file_manager.section_headers,
                    &file_manager.data_directory,
                ) {
                    Ok(resource_tree) => resource_tree,
                    Err(e) => {
                        println!("Error: {}", e.to_string());
                        return;
                    }
                };
                let output_path = PathBuf::from(output);
                match resource_tree.extract_resources(
                    &mut file,
                    &output_path,
                    &*file_manager.nt_head,
                    &file_manager.section_headers,
                    &file_manager.data_directory,
                ) {
                    Ok(extracted_files) => extracted_files,
                    Err(e) => {
                        println!("Error: {}", e.to_string());
                        return;
                    }
                };
            }
        }
    }
}
