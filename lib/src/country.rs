use std::collections::HashMap;
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
pub struct Country {
    pub name: &'static str,
    pub flag: &'static str,
}

static COUNTRIES: AtomicPtr<HashMap<&'static str, Country>> = AtomicPtr::new(std::ptr::null_mut());

fn init_countries() {
    if !COUNTRIES.load(Ordering::Acquire).is_null() {
        return;
    }
    log::info!("Initialize countries...");
    let mut countries = HashMap::new();
    countries.insert(
        "AF",
        Country {
            name: "Afghanistan",
            flag: "\u{1F1E6}\u{1F1EB}",
        },
    );
    countries.insert(
        "AFG",
        Country {
            name: "Afghanistan",
            flag: "\u{1F1E6}\u{1F1EB}",
        },
    );
    countries.insert(
        "AL",
        Country {
            name: "Albania",
            flag: "\u{1F1E6}\u{1F1F1}",
        },
    );
    countries.insert(
        "ALB",
        Country {
            name: "Albania",
            flag: "\u{1F1E6}\u{1F1F1}",
        },
    );
    countries.insert(
        "DZ",
        Country {
            name: "Algeria",
            flag: "\u{1F1E9}\u{1F1FF}",
        },
    );
    countries.insert(
        "DZA",
        Country {
            name: "Algeria",
            flag: "\u{1F1E9}\u{1F1FF}",
        },
    );
    countries.insert(
        "AS",
        Country {
            name: "American Samoa",
            flag: "\u{1F1E6}\u{1F1F8}",
        },
    );
    countries.insert(
        "ASM",
        Country {
            name: "American Samoa",
            flag: "\u{1F1E6}\u{1F1F8}",
        },
    );
    countries.insert(
        "AD",
        Country {
            name: "Andorra",
            flag: "\u{1F1E6}\u{1F1E9}",
        },
    );
    countries.insert(
        "AND",
        Country {
            name: "Andorra",
            flag: "\u{1F1E6}\u{1F1E9}",
        },
    );
    countries.insert(
        "AO",
        Country {
            name: "Angola",
            flag: "\u{1F1E6}\u{1F1F4}",
        },
    );
    countries.insert(
        "AGO",
        Country {
            name: "Angola",
            flag: "\u{1F1E6}\u{1F1F4}",
        },
    );
    countries.insert(
        "AI",
        Country {
            name: "Anguilla",
            flag: "\u{1F1E6}\u{1F1EE}",
        },
    );
    countries.insert(
        "AIA",
        Country {
            name: "Anguilla",
            flag: "\u{1F1E6}\u{1F1EE}",
        },
    );
    countries.insert(
        "AQ",
        Country {
            name: "Antarctica",
            flag: "\u{1F1E6}\u{1F1F6}",
        },
    );
    countries.insert(
        "ATA",
        Country {
            name: "Antarctica",
            flag: "\u{1F1E6}\u{1F1F6}",
        },
    );
    countries.insert(
        "AG",
        Country {
            name: "Antigua and Barbuda",
            flag: "\u{1F1E6}\u{1F1EC}",
        },
    );
    countries.insert(
        "ATG",
        Country {
            name: "Antigua and Barbuda",
            flag: "\u{1F1E6}\u{1F1EC}",
        },
    );
    countries.insert(
        "AR",
        Country {
            name: "Argentina",
            flag: "\u{1F1E6}\u{1F1F7}",
        },
    );
    countries.insert(
        "ARG",
        Country {
            name: "Argentina",
            flag: "\u{1F1E6}\u{1F1F7}",
        },
    );
    countries.insert(
        "AM",
        Country {
            name: "Armenia",
            flag: "\u{1F1E6}\u{1F1F2}",
        },
    );
    countries.insert(
        "ARM",
        Country {
            name: "Armenia",
            flag: "\u{1F1E6}\u{1F1F2}",
        },
    );
    countries.insert(
        "AW",
        Country {
            name: "Aruba",
            flag: "\u{1F1E6}\u{1F1FC}",
        },
    );
    countries.insert(
        "ABW",
        Country {
            name: "Aruba",
            flag: "\u{1F1E6}\u{1F1FC}",
        },
    );
    countries.insert(
        "AU",
        Country {
            name: "Australia",
            flag: "\u{1F1E6}\u{1F1FA}",
        },
    );
    countries.insert(
        "AUS",
        Country {
            name: "Australia",
            flag: "\u{1F1E6}\u{1F1FA}",
        },
    );
    countries.insert(
        "AT",
        Country {
            name: "Austria",
            flag: "\u{1F1E6}\u{1F1F9}",
        },
    );
    countries.insert(
        "AUT",
        Country {
            name: "Austria",
            flag: "\u{1F1E6}\u{1F1F9}",
        },
    );
    countries.insert(
        "AZ",
        Country {
            name: "Azerbaijan",
            flag: "\u{1F1E6}\u{1F1FF}",
        },
    );
    countries.insert(
        "AZE",
        Country {
            name: "Azerbaijan",
            flag: "\u{1F1E6}\u{1F1FF}",
        },
    );
    countries.insert(
        "BS",
        Country {
            name: "Bahamas",
            flag: "\u{1F1E7}\u{1F1F8}",
        },
    );
    countries.insert(
        "BHS",
        Country {
            name: "Bahamas",
            flag: "\u{1F1E7}\u{1F1F8}",
        },
    );
    countries.insert(
        "BH",
        Country {
            name: "Bahrain",
            flag: "\u{1F1E7}\u{1F1ED}",
        },
    );
    countries.insert(
        "BHR",
        Country {
            name: "Bahrain",
            flag: "\u{1F1E7}\u{1F1ED}",
        },
    );
    countries.insert(
        "BD",
        Country {
            name: "Bangladesh",
            flag: "\u{1F1E7}\u{1F1E9}",
        },
    );
    countries.insert(
        "BGD",
        Country {
            name: "Bangladesh",
            flag: "\u{1F1E7}\u{1F1E9}",
        },
    );
    countries.insert(
        "BB",
        Country {
            name: "Barbados",
            flag: "\u{1F1E7}\u{1F1E7}",
        },
    );
    countries.insert(
        "BRB",
        Country {
            name: "Barbados",
            flag: "\u{1F1E7}\u{1F1E7}",
        },
    );
    countries.insert(
        "BY",
        Country {
            name: "Belarus",
            flag: "\u{1F1E7}\u{1F1FE}",
        },
    );
    countries.insert(
        "BLR",
        Country {
            name: "Belarus",
            flag: "\u{1F1E7}\u{1F1FE}",
        },
    );
    countries.insert(
        "BE",
        Country {
            name: "Belgium",
            flag: "\u{1F1E7}\u{1F1EA}",
        },
    );
    countries.insert(
        "BEL",
        Country {
            name: "Belgium",
            flag: "\u{1F1E7}\u{1F1EA}",
        },
    );
    countries.insert(
        "BZ",
        Country {
            name: "Belize",
            flag: "\u{1F1E7}\u{1F1FF}",
        },
    );
    countries.insert(
        "BLZ",
        Country {
            name: "Belize",
            flag: "\u{1F1E7}\u{1F1FF}",
        },
    );
    countries.insert(
        "BJ",
        Country {
            name: "Benin",
            flag: "\u{1F1E7}\u{1F1EF}",
        },
    );
    countries.insert(
        "BEN",
        Country {
            name: "Benin",
            flag: "\u{1F1E7}\u{1F1EF}",
        },
    );
    countries.insert(
        "BM",
        Country {
            name: "Bermuda",
            flag: "\u{1F1E7}\u{1F1F2}",
        },
    );
    countries.insert(
        "BMU",
        Country {
            name: "Bermuda",
            flag: "\u{1F1E7}\u{1F1F2}",
        },
    );
    countries.insert(
        "BT",
        Country {
            name: "Bhutan",
            flag: "\u{1F1E7}\u{1F1F9}",
        },
    );
    countries.insert(
        "BTN",
        Country {
            name: "Bhutan",
            flag: "\u{1F1E7}\u{1F1F9}",
        },
    );
    countries.insert(
        "BO",
        Country {
            name: "Bolivia",
            flag: "\u{1F1E7}\u{1F1F4}",
        },
    );
    countries.insert(
        "BOL",
        Country {
            name: "Bolivia",
            flag: "\u{1F1E7}\u{1F1F4}",
        },
    );
    countries.insert(
        "BA",
        Country {
            name: "Bosnia and Herzegovina",
            flag: "\u{1F1E7}\u{1F1E6}",
        },
    );
    countries.insert(
        "BIH",
        Country {
            name: "Bosnia and Herzegovina",
            flag: "\u{1F1E7}\u{1F1E6}",
        },
    );
    countries.insert(
        "BW",
        Country {
            name: "Botswana",
            flag: "\u{1F1E7}\u{1F1FC}",
        },
    );
    countries.insert(
        "BWA",
        Country {
            name: "Botswana",
            flag: "\u{1F1E7}\u{1F1FC}",
        },
    );
    countries.insert(
        "BR",
        Country {
            name: "Brazil",
            flag: "\u{1F1E7}\u{1F1F7}",
        },
    );
    countries.insert(
        "BRA",
        Country {
            name: "Brazil",
            flag: "\u{1F1E7}\u{1F1F7}",
        },
    );
    countries.insert(
        "IO",
        Country {
            name: "British Indian Ocean Territory",
            flag: "\u{1F1EE}\u{1F1F4}",
        },
    );
    countries.insert(
        "IOT",
        Country {
            name: "British Indian Ocean Territory",
            flag: "\u{1F1EE}\u{1F1F4}",
        },
    );
    countries.insert(
        "VG",
        Country {
            name: "British Virgin Islands",
            flag: "\u{1F1FB}\u{1F1EC}",
        },
    );
    countries.insert(
        "VGB",
        Country {
            name: "British Virgin Islands",
            flag: "\u{1F1FB}\u{1F1EC}",
        },
    );
    countries.insert(
        "BN",
        Country {
            name: "Brunei",
            flag: "\u{1F1E7}\u{1F1F3}",
        },
    );
    countries.insert(
        "BRN",
        Country {
            name: "Brunei",
            flag: "\u{1F1E7}\u{1F1F3}",
        },
    );
    countries.insert(
        "BG",
        Country {
            name: "Bulgaria",
            flag: "\u{1F1E7}\u{1F1EC}",
        },
    );
    countries.insert(
        "BGR",
        Country {
            name: "Bulgaria",
            flag: "\u{1F1E7}\u{1F1EC}",
        },
    );
    countries.insert(
        "BF",
        Country {
            name: "Burkina Faso",
            flag: "\u{1F1E7}\u{1F1EB}",
        },
    );
    countries.insert(
        "BFA",
        Country {
            name: "Burkina Faso",
            flag: "\u{1F1E7}\u{1F1EB}",
        },
    );
    countries.insert(
        "BI",
        Country {
            name: "Burundi",
            flag: "\u{1F1E7}\u{1F1EE}",
        },
    );
    countries.insert(
        "BDI",
        Country {
            name: "Burundi",
            flag: "\u{1F1E7}\u{1F1EE}",
        },
    );
    countries.insert(
        "KH",
        Country {
            name: "Cambodia",
            flag: "\u{1F1F0}\u{1F1ED}",
        },
    );
    countries.insert(
        "KHM",
        Country {
            name: "Cambodia",
            flag: "\u{1F1F0}\u{1F1ED}",
        },
    );
    countries.insert(
        "CM",
        Country {
            name: "Cameroon",
            flag: "\u{1F1E8}\u{1F1F2}",
        },
    );
    countries.insert(
        "CMR",
        Country {
            name: "Cameroon",
            flag: "\u{1F1E8}\u{1F1F2}",
        },
    );
    countries.insert(
        "CA",
        Country {
            name: "Canada",
            flag: "\u{1F1E8}\u{1F1E6}",
        },
    );
    countries.insert(
        "CAN",
        Country {
            name: "Canada",
            flag: "\u{1F1E8}\u{1F1E6}",
        },
    );
    countries.insert(
        "CV",
        Country {
            name: "Cape Verde",
            flag: "\u{1F1E8}\u{1F1FB}",
        },
    );
    countries.insert(
        "CPV",
        Country {
            name: "Cape Verde",
            flag: "\u{1F1E8}\u{1F1FB}",
        },
    );
    countries.insert(
        "KY",
        Country {
            name: "Cayman Islands",
            flag: "\u{1F1F0}\u{1F1FE}",
        },
    );
    countries.insert(
        "CYM",
        Country {
            name: "Cayman Islands",
            flag: "\u{1F1F0}\u{1F1FE}",
        },
    );
    countries.insert(
        "CF",
        Country {
            name: "Central African Republic",
            flag: "\u{1F1E8}\u{1F1EB}",
        },
    );
    countries.insert(
        "CAF",
        Country {
            name: "Central African Republic",
            flag: "\u{1F1E8}\u{1F1EB}",
        },
    );
    countries.insert(
        "TD",
        Country {
            name: "Chad",
            flag: "\u{1F1F9}\u{1F1E9}",
        },
    );
    countries.insert(
        "TCD",
        Country {
            name: "Chad",
            flag: "\u{1F1F9}\u{1F1E9}",
        },
    );
    countries.insert(
        "CL",
        Country {
            name: "Chile",
            flag: "\u{1F1E8}\u{1F1F1}",
        },
    );
    countries.insert(
        "CHL",
        Country {
            name: "Chile",
            flag: "\u{1F1E8}\u{1F1F1}",
        },
    );
    countries.insert(
        "CN",
        Country {
            name: "China",
            flag: "\u{1F1E8}\u{1F1F3}",
        },
    );
    countries.insert(
        "CHN",
        Country {
            name: "China",
            flag: "\u{1F1E8}\u{1F1F3}",
        },
    );
    countries.insert(
        "CX",
        Country {
            name: "Christmas Island",
            flag: "\u{1F1E8}\u{1F1FD}",
        },
    );
    countries.insert(
        "CXR",
        Country {
            name: "Christmas Island",
            flag: "\u{1F1E8}\u{1F1FD}",
        },
    );
    countries.insert(
        "CC",
        Country {
            name: "Cocos Islands",
            flag: "\u{1F1E8}\u{1F1E8}",
        },
    );
    countries.insert(
        "CCK",
        Country {
            name: "Cocos Islands",
            flag: "\u{1F1E8}\u{1F1E8}",
        },
    );
    countries.insert(
        "CO",
        Country {
            name: "Colombia",
            flag: "\u{1F1E8}\u{1F1F4}",
        },
    );
    countries.insert(
        "COL",
        Country {
            name: "Colombia",
            flag: "\u{1F1E8}\u{1F1F4}",
        },
    );
    countries.insert(
        "KM",
        Country {
            name: "Comoros",
            flag: "\u{1F1F0}\u{1F1F2}",
        },
    );
    countries.insert(
        "COM",
        Country {
            name: "Comoros",
            flag: "\u{1F1F0}\u{1F1F2}",
        },
    );
    countries.insert(
        "CD",
        Country {
            name: "Democratic Republic of the Congo",
            flag: "\u{1F1E8}\u{1F1EC}",
        },
    );
    countries.insert(
        "COD",
        Country {
            name: "Democratic Republic of the Congo",
            flag: "\u{1F1E8}\u{1F1EC}",
        },
    );
    countries.insert(
        "CK",
        Country {
            name: "Cook Islands",
            flag: "\u{1F1E8}\u{1F1F0}",
        },
    );
    countries.insert(
        "COK",
        Country {
            name: "Cook Islands",
            flag: "\u{1F1E8}\u{1F1F0}",
        },
    );
    countries.insert(
        "CR",
        Country {
            name: "Costa Rica",
            flag: "\u{1F1E8}\u{1F1F7}",
        },
    );
    countries.insert(
        "CRI",
        Country {
            name: "Costa Rica",
            flag: "\u{1F1E8}\u{1F1F7}",
        },
    );
    countries.insert(
        "HR",
        Country {
            name: "Croatia",
            flag: "\u{1F1ED}\u{1F1F7}",
        },
    );
    countries.insert(
        "HRV",
        Country {
            name: "Croatia",
            flag: "\u{1F1ED}\u{1F1F7}",
        },
    );
    countries.insert(
        "CU",
        Country {
            name: "Cuba",
            flag: "\u{1F1E8}\u{1F1FA}",
        },
    );
    countries.insert(
        "CUB",
        Country {
            name: "Cuba",
            flag: "\u{1F1E8}\u{1F1FA}",
        },
    );
    countries.insert(
        "CW",
        Country {
            name: "Curaçao",
            flag: "\u{1F1E8}\u{1F1FC}",
        },
    );
    countries.insert(
        "CUW",
        Country {
            name: "Curaçao",
            flag: "\u{1F1E8}\u{1F1FC}",
        },
    );
    countries.insert(
        "CY",
        Country {
            name: "Cyprus",
            flag: "\u{1F1E8}\u{1F1FE}",
        },
    );
    countries.insert(
        "CYP",
        Country {
            name: "Cyprus",
            flag: "\u{1F1E8}\u{1F1FE}",
        },
    );
    countries.insert(
        "CZ",
        Country {
            name: "Czech Republic",
            flag: "\u{1F1E8}\u{1F1FF}",
        },
    );
    countries.insert(
        "CZE",
        Country {
            name: "Czech Republic",
            flag: "\u{1F1E8}\u{1F1FF}",
        },
    );
    countries.insert(
        "CI",
        Country {
            name: "Ivory Coast",
            flag: "\u{1F1E8}\u{1F1EE}",
        },
    );
    countries.insert(
        "CIV",
        Country {
            name: "Ivory Coast",
            flag: "\u{1F1E8}\u{1F1EE}",
        },
    );
    countries.insert(
        "DK",
        Country {
            name: "Denmark",
            flag: "\u{1F1E9}\u{1F1F0}",
        },
    );
    countries.insert(
        "DNK",
        Country {
            name: "Denmark",
            flag: "\u{1F1E9}\u{1F1F0}",
        },
    );
    countries.insert(
        "DJ",
        Country {
            name: "Djibouti",
            flag: "\u{1F1E9}\u{1F1EF}",
        },
    );
    countries.insert(
        "DJI",
        Country {
            name: "Djibouti",
            flag: "\u{1F1E9}\u{1F1EF}",
        },
    );
    countries.insert(
        "DM",
        Country {
            name: "Dominica",
            flag: "\u{1F1E9}\u{1F1F2}",
        },
    );
    countries.insert(
        "DMA",
        Country {
            name: "Dominica",
            flag: "\u{1F1E9}\u{1F1F2}",
        },
    );
    countries.insert(
        "DO",
        Country {
            name: "Dominican Republic",
            flag: "\u{1F1E9}\u{1F1F4}",
        },
    );
    countries.insert(
        "DOM",
        Country {
            name: "Dominican Republic",
            flag: "\u{1F1E9}\u{1F1F4}",
        },
    );
    countries.insert(
        "TL",
        Country {
            name: "East Timor",
            flag: "\u{1F1F9}\u{1F1F1}",
        },
    );
    countries.insert(
        "TLS",
        Country {
            name: "East Timor",
            flag: "\u{1F1F9}\u{1F1F1}",
        },
    );
    countries.insert(
        "EC",
        Country {
            name: "Ecuador",
            flag: "\u{1F1EA}\u{1F1E8}",
        },
    );
    countries.insert(
        "ECU",
        Country {
            name: "Ecuador",
            flag: "\u{1F1EA}\u{1F1E8}",
        },
    );
    countries.insert(
        "EG",
        Country {
            name: "Egypt",
            flag: "\u{1F1EA}\u{1F1EC}",
        },
    );
    countries.insert(
        "EGY",
        Country {
            name: "Egypt",
            flag: "\u{1F1EA}\u{1F1EC}",
        },
    );
    countries.insert(
        "SV",
        Country {
            name: "El Salvador",
            flag: "\u{1F1F8}\u{1F1FB}",
        },
    );
    countries.insert(
        "SLV",
        Country {
            name: "El Salvador",
            flag: "\u{1F1F8}\u{1F1FB}",
        },
    );
    countries.insert(
        "EN",
        Country {
            name: "England",
            flag: "\u{1F3F4}\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}",
        },
    );
    countries.insert(
        "ENG",
        Country {
            name: "England",
            flag: "\u{1F3F4}\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}",
        },
    );
    countries.insert(
        "GQ",
        Country {
            name: "Equatorial Guinea",
            flag: "\u{1F1EC}\u{1F1F6}",
        },
    );
    countries.insert(
        "GNQ",
        Country {
            name: "Equatorial Guinea",
            flag: "\u{1F1EC}\u{1F1F6}",
        },
    );
    countries.insert(
        "ER",
        Country {
            name: "Eritrea",
            flag: "\u{1F1EA}\u{1F1F7}",
        },
    );
    countries.insert(
        "ERI",
        Country {
            name: "Eritrea",
            flag: "\u{1F1EA}\u{1F1F7}",
        },
    );
    countries.insert(
        "EE",
        Country {
            name: "Estonia",
            flag: "\u{1F1EA}\u{1F1EA}",
        },
    );
    countries.insert(
        "EST",
        Country {
            name: "Estonia",
            flag: "\u{1F1EA}\u{1F1EA}",
        },
    );
    countries.insert(
        "ET",
        Country {
            name: "Ethiopia",
            flag: "\u{1F1EA}\u{1F1F9}",
        },
    );
    countries.insert(
        "ETH",
        Country {
            name: "Ethiopia",
            flag: "\u{1F1EA}\u{1F1F9}",
        },
    );
    countries.insert(
        "FK",
        Country {
            name: "Falkland Islands",
            flag: "\u{1F1EB}\u{1F1F0}",
        },
    );
    countries.insert(
        "FLK",
        Country {
            name: "Falkland Islands",
            flag: "\u{1F1EB}\u{1F1F0}",
        },
    );
    countries.insert(
        "FO",
        Country {
            name: "Faroe Islands",
            flag: "\u{1F1EB}\u{1F1F4}",
        },
    );
    countries.insert(
        "FRO",
        Country {
            name: "Faroe Islands",
            flag: "\u{1F1EB}\u{1F1F4}",
        },
    );
    countries.insert(
        "FJ",
        Country {
            name: "Fiji",
            flag: "\u{1F1EB}\u{1F1EF}",
        },
    );
    countries.insert(
        "FJI",
        Country {
            name: "Fiji",
            flag: "\u{1F1EB}\u{1F1EF}",
        },
    );
    countries.insert(
        "FI",
        Country {
            name: "Finland",
            flag: "\u{1F1EB}\u{1F1EE}",
        },
    );
    countries.insert(
        "FIN",
        Country {
            name: "Finland",
            flag: "\u{1F1EB}\u{1F1EE}",
        },
    );
    countries.insert(
        "FR",
        Country {
            name: "France",
            flag: "\u{1F1EB}\u{1F1F7}",
        },
    );
    countries.insert(
        "FRA",
        Country {
            name: "France",
            flag: "\u{1F1EB}\u{1F1F7}",
        },
    );
    countries.insert(
        "PF",
        Country {
            name: "French Polynesia",
            flag: "\u{1F1F5}\u{1F1EB}",
        },
    );
    countries.insert(
        "PYF",
        Country {
            name: "French Polynesia",
            flag: "\u{1F1F5}\u{1F1EB}",
        },
    );
    countries.insert(
        "GA",
        Country {
            name: "Gabon",
            flag: "\u{1F1EC}\u{1F1E6}",
        },
    );
    countries.insert(
        "GAB",
        Country {
            name: "Gabon",
            flag: "\u{1F1EC}\u{1F1E6}",
        },
    );
    countries.insert(
        "GM",
        Country {
            name: "Gambia",
            flag: "\u{1F1EC}\u{1F1F2}",
        },
    );
    countries.insert(
        "GMB",
        Country {
            name: "Gambia",
            flag: "\u{1F1EC}\u{1F1F2}",
        },
    );
    countries.insert(
        "GE",
        Country {
            name: "Georgia",
            flag: "\u{1F1EC}\u{1F1EA}",
        },
    );
    countries.insert(
        "GEO",
        Country {
            name: "Georgia",
            flag: "\u{1F1EC}\u{1F1EA}",
        },
    );
    countries.insert(
        "DE",
        Country {
            name: "Germany",
            flag: "\u{1F1E9}\u{1F1EA}",
        },
    );
    countries.insert(
        "DEU",
        Country {
            name: "Germany",
            flag: "\u{1F1E9}\u{1F1EA}",
        },
    );
    countries.insert(
        "GH",
        Country {
            name: "Ghana",
            flag: "\u{1F1EC}\u{1F1ED}",
        },
    );
    countries.insert(
        "GHA",
        Country {
            name: "Ghana",
            flag: "\u{1F1EC}\u{1F1ED}",
        },
    );
    countries.insert(
        "GI",
        Country {
            name: "Gibraltar",
            flag: "\u{1F1EC}\u{1F1EE}",
        },
    );
    countries.insert(
        "GIB",
        Country {
            name: "Gibraltar",
            flag: "\u{1F1EC}\u{1F1EE}",
        },
    );
    countries.insert(
        "GR",
        Country {
            name: "Greece",
            flag: "\u{1F1EC}\u{1F1F7}",
        },
    );
    countries.insert(
        "GRC",
        Country {
            name: "Greece",
            flag: "\u{1F1EC}\u{1F1F7}",
        },
    );
    countries.insert(
        "GL",
        Country {
            name: "Greenland",
            flag: "\u{1F1EC}\u{1F1F1}",
        },
    );
    countries.insert(
        "GRL",
        Country {
            name: "Greenland",
            flag: "\u{1F1EC}\u{1F1F1}",
        },
    );
    countries.insert(
        "GD",
        Country {
            name: "Grenada",
            flag: "\u{1F1EC}\u{1F1E9}",
        },
    );
    countries.insert(
        "GRD",
        Country {
            name: "Grenada",
            flag: "\u{1F1EC}\u{1F1E9}",
        },
    );
    countries.insert(
        "GU",
        Country {
            name: "Guam",
            flag: "\u{1F1EC}\u{1F1FA}",
        },
    );
    countries.insert(
        "GUM",
        Country {
            name: "Guam",
            flag: "\u{1F1EC}\u{1F1FA}",
        },
    );
    countries.insert(
        "GT",
        Country {
            name: "Guatemala",
            flag: "\u{1F1EC}\u{1F1F9}",
        },
    );
    countries.insert(
        "GTM",
        Country {
            name: "Guatemala",
            flag: "\u{1F1EC}\u{1F1F9}",
        },
    );
    countries.insert(
        "GG",
        Country {
            name: "Guernsey",
            flag: "\u{1F1EC}\u{1F1EC}",
        },
    );
    countries.insert(
        "GGY",
        Country {
            name: "Guernsey",
            flag: "\u{1F1EC}\u{1F1EC}",
        },
    );
    countries.insert(
        "GN",
        Country {
            name: "Guinea",
            flag: "\u{1F1EC}\u{1F1F3}",
        },
    );
    countries.insert(
        "GIN",
        Country {
            name: "Guinea",
            flag: "\u{1F1EC}\u{1F1F3}",
        },
    );
    countries.insert(
        "GW",
        Country {
            name: "Guinea-Bissau",
            flag: "\u{1F1EC}\u{1F1FC}",
        },
    );
    countries.insert(
        "GNB",
        Country {
            name: "Guinea-Bissau",
            flag: "\u{1F1EC}\u{1F1FC}",
        },
    );
    countries.insert(
        "GY",
        Country {
            name: "Guyana",
            flag: "\u{1F1EC}\u{1F1FE}",
        },
    );
    countries.insert(
        "GUY",
        Country {
            name: "Guyana",
            flag: "\u{1F1EC}\u{1F1FE}",
        },
    );
    countries.insert(
        "HT",
        Country {
            name: "Haiti",
            flag: "\u{1F1ED}\u{1F1F9}",
        },
    );
    countries.insert(
        "HTI",
        Country {
            name: "Haiti",
            flag: "\u{1F1ED}\u{1F1F9}",
        },
    );
    countries.insert(
        "HN",
        Country {
            name: "Honduras",
            flag: "\u{1F1ED}\u{1F1F3}",
        },
    );
    countries.insert(
        "HND",
        Country {
            name: "Honduras",
            flag: "\u{1F1ED}\u{1F1F3}",
        },
    );
    countries.insert(
        "HK",
        Country {
            name: "Hong Kong",
            flag: "\u{1F1ED}\u{1F1F0}",
        },
    );
    countries.insert(
        "HKG",
        Country {
            name: "Hong Kong",
            flag: "\u{1F1ED}\u{1F1F0}",
        },
    );
    countries.insert(
        "HU",
        Country {
            name: "Hungary",
            flag: "\u{1F1ED}\u{1F1FA}",
        },
    );
    countries.insert(
        "HUN",
        Country {
            name: "Hungary",
            flag: "\u{1F1ED}\u{1F1FA}",
        },
    );
    countries.insert(
        "IS",
        Country {
            name: "Iceland",
            flag: "\u{1F1EE}\u{1F1F8}",
        },
    );
    countries.insert(
        "ISL",
        Country {
            name: "Iceland",
            flag: "\u{1F1EE}\u{1F1F8}",
        },
    );
    countries.insert(
        "IN",
        Country {
            name: "India",
            flag: "\u{1F1EE}\u{1F1F3}",
        },
    );
    countries.insert(
        "IND",
        Country {
            name: "India",
            flag: "\u{1F1EE}\u{1F1F3}",
        },
    );
    countries.insert(
        "ID",
        Country {
            name: "Indonesia",
            flag: "\u{1F1EE}\u{1F1E9}",
        },
    );
    countries.insert(
        "IDN",
        Country {
            name: "Indonesia",
            flag: "\u{1F1EE}\u{1F1E9}",
        },
    );
    countries.insert(
        "IR",
        Country {
            name: "Iran",
            flag: "\u{1F1EE}\u{1F1F7}",
        },
    );
    countries.insert(
        "IRN",
        Country {
            name: "Iran",
            flag: "\u{1F1EE}\u{1F1F7}",
        },
    );
    countries.insert(
        "IQ",
        Country {
            name: "Iraq",
            flag: "\u{1F1EE}\u{1F1F6}",
        },
    );
    countries.insert(
        "IRQ",
        Country {
            name: "Iraq",
            flag: "\u{1F1EE}\u{1F1F6}",
        },
    );
    countries.insert(
        "IE",
        Country {
            name: "Ireland",
            flag: "\u{1F1EE}\u{1F1EA}",
        },
    );
    countries.insert(
        "IRL",
        Country {
            name: "Ireland",
            flag: "\u{1F1EE}\u{1F1EA}",
        },
    );
    countries.insert(
        "IM",
        Country {
            name: "Isle of Man",
            flag: "\u{1F1EE}\u{1F1F2}",
        },
    );
    countries.insert(
        "IMN",
        Country {
            name: "Isle of Man",
            flag: "\u{1F1EE}\u{1F1F2}",
        },
    );
    countries.insert(
        "IL",
        Country {
            name: "Israel",
            flag: "\u{1F1EE}\u{1F1F1}",
        },
    );
    countries.insert(
        "ISR",
        Country {
            name: "Israel",
            flag: "\u{1F1EE}\u{1F1F1}",
        },
    );
    countries.insert(
        "IT",
        Country {
            name: "Italy",
            flag: "\u{1F1EE}\u{1F1F9}",
        },
    );
    countries.insert(
        "ITA",
        Country {
            name: "Italy",
            flag: "\u{1F1EE}\u{1F1F9}",
        },
    );
    countries.insert(
        "JM",
        Country {
            name: "Jamaica",
            flag: "\u{1F1EF}\u{1F1F2}",
        },
    );
    countries.insert(
        "JAM",
        Country {
            name: "Jamaica",
            flag: "\u{1F1EF}\u{1F1F2}",
        },
    );
    countries.insert(
        "JP",
        Country {
            name: "Japan",
            flag: "\u{1F1EF}\u{1F1F5}",
        },
    );
    countries.insert(
        "JPN",
        Country {
            name: "Japan",
            flag: "\u{1F1EF}\u{1F1F5}",
        },
    );
    countries.insert(
        "JE",
        Country {
            name: "Jersey",
            flag: "\u{1F1EF}\u{1F1EA}",
        },
    );
    countries.insert(
        "JEY",
        Country {
            name: "Jersey",
            flag: "\u{1F1EF}\u{1F1EA}",
        },
    );
    countries.insert(
        "JO",
        Country {
            name: "Jordan",
            flag: "\u{1F1EF}\u{1F1F4}",
        },
    );
    countries.insert(
        "JOR",
        Country {
            name: "Jordan",
            flag: "\u{1F1EF}\u{1F1F4}",
        },
    );
    countries.insert(
        "KZ",
        Country {
            name: "Kazakhstan",
            flag: "\u{1F1F0}\u{1F1FF}",
        },
    );
    countries.insert(
        "KAZ",
        Country {
            name: "Kazakhstan",
            flag: "\u{1F1F0}\u{1F1FF}",
        },
    );
    countries.insert(
        "KE",
        Country {
            name: "Kenya",
            flag: "\u{1F1F0}\u{1F1EA}",
        },
    );
    countries.insert(
        "KEN",
        Country {
            name: "Kenya",
            flag: "\u{1F1F0}\u{1F1EA}",
        },
    );
    countries.insert(
        "KI",
        Country {
            name: "Kiribati",
            flag: "\u{1F1F0}\u{1F1EE}",
        },
    );
    countries.insert(
        "KIR",
        Country {
            name: "Kiribati",
            flag: "\u{1F1F0}\u{1F1EE}",
        },
    );
    countries.insert(
        "XK",
        Country {
            name: "Kosovo",
            flag: "\u{1F1FD}\u{1F1F0}",
        },
    );
    countries.insert(
        "XKX",
        Country {
            name: "Kosovo",
            flag: "\u{1F1FD}\u{1F1F0}",
        },
    );
    countries.insert(
        "KW",
        Country {
            name: "Kuwait",
            flag: "\u{1F1F0}\u{1F1FC}",
        },
    );
    countries.insert(
        "KWT",
        Country {
            name: "Kuwait",
            flag: "\u{1F1F0}\u{1F1FC}",
        },
    );
    countries.insert(
        "KG",
        Country {
            name: "Kyrgyzstan",
            flag: "\u{1F1F0}\u{1F1EC}",
        },
    );
    countries.insert(
        "KGZ",
        Country {
            name: "Kyrgyzstan",
            flag: "\u{1F1F0}\u{1F1EC}",
        },
    );
    countries.insert(
        "LA",
        Country {
            name: "Laos",
            flag: "\u{1F1F1}\u{1F1E6}",
        },
    );
    countries.insert(
        "LAO",
        Country {
            name: "Laos",
            flag: "\u{1F1F1}\u{1F1E6}",
        },
    );
    countries.insert(
        "LV",
        Country {
            name: "Latvia",
            flag: "\u{1F1F1}\u{1F1FB}",
        },
    );
    countries.insert(
        "LVA",
        Country {
            name: "Latvia",
            flag: "\u{1F1F1}\u{1F1FB}",
        },
    );
    countries.insert(
        "LB",
        Country {
            name: "Lebanon",
            flag: "\u{1F1F1}\u{1F1E7}",
        },
    );
    countries.insert(
        "LBN",
        Country {
            name: "Lebanon",
            flag: "\u{1F1F1}\u{1F1E7}",
        },
    );
    countries.insert(
        "LS",
        Country {
            name: "Lesotho",
            flag: "\u{1F1F1}\u{1F1F8}",
        },
    );
    countries.insert(
        "LSO",
        Country {
            name: "Lesotho",
            flag: "\u{1F1F1}\u{1F1F8}",
        },
    );
    countries.insert(
        "LR",
        Country {
            name: "Liberia",
            flag: "\u{1F1F1}\u{1F1F7}",
        },
    );
    countries.insert(
        "LBR",
        Country {
            name: "Liberia",
            flag: "\u{1F1F1}\u{1F1F7}",
        },
    );
    countries.insert(
        "LY",
        Country {
            name: "Libya",
            flag: "\u{1F1F1}\u{1F1FE}",
        },
    );
    countries.insert(
        "LBY",
        Country {
            name: "Libya",
            flag: "\u{1F1F1}\u{1F1FE}",
        },
    );
    countries.insert(
        "LI",
        Country {
            name: "Liechtenstein",
            flag: "\u{1F1F1}\u{1F1EE}",
        },
    );
    countries.insert(
        "LIE",
        Country {
            name: "Liechtenstein",
            flag: "\u{1F1F1}\u{1F1EE}",
        },
    );
    countries.insert(
        "LT",
        Country {
            name: "Lithuania",
            flag: "\u{1F1F1}\u{1F1F9}",
        },
    );
    countries.insert(
        "LTU",
        Country {
            name: "Lithuania",
            flag: "\u{1F1F1}\u{1F1F9}",
        },
    );
    countries.insert(
        "LU",
        Country {
            name: "Luxembourg",
            flag: "\u{1F1F1}\u{1F1FA}",
        },
    );
    countries.insert(
        "LUX",
        Country {
            name: "Luxembourg",
            flag: "\u{1F1F1}\u{1F1FA}",
        },
    );
    countries.insert(
        "MO",
        Country {
            name: "Macao",
            flag: "\u{1F1F2}\u{1F1F4}",
        },
    );
    countries.insert(
        "MAC",
        Country {
            name: "Macao",
            flag: "\u{1F1F2}\u{1F1F4}",
        },
    );
    countries.insert(
        "MK",
        Country {
            name: "Macedonia",
            flag: "\u{1F1F2}\u{1F1F0}",
        },
    );
    countries.insert(
        "MKD",
        Country {
            name: "Macedonia",
            flag: "\u{1F1F2}\u{1F1F0}",
        },
    );
    countries.insert(
        "MG",
        Country {
            name: "Madagascar",
            flag: "\u{1F1F2}\u{1F1EC}",
        },
    );
    countries.insert(
        "MDG",
        Country {
            name: "Madagascar",
            flag: "\u{1F1F2}\u{1F1EC}",
        },
    );
    countries.insert(
        "MW",
        Country {
            name: "Malawi",
            flag: "\u{1F1F2}\u{1F1FC}",
        },
    );
    countries.insert(
        "MWI",
        Country {
            name: "Malawi",
            flag: "\u{1F1F2}\u{1F1FC}",
        },
    );
    countries.insert(
        "MY",
        Country {
            name: "Malaysia",
            flag: "\u{1F1F2}\u{1F1FE}",
        },
    );
    countries.insert(
        "MYS",
        Country {
            name: "Malaysia",
            flag: "\u{1F1F2}\u{1F1FE}",
        },
    );
    countries.insert(
        "MV",
        Country {
            name: "Maldives",
            flag: "\u{1F1F2}\u{1F1FB}",
        },
    );
    countries.insert(
        "MDV",
        Country {
            name: "Maldives",
            flag: "\u{1F1F2}\u{1F1FB}",
        },
    );
    countries.insert(
        "ML",
        Country {
            name: "Mali",
            flag: "\u{1F1F2}\u{1F1F1}",
        },
    );
    countries.insert(
        "MLI",
        Country {
            name: "Mali",
            flag: "\u{1F1F2}\u{1F1F1}",
        },
    );
    countries.insert(
        "MT",
        Country {
            name: "Malta",
            flag: "\u{1F1F2}\u{1F1F9}",
        },
    );
    countries.insert(
        "MLT",
        Country {
            name: "Malta",
            flag: "\u{1F1F2}\u{1F1F9}",
        },
    );
    countries.insert(
        "MH",
        Country {
            name: "Marshall Islands",
            flag: "\u{1F1F2}\u{1F1ED}",
        },
    );
    countries.insert(
        "MHL",
        Country {
            name: "Marshall Islands",
            flag: "\u{1F1F2}\u{1F1ED}",
        },
    );
    countries.insert(
        "MR",
        Country {
            name: "Mauritania",
            flag: "\u{1F1F2}\u{1F1F7}",
        },
    );
    countries.insert(
        "MRT",
        Country {
            name: "Mauritania",
            flag: "\u{1F1F2}\u{1F1F7}",
        },
    );
    countries.insert(
        "MU",
        Country {
            name: "Mauritius",
            flag: "\u{1F1F2}\u{1F1FA}",
        },
    );
    countries.insert(
        "MUS",
        Country {
            name: "Mauritius",
            flag: "\u{1F1F2}\u{1F1FA}",
        },
    );
    countries.insert(
        "YT",
        Country {
            name: "Mayotte",
            flag: "\u{1F1FE}\u{1F1F9}",
        },
    );
    countries.insert(
        "MYT",
        Country {
            name: "Mayotte",
            flag: "\u{1F1FE}\u{1F1F9}",
        },
    );
    countries.insert(
        "MX",
        Country {
            name: "Mexico",
            flag: "\u{1F1F2}\u{1F1FD}",
        },
    );
    countries.insert(
        "MEX",
        Country {
            name: "Mexico",
            flag: "\u{1F1F2}\u{1F1FD}",
        },
    );
    countries.insert(
        "FM",
        Country {
            name: "Micronesia",
            flag: "\u{1F1EB}\u{1F1F2}",
        },
    );
    countries.insert(
        "FSM",
        Country {
            name: "Micronesia",
            flag: "\u{1F1EB}\u{1F1F2}",
        },
    );
    countries.insert(
        "MD",
        Country {
            name: "Moldova",
            flag: "\u{1F1F2}\u{1F1E9}",
        },
    );
    countries.insert(
        "MDA",
        Country {
            name: "Moldova",
            flag: "\u{1F1F2}\u{1F1E9}",
        },
    );
    countries.insert(
        "MC",
        Country {
            name: "Monaco",
            flag: "\u{1F1F2}\u{1F1E8}",
        },
    );
    countries.insert(
        "MCO",
        Country {
            name: "Monaco",
            flag: "\u{1F1F2}\u{1F1E8}",
        },
    );
    countries.insert(
        "MN",
        Country {
            name: "Mongolia",
            flag: "\u{1F1F2}\u{1F1F3}",
        },
    );
    countries.insert(
        "MNG",
        Country {
            name: "Mongolia",
            flag: "\u{1F1F2}\u{1F1F3}",
        },
    );
    countries.insert(
        "ME",
        Country {
            name: "Montenegro",
            flag: "\u{1F1F2}\u{1F1EA}",
        },
    );
    countries.insert(
        "MNE",
        Country {
            name: "Montenegro",
            flag: "\u{1F1F2}\u{1F1EA}",
        },
    );
    countries.insert(
        "MS",
        Country {
            name: "Montserrat",
            flag: "\u{1F1F2}\u{1F1F8}",
        },
    );
    countries.insert(
        "MSR",
        Country {
            name: "Montserrat",
            flag: "\u{1F1F2}\u{1F1F8}",
        },
    );
    countries.insert(
        "MA",
        Country {
            name: "Morocco",
            flag: "\u{1F1F2}\u{1F1E6}",
        },
    );
    countries.insert(
        "MAR",
        Country {
            name: "Morocco",
            flag: "\u{1F1F2}\u{1F1E6}",
        },
    );
    countries.insert(
        "MZ",
        Country {
            name: "Mozambique",
            flag: "\u{1F1F2}\u{1F1FF}",
        },
    );
    countries.insert(
        "MOZ",
        Country {
            name: "Mozambique",
            flag: "\u{1F1F2}\u{1F1FF}",
        },
    );
    countries.insert(
        "MM",
        Country {
            name: "Myanmar",
            flag: "\u{1F1F2}\u{1F1F2}",
        },
    );
    countries.insert(
        "MMR",
        Country {
            name: "Myanmar",
            flag: "\u{1F1F2}\u{1F1F2}",
        },
    );
    countries.insert(
        "NA",
        Country {
            name: "Namibia",
            flag: "\u{1F1F3}\u{1F1E6}",
        },
    );
    countries.insert(
        "NAM",
        Country {
            name: "Namibia",
            flag: "\u{1F1F3}\u{1F1E6}",
        },
    );
    countries.insert(
        "NR",
        Country {
            name: "Nauru",
            flag: "\u{1F1F3}\u{1F1F7}",
        },
    );
    countries.insert(
        "NRU",
        Country {
            name: "Nauru",
            flag: "\u{1F1F3}\u{1F1F7}",
        },
    );
    countries.insert(
        "NP",
        Country {
            name: "Nepal",
            flag: "\u{1F1F3}\u{1F1F5}",
        },
    );
    countries.insert(
        "NPL",
        Country {
            name: "Nepal",
            flag: "\u{1F1F3}\u{1F1F5}",
        },
    );
    countries.insert(
        "NL",
        Country {
            name: "Netherlands",
            flag: "\u{1F1F3}\u{1F1F1}",
        },
    );
    countries.insert(
        "NLD",
        Country {
            name: "Netherlands",
            flag: "\u{1F1F3}\u{1F1F1}",
        },
    );
    countries.insert(
        "NC",
        Country {
            name: "New Caledonia",
            flag: "\u{1F1F3}\u{1F1E8}",
        },
    );
    countries.insert(
        "NCL",
        Country {
            name: "New Caledonia",
            flag: "\u{1F1F3}\u{1F1E8}",
        },
    );
    countries.insert(
        "NZ",
        Country {
            name: "New Zealand",
            flag: "\u{1F1F3}\u{1F1FF}",
        },
    );
    countries.insert(
        "NZL",
        Country {
            name: "New Zealand",
            flag: "\u{1F1F3}\u{1F1FF}",
        },
    );
    countries.insert(
        "NI",
        Country {
            name: "Nicaragua",
            flag: "\u{1F1F3}\u{1F1EE}",
        },
    );
    countries.insert(
        "NIC",
        Country {
            name: "Nicaragua",
            flag: "\u{1F1F3}\u{1F1EE}",
        },
    );
    countries.insert(
        "NE",
        Country {
            name: "Niger",
            flag: "\u{1F1F3}\u{1F1EA}",
        },
    );
    countries.insert(
        "NER",
        Country {
            name: "Niger",
            flag: "\u{1F1F3}\u{1F1EA}",
        },
    );
    countries.insert(
        "NG",
        Country {
            name: "Nigeria",
            flag: "\u{1F1F3}\u{1F1EC}",
        },
    );
    countries.insert(
        "NGA",
        Country {
            name: "Nigeria",
            flag: "\u{1F1F3}\u{1F1EC}",
        },
    );
    countries.insert(
        "NU",
        Country {
            name: "Niue",
            flag: "\u{1F1F3}\u{1F1FA}",
        },
    );
    countries.insert(
        "NIU",
        Country {
            name: "Niue",
            flag: "\u{1F1F3}\u{1F1FA}",
        },
    );
    countries.insert(
        "KP",
        Country {
            name: "North Korea",
            flag: "\u{1F1F0}\u{1F1F5}",
        },
    );
    countries.insert(
        "PRK",
        Country {
            name: "North Korea",
            flag: "\u{1F1F0}\u{1F1F5}",
        },
    );
    countries.insert(
        "MP",
        Country {
            name: "Northern Mariana Islands",
            flag: "\u{1F1F2}\u{1F1F5}",
        },
    );
    countries.insert(
        "MNP",
        Country {
            name: "Northern Mariana Islands",
            flag: "\u{1F1F2}\u{1F1F5}",
        },
    );
    countries.insert(
        "NO",
        Country {
            name: "Norway",
            flag: "\u{1F1F3}\u{1F1F4}",
        },
    );
    countries.insert(
        "NOR",
        Country {
            name: "Norway",
            flag: "\u{1F1F3}\u{1F1F4}",
        },
    );
    countries.insert(
        "OM",
        Country {
            name: "Oman",
            flag: "\u{1F1F4}\u{1F1F2}",
        },
    );
    countries.insert(
        "OMN",
        Country {
            name: "Oman",
            flag: "\u{1F1F4}\u{1F1F2}",
        },
    );
    countries.insert(
        "PK",
        Country {
            name: "Pakistan",
            flag: "\u{1F1F5}\u{1F1F0}",
        },
    );
    countries.insert(
        "PAK",
        Country {
            name: "Pakistan",
            flag: "\u{1F1F5}\u{1F1F0}",
        },
    );
    countries.insert(
        "PW",
        Country {
            name: "Palau",
            flag: "\u{1F1F5}\u{1F1FC}",
        },
    );
    countries.insert(
        "PLW",
        Country {
            name: "Palau",
            flag: "\u{1F1F5}\u{1F1FC}",
        },
    );
    countries.insert(
        "PS",
        Country {
            name: "Palestine",
            flag: "\u{1F1F5}\u{1F1F8}",
        },
    );
    countries.insert(
        "PSE",
        Country {
            name: "Palestine",
            flag: "\u{1F1F5}\u{1F1F8}",
        },
    );
    countries.insert(
        "PA",
        Country {
            name: "Panama",
            flag: "\u{1F1F5}\u{1F1E6}",
        },
    );
    countries.insert(
        "PAN",
        Country {
            name: "Panama",
            flag: "\u{1F1F5}\u{1F1E6}",
        },
    );
    countries.insert(
        "PG",
        Country {
            name: "Papua New Guinea",
            flag: "\u{1F1F5}\u{1F1EC}",
        },
    );
    countries.insert(
        "PNG",
        Country {
            name: "Papua New Guinea",
            flag: "\u{1F1F5}\u{1F1EC}",
        },
    );
    countries.insert(
        "PY",
        Country {
            name: "Paraguay",
            flag: "\u{1F1F5}\u{1F1FE}",
        },
    );
    countries.insert(
        "PRY",
        Country {
            name: "Paraguay",
            flag: "\u{1F1F5}\u{1F1FE}",
        },
    );
    countries.insert(
        "PE",
        Country {
            name: "Peru",
            flag: "\u{1F1F5}\u{1F1EA}",
        },
    );
    countries.insert(
        "PER",
        Country {
            name: "Peru",
            flag: "\u{1F1F5}\u{1F1EA}",
        },
    );
    countries.insert(
        "PH",
        Country {
            name: "Philippines",
            flag: "\u{1F1F5}\u{1F1ED}",
        },
    );
    countries.insert(
        "PHL",
        Country {
            name: "Philippines",
            flag: "\u{1F1F5}\u{1F1ED}",
        },
    );
    countries.insert(
        "PN",
        Country {
            name: "Pitcairn",
            flag: "\u{1F1F5}\u{1F1F3}",
        },
    );
    countries.insert(
        "PCN",
        Country {
            name: "Pitcairn",
            flag: "\u{1F1F5}\u{1F1F3}",
        },
    );
    countries.insert(
        "PL",
        Country {
            name: "Poland",
            flag: "\u{1F1F5}\u{1F1F1}",
        },
    );
    countries.insert(
        "POL",
        Country {
            name: "Poland",
            flag: "\u{1F1F5}\u{1F1F1}",
        },
    );
    countries.insert(
        "PT",
        Country {
            name: "Portugal",
            flag: "\u{1F1F5}\u{1F1F9}",
        },
    );
    countries.insert(
        "PRT",
        Country {
            name: "Portugal",
            flag: "\u{1F1F5}\u{1F1F9}",
        },
    );
    countries.insert(
        "PR",
        Country {
            name: "Puerto Rico",
            flag: "\u{1F1F5}\u{1F1F7}",
        },
    );
    countries.insert(
        "PRI",
        Country {
            name: "Puerto Rico",
            flag: "\u{1F1F5}\u{1F1F7}",
        },
    );
    countries.insert(
        "QA",
        Country {
            name: "Qatar",
            flag: "\u{1F1F6}\u{1F1E6}",
        },
    );
    countries.insert(
        "QAT",
        Country {
            name: "Qatar",
            flag: "\u{1F1F6}\u{1F1E6}",
        },
    );
    countries.insert(
        "RO",
        Country {
            name: "Romania",
            flag: "\u{1F1F7}\u{1F1F4}",
        },
    );
    countries.insert(
        "ROU",
        Country {
            name: "Romania",
            flag: "\u{1F1F7}\u{1F1F4}",
        },
    );
    countries.insert(
        "RU",
        Country {
            name: "Russia",
            flag: "\u{1F1F7}\u{1F1FA}",
        },
    );
    countries.insert(
        "RUS",
        Country {
            name: "Russia",
            flag: "\u{1F1F7}\u{1F1FA}",
        },
    );
    countries.insert(
        "RW",
        Country {
            name: "Rwanda",
            flag: "\u{1F1F7}\u{1F1FC}",
        },
    );
    countries.insert(
        "RWA",
        Country {
            name: "Rwanda",
            flag: "\u{1F1F7}\u{1F1FC}",
        },
    );
    countries.insert(
        "RE",
        Country {
            name: "Reunion",
            flag: "\u{1F1F7}\u{1F1EA}",
        },
    );
    countries.insert(
        "REU",
        Country {
            name: "Reunion",
            flag: "\u{1F1F7}\u{1F1EA}",
        },
    );
    countries.insert(
        "WS",
        Country {
            name: "Samoa",
            flag: "\u{1F1FC}\u{1F1F8}",
        },
    );
    countries.insert(
        "WSM",
        Country {
            name: "Samoa",
            flag: "\u{1F1FC}\u{1F1F8}",
        },
    );
    countries.insert(
        "SM",
        Country {
            name: "San Marino",
            flag: "\u{1F1F8}\u{1F1F2}",
        },
    );
    countries.insert(
        "SMR",
        Country {
            name: "San Marino",
            flag: "\u{1F1F8}\u{1F1F2}",
        },
    );
    countries.insert(
        "SA",
        Country {
            name: "Saudi Arabia",
            flag: "\u{1F1F8}\u{1F1E6}",
        },
    );
    countries.insert(
        "SAU",
        Country {
            name: "Saudi Arabia",
            flag: "\u{1F1F8}\u{1F1E6}",
        },
    );
    countries.insert(
        "SN",
        Country {
            name: "Senegal",
            flag: "\u{1F1F8}\u{1F1F3}",
        },
    );
    countries.insert(
        "SEN",
        Country {
            name: "Senegal",
            flag: "\u{1F1F8}\u{1F1F3}",
        },
    );
    countries.insert(
        "RS",
        Country {
            name: "Serbia",
            flag: "\u{1F1F7}\u{1F1F8}",
        },
    );
    countries.insert(
        "SRB",
        Country {
            name: "Serbia",
            flag: "\u{1F1F7}\u{1F1F8}",
        },
    );
    countries.insert(
        "SC",
        Country {
            name: "Seychelles",
            flag: "\u{1F1F8}\u{1F1E8}",
        },
    );
    countries.insert(
        "SYC",
        Country {
            name: "Seychelles",
            flag: "\u{1F1F8}\u{1F1E8}",
        },
    );
    countries.insert(
        "SL",
        Country {
            name: "Sierra Leone",
            flag: "\u{1F1F8}\u{1F1F1}",
        },
    );
    countries.insert(
        "SLE",
        Country {
            name: "Sierra Leone",
            flag: "\u{1F1F8}\u{1F1F1}",
        },
    );
    countries.insert(
        "SG",
        Country {
            name: "Singapore",
            flag: "\u{1F1F8}\u{1F1EC}",
        },
    );
    countries.insert(
        "SGP",
        Country {
            name: "Singapore",
            flag: "\u{1F1F8}\u{1F1EC}",
        },
    );
    countries.insert(
        "SX",
        Country {
            name: "Sint Maarten",
            flag: "\u{1F1F8}\u{1F1FD}",
        },
    );
    countries.insert(
        "SXM",
        Country {
            name: "Sint Maarten",
            flag: "\u{1F1F8}\u{1F1FD}",
        },
    );
    countries.insert(
        "SK",
        Country {
            name: "Slovakia",
            flag: "\u{1F1F8}\u{1F1F0}",
        },
    );
    countries.insert(
        "SVK",
        Country {
            name: "Slovakia",
            flag: "\u{1F1F8}\u{1F1F0}",
        },
    );
    countries.insert(
        "SI",
        Country {
            name: "Slovenia",
            flag: "\u{1F1F8}\u{1F1EE}",
        },
    );
    countries.insert(
        "SVN",
        Country {
            name: "Slovenia",
            flag: "\u{1F1F8}\u{1F1EE}",
        },
    );
    countries.insert(
        "SB",
        Country {
            name: "Solomon Islands",
            flag: "\u{1F1F8}\u{1F1E7}",
        },
    );
    countries.insert(
        "SLB",
        Country {
            name: "Solomon Islands",
            flag: "\u{1F1F8}\u{1F1E7}",
        },
    );
    countries.insert(
        "SO",
        Country {
            name: "Somalia",
            flag: "\u{1F1F8}\u{1F1F4}",
        },
    );
    countries.insert(
        "SOM",
        Country {
            name: "Somalia",
            flag: "\u{1F1F8}\u{1F1F4}",
        },
    );
    countries.insert(
        "ZA",
        Country {
            name: "South Africa",
            flag: "\u{1F1FF}\u{1F1E6}",
        },
    );
    countries.insert(
        "ZAF",
        Country {
            name: "South Africa",
            flag: "\u{1F1FF}\u{1F1E6}",
        },
    );
    countries.insert(
        "KR",
        Country {
            name: "South Korea",
            flag: "\u{1F1F0}\u{1F1F7}",
        },
    );
    countries.insert(
        "KOR",
        Country {
            name: "South Korea",
            flag: "\u{1F1F0}\u{1F1F7}",
        },
    );
    countries.insert(
        "SS",
        Country {
            name: "South Sudan",
            flag: "\u{1F1F8}\u{1F1F8}",
        },
    );
    countries.insert(
        "SSD",
        Country {
            name: "South Sudan",
            flag: "\u{1F1F8}\u{1F1F8}",
        },
    );
    countries.insert(
        "ES",
        Country {
            name: "Spain",
            flag: "\u{1F1EA}\u{1F1F8}",
        },
    );
    countries.insert(
        "ESP",
        Country {
            name: "Spain",
            flag: "\u{1F1EA}\u{1F1F8}",
        },
    );
    countries.insert(
        "LK",
        Country {
            name: "Sri Lanka",
            flag: "\u{1F1F1}\u{1F1F0}",
        },
    );
    countries.insert(
        "LKA",
        Country {
            name: "Sri Lanka",
            flag: "\u{1F1F1}\u{1F1F0}",
        },
    );
    countries.insert(
        "SD",
        Country {
            name: "Sudan",
            flag: "\u{1F1F8}\u{1F1E9}",
        },
    );
    countries.insert(
        "SDN",
        Country {
            name: "Sudan",
            flag: "\u{1F1F8}\u{1F1E9}",
        },
    );
    countries.insert(
        "SR",
        Country {
            name: "Suriname",
            flag: "\u{1F1F8}\u{1F1F7}",
        },
    );
    countries.insert(
        "SUR",
        Country {
            name: "Suriname",
            flag: "\u{1F1F8}\u{1F1F7}",
        },
    );
    countries.insert(
        "SJ",
        Country {
            name: "Svalbard and Jan Mayen",
            flag: "\u{1F1F8}\u{1F1EF}",
        },
    );
    countries.insert(
        "SJM",
        Country {
            name: "Svalbard and Jan Mayen",
            flag: "\u{1F1F8}\u{1F1EF}",
        },
    );
    countries.insert(
        "SE",
        Country {
            name: "Sweden",
            flag: "\u{1F1F8}\u{1F1EA}",
        },
    );
    countries.insert(
        "SWE",
        Country {
            name: "Sweden",
            flag: "\u{1F1F8}\u{1F1EA}",
        },
    );
    countries.insert(
        "CH",
        Country {
            name: "Switzerland",
            flag: "\u{1F1E8}\u{1F1ED}",
        },
    );
    countries.insert(
        "CHE",
        Country {
            name: "Switzerland",
            flag: "\u{1F1E8}\u{1F1ED}",
        },
    );
    countries.insert(
        "SY",
        Country {
            name: "Syria",
            flag: "\u{1F1F8}\u{1F1FE}",
        },
    );
    countries.insert(
        "SYR",
        Country {
            name: "Syria",
            flag: "\u{1F1F8}\u{1F1FE}",
        },
    );
    countries.insert(
        "TW",
        Country {
            name: "Taiwan",
            flag: "\u{1F1F9}\u{1F1FC}",
        },
    );
    countries.insert(
        "TWN",
        Country {
            name: "Taiwan",
            flag: "\u{1F1F9}\u{1F1FC}",
        },
    );
    countries.insert(
        "TJ",
        Country {
            name: "Tajikistan",
            flag: "\u{1F1F9}\u{1F1EF}",
        },
    );
    countries.insert(
        "TJK",
        Country {
            name: "Tajikistan",
            flag: "\u{1F1F9}\u{1F1EF}",
        },
    );
    countries.insert(
        "TZ",
        Country {
            name: "Tanzania",
            flag: "\u{1F1F9}\u{1F1FF}",
        },
    );
    countries.insert(
        "TZA",
        Country {
            name: "Tanzania",
            flag: "\u{1F1F9}\u{1F1FF}",
        },
    );
    countries.insert(
        "TH",
        Country {
            name: "Thailand",
            flag: "\u{1F1F9}\u{1F1ED}",
        },
    );
    countries.insert(
        "THA",
        Country {
            name: "Thailand",
            flag: "\u{1F1F9}\u{1F1ED}",
        },
    );
    countries.insert(
        "TG",
        Country {
            name: "Togo",
            flag: "\u{1F1F9}\u{1F1EC}",
        },
    );
    countries.insert(
        "TGO",
        Country {
            name: "Togo",
            flag: "\u{1F1F9}\u{1F1EC}",
        },
    );
    countries.insert(
        "TK",
        Country {
            name: "Tokelau",
            flag: "\u{1F1F9}\u{1F1F0}",
        },
    );
    countries.insert(
        "TKL",
        Country {
            name: "Tokelau",
            flag: "\u{1F1F9}\u{1F1F0}",
        },
    );
    countries.insert(
        "TO",
        Country {
            name: "Tonga",
            flag: "\u{1F1F9}\u{1F1F4}",
        },
    );
    countries.insert(
        "TON",
        Country {
            name: "Tonga",
            flag: "\u{1F1F9}\u{1F1F4}",
        },
    );
    countries.insert(
        "TT",
        Country {
            name: "Trinidad and Tobago",
            flag: "\u{1F1F9}\u{1F1F9}",
        },
    );
    countries.insert(
        "TTO",
        Country {
            name: "Trinidad and Tobago",
            flag: "\u{1F1F9}\u{1F1F9}",
        },
    );
    countries.insert(
        "TN",
        Country {
            name: "Tunisia",
            flag: "\u{1F1F9}\u{1F1F3}",
        },
    );
    countries.insert(
        "TUN",
        Country {
            name: "Tunisia",
            flag: "\u{1F1F9}\u{1F1F3}",
        },
    );
    countries.insert(
        "TR",
        Country {
            name: "Turkey",
            flag: "\u{1F1F9}\u{1F1F7}",
        },
    );
    countries.insert(
        "TUR",
        Country {
            name: "Turkey",
            flag: "\u{1F1F9}\u{1F1F7}",
        },
    );
    countries.insert(
        "TM",
        Country {
            name: "Turkmenistan",
            flag: "\u{1F1F9}\u{1F1F2}",
        },
    );
    countries.insert(
        "TKM",
        Country {
            name: "Turkmenistan",
            flag: "\u{1F1F9}\u{1F1F2}",
        },
    );
    countries.insert(
        "TC",
        Country {
            name: "Turks and Caicos Islands",
            flag: "\u{1F1F9}\u{1F1E8}",
        },
    );
    countries.insert(
        "TCA",
        Country {
            name: "Turks and Caicos Islands",
            flag: "\u{1F1F9}\u{1F1E8}",
        },
    );
    countries.insert(
        "TV",
        Country {
            name: "Tuvalu",
            flag: "\u{1F1F9}\u{1F1FB}",
        },
    );
    countries.insert(
        "TUV",
        Country {
            name: "Tuvalu",
            flag: "\u{1F1F9}\u{1F1FB}",
        },
    );
    countries.insert(
        "VI",
        Country {
            name: "U.S. Virgin Islands",
            flag: "\u{1F1FB}\u{1F1EE}",
        },
    );
    countries.insert(
        "VIR",
        Country {
            name: "U.S. Virgin Islands",
            flag: "\u{1F1FB}\u{1F1EE}",
        },
    );
    countries.insert(
        "UG",
        Country {
            name: "Uganda",
            flag: "\u{1F1FA}\u{1F1EC}",
        },
    );
    countries.insert(
        "UGA",
        Country {
            name: "Uganda",
            flag: "\u{1F1FA}\u{1F1EC}",
        },
    );
    countries.insert(
        "UA",
        Country {
            name: "Ukraine",
            flag: "\u{1F1FA}\u{1F1E6}",
        },
    );
    countries.insert(
        "UKR",
        Country {
            name: "Ukraine",
            flag: "\u{1F1FA}\u{1F1E6}",
        },
    );
    countries.insert(
        "AE",
        Country {
            name: "United Arab Emirates",
            flag: "\u{1F1E6}\u{1F1EA}",
        },
    );
    countries.insert(
        "ARE",
        Country {
            name: "United Arab Emirates",
            flag: "\u{1F1E6}\u{1F1EA}",
        },
    );
    countries.insert(
        "GB",
        Country {
            name: "United Kingdom",
            flag: "\u{1F1EC}\u{1F1E7}",
        },
    );
    countries.insert(
        "GBR",
        Country {
            name: "United Kingdom",
            flag: "\u{1F1EC}\u{1F1E7}",
        },
    );
    countries.insert(
        "US",
        Country {
            name: "United States",
            flag: "\u{1F1FA}\u{1F1F8}",
        },
    );
    countries.insert(
        "USA",
        Country {
            name: "United States",
            flag: "\u{1F1FA}\u{1F1F8}",
        },
    );
    countries.insert(
        "UY",
        Country {
            name: "Uruguay",
            flag: "\u{1F1FA}\u{1F1FE}",
        },
    );
    countries.insert(
        "URY",
        Country {
            name: "Uruguay",
            flag: "\u{1F1FA}\u{1F1FE}",
        },
    );
    countries.insert(
        "UZ",
        Country {
            name: "Uzbekistan",
            flag: "\u{1F1FA}\u{1F1FF}",
        },
    );
    countries.insert(
        "UZB",
        Country {
            name: "Uzbekistan",
            flag: "\u{1F1FA}\u{1F1FF}",
        },
    );
    countries.insert(
        "VU",
        Country {
            name: "Vanuatu",
            flag: "\u{1F1FB}\u{1F1FA}",
        },
    );
    countries.insert(
        "VUT",
        Country {
            name: "Vanuatu",
            flag: "\u{1F1FB}\u{1F1FA}",
        },
    );
    countries.insert(
        "VA",
        Country {
            name: "Vatican City",
            flag: "\u{1F1FB}\u{1F1E6}",
        },
    );
    countries.insert(
        "VAT",
        Country {
            name: "Vatican City",
            flag: "\u{1F1FB}\u{1F1E6}",
        },
    );
    countries.insert(
        "VE",
        Country {
            name: "Venezuela",
            flag: "\u{1F1FB}\u{1F1EA}",
        },
    );
    countries.insert(
        "VEN",
        Country {
            name: "Venezuela",
            flag: "\u{1F1FB}\u{1F1EA}",
        },
    );
    countries.insert(
        "VN",
        Country {
            name: "Vietnam",
            flag: "\u{1F1FB}\u{1F1F3}",
        },
    );
    countries.insert(
        "VNM",
        Country {
            name: "Vietnam",
            flag: "\u{1F1FB}\u{1F1F3}",
        },
    );
    countries.insert(
        "WF",
        Country {
            name: "Wallis and Futuna",
            flag: "\u{1F1FC}\u{1F1EB}",
        },
    );
    countries.insert(
        "WLF",
        Country {
            name: "Wallis and Futuna",
            flag: "\u{1F1FC}\u{1F1EB}",
        },
    );
    countries.insert(
        "EH",
        Country {
            name: "Western Sahara",
            flag: "\u{1F1EA}\u{1F1ED}",
        },
    );
    countries.insert(
        "ESH",
        Country {
            name: "Western Sahara",
            flag: "\u{1F1EA}\u{1F1ED}",
        },
    );
    countries.insert(
        "YE",
        Country {
            name: "Yemen",
            flag: "\u{1F1FE}\u{1F1EA}",
        },
    );
    countries.insert(
        "YEM",
        Country {
            name: "Yemen",
            flag: "\u{1F1FE}\u{1F1EA}",
        },
    );
    countries.insert(
        "ZM",
        Country {
            name: "Zambia",
            flag: "\u{1F1FF}\u{1F1F2}",
        },
    );
    countries.insert(
        "ZMB",
        Country {
            name: "Zambia",
            flag: "\u{1F1FF}\u{1F1F2}",
        },
    );
    countries.insert(
        "ZW",
        Country {
            name: "Zimbabwe",
            flag: "\u{1F1FF}\u{1F1FC}",
        },
    );
    countries.insert(
        "ZWE",
        Country {
            name: "Zimbabwe",
            flag: "\u{1F1FF}\u{1F1FC}",
        },
    );

    COUNTRIES.store(Box::into_raw(Box::new(countries)), Ordering::Release);
}

pub fn get_country(name: &str) -> Option<&'static Country> {
    init_countries();
    let countries = unsafe { &*COUNTRIES.load(Ordering::Acquire) };

    countries.get(name)
}
