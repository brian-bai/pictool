use clap::Parser;
use pictool::{Action, Transformer};
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Picture manipulating program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    /// File path of the source pictures
    #[arg(short, long)]
    source: String,

    /// File path of the target pictures
    #[arg(short, long)]
    target: String,

    /// Target picture size, current requirement of meshphormer is 224x224
    #[arg(short, long, default_value_t=224)]
    psize: u32,

    #[command(subcommand)]
    action: Action,
}

fn main() -> Result<()> {
    let args = Args::parse();
    //TODO: replace println! with log
    println!("Source Directory: {} \nTarget Directory: {} \nPicture Size: {}", args.source, args.target, args.psize);

    for entry in fs::read_dir(args.source)? {
        let entry = entry?;
        let path = entry.path();

        let file_stem = if let Some(stem) = path.file_stem() {
            stem
        } else {
            //TODO: replace with log
            println!("No File Name!");
            continue;
        };

        //TODO: try display action
        let action = match args.action {
            Action::Resize => "resize",
            Action::Flip => "flip",
            Action::Rotate { degree: _ } => "rotate",
        };

        if let Some(name) = file_stem.to_str() {

            //TODO: replace with format!
            let mut target_name = String::from(name);
            target_name.push('_');
            target_name.push_str(action);

            let mut suffix = String::new();

            if let Action:: Rotate {degree} = args.action {
                suffix.push('_');
                suffix.push_str(&degree.to_string());
            }

            //TODO: remove hard code
            suffix.push_str(".jpg");
            target_name.push_str(&suffix);

            let target_path = Path::new(&args.target).join(target_name);

            if let Some(src) = path.to_str() {
                match args.action.transform(src, args.psize) {
                    Ok(img) => {
                        if let Err(err) = img.save(&target_path) {
                            println!("Error when save {:?}: {}", target_path, err);
                        }
                        //TODO: else log
                    }
                    Err(_) => {
                        //TODO: log
                        println!("Error when {:?} {:?}", args.action, path.to_str());
                    }
                }
            }
        }

    }


    Ok(())
}
