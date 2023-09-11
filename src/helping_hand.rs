use std::collections::HashMap;

pub fn transliterate(word: &String) -> String {
    let mut result = word.clone();
    let matches = [
        ("а", "a"),
        ("б", "b"),
        ("в", "v"),
        ("г", "g"),
        ("д", "d"),
        ("е", "e"),
        ("ё", "e"),
        ("ж", "zh"),
        ("з", "z"),
        ("и", "i"),
        ("й", "i"),
        ("к", "k"),
        ("л", "l"),
        ("м", "m"),
        ("н", "n"),
        ("о", "o"),
        ("п", "p"),
        ("р", "r"),
        ("с", "s"),
        ("т", "t"),
        ("у", "u"),
        ("ф", "f"),
        ("х", "h"),
        ("ц", "c"),
        ("ч", "cz"),
        ("ш", "sh"),
        ("щ", "scz"),
        ("ъ", ""),
        ("ы", "y"),
        ("ь", ""),
        ("э", "e"),
        ("ю", "yu"),
        ("я", "ja"),
        ("А", "A"),
        ("Б", "B"),
        ("В", "V"),
        ("Г", "G"),
        ("Д", "D"),
        ("Е", "E"),
        ("Ё", "E"),
        ("Ж", "ZH"),
        ("З", "Z"),
        ("И", "I"),
        ("Й", "I"),
        ("К", "K"),
        ("Л", "L"),
        ("М", "M"),
        ("Н", "N"),
        ("О", "O"),
        ("П", "P"),
        ("Р", "R"),
        ("С", "S"),
        ("Т", "T"),
        ("У", "U"),
        ("Ф", "F"),
        ("Х", "H"),
        ("Ц", "C"),
        ("Ч", "CZ"),
        ("Ш", "sh"),
        ("Щ", "sch"),
        ("Ъ", ""),
        ("Ы", "y"),
        ("Ь", ""),
        ("Э", "e"),
        ("Ю", "yu"),
        ("Я", "ya"),
        (",", ""),
        ("?", ""),
        (" ", " "),
        ("~", ""),
        ("!", ""),
        ("@", ""),
        ("#", ""),
        ("Є", "e"),
        ("—", ""),
    ];
    let dict: HashMap<&str, &str> = HashMap::from(matches);
    for (ru, english) in dict {
        result = result.replace(ru, english);
    }
    result
}


pub fn to_cert_name(name: String) -> String {
    let tmp: Vec<&str> = name.split(" ").collect();
    if tmp.len() >= 3 {
        format!(
            "{}.{}.{}",
            tmp[0].to_string(),
            tmp[1]
                .chars()
                .nth(0)
                .expect("SOMETHING WRONG WITH THE STRING")
                .to_string(),
            tmp[2]
                .chars()
                .nth(0)
                .expect("SOMETHING WRONG WITH THE STRING")
                .to_string(),
        )
    } else if tmp.len() == 2 {
        format!(
            "{}.{}",
            tmp[0].to_string(),
            tmp[1]
                .chars()
                .nth(0)
                .expect("SOMETHING WRONG WITH THE STRING")
                .to_string(),
        )
    } else {
        "".to_string()
    }
}
