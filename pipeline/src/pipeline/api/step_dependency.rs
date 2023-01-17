use std::cell::RefCell;
use std::path::PathBuf;

use crate::error::{Error, Result};
use crossbeam_channel::Sender;
use regex::Regex;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::{R1NStore, XvcEntity};
use xvc_logging::{debug, XvcOutputLine};
use xvc_walker::AbsolutePath;

use crate::{pipeline::deps, XvcDependency, XvcParamFormat, XvcPipeline, XvcStep};

/// Entry point for `xvc pipeline step dependency` command.
/// Add a set of new dependencies to the given step in the pipeline.
///
/// Parses dependencies using member functions, in order to avoid a single function with a lot of parameters.
pub struct XvcDependencyList<'a> {
    output_snd: &'a Sender<XvcOutputLine>,
    xvc_root: &'a XvcRoot,
    current_dir: &'a AbsolutePath,
    pipeline_e: XvcEntity,
    step_e: XvcEntity,
    step: XvcStep,
    deps: RefCell<Vec<XvcDependency>>,
}

impl<'a> XvcDependencyList<'a> {
    /// Create a new dependency list.
    ///
    /// Finds the pipeline and the step with their names, and creates a new dependency list.
    pub fn new(
        output_snd: &'a Sender<XvcOutputLine>,
        xvc_root: &'a XvcRoot,
        pipeline_name: &'a str,
        step_name: &'a str,
    ) -> Result<Self> {
        let current_dir = xvc_root.config().current_dir()?;
        let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
        let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, &step_name)?;
        Ok(Self {
            xvc_root,
            pipeline_e,
            step_e,
            step,
            deps: RefCell::new(Vec::new()),
            output_snd,
            current_dir,
        })
    }

    /// Add file dependencies
    pub fn files(&mut self, files: Option<Vec<String>>) -> Result<&mut Self> {
        let current_dir = self.current_dir;
        if let Some(files) = files {
            let mut deps = self.deps.borrow_mut();
            for file in files {
                let path = XvcPath::new(self.xvc_root, current_dir, &PathBuf::from(file))?;
                deps.push(XvcDependency::File { path });
            }
        }
        Ok(self)
    }

    /// Add directory dependencies
    pub fn directories(&mut self, directories: Option<Vec<String>>) -> Result<&mut Self> {
        let current_dir = self.current_dir;
        if let Some(directories) = directories {
            let mut deps = self.deps.borrow_mut();
            for directory in directories {
                let path = XvcPath::new(self.xvc_root, current_dir, &PathBuf::from(directory))?;
                deps.push(XvcDependency::Directory { path });
            }
        }
        Ok(self)
    }

    /// Add glob dependencies.
    pub fn globs(&mut self, globs: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(globs) = globs {
            let mut deps = self.deps.borrow_mut();
            for glob in globs {
                deps.push(XvcDependency::Glob { glob });
            }
        }
        Ok(self)
    }

    /// Add param dependencies.
    ///
    /// Param dependencies must be in the form `param_name` or
    /// `param_file::param_name`. In the first form, `param_file` is retrieved
    /// from [`deps::conf_params_file`].
    pub fn params(&mut self, params: Option<Vec<String>>) -> Result<&mut Self> {
        let current_dir = self.current_dir;
        if let Some(params) = params {
            let param_splitter = Regex::new(r"((?P<param_file>.*)::)?(?P<param_name>.*)").unwrap();
            let default_param_file_name = deps::conf_params_file(self.xvc_root.config())?;
            let mut deps = self.deps.borrow_mut();
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
                let path = XvcPath::new(self.xvc_root, current_dir, &pathbuf)?;
                deps.push(XvcDependency::Param { format, path, key });
            }
        }
        Ok(self)
    }

    /// Add pipeline dependencies via their names.
    ///
    /// Note that, these are not implemented yet in the `run` command.
    pub fn pipelines(&mut self, pipelines: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(pipelines) = pipelines {
            let mut deps = self.deps.borrow_mut();
            for pipeline_name in pipelines {
                let (dep_pipeline_e, dep_pipeline) =
                    XvcPipeline::from_name(self.xvc_root, &pipeline_name)?;
                if dep_pipeline_e == self.pipeline_e {
                    return Err(Error::PipelineCannotDependToItself);
                }
                deps.push(XvcDependency::Pipeline {
                    name: dep_pipeline.name,
                });
            }
        }
        Ok(self)
    }

    /// Add step dependencies via their names.
    pub fn steps(&mut self, steps: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(steps) = steps {
            let mut deps = self.deps.borrow_mut();
            for step_name in steps {
                let (dep_step_e, dep_step) =
                    XvcStep::from_name(self.xvc_root, &self.pipeline_e, &step_name)?;
                if dep_step_e == self.step_e {
                    return Err(Error::StepCannotDependToItself);
                }
                deps.push(XvcDependency::Step {
                    name: dep_step.name,
                });
            }
        }
        Ok(self)
    }

    /// Add regex dependencies.
    ///
    /// Regex dependencies must be in the form `regex_file:/(?P<regex>.+)`.
    pub fn regexes(&mut self, regexes: Option<Vec<String>>) -> Result<&mut Self> {
        let current_dir = self.xvc_root.config().current_dir()?;
        if let Some(regexes) = regexes {
            let regex_splitter = Regex::new(r"(?P<regex_file>[^:/]+):/(?P<regex>.+)").unwrap();
            let mut deps = self.deps.borrow_mut();
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
                let path = XvcPath::new(self.xvc_root, current_dir, &pathbuf)?;
                deps.push(XvcDependency::Regex {
                    path,
                    regex: regex_str,
                });
            }
        }

        Ok(self)
    }

    /// Add lines dependencies.
    /// Lines dependencies must be in the form `file::begin-end`, where begin
    /// and end are digit strings. If begin is omitted, it defaults to 0. If end
    /// is omitted, it defaults to [usize::MAX]
    pub fn lines(&mut self, lines: Option<Vec<String>>) -> Result<&mut Self> {
        let current_dir = self.current_dir;
        if let Some(lines) = lines {
            let mut deps = self.deps.borrow_mut();
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
                let path = XvcPath::new(self.xvc_root, current_dir, &pathbuf)?;
                deps.push(XvcDependency::Lines { path, begin, end });
            }
        }
        Ok(self)
    }

    /// Records dependencies the store, as childs of `self.step`.
    pub fn record(&self) -> Result<()> {
        self.xvc_root
            .with_r1nstore_mut(|rs: &mut R1NStore<XvcStep, XvcDependency>| {
                let output_snd = self.output_snd;
                for d in self.deps.borrow().iter() {
                    debug!(output_snd, "Adding {:?}", &d);
                    rs.insert(
                        self.step_e,
                        self.step.clone(),
                        self.xvc_root.new_entity(),
                        d.clone(),
                    );
                }
                Ok(())
            })?;

        Ok(())
    }
}
