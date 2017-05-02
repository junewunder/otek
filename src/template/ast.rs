
pub fn vec_to_string<'a>(v: &Vec<&'a str>) -> String {
    v.into_iter()
    .fold(String::new(), |mut acc, l| { acc.push_str(&l); acc })
}
