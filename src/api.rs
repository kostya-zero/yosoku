use reqwest::blocking::get;

pub struct API;
impl API {
    pub fn send(uri: &String) -> String {
        let resp = get(uri).expect("Error").text();
        let result = resp.expect("Error").to_string();
        return result;
    }
}
