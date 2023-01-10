use fancy_regex::Regex as Fancy_re;
use pcre::Pcre;

fn main() {
    let text = "〈〈@ki-〉d〉 〈@ki-〉 〈@hello〉 〈nyan〈〈@mail〉〉.uz〉 〈nya〈〈@mail〉〉.uz〉 g'zal";

    // each should return only unwrapped!!!!
    fancy(text);
    pcre(text);
}

fn fancy(text: &str) {
    println!("FANCY REGEX IMPLEMENTATION");
    let re = Fancy_re::new("([^〈〉](?![^〈]*〉))+").unwrap();
    for capture in re.captures_iter(&text.clone()) {
        let capture = capture.unwrap()[0].to_string();
        println!("{}", capture.trim());
    }
}

fn pcre(text: &str) {
    println!("\nPCRE REGEX IMPLEMENTATION");
    let mut re: Pcre = Pcre::compile("([^〈〉](?![^〈]*〉))+").unwrap();
    let matches = re.matches(&text);

    for m in matches {
        let captured = String::from(m.group(0));
        println!("{}", captured.trim());
    }
}