use std::convert::From;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;
use ucl;
use url::{ParseError, Url};

#[derive(PartialEq, Debug)]
pub enum RepoError {
    URLError(ParseError),
    UCIError,
    NameError,
}

impl From<ParseError> for RepoError {
    fn from(e: ParseError) -> RepoError {
        RepoError::URLError(e)
    }
}

#[derive(Debug, PartialEq)]
pub struct Repo {
    pub name: String,
    pub url: Option<Url>,
    pub enabled: bool,
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            name: String::new(),
            url: None,
            enabled: true,
        }
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let url_section = match self.url {
            Some(ref url) => format!("url:{}", url),
            _ => "".to_string(),
        };
        write!(
            f,
            "{} [enabled:{} {}]",
            self.name, self.enabled, url_section
        )
    }
}

/// internal function that clean up a line, removing comments and whitespaces
fn line_trim(st: &str) -> String {
    let mut ret = String::new();
    for c in st.chars().filter(|&c| !c.is_whitespace()) {
        if c == '#' {
            return ret;
        }
        ret.push(c);
    }
    ret
}

/// Input has to be already trimmed
fn get_section_name(st: &str) -> Option<String> {
    let idx_desc_start = match st.find('{') {
        None => return None,
        Some(x) => x,
    };
    let idx_first_colon = match st.find(':') {
        None => return None,
        Some(x) => x,
    };
    if idx_first_colon > idx_desc_start {
        return None;
    }
    match st.splitn(2, ':').nth(0) {
        None => None,
        Some(x) => Some(x.to_string()),
    }
}

/// it parses one repository description
pub fn parse_string(entry: String) -> Result<Repo, RepoError> {
    let trimmed = entry
        .lines()
        .map(line_trim)
        .fold(String::new(), |acc, x| acc + &x);
    let mut r = Repo::new();
    if let Some(name) = get_section_name(trimmed.as_ref()) {
        r.name = name;
    } else {
        return Err(RepoError::NameError);
    }
    let parsy = ucl::Parser::new();
    if let Ok(config) = parsy.parse(trimmed) {
        let url_path = r.name.clone() + ".url";
        if let Some(url_obj) = config.fetch_path(url_path) {
            if let Some(url) = url_obj.as_string() {
                r.url = Some(Url::parse(&url)?);
            }
        }
        if let Some(enabled_obj) = config.fetch_path(r.name.clone() + ".enabled") {
            if let Some(enabled) = enabled_obj.as_bool() {
                r.enabled = enabled;
            }
        }
        Ok(r)
    } else {
        Err(RepoError::UCIError)
    }
}

pub fn multi_parse_filename(filename: &Path) -> Vec<Repo> {
    let mut repos: Vec<Repo> = Vec::new();
    if let Ok(f) = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(filename)
    {
        let mut line = String::new();
        let mut entry = String::new();
        let buf_reader = &mut BufReader::new(f);
        while let Ok(x) = buf_reader.read_line(&mut line) {
            if x == 0 {
                break;
            }
            let trimmed = line_trim(&line);
            entry += &trimmed;
            line.clear();
            let open = entry.chars().filter(|x| *x == '{').count();
            if open != 0 && open == entry.chars().filter(|x| *x == '}').count() {
                if let Ok(x) = parse_string(entry.clone()) {
                    merge_repo(&mut repos, x);
                    entry.clear();
                }
            }
        }
    }
    repos
}

/// Parse a string, containing only one repo description
impl From<String> for Repo {
    fn from(s: String) -> Repo {
        match parse_string(s) {
            Ok(x) => x,
            _ => Repo::new(),
        }
    }
}

pub fn merge_repo(v: &mut Vec<Repo>, r: Repo) {
    if let Some(x) = v.iter().position(|z| z.name == r.name) {
        v[x].enabled = r.enabled;
        if r.url.is_some() {
            v[x].url = r.url;
        }
    } else {
        v.push(r);
    }
}

