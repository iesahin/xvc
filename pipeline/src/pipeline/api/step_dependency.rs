use std::cell::RefCell;
use std::path::PathBuf;

use crate::deps::{LinesDep, RegexDep};
use crate::error::{Error, Result};
use crate::pipeline::deps::file::FileDep;
use crate::pipeline::deps::generic::GenericDep;
use crate::pipeline::deps::glob::GlobDep;
use crate::pipeline::deps::glob_items::GlobItemsDep;
use crate::pipeline::deps::line_items::LineItemsDep;
use crate::pipeline::deps::regex_items::RegexItemsDep;
use crate::pipeline::deps::step::StepDep;
use crate::pipeline::deps::url::UrlDigestDep;
use crate::pipeline::deps::ParamDep;
use crate::pipeline::step::StepSubCommand;

use regex::Regex;
use url::Url;
use xvc_core::{XvcPath, XvcRoot};
use xvc_ecs::{R1NStore, XvcEntity};
use xvc_logging::{debug, watch, XvcOutputSender};
use xvc_walker::AbsolutePath;

use crate::{pipeline::deps, XvcDependency, XvcParamFormat, XvcPipeline, XvcStep};

/// Entry point for `xvc pipeline step dependency` command.
/// Add a set of new dependencies to the given step in the pipeline.
///
/// Unlike other entry points, this receives the options directly to avoid a long list of
/// parameters.
pub fn cmd_step_dependency(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    pipeline_name: &str,
    cmd_opts: StepSubCommand,
) -> Result<()> {
    if let StepSubCommand::Dependency {
        step_name,
        generics,
        urls,
        files,
        steps,
        glob_items,
        globs,
        params,
        regex_items,
        regexes,
        line_items,
        lines,
    } = cmd_opts
    {
        XvcDependencyList::new(output_snd, xvc_root, pipeline_name, &step_name)?
            .files(files)?
            .glob_items(glob_items)?
            .globs(globs)?
            .params(params)?
            .steps(steps)?
            .generic_commands(generics)?
            .regexes(regexes)?
            .regex_items(regex_items)?
            .lines(lines)?
            .line_items(line_items)?
            .urls(urls)?
            .record()
    } else {
        Err(anyhow::anyhow!("This method is only for StepSubCommand::Dependency").into())
    }
}
///
/// Parses dependencies using member functions, in order to avoid a single function with a lot of parameters.
pub struct XvcDependencyList<'a> {
    output_snd: &'a XvcOutputSender,
    xvc_root: &'a XvcRoot,
    current_dir: &'a AbsolutePath,
    step_e: XvcEntity,
    step: XvcStep,
    deps: RefCell<Vec<XvcDependency>>,
}

impl<'a> XvcDependencyList<'a> {
    /// Create a new dependency list.
    ///
    /// Finds the pipeline and the step with their names, and creates a new dependency list.
    pub fn new(
        output_snd: &'a XvcOutputSender,
        xvc_root: &'a XvcRoot,
        pipeline_name: &'a str,
        step_name: &'a str,
    ) -> Result<Self> {
        let current_dir = xvc_root.config().current_dir()?;
        let (pipeline_e, _) = XvcPipeline::from_name(xvc_root, pipeline_name)?;
        let (step_e, step) = XvcStep::from_name(xvc_root, &pipeline_e, step_name)?;
        Ok(Self {
            xvc_root,
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
                let file_dep = FileDep::new(XvcPath::new(
                    self.xvc_root,
                    current_dir,
                    &PathBuf::from(file),
                )?);
                deps.push(XvcDependency::File(file_dep));
            }
        }
        Ok(self)
    }

    /// Add glob dependencies.
    pub fn glob_items(&mut self, glob_items: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(globs) = glob_items {
            let mut deps = self.deps.borrow_mut();
            for glob in globs {
                let glob_dep = GlobItemsDep::new(glob);
                deps.push(XvcDependency::GlobItems(glob_dep));
            }
        }
        Ok(self)
    }

    /// Add glob digest dependencies.
    pub fn globs(&mut self, globs: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(globs) = globs {
            let mut deps = self.deps.borrow_mut();
            for glob in globs {
                let glob_dep = GlobDep::new(glob);
                deps.push(XvcDependency::Glob(glob_dep));
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
                let param_dep = ParamDep::new(&path, Some(format), key)?;
                deps.push(XvcDependency::Param(param_dep));
            }
        }
        Ok(self)
    }

    /// Add pipeline dependencies via their names.
    ///
    /// Note that, these are not implemented yet in the `run` command.
    pub fn generic_commands(&mut self, generics: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(generics) = generics {
            let mut deps = self.deps.borrow_mut();
            for generic_command in generics {
                let generic_dep = GenericDep::new(generic_command);
                deps.push(XvcDependency::Generic(generic_dep));
            }
        }
        Ok(self)
    }

