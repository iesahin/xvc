use std::path::PathBuf;

use crate::error::Error;
use log::info;
use regex::Regex;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::R1NStore;

use crate::{pipeline::deps, XvcDependency, XvcParamFormat, XvcPipeline, XvcStep};

#[allow(clippy::too_many_arguments)]
pub fn cmd_step_dependency(
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    step_name: String,
    files: Option<Vec<String>>,
    directories: Option<Vec<String>>,
    globs: Option<Vec<String>>,
    params: Option<Vec<String>>,
    steps: Option<Vec<String>>,
    pipelines: Option<Vec<String>>,
    regexes: Option<Vec<String>>,
    lines: Option<Vec<String>>,
) -> Result<(), Error> {
    let current_dir = xvc_root.config().current_dir()?;
    let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
    let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, &step_name)?;

    let mut deps: Vec<XvcDependency> = Vec::new();

    if let Some(files) = files {
        for file in files {
            let path = XvcPath::new(xvc_root, current_dir, &PathBuf::from(file))?;
            deps.push(XvcDependency::File { path });
        }
    }

    if let Some(directories) = directories {
        for directory in directories {
            let path = XvcPath::new(xvc_root, current_dir, &PathBuf::from(directory))?;
            deps.push(XvcDependency::Directory { path });
        }
    }

    if let Some(globs) = globs {
        for glob in globs {
            deps.push(XvcDependency::Glob { glob });
        }
    }

    if let Some(params) = params {
        let param_splitter = Regex::new(r"((?P<param_file>.*)::)?(?P<param_name>.*)").unwrap();
        let default_param_file_name = deps::conf_params_file(xvc_root.config())?;
        for param in params {
            let captures = match param_splitter.captures(&param) {
                Some(captures) => captures,
                None => {
                    return Err(Error::InvalidParameterFormat { param });
                }
            };

            let param_file = match captures.name("param_file") {
                Some(param_file) => param_file.as_str(),
                None => default_param_file_name.as_str(),
            };
            let key = match captures.name("param_name") {
                Some(param_name) => param_name.as_str().to_string(),
                None => {
                    return Err(Error::InvalidParameterFormat { param });
                }
            };
            let pathbuf = PathBuf::from(param_file);
            let format = XvcParamFormat::from_path(&pathbuf);
            let path = XvcPath::new(xvc_root, current_dir, &pathbuf)?;
            deps.push(XvcDependency::Param { format, path, key });
        }
    }

    if let Some(pipelines) = pipelines {
        for pipeline_name in pipelines {
            let (dep_pipeline_e, dep_pipeline) = XvcPipeline::from_name(xvc_root, &pipeline_name)?;
            if dep_pipeline_e == pipeline_e {
                return Err(Error::PipelineCannotDependToItself);
            }
            deps.push(XvcDependency::Pipeline {
                name: dep_pipeline.name,
            });
        }
    }

    if let Some(steps) = steps {
        let current_step = step.clone();
        for step in steps {
            let step_val = XvcStep { name: step };
            if step_val == current_step {
                return Err(Error::StepCannotDependToItself);
            }
            // We intend to create the dependency link between steps with names, not entity ids.
            deps.push(XvcDependency::Step {
                name: step_val.name,
            });
        }
    }

    if let Some(regexes) = regexes {
        let regex_splitter = Regex::new(r"(?P<regex_file>[^:/]+):/(?P<regex>.+)").unwrap();
        for regex in regexes {
            let captures = match regex_splitter.captures(&regex) {
                Some(captures) => captures,
                None => {
                    return Err(Error::InvalidRegexFormat { regex });
                }
            };

            let regex_file = match captures.name("regex_file") {
                Some(regex_file) => regex_file.as_str(),
                None => {
                    return Err(Error::InvalidRegexFormat { regex });
                }
            };

            let regex_str = match captures.name("regex") {
                Some(regex_str) => regex_str.as_str().to_string(),
                None => {
                    return Err(Error::InvalidRegexFormat { regex });
                }
            };

            // Check if the supplied regexp is well formed
            if Regex::new(&regex_str).is_err() {
                return Err(Error::InvalidRegexFormat { regex: regex_str });
            }

            let pathbuf = PathBuf::from(regex_file);
            let path = XvcPath::new(xvc_root, current_dir, &pathbuf)?;
            deps.push(XvcDependency::Regex {
                path,
                regex: regex_str,
            });
        }
    }

    if let Some(lines) = lines {
        let lines_splitter =
            Regex::new(r"(?P<file>[^:]+)::(?P<begin>[0-9]*)-(?P<end>[0-9]*)").unwrap();
        for line in lines {
            let line_c = line.clone();
            let captures = match lines_splitter.captures(&line_c) {
                Some(captures) => captures,
                None => {
                    return Err(Error::InvalidLinesFormat { line });
                }
            };

            let lines_file = match captures.name("file") {
                Some(lines_file) => lines_file.as_str(),
                None => {
                    return Err(Error::InvalidLinesFormat { line });
                }
            };

            let lines_begin_str = match captures.name("begin") {
                Some(begin_str) => begin_str.as_str().to_string(),
                None => {
                    return Err(Error::InvalidLinesFormat { line });
                }
            };

            let lines_end_str = match captures.name("end") {
                Some(end_str) => end_str.as_str().to_string(),
                None => {
                    return Err(Error::InvalidLinesFormat { line });
                }
            };

            let begin = match lines_begin_str.len() {
                0 => 0usize,
                _ => lines_begin_str
                    .parse::<usize>()
                    .map_err(|_| Error::InvalidLinesFormat { line: line.clone() })?,
            };

            let end = match lines_end_str.len() {
                0 => usize::MAX,
                _ => lines_end_str
                    .parse::<usize>()
                    .map_err(|_| Error::InvalidLinesFormat { line: line.clone() })?,
            };

            let pathbuf = PathBuf::from(lines_file);
            let path = XvcPath::new(xvc_root, current_dir, &pathbuf)?;
            deps.push(XvcDependency::Lines { path, begin, end });
        }
    }

    xvc_root.with_r1nstore_mut(|rs: &mut R1NStore<XvcStep, XvcDependency>| {
        for d in &deps {
            rs.insert(step_e, step.clone(), xvc_root.new_entity(), d.clone());
            info!("Adding {:?}", step.clone());
        }
        Ok(())
    })?;

    Ok(())
}
