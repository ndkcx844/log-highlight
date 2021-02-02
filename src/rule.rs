use std::fs::File;
use std::io::{Read, BufReader};
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

const DEFAULT_RULES: &str = r###"
# pattern format is (HEADER)(TARGET)(TAILER)
# only TARGET will be colorized.
# escape is needed in double-quoted string   -> "tab\\tbefore"
# but not needed in multiline literal string -> '''tab\tbefore'''

# date
[[rules]]
pattern = '''($HEAD)([0-9]{4}-[0-9]{2}-[0-9]{2}|[0-9]{3}/[0-9]{2}/[0-9]{2})($TAIL)'''
color   = "$CADETBLUE"

[[rules]]
pattern = '''($HEAD)([A-Z][a-z][a-z] [ 1-9][0-9])($TAIL)'''
color   = "$CADETBLUE"

# time
[[rules]]
pattern = '''(^|\s)([0-9]{2}:[0-9]{2}:[0-9]{2} \+[0-2][0-9]{3}|[0-9]{2}:[0-9]{2}:[0-9]{2})([^\w:]|\r|$)'''
color   = "$SALMON"

[[rules]]
pattern = '''(^|\s)([0-9]{2}:[0-9]{2}:[0-9]{2}\.[0-9]{0,6} \+[0-2][0-9]{3}|[0-9]{2}:[0-9]{2}:[0-9]{2}\.[0-9]{0,6})([^\w:]|\r|$)'''
color   = "$SALMON"

# mail address
[[rules]]
pattern = '''($HEAD)([\w.-]+@[\w.-]+)($TAIL)'''
color   = "$DARKKHAKI"

# ip address
[[rules]]
pattern = '''()([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}(\/[0-9]{1,2}|[:.][0-9]{1,5}|))()'''
color   = "$DARKORANGE"

# mac address
[[rules]]
pattern = '''($HEAD)((?i)[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2})($TAIL)'''
color   = "$MEDIUMPURPLE"

# path
[[rules]]
pattern = '''($HEAD)(/[^/]+/[a-zA-Z0-9./_\-]{4,})($TAIL)'''
color   = "$ROSYBROWN"

# quoted string
[[rules]]
pattern = '''()('[^']+'|"[^"]+")()'''
color   = "$DODGERBLUE"

# result keyword
[[rules]]
pattern = '''($HEAD)((?i)(ok|good))($TAIL)'''
color   = "$STEELBLUE"

[[rules]]
pattern = '''($HEAD)((?i)yes)($TAIL)'''
color   = "$GREENYELLOW"

[[rules]]
pattern = '''($HEAD)((?i)no)($TAIL)'''
color   = "$HOTPINK"

[[rules]]
pattern = '''($HEAD)((?i)(ng|bad|err))($TAIL)'''
color   = "$RED"

[[rules]]
pattern = '''($HEAD)((?i)(failed|failure))($TAIL)'''
color   = "$RED"

[[rules]]
pattern = '''($HEAD)((?i)exception)($TAIL)'''
color   = "$RED"

[[rules]]
pattern = '''($HEAD)((?i)abnormally)($TAIL)'''
color   = "$RED"

[[rules]]
pattern = '''($HEAD)((?i)normally)($TAIL)'''
color   = "$STEELBLUE"

[[rules]]
pattern = '''($HEAD)((?i)normally)($TAIL)'''
color   = "$STEELBLUE"

[[rules]]
pattern = '''()((?i)(succee?ded|succee?d))()'''
color   = "$STEELBLUE"

[[rules]]
pattern = '''()((?i)(successfully|successful|success))()'''
color   = "$STEELBLUE"

# function name
[[rules]]
pattern = '''(critical|fatal|abort|error|warning|warn|info|debug):( [\w]+:)()'''
color   = "$GRAY"

[[rules]]
pattern = '''(\[[0-9]{1,6}:[0-9]{1,6}\]:|\[[0-9]{1,6}\]:)( [\w]+:)()'''
color   = "$GRAY"

[[rules]]
pattern = '''($HEAD)([\w]+\(\))($TAIL)'''
color   = "$GRAY"

# process name
[[rules]]
pattern = '''($HEAD)([\w.-]+)(\[[0-9]{1,6}\]|\[[0-9]{1,6}:[0-9]{1,6}\])'''
color   = "$LIGHTSLATEGREY"

# pid
[[rules]]
pattern = '''($HEAD)((?i)\[[0-9]{1,6}:[0-9]{1,6}\]|\[[0-9]{1,6}\])()'''
color   = "$LIGHTCORAL"

# loglevel
[[rules]]
pattern = '''($HEAD)((?i)(fatal|abort|alert|error))(:)'''
color   = "$RED"

[[rules]]
pattern = '''($HEAD)((?i)warn(ing)?)(:)'''
color   = "$RED"

[[rules]]
pattern = '''($HEAD)((?i)info)(:)'''
color   = "$CYAN"

[[rules]]
pattern = '''($HEAD)((?i)debug)(:)'''
color   = "$DARKSEAGREEN"

# second/minute/hour/day
[[rules]]
pattern = '''($HEAD)([0-9]+)(?i)(seconds?|sec|minutes?|min|hours?|days?)($TAIL)'''
color   = "$YELLOW"

# numeric
[[rules]]
pattern = '''([ :=\(])([0-9]+)([\r \)]|$)'''
color   = "$YELLOW"