    /// Add step dependencies via their names.
    pub fn steps(&mut self, steps: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(steps) = steps {
            let mut deps = self.deps.borrow_mut();
            for step_name in steps {
                let step_dep = StepDep::new(step_name);
                deps.push(XvcDependency::Step(step_dep));
            }
        }
        Ok(self)
    }

    fn split_regex_expressions(
        &self,
        regexes: Option<Vec<String>>,
    ) -> Result<Vec<(XvcPath, String)>> {
        let mut vec = Vec::new();
        let current_dir = self.xvc_root.config().current_dir()?;
        if let Some(regexes) = regexes {
            let regex_splitter = Regex::new(r"(?P<regex_file>[^:/]+):/(?P<regex>.+)").unwrap();
            for regex in regexes {
                let captures = match regex_splitter.captures(&regex) {
                    Some(captures) => captures,
                    None => {
                        return Err(Error::InvalidRegexFormat { regex });
                    }
                };
                watch!(captures);
                let regex_file = match captures.name("regex_file") {
                    Some(regex_file) => regex_file.as_str(),
                    None => {
                        return Err(Error::InvalidRegexFormat { regex });
                    }
                };
                watch!(regex_file);

                let regex_str = match captures.name("regex") {
                    Some(regex_str) => regex_str.as_str().to_string(),
                    None => {
                        return Err(Error::InvalidRegexFormat { regex });
                    }
                };
                watch!(regex_str);

                // Check if the supplied regexp is well formed
                if Regex::new(&regex_str).is_err() {
                    return Err(Error::InvalidRegexFormat { regex: regex_str });
                }

                let pathbuf = PathBuf::from(regex_file);
                let path = XvcPath::new(self.xvc_root, current_dir, &pathbuf)?;
                vec.push((path, regex_str));
            }
        }

        Ok(vec)
    }

    /// Add regex dependencies.
    ///
    /// Regex dependencies must be in the form `regex_file:/(?P<regex>.+)`.
    pub fn regex_items(&mut self, regex_items: Option<Vec<String>>) -> Result<&mut Self> {
        let regex_splits = self.split_regex_expressions(regex_items)?;
        {
            let mut deps = self.deps.borrow_mut();
            regex_splits.into_iter().for_each(|(path, regex_str)| {
                let regex_dep = RegexItemsDep::new(path, regex_str);
                deps.push(XvcDependency::RegexItems(regex_dep));
            });
        }
        Ok(self)
    }

    /// Add regex dependencies.
    ///
    /// Regex dependencies must be in the form `regex_file:/(?P<regex>.+)`.
    pub fn regexes(&mut self, regexes: Option<Vec<String>>) -> Result<&mut Self> {
        let regex_splits = self.split_regex_expressions(regexes)?;
        {
            let mut deps = self.deps.borrow_mut();
            regex_splits.into_iter().for_each(|(path, regex_str)| {
                let regex_dep = RegexDep::new(path, regex_str);
                deps.push(XvcDependency::Regex(regex_dep));
            });
        }
        Ok(self)
    }

    pub fn urls(&mut self, urls: Option<Vec<String>>) -> Result<&mut Self> {
        if let Some(urls) = urls {
            let mut deps = self.deps.borrow_mut();
            for url in urls {
                let url = Url::parse(&url)?;
                let url_dep = UrlDigestDep::new(url);
                deps.push(XvcDependency::UrlDigest(url_dep));
            }
        }
        Ok(self)
    }

    /// Add lines dependencies.
    /// Lines dependencies must be in the form `file::begin-end`, where begin
    /// and end are digit strings. If begin is omitted, it defaults to 0. If end
    /// is omitted, it defaults to [usize::MAX]
    fn split_line_options(
        &mut self,
        lines: Option<Vec<String>>,
    ) -> Result<Vec<(XvcPath, usize, usize)>> {
        let mut vec = Vec::new();
        let current_dir = self.current_dir;
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
                let path = XvcPath::new(self.xvc_root, current_dir, &pathbuf)?;
                vec.push((path, begin, end));
            }
        }
        Ok(vec)
    }

    pub fn lines(&mut self, lines: Option<Vec<String>>) -> Result<&mut Self> {
        let lines_options = self.split_line_options(lines)?;
        {
            let mut deps = self.deps.borrow_mut();
            lines_options.into_iter().for_each(|(path, begin, end)| {
                let lines_dep = LinesDep::new(path, begin, end);
                deps.push(XvcDependency::Lines(lines_dep));
            });
        }
        Ok(self)
    }
    pub fn line_items(&mut self, line_items: Option<Vec<String>>) -> Result<&mut Self> {
        let lines_options = self.split_line_options(line_items)?;
        {
            let mut deps = self.deps.borrow_mut();
            lines_options.into_iter().for_each(|(path, begin, end)| {
                let lines_dep = LineItemsDep::new(path, begin, end);
                deps.push(XvcDependency::LineItems(lines_dep));
            });
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

