extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{TextView, Dialog, EditView, SelectView, LinearLayout, DummyView};

pub mod content;
use content::*;

fn main() {
    // Initial setup
    let mut main = Cursive::default();

    main.add_global_callback('q', |s| s.quit());
    main.add_global_callback('s', |s| search(s));
    main.add_global_callback('t', |s| match s.pop_layer() {
        _ => ()
    });

    main.run();
}

fn search(s: &mut Cursive){

    fn go(s: &mut Cursive, search: &str) {
        s.pop_layer();
        let mut result = vec![];
        match get_search_results(search) {
            Ok(x) => result = x,
            Err(e) => pop_error(s,handler(e)),
        };
        let choose_result = SelectView::<String>::new()
            .with_all_str(result)
            .on_submit(on_submit);
        s.add_layer(Dialog::around(choose_result)
                    .title("Search Results"));
    }

    s.add_layer(Dialog::around(EditView::new()
                               .on_submit(go)
                               .with_id("search")
                               .fixed_size(( 15,2 )))
                .title("Search for a page")
                .button("Go", |s| {
                    let search_txt = s.call_on_id( "search", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    go(s, &search_txt);
                })
                .button("Cancel", |s| match s.pop_layer(){
                    _ => ()
                }));
}

fn on_submit(s: &mut Cursive, name: &String) {
    s.pop_layer();

    let heading: String = name.clone();
    let url = query_url_gen(&name.replace(" ", "_"));
    let res = reqwest::get(&url).unwrap();

    let mut extract = String::new();

    match get_extract(res) {
        Ok(x) => extract = x,
        Err(e) => pop_error(s, handler(e))
    };

    s.add_layer(
        LinearLayout::vertical()
        .child(TextView::new(heading).h_align(HAlign::Center))
        .child(DummyView.fixed_height(1))
        .child(TextView::new(extract)
               .fixed_width(85))
        );
}