# operation keyword
[[rules]]
pattern = '''($HEAD)((?i)started|start|stopped|stop|exited|exit|end|finished|finish|completed|complete)($TAIL)'''
color   = "$LIGHTCYAN"

# networking keyword
[[rules]]
pattern = '''($HEAD)(src|de?st|udp|tcp|from|to|port)($TAIL)'''
color   = "$INDIANRED"

[patterns]
"$HEAD" = '''^|\b|\W'''
"$TAIL" = '''\r|$|\b|\W'''

[colors]
# NOTE: TOML file allows only \uXXXX format.
"$RED"            = "\u001b[1;31m"
"$YELLOW"         = "\u001b[1;33m"
"$CYAN"           = "\u001b[1;36m"
"$CADETBLUE"      = "\u001b[0;38;05;73m"
"$MEDIUMPURPLE"   = "\u001b[0;38;05;104m"
"$LIGHTSLATEGREY" = "\u001b[0;38;05;103m"
"$DARKSEAGREEN"   = "\u001b[0;38;05;108m"
"$INDIANRED"      = "\u001b[0;38;05;167m"
"$GREENYELLOW"    = "\u001b[0;38;05;154m"
"$LIGHTCYAN"      = "\u001b[0;38;05;195m"
"$HOTPINK"        = "\u001b[0;38;05;206m"
"$STEELBLUE"      = "\u001b[0;38;05;75m"
"$ROSYBROWN"      = "\u001b[0;38;05;138m"
"$DARKORANGE"     = "\u001b[0;38;05;208m"
"$DARKKHAKI"      = "\u001b[0;38;05;143m"
"$SALMON"         = "\u001b[0;38;05;209m"
"$DODGERBLUE"     = "\u001b[0;38;05;33m"
"$LIGHTCORAL"     = "\u001b[0;38;05;210m"
"$GRAY"           = "\u001b[0;38;05;245m"
"###;

#[derive(Debug, Deserialize)]
struct Patterns {
    patterns: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Colors {
    colors: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub pattern: String,
    pub color: String,
}

impl Rules {
    pub fn new(conf: &str) -> Result<Self, Box<dyn Error>> {
        let rules: Vec<Rule> = if conf.is_empty() {
            Rules::load_rules_from_string(DEFAULT_RULES.to_string())?
        } else {
            Rules::load_rules_from_file(conf)?
        };
        Ok(Rules{rules})
    }

    pub fn show() {
        print!("{}", DEFAULT_RULES);
    }

    fn load_rules_from_file(file: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
        // NOTE: toml file format
        // [[rules]]
        // pattern = "()([0-9]{4}-[0-9]{2}-[0-9]{2}|[0-9]{3}/[0-9]{2}/[0-9]{2})()"
        // color   = "\u001b[1;31m"
        let mut reader = File::open(file).map(|f| BufReader::new(f))?;
        let mut file_content = String::new();
        reader.read_to_string(&mut file_content)?;

        let rules = Rules::load_rules_from_string(file_content)?;
        Ok(rules)
    }

    fn load_rules_from_string(string: String) -> Result<Vec<Rule>, Box<dyn Error>> {
        let _rules: Rules = toml::from_str(&string)?;
        // TODO: patternsとcolorsは設定が存在しないケースも考慮して
        //       Option<T>にしたほうがよさげ
        let _patterns: Patterns = toml::from_str(&string)?;
        let _colors: Colors = toml::from_str(&string)?;

        let mut rules: Vec<Rule> = Vec::with_capacity(_rules.rules.len());
        for rule in _rules.rules {
            let mut pattern = rule.pattern;
            for (_pattern, text) in &_patterns.patterns {
                let regex = Regex::new(&regex::escape(&_pattern))?;
                pattern = regex.replace(&pattern, &text[..]).to_string();
            }
            let mut color = rule.color;
            for (_color, text) in &_colors.colors {
                let regex = Regex::new(&regex::escape(&_color))?;
                color = regex.replace(&color, &text[..]).to_string();
            }
            rules.push(Rule{pattern, color});
        }

        // XXX: regexインスタンスを使いまわせれば早くできそうだがコンパイルが通らない
        // struct Replacer {
        //     regex: Regex,
        //     text: String,
        // }

        // let mut patterns: Vec<Replacer> = Vec::with_capacity(_patterns.patterns.len());
        // for (pattern, text) in _patterns.patterns {
        //     let regex = Regex::new(&pattern)?;
        //     patterns.push(Replacer{regex, text});
        // }

        // let mut colors: Vec<Replacer> = Vec::with_capacity(_colors.colors.len());
        // for (color, text) in _colors.colors {
        //     let regex = Regex::new(&color)?;
        //     colors.push(Replacer{regex, text});
        // }

        // let mut rules: Vec<Rule> = Vec::with_capacity(_rules.rules.len());
        // for rule in _rules.rules {
        //     let mut pattern = rule.pattern;
        //     for _pattern in patterns {
        //         pattern = _pattern.regex.replace(&pattern, &_pattern.text[..]).to_string();
        //     }
        //     let mut color = rule.color;
        //     for _color in colors {
        //         color = _color.regex.replace(&color, &_color.text[..]).to_string();
        //     }
        //     rules.push(Rule{pattern, color});
        // }

        Ok(rules)
    }
}
