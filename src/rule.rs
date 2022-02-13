pub struct Rule {
    pub from: String,
    pub to: String,
}

impl Rule {
    pub fn default_rules() -> Vec<Rule> {
        Vec::from([
            Rule {
                from: String::from("。”，"),
                to: String::from(".\", "),
            },
            Rule {
                from: String::from("，”，"),
                to: String::from(",\", "),
            },
            Rule {
                from: String::from("？”，"),
                to: String::from("?\", "),
            },
            Rule {
                from: String::from("！”，"),
                to: String::from("!\", "),
            },
            Rule {
                from: String::from("；”，"),
                to: String::from(";\", "),
            },
            Rule {
                from: String::from("……”，"),
                to: String::from("...\", "),
            },
            Rule {
                from: String::from("。’，"),
                to: String::from(".', "),
            },
            Rule {
                from: String::from("，’，"),
                to: String::from(",', "),
            },
            Rule {
                from: String::from("？’，"),
                to: String::from("?', "),
            },
            Rule {
                from: String::from("！’，"),
                to: String::from("!', "),
            },
            Rule {
                from: String::from("；’，"),
                to: String::from(";', "),
            },
            Rule {
                from: String::from("……’，"),
                to: String::from("...', "),
            },
            Rule {
                from: String::from("。」，"),
                to: String::from(".', "),
            },
            Rule {
                from: String::from("，」，"),
                to: String::from(",', "),
            },
            Rule {
                from: String::from("？」，"),
                to: String::from("?', "),
            },
            Rule {
                from: String::from("！」，"),
                to: String::from("!', "),
            },
            Rule {
                from: String::from("；」，"),
                to: String::from(";', "),
            },
            Rule {
                from: String::from("……」，"),
                to: String::from("...', "),
            },
            Rule {
                from: String::from("。』，"),
                to: String::from(".\" "),
            },
            Rule {
                from: String::from("，』，"),
                to: String::from(",\" "),
            },
            Rule {
                from: String::from("？』，"),
                to: String::from("?\" "),
            },
            Rule {
                from: String::from("！』，"),
                to: String::from("!\" "),
            },
            Rule {
                from: String::from("；』，"),
                to: String::from(";\" "),
            },
            Rule {
                from: String::from("……』，"),
                to: String::from("...\" "),
            },
            Rule {
                from: String::from("。”"),
                to: String::from(".\" "),
            },
            Rule {
                from: String::from("，”"),
                to: String::from(",\" "),
            },
            Rule {
                from: String::from("？”"),
                to: String::from("?\" "),
            },
            Rule {
                from: String::from("！”"),
                to: String::from("!\" "),
            },
            Rule {
                from: String::from("；”"),
                to: String::from(";\" "),
            },
            Rule {
                from: String::from("……”"),
                to: String::from("...\" "),
            },
            Rule {
                from: String::from("。’"),
                to: String::from(".' "),
            },
            Rule {
                from: String::from("，’"),
                to: String::from(",' "),
            },
            Rule {
                from: String::from("？’"),
                to: String::from("?' "),
            },
            Rule {
                from: String::from("！’"),
                to: String::from("!' "),
            },
            Rule {
                from: String::from("；’"),
                to: String::from(";' "),
            },
            Rule {
                from: String::from("……’"),
                to: String::from("...' "),
            },
            Rule {
                from: String::from("。」"),
                to: String::from(".' "),
            },
            Rule {
                from: String::from("，」"),
                to: String::from(",' "),
            },
            Rule {
                from: String::from("？」"),
                to: String::from("?' "),
            },
            Rule {
                from: String::from("！」"),
                to: String::from("!' "),
            },
            Rule {
                from: String::from("；」"),
                to: String::from(";' "),
            },
            Rule {
                from: String::from("……」"),
                to: String::from("...' "),
            },
            Rule {
                from: String::from("。』"),
                to: String::from(".\" "),
            },
            Rule {
                from: String::from("，』"),
                to: String::from(",\" "),
            },
            Rule {
                from: String::from("？』"),
                to: String::from("?\" "),
            },
            Rule {
                from: String::from("！』"),
                to: String::from("!\" "),
            },
            Rule {
                from: String::from("；』"),
                to: String::from(";\" "),
            },
            Rule {
                from: String::from("……』"),
                to: String::from("...\" "),
            },
            Rule {
                from: String::from("。“"),
                to: String::from(". \""),
            },
            Rule {
                from: String::from("，“"),
                to: String::from(", \""),
            },
            Rule {
                from: String::from("？“"),
                to: String::from("? \""),
            },
            Rule {
                from: String::from("！“"),
                to: String::from("! \""),
            },
            Rule {
                from: String::from("；“"),
                to: String::from("; \""),
            },
            Rule {
                from: String::from("……“"),
                to: String::from("... \""),
            },
            Rule {
                from: String::from("\n“"),
                to: String::from("\n\""),
            },
            Rule {
                from: String::from("。‘"),
                to: String::from(". '"),
            },
            Rule {
                from: String::from("，‘"),
                to: String::from(", '"),
            },
            Rule {
                from: String::from("？‘"),
                to: String::from("? '"),
            },
            Rule {
                from: String::from("！‘"),
                to: String::from("! '"),
            },
            Rule {
                from: String::from("；‘"),
                to: String::from("; '"),
            },
            Rule {
                from: String::from("……‘"),
                to: String::from("... '"),
            },
            Rule {
                from: String::from("\n‘"),
                to: String::from("\n'"),
            },
            Rule {
                from: String::from("。『"),
                to: String::from(". \""),
            },
            Rule {
                from: String::from("，『"),
                to: String::from(", \""),
            },
            Rule {
                from: String::from("？『"),
                to: String::from("? \""),
            },
            Rule {
                from: String::from("！『"),
                to: String::from("! \""),
            },
            Rule {
                from: String::from("；『"),
                to: String::from("; \""),
            },
            Rule {
                from: String::from("……『"),
                to: String::from("... \""),
            },
            Rule {
                from: String::from("\n『"),
                to: String::from("\n\""),
            },
            Rule {
                from: String::from("。「"),
                to: String::from(". '"),
            },
            Rule {
                from: String::from("，「"),
                to: String::from(", '"),
            },
            Rule {
                from: String::from("？「"),
                to: String::from("? '"),
            },
            Rule {
                from: String::from("！「"),
                to: String::from("! '"),
            },
            Rule {
                from: String::from("；「"),
                to: String::from("; '"),
            },
            Rule {
                from: String::from("……「"),
                to: String::from("... '"),
            },
            Rule {
                from: String::from("\n「"),
                to: String::from("\n'"),
            },
            Rule {
                from: String::from("“"),
                to: String::from(" \""),
            },
            Rule {
                from: String::from("‘"),
                to: String::from(" '"),
            },
            Rule {
                from: String::from("「"),
                to: String::from(" '"),
            },
            Rule {
                from: String::from("『"),
                to: String::from(" \""),
            },
            Rule {
                from: String::from("”"),
                to: String::from("\" "),
            },
            Rule {
                from: String::from("’"),
                to: String::from("' "),
            },
            Rule {
                from: String::from("」"),
                to: String::from("' "),
            },
            Rule {
                from: String::from("』"),
                to: String::from("\" "),
            },
            Rule {
                from: String::from("（"),
                to: String::from(" ("),
            },
            Rule {
                from: String::from("）"),
                to: String::from(") "),
            },
            Rule {
                from: String::from("【"),
                to: String::from(" ["),
            },
            Rule {
                from: String::from("】"),
                to: String::from("] "),
            },
            Rule {
                from: String::from("——"),
                to: String::from(" - "),
            },
            Rule {
                from: String::from("。"),
                to: String::from(". "),
            },
            Rule {
                from: String::from("，"),
                to: String::from(", "),
            },
            Rule {
                from: String::from("？"),
                to: String::from("? "),
            },
            Rule {
                from: String::from("！"),
                to: String::from("! "),
            },
            Rule {
                from: String::from("；"),
                to: String::from("; "),
            },
            Rule {
                from: String::from("……"),
                to: String::from("... "),
            },
            Rule {
                from: String::from("："),
                to: String::from(": "),
            },
            Rule {
                from: String::from("、"),
                to: String::from(", "),
            },
        ])
    }
}
