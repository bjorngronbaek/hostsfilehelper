use regex::Regex;
use std::fmt;
use failure::_core::fmt::Formatter;

pub struct HostsFileLine {
    raw: String,
    pub ip: Option<String>,
    pub hosts: Option<String>,
    pub comment: Option<String>,
}

impl fmt::Display for HostsFileLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.raw)
    }
}

pub struct HostsFile {
    pub entries: Vec<HostsFileLine>,
}

impl HostsFileLine {
    pub fn from_string(line: &str) -> HostsFileLine {
        let raw = String::from(line);
        let line = line.trim();

        /* if trimmed line is empty, just return the raw line */
        if line == "" {
            return HostsFileLine::from_raw(raw);
        }

        /* if the trimmed line starts with # it is just a comment */
        let comment_regex: Regex = Regex::new(r"^#.*").unwrap();
        if comment_regex.is_match(line) {
            return HostsFileLine::from_comment(raw, String::from(line));
        }

        /* split string on # to get real and comment parts */
        let splits: Vec<String> = line.split("#").map(|s| s.to_string()).collect();
        let comment = splits.get(1).map(String::from);

        /* split on white space to get ip and host part */
        let real_parts: Vec<String> = splits.get(0).unwrap().split(" ").map(|s| s.to_string()).collect();

        let ip = real_parts.get(0).map(String::from);
        let hosts = real_parts.get(1).map(String::from);

        HostsFileLine {
            raw,
            ip,
            hosts,
            comment
        }
    }

    fn from_empty() -> HostsFileLine {
        HostsFileLine{
            raw: String::from(""),
            ip: None,
            hosts: None,
            comment: None,
        }
    }

    fn from_raw(raw: String) -> HostsFileLine {
        HostsFileLine{
            raw,
            ip: None,
            hosts: None,
            comment: None,
        }
    }

    fn from_comment(raw: String, comment: String) -> HostsFileLine {
        HostsFileLine{
            raw,
            ip: None,
            hosts: None,
            comment: Some(comment)
        }
    }

    pub fn contains_host(&self) -> bool {
        let has_ip;
        match &self.ip {
            Some(_ip) => has_ip = true,
            None => has_ip = false,
        }

        return has_ip
    }
}

impl HostsFile {
    pub fn from_string(s: &str) -> HostsFile {
        let lines: Vec<HostsFileLine> = s
            .lines()
            .map(|l| HostsFileLine::from_string(l))
            .collect::<Vec<HostsFileLine>>();

        HostsFile { entries: lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINE: &str = "127.0.0.1 bjgcybot-local.azurewebsites.net # local test site";
    static COMMENT_LINE: &str = "# some comment";

    #[test]
    fn hosts_file_line_raw_should_be_equal_to_input(){
        let h = HostsFileLine::from_string(TEST_LINE);
        assert_eq!(h.raw,TEST_LINE);
    }

    #[test]
    fn hosts_file_comment_line_should_contain_a_comment(){
        let h = HostsFileLine::from_string(COMMENT_LINE);
        assert_eq!(COMMENT_LINE,h.raw);
        assert!(h.comment.is_some());
        assert_eq!(COMMENT_LINE,h.comment.unwrap());
        assert!(h.ip.is_none());
        assert!(h.hosts.is_none());
    }

    #[test]
    fn single_line_contains_comment() {
        let h = HostsFileLine::from_string(TEST_LINE);
        assert!(h.comment.is_some());
        assert_eq!(" local test site",h.comment.unwrap());
    }

    #[test]
    fn single_line_contains_ip() {
        let h = HostsFileLine::from_string(TEST_LINE);
        assert!(h.ip.is_some());
        assert_eq!("127.0.0.1",h.ip.unwrap());
    }

    #[test]
    fn single_line_contains_hosts() {
        let h = HostsFileLine::from_string(TEST_LINE);
        assert!(h.hosts.is_some());
        assert_eq!("bjgcybot-local.azurewebsites.net",h.hosts.unwrap());
    }

    #[test]
    fn single_line_with_comment_line_break_should_return_one_line() {
        let h = HostsFile::from_string("\r");
        assert_eq!(h.entries.len(),1)
    }
}