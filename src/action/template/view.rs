use log;
use std::process::exit;

use crate::action;
use crate::cli::input::select;
use crate::config::Config;
use crate::out;
use clap::ArgMatches;

pub fn view(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");

  // Get repository
  let repository = match action::get_repository(&config, repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    },
  };

  // Check if templates exist
  if repository.get_templates().len() <= 0 {
    eprintln!("No templates exist in repository: {}", repository.config.name);
    exit(1);
  }

  // Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match select("template", &templates) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    }
  } else {
    String::from(template_name.unwrap())
  };

  // Get the template
  let template = match repository.get_template_by_name(&template_name) {
    Ok(template) => template,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  out::info::display_template(template);
}
