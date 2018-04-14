extern crate serde;
#[macro_use]
extern crate serde_derive;

mod codes;

pub use codes::ALL;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso31662 {
    pub code: &'static str,
    pub name: &'static str,
    pub division: &'static str,
    pub parent: Option<&'static str>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
}

pub fn find(code: &str) -> Option<&'static Iso31662> {
    match codes::ALL.binary_search_by_key(&code, |c| &c.code) {
        Ok(index) => Some(&codes::ALL[index]),
        Err(_) => None,
    }
}

pub fn all_parents(mut code: &str) -> Vec<&'static Iso31662> {
    let mut result = vec![];
    loop {
        if let Some(value) = find(code) {
            result.push(value);
            if value.parent.is_none() {
                return result;
            }
            code = value.parent.unwrap();
        } else {
            return result;
        }
    }
}

pub fn drill_down(
    country_code: &str,
    administrative: &str,
    city: &str,
) -> Option<&'static Iso31662> {
    let index = match codes::ALL.binary_search_by_key(&country_code, |c| &c.code) {
        Ok(index) => index,
        Err(_) => return None,
    };

    let city = codes::ALL[index..]
        .iter()
        .take_while(|c| c.code.starts_with(country_code))
        .find(|c| c.name == city);

    // If we found a city return it
    if city.is_some() {
        return city;
    }

    // if not try the administrative and return that
    codes::ALL[index..]
        .iter()
        .take_while(|c| c.code.starts_with(country_code))
        .find(|c| c.name == administrative)
}

#[cfg(test)]
mod tests {

    use super::*;
    use codes;

    // #[test]
    // fn sort() {
    //     let mut a = codes::ALL.to_vec();
    //     a.sort_by_key(|c| c.code);
    //     println!("{:?}", a);
    //     assert!(false);
    // }

    #[test]
    fn find_code() {
        let index = codes::ALL
            .binary_search_by_key(&"ES-MD", |c| &c.code)
            .unwrap();
        assert_eq!(codes::ALL[index].code, "ES-MD");
        assert_eq!(find("GB-SCT").unwrap().code, "GB-SCT");
    }

    #[test]
    fn augment() {
        let index = codes::ALL.binary_search_by_key(&"ES", |c| &c.code).unwrap();
        let madrid = codes::ALL[index..]
            .iter()
            .find(|c| c.name == "Madrid")
            .unwrap();
        assert_eq!(madrid.code, "ES-M");
    }

    #[test]
    fn parents() {
        let parents = all_parents("ES-M");
        assert_eq!(
            parents.iter().map(|c| c.code).collect::<Vec<_>>(),
            vec!["ES-M", "ES-MD", "ES"]
        );
    }

    #[test]
    fn dilldown() {
        assert_eq!(
            drill_down("ES", "Comunidad de Madrid", "Madrid")
                .unwrap()
                .code,
            "ES-M"
        );
        assert_eq!(
            drill_down("DE", "Bayern", "Middle Franconia").unwrap().code,
            "DE-BY"
        );
        assert_eq!(
            drill_down("IE", "Leinster", "Dublin 2").unwrap().code,
            "IE-L"
        );
        assert_eq!(
            drill_down("CA", "Quebec", "Montreal").unwrap().code,
            "CA-QC"
        );
    }
}
