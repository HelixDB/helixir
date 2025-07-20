extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::{parse_macro_input, ItemFn};

struct Answer {
    lesson_number: u32,
    queries: Option<String>,
    hql_src: Option<String>,
}

#[proc_macro_attribute]
pub fn parse_answers(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let main_fn = parse_macro_input!(item as ItemFn);
    let visibility = main_fn.vis;
    let signature = main_fn.sig;
    let body = main_fn.block;
    let mut answers = Vec::new();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let project_root = Path::new(&manifest_dir);

    // read from ../lesson_answers/lesson_<lesson_number>.rs
    for lesson_number in 1..=24 {
        let query_answer_path =
            project_root.join(format!("query_answers/lesson{}.json", lesson_number));
        let hql_answer_path =
            project_root.join(format!("lesson_answers/lesson{}_queries.hx", lesson_number));

        let lesson_answers = std::fs::read_to_string(&query_answer_path).ok();
        let lesson_queries = std::fs::read_to_string(&hql_answer_path).ok();

        answers.push(Answer {
            lesson_number,
            queries: lesson_answers,
            hql_src: lesson_queries,
        });
    }

    let lessons = answers.iter().map(
        |Answer {
             lesson_number,
             queries,
             hql_src,
         }| {
            let queries_value = queries.clone().unwrap_or(String::new());
            let hql_src_value = hql_src.clone().unwrap_or(String::new());
            quote! {
                (#lesson_number, Lesson {
                    query_answer: #queries_value.to_string(),
                    hql_answer: #hql_src_value.to_string(),
                })
            }
        },
    );

    let expanded = quote! {
        use std::collections::HashMap;
        pub struct Lesson {
            pub query_answer: String,
            pub hql_answer: String,
        }
        #visibility #signature {
            let lessons = HashMap::from([
                #(#lessons),*
            ]);
            #body
        }
    };

    TokenStream::from(expanded)
}
