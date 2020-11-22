
extern crate djed_base64_util;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate regex;
extern crate djed_rsx_images;
extern crate djed_rsx_parser;
extern crate djed_rsx_stylesheet;

/*use std::env;
use std::fs::File;
use std::io::Read;

use regex::Regex;
use djed_rsx_images::encoded::EncodedImage;*/
use djed_rsx_parser::parse as parse_rsx;
//use djed_rsx_stylesheet::servo_css_parser::parse as parse_css;
//use djed_rsx_stylesheet::servo_css_parser::types::*;
//use djed_rsx_stylesheet::styles::types::Stylesheet;

#[proc_macro]
pub fn rsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let (ast, _) = parse_rsx(&source).unwrap();

    let expanded = quote! {
        fragment! {
            #ast
        }
    };

    expanded.parse().unwrap()
}
/*
#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let len = source.len();

    let origin = Origin::UserAgent;
    let qm = QuirksMode::NoQuirks;
    let media = MediaList::empty();
    let url = Url::parse("about::inline").unwrap();

    let css = if source.chars().nth(0).unwrap() == '"' {
        let file_path = env::current_dir().unwrap().join(&source[1..len - 1]);

        let mut file_contents = String::new();
        File::open(&file_path)
            .expect(&format!(
                "Couldn't open file {}",
                file_path.to_string_lossy()
            ))
            .read_to_string(&mut file_contents)
            .expect(&format!(
                "Couldn't read file {}",
                file_path.to_string_lossy()
            ));

        file_contents
    } else {
        // When converting TokenStreams to Strings, a whitespace is inserted between
        // each and every token. This unfortunately means that whitespace is also
        // inserted in selectors, turning ".foo" into ". foo" which isn't valid CSS.
        // Same goes to rule names, such as "margin-left" becoming "margin - left".
        // Crudely find and fix those occurrences.

        let re_selectors = Regex::new(r"(?P<type>[.#])\s(?P<name>[a-zA-Z0-9]+)").unwrap();
        let file_contents = re_selectors.replace_all(&source, "$type$name");

        let re_rules = Regex::new(r"(?P<start>[a-zA-Z0-9]+)\s-\s(?P<end>[a-zA-Z0-9]+)\s").unwrap();
        let file_contents = re_rules.replace_all(&file_contents, "$start-$end");

        file_contents.into_owned()
    };

    let parsed = parse_css(&css, url, origin, qm, media);
    let stylesheet: Stylesheet = parsed.into();

    let expanded = quote! {
        #stylesheet
    };

    expanded.parse().unwrap()
}

#[proc_macro]
pub fn load_image(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let len = source.len();

    let input_path = &source[1..len - 1];
    let file_path = env::current_dir().unwrap().join(input_path);

    let mut file_contents = Vec::new();
    File::open(&file_path)
        .expect(&format!(
            "Couldn't open file {}",
            file_path.to_string_lossy()
        ))
        .read_to_end(&mut file_contents)
        .expect(&format!(
            "Couldn't read file {}",
            file_path.to_string_lossy()
        ));

    let format = EncodedImage::guess_format(&file_contents).unwrap();
    let size = EncodedImage::get_dimensions(format, &file_contents).unwrap();
    let data_uri = base64_util::to_image_data_uri(format.as_ref(), &file_contents);
    let bytes_path = format!("../{}", input_path);

    let expanded = quote! {
        EncodedImage::BytesAndDataUri {
            format: #format,
            bytes: ::std::rc::Rc::new(include_bytes!(#bytes_path).to_vec()),
            data_uri: ::std::rc::Rc::new(#data_uri.to_string()),
            size_info: Some(#size)
        }
    };

    expanded.parse().unwrap()
}

#[proc_macro]
pub fn load_font(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let len = source.len();

    let input_path = &source[1..len - 1];
    let file_path = env::current_dir().unwrap().join(input_path);

    let mut file_contents = Vec::new();
    File::open(&file_path)
        .expect(&format!(
            "Couldn't open file {}",
            file_path.to_string_lossy()
        ))
        .read_to_end(&mut file_contents)
        .expect(&format!(
            "Couldn't read file {}",
            file_path.to_string_lossy()
        ));

    let data_uri = base64_util::to_font_data_uri(&file_contents);
    let bytes_path = format!("../{}", input_path);

    let expanded = quote! {
        EncodedFont::BytesAndDataUri {
            bytes: ::std::rc::Rc::new(include_bytes!(#bytes_path).to_vec()),
            data_uri: ::std::rc::Rc::new(#data_uri.to_string())
        }
    };

    expanded.parse().unwrap()
}
*/