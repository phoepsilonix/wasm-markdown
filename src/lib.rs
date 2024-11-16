#![no_main]

#[cfg(feature = "wee_alloc")] // wee_allocがfeatureに設定されたときに有効
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd, CodeBlockKind };
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[no_mangle]
pub fn pulldown_cmark(text: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_OLD_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    opts.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);

    let mut _in_code_block = false;
    let mut p = Vec::new();
    let mut lang = String::new();
    let mut code = String::new();
    for event in Parser::new_ext(text, opts) {
        match event {
            Event::Start(Tag::CodeBlock(info)) => {
                _in_code_block = true;
                if let CodeBlockKind::Fenced(info) = info {
                    lang = info.to_string();
                    //lang = info.split(' ').next().unwrap().to_string();
                   // if !lang.is_empty() {
                   //     syntax = SYNTAX_SET.find_syntax_by_token(lang);
                   // }
                }
            }
            Event::Text(t) => {
                if _in_code_block {
                    code.push_str(&t);
                } else {
                    let replacer = gh_emoji::Replacer::new();
                    let s = replacer.replace_all(&t);
                    p.push(Event::Text(s.to_string().into()));
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                _in_code_block = false;
                //let ss = SyntaxSet::load_defaults_newlines();
                //let syntax = ss.find_syntax_by_token(&lang).unwrap_or_else(|| ss.find_syntax_plain_text());
                //let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &ss, prefixed_style);
                //let _ = html_generator.parse_html_for_line_which_includes_newline(&code);
                //for line in code.lines() {
                //    let _ = html_generator.parse_html_for_line(&line);
                //}
                //let html = html_generator.finalize();
                let html = code;
                p.push(Event::Html(
                        format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, html).into(),
                        ));
                code = String::new();
                //syntax = None;
            }
            _ => {
                p.push(event);
            }
        }
    };

    let mut html_output = String::new();
    html::push_html(&mut html_output, p.into_iter());
    html_output
}

