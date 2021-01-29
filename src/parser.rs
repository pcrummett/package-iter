use crate::error::*;
use crate::package::*;
use nom::{
    character::complete::{alphanumeric1, char, multispace0, multispace1, not_line_ending},
    combinator::not,
    multi::many1,
    sequence::{preceded, terminated},
};
use std::convert::TryFrom;

impl TryFrom<&str> for Package {
    type Error = Error;

    fn try_from(data: &str) -> Result<Self> {
        let mut pkg = Package::default();

        for x in Tokenizer::from(data) {
            match x.name.to_lowercase().as_str() {
                "name" if x.values.first().is_some() => pkg.name = x.values[0].to_owned(),
                "base" if x.values.first().is_some() => pkg.base = x.values[0].to_owned(),
                "filename" if x.values.first().is_some() => pkg.filename = x.values[0].to_owned(),
                "version" if x.values.first().is_some() => pkg.version = x.values[0].to_owned(),
                "desc" if x.values.first().is_some() => pkg.desc = x.values[0].to_owned(),
                "url" if x.values.first().is_some() => pkg.url = x.values[0].to_owned(),
                "csize" if x.values.first().is_some() => {
                    pkg.size = x.values[0]
                        .parse::<u64>()
                        .map_err(|_| Error::PackageParseSize)?
                }
                "isize" if x.values.first().is_some() => {
                    pkg.isize = x.values[0]
                        .parse::<u64>()
                        .map_err(|_| Error::PackageParseSize)?
                }
                "arch" if x.values.first().is_some() => pkg.arch = x.values[0].to_owned(),
                "md5sum" if x.values.first().is_some() => pkg.md5sum = x.values[0].to_owned(),
                "sha256sum" if x.values.first().is_some() => pkg.sha256sum = x.values[0].to_owned(),
                "pgpsig" if x.values.first().is_some() => pkg.pgpsig = x.values[0].to_owned(),
                "builddate" if x.values.first().is_some() => {
                    pkg.build_date = x.values[0].to_owned()
                }
                "packager" if x.values.first().is_some() => pkg.packager = x.values[0].to_owned(),
                "license" if x.values.first().is_some() => {
                    pkg.licenses = x.values.iter().map(|x| x.to_string()).collect()
                }
                "provides" if x.values.first().is_some() => {
                    pkg.provides = x.values.iter().map(|x| x.to_string()).collect()
                }
                "depends" if x.values.first().is_some() => {
                    pkg.depends = x.values.iter().map(|x| x.to_string()).collect()
                }
                "makedepends" if x.values.first().is_some() => {
                    pkg.make_depends = x.values.iter().map(|x| x.to_string()).collect()
                }
                "optionaldepends" if x.values.first().is_some() => {
                    pkg.optional_depends = x.values.iter().map(|x| x.to_string()).collect()
                }
                "checkdepends" if x.values.first().is_some() => {
                    pkg.check_depends = x.values.iter().map(|x| x.to_string()).collect()
                }
                _ => return Err(Error::PackagePropertyMissing(x.name.to_string())),
            }
        }

        // TODO: Validate required properties were hit

        Ok(pkg)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Token<'a> {
    pub(crate) name: &'a str,
    pub(crate) values: Vec<&'a str>,
}

/// Tokenizer converts a Arch Linux package description string into a set of Tokens
pub(crate) struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(data: &'a str) -> Self {
        Self { input: data }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    /// Iterator on the tokens
    fn next(&mut self) -> Option<Self::Item> {
        match token(self.input) {
            Ok((i, x)) => {
                self.input = i;
                Some(x)
            }
            Err(_) => None,
        }
    }
}

/// Parse out a token
fn token(input: &str) -> nom::IResult<&str, Token> {
    let (input, name) = name(input)?;
    let (input, values) = many1(values)(input)?;
    Ok((input, Token { name, values }))
}

/// Parse token values
fn values(input: &str) -> nom::IResult<&str, &str> {
    preceded(
        multispace0,
        preceded(not(char('%')), terminated(not_line_ending, multispace1)),
    )(input)
}

/// Parse token name
fn name(input: &str) -> nom::IResult<&str, &str> {
    preceded(
        multispace0,
        preceded(char('%'), terminated(alphanumeric1, char('%'))),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiple() {
        assert_eq!(
            Package::try_from(
                "%NAME%\nsupertux\n\n%FILENAME%\nsupertux-0.6.2-3-x86_64.pkg.tar.zst\n"
            )
            .unwrap(),
            Package {
                name: "supertux".to_owned(),
                filename: "supertux-0.6.2-3-x86_64.pkg.tar.zst".to_owned(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_parse_single() {
        assert_eq!(
            Package::try_from("%FILENAME%\nsupertux-0.6.2-3-x86_64.pkg.tar.zst\n").unwrap(),
            Package {
                filename: "supertux-0.6.2-3-x86_64.pkg.tar.zst".to_owned(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%NAME%\nsupertux\n").unwrap(),
            Package {
                name: "supertux".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%BASE%\nsupertux\n").unwrap(),
            Package {
                base: "supertux".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%VERSION%\n0.6.2-3\n").unwrap(),
            Package {
                version: "0.6.2-3".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%DESC%\nA classic 2D jump'n'run sidescroller\n").unwrap(),
            Package {
                desc: "A classic 2D jump'n'run sidescroller".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%CSIZE%\n157518488\n").unwrap(),
            Package {
                size: 157518488u64,
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%ISIZE%\n229551408\n").unwrap(),
            Package {
                isize: 229551408u64,
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%MD5SUM%\nbc9013783217dff3081d4daa4c222c32\n").unwrap(),
            Package {
                md5sum: "bc9013783217dff3081d4daa4c222c32".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%SHA256SUM%\nc1d14f744...05a866f40d7967a57db\n").unwrap(),
            Package {
                sha256sum: "c1d14f744...05a866f40d7967a57db".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%PGPSIG%\niQIzBAABC/rcNxzD9nZZrjEDXSBjbo=\n").unwrap(),
            Package {
                pgpsig: "iQIzBAABC/rcNxzD9nZZrjEDXSBjbo=".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%URL%\nhttps://github.com/supertux/supertux\n").unwrap(),
            Package {
                url: "https://github.com/supertux/supertux".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%LICENSE%\nGPL\n").unwrap(),
            Package {
                licenses: vec!["GPL".to_string()],
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%ARCH%\nx86_64\n").unwrap(),
            Package {
                arch: "x86_64".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%BUILDDATE%\n1607789295\n").unwrap(),
            Package {
                build_date: "1607789295".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%PACKAGER%\nFelix Yan <felixonmars@archlinux.org>\n").unwrap(),
            Package {
                packager: "Felix Yan <felixonmars@archlinux.org>".to_string(),
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%DEPENDS%\ncurl\nopenal\nlibvorbis\n\n").unwrap(),
            Package {
                depends: vec![
                    "curl".to_string(),
                    "openal".to_string(),
                    "libvorbis".to_string()
                ],
                ..Default::default()
            }
        );
        assert_eq!(
            Package::try_from("%MAKEDEPENDS%\ncmake\nboost\n\n").unwrap(),
            Package {
                make_depends: vec!["cmake".to_string(), "boost".to_string(),],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_multiple_tokens() {
        let (i, x) = token("%name1%\nval1\n\n%name2%\nval2\n\n").unwrap();
        assert_eq!(
            (i, x),
            (
                "%name2%\nval2\n\n",
                Token {
                    name: "name1",
                    values: vec!["val1"]
                }
            )
        );
        let (i, x) = token(i).unwrap();
        assert_eq!(
            (i, x),
            (
                "",
                Token {
                    name: "name2",
                    values: vec!["val2"]
                }
            )
        );
    }

    #[test]
    fn test_single_token() {
        assert_eq!(
            token("%foo%\nfoo\n"),
            Ok((
                "",
                Token {
                    name: "foo",
                    values: vec!["foo"]
                }
            ))
        );
        assert_eq!(
            token("foo%\nfoo\n"),
            Err(nom::Err::Error(nom::error::Error {
                input: "foo%\nfoo\n",
                code: nom::error::ErrorKind::Char
            }))
        );
        assert_eq!(
            token("%FILENAME%\nsupertux-0.6.2-3-x86_64.pkg.tar.zst\n"),
            Ok((
                "",
                Token {
                    name: "FILENAME",
                    values: vec!["supertux-0.6.2-3-x86_64.pkg.tar.zst"]
                }
            ))
        );
    }

    #[test]
    fn test_value() {
        // Single item
        assert_eq!(values("foo1\n\n"), Ok(("", "foo1")));
        assert_eq!(values("foo1\r\n\n"), Ok(("", "foo1")));
        assert_eq!(values("\nfoo1\r\n\n"), Ok(("", "foo1")));
        assert_eq!(values("\n\rfoo1\r\n\n"), Ok(("", "foo1")));
        assert_eq!(values("\n\r  foo1\r\n\n"), Ok(("", "foo1")));
        assert_eq!(values("foo1\nfoo2\n"), Ok(("foo2\n", "foo1")));

        // Multi items
        let (i, x) = values("foo1\nfoo2\n").unwrap();
        assert_eq!((i, x), ("foo2\n", "foo1"));
        let (i, x) = values(i).unwrap();
        assert_eq!((i, x), ("", "foo2"));

        let (i, x) = values("\nfoo1\nfoo2\n").unwrap();
        assert_eq!((i, x), ("foo2\n", "foo1"));
        let (i, x) = values(i).unwrap();
        assert_eq!((i, x), ("", "foo2"));

        let (i, x) = values("\nfoo1\r\nfoo2\r\n").unwrap();
        assert_eq!((i, x), ("foo2\r\n", "foo1"));
        let (i, x) = values(i).unwrap();
        assert_eq!((i, x), ("", "foo2"));

        let (i, x) = values("\n  foo1\r\nfoo2\r\n").unwrap();
        assert_eq!((i, x), ("foo2\r\n", "foo1"));
        let (i, x) = values(i).unwrap();
        assert_eq!((i, x), ("", "foo2"));
    }

    #[test]
    fn test_name() {
        assert_eq!(name("%foo%"), Ok(("", "foo")));
        assert_eq!(name("%FOO%"), Ok(("", "FOO")));
        assert_eq!(name(" %foo%"), Ok(("", "foo")));
        assert_eq!(name(" \n%foo%"), Ok(("", "foo")));
        assert_eq!(name(" \n%foo%\n"), Ok(("\n", "foo")));
    }
}
