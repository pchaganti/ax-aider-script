use anyhow::Ok;
use args::Args;
use clap::Parser;

use command_template::CommandTemplate;

mod aider_command;
mod args;
mod command_template;
mod filters;
mod markdown_doc;
mod str;

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();

    let template = args.read_template()?;
    let template_name = args.get_template_name();
    let cmd_template = CommandTemplate::parse_with_name(&template, template_name)?;

    let aider_cmd = cmd_template.apply_args(&args.template_arguments)?;

    if args.preview_message {
        println!("Generated message:");
        println!("------------------");
        println!();
        println!("{}", aider_cmd.message);

        return Ok(());
    }

    let mut cmd = aider_cmd.to_shell_command();
    cmd.status()?;

    Ok(())
}
