use super::*;
use regex::Regex;
use svgbuilder::*;

pub struct Sfen {
    ban: String,
    teban: String,
    tegoma: String,
    nteme: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Teban {
    Sente,
    Gote,
    None,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KomaType {
    Aki,
    Fu,
    Kyosha,
    Keima,
    Gin,
    Kin,
    Kaku,
    Hisha,
    Gyoku,
}

impl KomaType {
    pub fn to_string(&self, promote: Promotion) -> String {
        let idx = [
            KomaType::Fu,
            KomaType::Kyosha,
            KomaType::Keima,
            KomaType::Gin,
            KomaType::Kin,
            KomaType::Kaku,
            KomaType::Hisha,
            KomaType::Gyoku,
            KomaType::Aki,
        ]
        .iter()
        .position(|&k| k == *self)
        .unwrap();
        match if promote.is_promoted() {
            "と杏圭全金馬龍玉"
        } else {
            "歩香桂銀金角飛玉"
        }
        .chars()
        .nth(idx)
        {
            Some(ch) => ch.to_string(),
            None => String::new(),
        }
    }

    pub fn from(ch: char) -> KomaType {
        let idx = "PLNSGBRK"
            .chars()
            .position(|k| k == ch.to_ascii_uppercase())
            .unwrap_or(8);
        [
            KomaType::Fu,
            KomaType::Kyosha,
            KomaType::Keima,
            KomaType::Gin,
            KomaType::Kin,
            KomaType::Kaku,
            KomaType::Hisha,
            KomaType::Gyoku,
            KomaType::Aki,
        ][idx]
    }
}

#[test]
fn fromtest() {
    let k = KomaType::from(' ');
    assert_eq!(k, KomaType::Aki);
    let k = KomaType::from('P');
    assert_eq!(k, KomaType::Fu);
    let k = KomaType::from('L');
    assert_eq!(k, KomaType::Kyosha);
    let k = KomaType::from('N');
    assert_eq!(k, KomaType::Keima);
    let k = KomaType::from('S');
    assert_eq!(k, KomaType::Gin);
    let k = KomaType::from('G');
    assert_eq!(k, KomaType::Kin);
    let k = KomaType::from('B');
    assert_eq!(k, KomaType::Kaku);
    let k = KomaType::from('R');
    assert_eq!(k, KomaType::Hisha);
    let k = KomaType::from('K');
    assert_eq!(k, KomaType::Gyoku);

    let k = KomaType::from('_');
    assert_eq!(k, KomaType::Aki);
    let k = KomaType::from('p');
    assert_eq!(k, KomaType::Fu);
    let k = KomaType::from('l');
    assert_eq!(k, KomaType::Kyosha);
    let k = KomaType::from('n');
    assert_eq!(k, KomaType::Keima);
    let k = KomaType::from('s');
    assert_eq!(k, KomaType::Gin);
    let k = KomaType::from('g');
    assert_eq!(k, KomaType::Kin);
    let k = KomaType::from('b');
    assert_eq!(k, KomaType::Kaku);
    let k = KomaType::from('r');
    assert_eq!(k, KomaType::Hisha);
    let k = KomaType::from('k');
    assert_eq!(k, KomaType::Gyoku);
}

#[test]
fn tostrtest() {
    let k = KomaType::from(' ');
    assert_eq!(k.to_string(Promotion::None), "");
    assert_eq!(k.to_string(Promotion::Promoted), "");
    let k = KomaType::from('P');
    assert_eq!(k.to_string(Promotion::None), "歩");
    assert_eq!(k.to_string(Promotion::Promoted), "と");
    let k = KomaType::from('L');
    assert_eq!(k.to_string(Promotion::None), "香");
    assert_eq!(k.to_string(Promotion::Promoted), "杏");
    let k = KomaType::from('N');
    assert_eq!(k.to_string(Promotion::None), "桂");
    assert_eq!(k.to_string(Promotion::Promoted), "圭");
    let k = KomaType::from('S');
    assert_eq!(k.to_string(Promotion::None), "銀");
    assert_eq!(k.to_string(Promotion::Promoted), "全");
    let k = KomaType::from('G');
    assert_eq!(k.to_string(Promotion::None), "金");
    assert_eq!(k.to_string(Promotion::Promoted), "金");
    let k = KomaType::from('B');
    assert_eq!(k.to_string(Promotion::None), "角");
    assert_eq!(k.to_string(Promotion::Promoted), "馬");
    let k = KomaType::from('R');
    assert_eq!(k.to_string(Promotion::None), "飛");
    assert_eq!(k.to_string(Promotion::Promoted), "龍");
    let k = KomaType::from('K');
    assert_eq!(k.to_string(Promotion::None), "玉");
    assert_eq!(k.to_string(Promotion::Promoted), "玉");
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Promotion {
    None,
    Promoted,
    NotPromoted,
}

impl Promotion {
    pub fn is_promoted(&self) -> bool {
        *self == Promotion::Promoted
    }
    pub fn is_notpromoted(&self) -> bool {
        *self == Promotion::NotPromoted
    }
    pub fn to_string(&self) -> String {
        match self {
            Promotion::Promoted => String::from("成"),
            Promotion::NotPromoted => String::from("不成"),
            Promotion::None => String::new(),
        }
    }
}

#[test]
fn promotest() {
    let prm = Promotion::None;
    assert_eq!(prm.is_promoted(), false);
    assert_eq!(prm.is_notpromoted(), false);
    assert_eq!(prm.to_string(), "");
    let prm = Promotion::Promoted;
    assert_eq!(prm.is_promoted(), true);
    assert_eq!(prm.is_notpromoted(), false);
    assert_eq!(prm.to_string(), "成");
    let prm = Promotion::NotPromoted;
    assert_eq!(prm.is_promoted(), false);
    assert_eq!(prm.is_notpromoted(), true);
    assert_eq!(prm.to_string(), "不成");
}

#[derive(Clone, Debug)]
pub struct Koma {
    koma: KomaType,
    promotion: Promotion,
    teban: Teban,
}

impl Koma {
    pub fn from(ch: char, promote: Promotion) -> Koma {
        let re = regex::Regex::new("[PLNSGBRK]").unwrap();
        Koma {
            koma: KomaType::from(ch),
            promotion: promote,
            teban: if re.is_match(&ch.to_ascii_uppercase().to_string()) {
                if ch.is_uppercase() {
                    Teban::Sente
                } else {
                    Teban::Gote
                }
            } else {
                Teban::None
            },
        }
    }

    pub fn fromcsa(csa: &str) -> Option<Koma> {
        let tbl = [
            ("FU", 'P', Promotion::None),
            ("KY", 'L', Promotion::None),
            ("KE", 'N', Promotion::None),
            ("GI", 'S', Promotion::None),
            ("KI", 'G', Promotion::None),
            ("KA", 'B', Promotion::None),
            ("HI", 'R', Promotion::None),
            ("OU", 'K', Promotion::None),
            ("TO", 'P', Promotion::Promoted),
            ("NY", 'L', Promotion::Promoted),
            ("NE", 'N', Promotion::Promoted),
            ("NG", 'S', Promotion::Promoted),
            // ("KI", 'G', Promotion::None),
            ("UM", 'B', Promotion::Promoted),
            ("RY", 'R', Promotion::Promoted),
            ("GY", 'K', Promotion::None),
        ];
        match tbl.iter().find(|e| e.0 == csa) {
            Some((_csa, ch, prm)) => Some(Koma::from(*ch, *prm)),
            None => None,
        }
    }

    pub fn to_string(&self) -> String {
        if self.teban == Teban::None || self.koma == KomaType::Aki {
            return String::from(" ・");
        }

        String::from(if self.teban == Teban::Sente { " " } else { "v" })
            + &self.koma.to_string(self.promotion)
    }

    pub fn to_kstring(&self) -> Option<String> {
        if self.teban == Teban::None || self.koma == KomaType::Aki {
            return None;
        }
        Some(self.koma.to_string(self.promotion))
    }

    pub fn is_blank(&self) -> bool {
        self.koma == KomaType::Aki
    }

    pub fn is_sente(&self) -> bool {
        self.teban == Teban::Sente
    }
    pub fn is_gote(&self) -> bool {
        self.teban == Teban::Gote
    }
}

#[test]
fn komatest() {
    let prm = Promotion::None;
    let k = Koma::from(' ', prm);
    assert_eq!(k.koma, KomaType::Aki);
    assert!(!k.promotion.is_promoted());
    assert!(!k.is_sente());
    assert!(!k.is_gote());
    assert!(k.is_blank());
    assert_eq!(k.to_string(), " ・");
    assert!(k.to_kstring().is_none());
    let k = Koma::from('P', prm);
    assert_eq!(k.koma, KomaType::Fu);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 歩");
    assert_eq!(k.to_kstring().unwrap(), "歩");
    let k = Koma::from('L', prm);
    assert_eq!(k.koma, KomaType::Kyosha);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 香");
    assert_eq!(k.to_kstring().unwrap(), "香");
    let k = Koma::from('N', prm);
    assert_eq!(k.koma, KomaType::Keima);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 桂");
    assert_eq!(k.to_kstring().unwrap(), "桂");
    let k = Koma::from('S', prm);
    assert_eq!(k.koma, KomaType::Gin);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 銀");
    assert_eq!(k.to_kstring().unwrap(), "銀");
    let k = Koma::from('G', prm);
    assert_eq!(k.koma, KomaType::Kin);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 金");
    assert_eq!(k.to_kstring().unwrap(), "金");
    let k = Koma::from('B', prm);
    assert_eq!(k.koma, KomaType::Kaku);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 角");
    assert_eq!(k.to_kstring().unwrap(), "角");
    let k = Koma::from('R', prm);
    assert_eq!(k.koma, KomaType::Hisha);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 飛");
    assert_eq!(k.to_kstring().unwrap(), "飛");
    let k = Koma::from('K', prm);
    assert_eq!(k.koma, KomaType::Gyoku);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 玉");
    assert_eq!(k.to_kstring().unwrap(), "玉");

    let k = Koma::from('_', prm);
    assert_eq!(k.koma, KomaType::Aki);
    assert!(!k.promotion.is_promoted());
    assert!(!k.is_sente());
    assert!(!k.is_gote());
    assert!(k.is_blank());
    assert_eq!(k.to_string(), " ・");
    assert!(k.to_kstring().is_none());
    let k = Koma::from('p', prm);
    assert_eq!(k.koma, KomaType::Fu);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v歩");
    assert_eq!(k.to_kstring().unwrap(), "歩");
    let k = Koma::from('l', prm);
    assert_eq!(k.koma, KomaType::Kyosha);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v香");
    assert_eq!(k.to_kstring().unwrap(), "香");
    let k = Koma::from('n', prm);
    assert_eq!(k.koma, KomaType::Keima);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v桂");
    assert_eq!(k.to_kstring().unwrap(), "桂");
    let k = Koma::from('s', prm);
    assert_eq!(k.koma, KomaType::Gin);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v銀");
    assert_eq!(k.to_kstring().unwrap(), "銀");
    let k = Koma::from('g', prm);
    assert_eq!(k.koma, KomaType::Kin);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v金");
    assert_eq!(k.to_kstring().unwrap(), "金");
    let k = Koma::from('b', prm);
    assert_eq!(k.koma, KomaType::Kaku);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v角");
    assert_eq!(k.to_kstring().unwrap(), "角");
    let k = Koma::from('r', prm);
    assert_eq!(k.koma, KomaType::Hisha);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v飛");
    assert_eq!(k.to_kstring().unwrap(), "飛");
    let k = Koma::from('k', prm);
    assert_eq!(k.koma, KomaType::Gyoku);
    assert!(!k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v玉");
    assert_eq!(k.to_kstring().unwrap(), "玉");

    let prm = Promotion::Promoted;
    let k = Koma::from(' ', prm);
    assert_eq!(k.koma, KomaType::Aki);
    assert!(k.promotion.is_promoted());
    assert!(!k.is_sente());
    assert!(!k.is_gote());
    assert!(k.is_blank());
    assert_eq!(k.to_string(), " ・");
    assert!(k.to_kstring().is_none());
    let k = Koma::from('P', prm);
    assert_eq!(k.koma, KomaType::Fu);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " と");
    assert_eq!(k.to_kstring().unwrap(), "と");
    let k = Koma::from('L', prm);
    assert_eq!(k.koma, KomaType::Kyosha);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 杏");
    assert_eq!(k.to_kstring().unwrap(), "杏");
    let k = Koma::from('N', prm);
    assert_eq!(k.koma, KomaType::Keima);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 圭");
    assert_eq!(k.to_kstring().unwrap(), "圭");
    let k = Koma::from('S', prm);
    assert_eq!(k.koma, KomaType::Gin);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 全");
    assert_eq!(k.to_kstring().unwrap(), "全");
    let k = Koma::from('G', prm);
    assert_eq!(k.koma, KomaType::Kin);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 金");
    assert_eq!(k.to_kstring().unwrap(), "金");
    let k = Koma::from('B', prm);
    assert_eq!(k.koma, KomaType::Kaku);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 馬");
    assert_eq!(k.to_kstring().unwrap(), "馬");
    let k = Koma::from('R', prm);
    assert_eq!(k.koma, KomaType::Hisha);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 龍");
    assert_eq!(k.to_kstring().unwrap(), "龍");
    let k = Koma::from('K', prm);
    assert_eq!(k.koma, KomaType::Gyoku);
    assert!(k.promotion.is_promoted());
    assert!(k.is_sente());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), " 玉");
    assert_eq!(k.to_kstring().unwrap(), "玉");

    let k = Koma::from('_', prm);
    assert_eq!(k.koma, KomaType::Aki);
    assert!(k.promotion.is_promoted());
    assert!(!k.is_sente());
    assert!(!k.is_gote());
    assert!(k.is_blank());
    assert_eq!(k.to_string(), " ・");
    assert!(k.to_kstring().is_none());
    let k = Koma::from('p', prm);
    assert_eq!(k.koma, KomaType::Fu);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "vと");
    assert_eq!(k.to_kstring().unwrap(), "と");
    let k = Koma::from('l', prm);
    assert_eq!(k.koma, KomaType::Kyosha);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v杏");
    assert_eq!(k.to_kstring().unwrap(), "杏");
    let k = Koma::from('n', prm);
    assert_eq!(k.koma, KomaType::Keima);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v圭");
    assert_eq!(k.to_kstring().unwrap(), "圭");
    let k = Koma::from('s', prm);
    assert_eq!(k.koma, KomaType::Gin);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v全");
    assert_eq!(k.to_kstring().unwrap(), "全");
    let k = Koma::from('g', prm);
    assert_eq!(k.koma, KomaType::Kin);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v金");
    assert_eq!(k.to_kstring().unwrap(), "金");
    let k = Koma::from('b', prm);
    assert_eq!(k.koma, KomaType::Kaku);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v馬");
    assert_eq!(k.to_kstring().unwrap(), "馬");
    let k = Koma::from('r', prm);
    assert_eq!(k.koma, KomaType::Hisha);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v龍");
    assert_eq!(k.to_kstring().unwrap(), "龍");
    let k = Koma::from('k', prm);
    assert_eq!(k.koma, KomaType::Gyoku);
    assert!(k.promotion.is_promoted());
    assert!(k.is_gote());
    assert!(!k.is_blank());
    assert_eq!(k.to_string(), "v玉");
    assert_eq!(k.to_kstring().unwrap(), "玉");
}

pub struct Tegoma {
    koma: KomaType,
    num: usize,
}

impl Tegoma {
    pub fn new(p: char, n: usize) -> Tegoma {
        Tegoma {
            koma: KomaType::from(p),
            num: n,
        }
    }
    pub fn to_kanji(&self) -> Result<String, String> {
        let kanji = self.koma.to_string(Promotion::None);
        let kanjinum = [
            "", "", /*"一"*/
            "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二", "十三", "十四",
            "十五", "十六", "十七", "十八",
        ];
        if self.num > 18 {
            return Err(kanji + &String::from("??"));
        }
        if self.num == 0 {
            return Ok(String::new());
        }
        Ok(kanji + &kanjinum[self.num])
    }
}

#[test]
fn testegoma() {
    let tg = Tegoma::new('P', 19);
    assert!(tg.to_kanji().is_err());
    assert_eq!(tg.to_kanji().err().unwrap(), "歩??");
    let tg = Tegoma::new('P', 0);
    assert!(tg.to_kanji().is_ok());
    assert_eq!(tg.to_kanji().unwrap(), "");
    let kanjinum = [
        /*"",*/ "", /*"一"*/
        "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二", "十三", "十四",
        "十五", "十六", "十七", "十八",
    ];
    for (i, x) in kanjinum.iter().enumerate() {
        let tg = Tegoma::new('P', i + 1);
        assert!(tg.to_kanji().is_ok());
        assert_eq!(tg.to_kanji().unwrap(), format!("歩{}", x));
    }
}

fn extractdan(txt: &str) -> Result<Vec<Koma>, String> {
    let mut res = Vec::<Koma>::new();
    let masu = txt.chars();
    let mut promote = Promotion::None;
    let rekoma = Regex::new("[PLNSGBRK]").unwrap();
    for ch in masu {
        match ch {
            '1'..='9' => {
                res.append(&mut vec![
                    Koma::from('?', Promotion::None);
                    ch.to_digit(10).unwrap() as usize
                ]);
            }
            ch if rekoma.is_match(&ch.to_ascii_uppercase().to_string()) => {
                res.push(Koma::from(ch, promote));
                promote = Promotion::None;
            }
            '+' => promote = Promotion::Promoted,
            _ => return Err(format!("{} is not allowed to use!!", ch)),
        }
    }
    Ok(res)
}

impl Sfen {
    pub fn new(text: &str) -> Sfen {
        let e: Vec<&str> = text.split(" ").collect();
        if e.len() < 4 {
            return Sfen {
                ban: String::new(),
                teban: String::new(),
                tegoma: String::new(),
                nteme: -2,
            };
        }
        Sfen {
            ban: e[0].to_string(),
            teban: e[1].to_string(),
            tegoma: e[2].to_string(),
            nteme: e[3].parse().unwrap_or(-1),
        }
    }
    fn tebanexp(&self) -> Result<String, String> {
        if self.teban == "b" {
            return Ok(String::from("先手の番です。"));
        }
        if self.teban == "w" {
            return Ok(String::from("後手の番です。"));
        }
        if self.teban == "fb" {
            return Ok(String::from("先手の勝ちです。"));
        }
        if self.teban == "fw" {
            return Ok(String::from("後手の勝ちです。"));
        }
        Err(format!("{} is invalid teban expression.", self.teban))
    }
    pub fn extractban(&self) -> Result<Vec<Vec<Koma>>, String> {
        let mut masus: Vec<Vec<Koma>> = Vec::new();
        let vdan: Vec<&str> = self.ban.split("/").collect();
        for e in vdan.iter() {
            match extractdan(e) {
                Ok(ret) => masus.push(ret),
                Err(msg) => return Err(msg),
            }
        }
        return Ok(masus);
    }
    fn extracttegoma(&self) -> Result<(Vec<Tegoma>, Vec<Tegoma>), String> {
        let resente = Regex::new("[PLNSGBRK]").unwrap();
        let regote = Regex::new("[plnsgbrk]").unwrap();
        let mut sentegoma = Vec::new();
        let mut gotegoma = Vec::new();
        let mut num = 0;
        for ch in self.tegoma.chars() {
            match ch {
                '1'..='9' => num = num * 10 + ch.to_digit(10).unwrap(),
                ch if resente.is_match(&ch.to_string()) => {
                    sentegoma.push(Tegoma::new(ch.to_ascii_lowercase(), num as usize));
                    num = 0;
                }
                ch if regote.is_match(&ch.to_string()) => {
                    gotegoma.push(Tegoma::new(ch, num as usize));
                    // gotegoma = gotegoma + &p2fu(ch, Promote::None) + &kanjinum(num as usize).unwrap();
                    num = 0;
                }
                '-' => break,
                _ => return Err(format!("{} is not allowed to use!!", ch)),
            }
        }
        Ok((sentegoma, gotegoma))
    }

    pub fn dump(&self, sn: &str, gn: &str, title: &str, lm: LastMove) -> String {
        let border = "+---------------------------+\n";
        let dannum = "一二三四五六七八九";
        let mut res = format!("  ９ ８ ７ ６ ５ ４ ３ ２ １\n{}", border);
        let vdan: Vec<&str> = self.ban.split("/").collect();
        for (i, e) in vdan.iter().enumerate() {
            match extractdan(e) {
                Ok(ret) => {
                    res = format!(
                        "{}|{}|{}\n",
                        res,
                        ret.iter()
                            .map(|koma| koma.to_string())
                            .collect::<Vec<String>>()
                            .join(""),
                        dannum.chars().nth(i).unwrap()
                    );
                }
                Err(msg) => return format!("error in [{}]:{}", e, msg),
            }
            // match dumpextractdan(e) {
            //     Ok(ret) => res = res + &ret + &dannum.chars().nth(i).unwrap().to_string() + "\n",
            // }
        }
        match self.extracttegoma() {
            Ok((sentegoma, gotegoma)) => {
                let tgmsen = if sentegoma.is_empty() {
                    String::from("なし")
                } else {
                    sentegoma
                        .iter()
                        .map(|t| t.to_kanji().unwrap())
                        .collect::<Vec<String>>()
                        .join("")
                };
                let tgmgo = if gotegoma.is_empty() {
                    String::from("なし")
                } else {
                    gotegoma
                        .iter()
                        .map(|t| t.to_kanji().unwrap())
                        .collect::<Vec<String>>()
                        .join("")
                };
                res = format!(
                    "後手：{}\n後手の持駒：{}\n{}{}先手の持駒：{}\n先手：{}\n",
                    gn, tgmgo, res, border, tgmsen, sn
                )
            }
            Err(msg) => return format!("error in [{}]:{}", self.tegoma, msg),
        }
        if lm.is_ok() {
            match lm.to_string() {
                Ok(msg) => {
                    return res + &format!("手数＝{}　{}\n* {}", self.nteme, msg, title);
                }
                Err(msg) => msg,
            }
        } else {
            match self.tebanexp() {
                Ok(msg) => {
                    return res + &format!("手数＝{}　{}\n* {}", self.nteme, msg, title);
                }
                Err(msg) => msg,
            }
        }
    }

    fn build_lastmove(&self, suji: usize, dan: usize) -> Tag {
        let mut glm = Tag::new("g");
        glm.newattrib("id", "lastmove");
        glm.newattrib(
            "transform",
            &format!("translate({}, {})", 180 - suji * 20, dan * 20 - 20),
        );
        let mut rect = Tag::new("rect");
        let atr = [
            ("x", "0"),
            ("y", "0"),
            ("width", "20"),
            ("height", "20"),
            ("fill", "#FF4"),
        ];
        for (nm, val) in atr {
            rect.newattrib(nm, val);
        }
        glm.addchild(rect);
        glm
    }

    fn buildboard(&self, lastmove: Option<(usize, usize)>) -> Result<Tag, String> {
        match self.extractban() {
            Ok(ban) => {
                let mut gban = Tag::new("g");
                gban.newattrib("id", "board");
                gban.newattrib("transform", "translate(35,65)");

                if lastmove.is_some() {
                    let lm = lastmove.unwrap();
                    let lm = self.build_lastmove(lm.0, lm.1);
                    gban.addchild(lm);
                }

                gban.addchild(banborder());

                for (i, dan) in ban.iter().enumerate() {
                    let mut gdan = Tag::new("g");
                    gdan.addattrib(Attrib::new("id", format!("dan{}", i + 1)));
                    gdan.addattrib(Attrib::new("transform", format!("translate(0,{})", i * 20)));
                    for (j, k) in dan.iter().enumerate() {
                        match komatag(k, j as i32, 0) {
                            Some(tag) => gdan.addchild(tag),
                            None => {}
                        }
                    }
                    if gdan.has_child() {
                        gban.addchild(gdan)
                    }
                }
                Ok(gban)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn buildtegoma(&self) -> Result<(Tag, Tag), String> {
        match self.extracttegoma() {
            Ok((sentegoma, gotegoma)) => {
                let mut st = Tag::new("g");
                st.newattrib("id", "stegoma");
                st.newattrib("transform", "translate(239,75)");
                let mut tt = Tag::new("g");
                tt.newattrib("transform", "translate(0,-7)");
                let mut poly = Tag::new("polygon");
                poly.newattrib("points", "0,-5 4,-4 5,5 -5,5 -4,-4");
                poly.newattrib("fill", "black");
                poly.newattrib("stroke", "black");
                tt.addchild(poly);
                st.addchild(tt);
                let mut y = 20;
                for tgm in sentegoma {
                    let mut tag = Tag::new("text");
                    let atr = [("x", "0"), ("font-size", "16px"), ("text-anchor", "middle")];
                    for (nm, val) in atr {
                        tag.newattrib(nm, val);
                    }
                    tag.addattrib(Attrib::new("y", format!("{}", y)));
                    tag.value = tgm.koma.to_string(Promotion::None);
                    st.addchild(tag);

                    if tgm.num > 1 {
                        let mut tag = Tag::new("text");
                        let atr = [("x", "8"), ("font-size", "12px"), ("text-anchor", "left")];
                        for (nm, val) in atr {
                            tag.newattrib(nm, val);
                        }
                        tag.addattrib(Attrib::new("y", format!("{}", y)));
                        tag.value = format!("{}", tgm.num);
                        st.addchild(tag);
                    }
                    y += 20;
                }

                let mut gt = Tag::new("g");
                gt.newattrib("id", "gtegoma");
                gt.newattrib("transform", "translate(9,75)");
                let mut tt = Tag::new("g");
                tt.newattrib("transform", "translate(0,-7)");
                let mut poly = Tag::new("polygon");
                poly.newattrib("points", "0,-5 4,-4 5,5 -5,5 -4,-4");
                poly.newattrib("fill", "none");
                poly.newattrib("stroke", "black");
                tt.addchild(poly);
                gt.addchild(tt);
                let mut y = 20;
                for tgm in gotegoma {
                    let mut tag = Tag::new("text");
                    let atr = [("x", "0"), ("font-size", "16px"), ("text-anchor", "middle")];
                    for (nm, val) in atr {
                        tag.newattrib(nm, val);
                    }
                    tag.addattrib(Attrib::new("y", format!("{}", y)));
                    tag.value = tgm.koma.to_string(Promotion::None);
                    gt.addchild(tag);

                    if tgm.num > 1 {
                        let mut tag = Tag::new("text");
                        let atr = [("x", "8"), ("font-size", "12px"), ("text-anchor", "left")];
                        for (nm, val) in atr {
                            tag.newattrib(nm, val);
                        }
                        tag.addattrib(Attrib::new("y", format!("{}", y)));
                        tag.value = format!("{}", tgm.num);
                        gt.addchild(tag);
                    }
                    y += 20;
                }
                Ok((st, gt))
            }
            Err(msg) => Err(msg),
        }
    }

    fn build_sentename(&self, name: String) -> Tag {
        let mut gs = Tag::new("g");
        gs.newattrib("id", "sname");
        gs.newattrib("transform", "translate(5,250)");
        let mut gp = Tag::new("g");
        gp.newattrib("transform", "translate(230,0)");
        let mut pl = Tag::new("polygon");
        let atr = [
            ("points", "10,0 18,2 20,20 0,20 2,2"),
            ("fill", "black"),
            ("stroke", "black"),
            ("stroke-width", "1"),
        ];
        for (nm, val) in atr {
            pl.newattrib(nm, val);
        }
        gp.addchild(pl);
        gs.addchild(gp);

        if name.is_empty() {
            return gs;
        }

        let mut txt = Tag::new("text");
        let atr = [
            ("x", "0"),
            ("y", "15"),
            ("font-size", "16px"),
            ("text-anchor", "left"),
            ("width", "230px"),
            ("text-overflow", "ellipsis"),
        ];
        for (nm, val) in atr {
            txt.newattrib(nm, val);
        }
        txt.value = name;
        gs.addchild(txt);
        gs
    }

    fn build_gotename(&self, name: String) -> Tag {
        let mut gg = Tag::new("g");
        gg.newattrib("id", "gname");
        gg.newattrib("transform", "translate(5,25)");
        let mut pl = Tag::new("polygon");
        let atr = [
            ("points", "10,0 18,2 20,20 0,20 2,2"),
            ("fill", "none"),
            ("stroke", "black"),
            ("stroke-width", "1"),
        ];
        for (nm, val) in atr {
            pl.newattrib(nm, val);
        }
        gg.addchild(pl);

        if name.is_empty() {
            return gg;
        }

        let mut txt = Tag::new("text");
        let atr = [
            ("x", "25"),
            ("y", "15"),
            ("font-size", "16px"),
            ("text-anchor", "left"),
            ("width", "230px"),
            ("text-overflow", "ellipsis"),
        ];
        for (nm, val) in atr {
            txt.newattrib(nm, val);
        }
        txt.value = name;
        gg.addchild(txt);
        gg
    }

    fn build_title(&self, title: String) -> Option<Tag> {
        if title.is_empty() {
            return None;
        }
        let mut gt = Tag::new("g");
        gt.newattrib("id", "title");
        // gg.newattrib("transform", "translate(5,25)");
        let mut txt = Tag::new("text");
        let atr = [
            ("x", "130"),
            ("y", "15"),
            ("font-size", "16px"),
            ("text-anchor", "middle"),
            ("width", "260px"),
            ("text-overflow", "ellipsis"),
        ];
        for (nm, val) in atr {
            txt.newattrib(nm, val);
        }
        txt.value = title;
        gt.addchild(txt);
        Some(gt)
    }

    fn build_teban(&self) -> Option<Tag> {
        let mut gt = Tag::new("g");
        gt.newattrib("id", "teban");

        let rectatb = [
            ("x", "0"),
            ("y", "0"),
            ("width", "30"),
            ("height", "30"),
            ("fill", "#F3C"),
            ("stroke", "none"),
        ];
        let polyatb = [
            ("points", "15,0 22.5,5 30,0 30,30 0,30 0,0 7.5,5"),
            ("fill", "#F3C"),
            ("stroke", "none"),
        ];
        if self.teban == "w" {
            gt.newattrib("transform", "translate(0,20)");

            let mut mark = Tag::new("rect");
            for (nm, val) in rectatb {
                mark.newattrib(nm, val);
            }
            gt.addchild(mark);
        } else if self.teban == "b" {
            gt.newattrib("transform", "translate(230,245)");
            let mut mark = Tag::new("rect");
            for (nm, val) in rectatb {
                mark.newattrib(nm, val);
            }
            gt.addchild(mark);
        } else if self.teban == "fw" {
            gt.newattrib("transform", "translate(30,20)");
            let mut mark = Tag::new("rect");
            for (nm, val) in polyatb {
                mark.newattrib(nm, val);
            }
            gt.addchild(mark);
        } else if self.teban == "fb" {
            gt.newattrib("transform", "translate(0,245)");
            let mut mark = Tag::new("rect");
            for (nm, val) in polyatb {
                mark.newattrib(nm, val);
            }
            gt.addchild(mark);
        } else {
            return None;
        }
        Some(gt)
    }

    pub fn to_svg(
        &self,
        lastmove: Option<(usize, usize)>,
        sname: String,
        gname: String,
        title: String,
    ) -> Result<SVG, String> {
        let mut top = Tag::new("g");
        let ttl = self.build_title(title);
        if ttl.is_some() {
            top.addchild(ttl.unwrap());
        }
        let tbn = self.build_teban();
        if tbn.is_some() {
            top.addchild(tbn.unwrap());
        }
        top.addchild(self.build_sentename(sname));
        top.addchild(self.build_gotename(gname));
        match self.buildboard(lastmove) {
            Ok(tag) => {
                top.addchild(tag);
            }
            Err(msg) => return Err(msg),
        }
        match self.buildtegoma() {
            Ok((st, gt)) => {
                top.addchild(st);
                top.addchild(gt);
            }
            Err(msg) => return Err(msg),
        }
        let mut svg = SVG::new();
        svg.tag.addchild(top);
        Ok(svg)
    }
}

fn komatag(k: &Koma, x: i32, y: i32) -> Option<Tag> {
    if k.is_blank() {
        return None;
    }

    let mut kt = Tag::new("g");
    kt.addattrib(Attrib::new(
        "transform",
        format!("translate({},{})", x * 20, y * 20),
    ));

    let mut tag = Tag::new("text");
    tag.newattrib("font-size", "18px");
    tag.newattrib("text-anchor", "middle");
    if k.is_sente() {
        tag.addattrib(Attrib::new("x", format!("{}", 10)));
        tag.addattrib(Attrib::new("y", format!("{}", 17)));
        tag.value = k.to_kstring().unwrap();
        kt.addchild(tag);

        return Some(kt);
    }

    // gote
    let mut gote = Tag::new("g");
    gote.newattrib("transform", "translate(10,10) rotate(180)");
    tag.newattrib("x", "0");
    tag.newattrib("y", "6");
    tag.value = k.to_kstring().unwrap();
    gote.addchild(tag);
    kt.addchild(gote);
    Some(kt)
}

fn banborder() -> Tag {
    let mut ret = Tag::new("g");
    ret.newattrib("id", "ban");
    let mut rect = Tag::new("rect");

    // <rect x='0' y='0' width='180' height='180' fill='none' stroke='black' stroke-width='2'/>
    let atr = [
        ("x", "0"),
        ("y", "0"),
        ("width", "180"),
        ("height", "180"),
        ("fill", "none"),
        ("stroke", "black"),
        ("stroke-width", "2"),
    ];
    for (nm, val) in atr {
        rect.newattrib(nm, val);
    }
    ret.addchild(rect);

    // horizontal lines
    for i in 0..4 {
        let mut rect = Tag::new("rect");
        let atr = [
            ("x", "0"),
            ("width", "180"),
            ("height", "20"),
            ("fill", "none"),
            ("stroke", "black"),
            ("stroke-width", "1"),
        ];
        for (nm, val) in atr {
            rect.newattrib(nm, val);
        }
        rect.addattrib(Attrib::new("y", format!("{}", i * 40 + 20)));
        ret.addchild(rect);
    }

    // vertical lines
    for i in 0..4 {
        let mut rect = Tag::new("rect");
        let atr = [
            ("y", "0"),
            ("width", "20"),
            ("height", "180"),
            ("fill", "none"),
            ("stroke", "black"),
            ("stroke-width", "1"),
        ];
        for (nm, val) in atr {
            rect.newattrib(nm, val);
        }
        rect.addattrib(Attrib::new("x", format!("{}", i * 40 + 20)));
        ret.addchild(rect);
    }
    // suji numbers
    let mut suji = Tag::new("g");
    suji.newattrib("transform", "translate(0,-5)");
    for (i, ch) in "９８７６５４３２１".chars().enumerate() {
        let atrs = [("y", "0"), ("font-size", "10px"), ("text-anchor", "middle")];
        let mut txt = Tag::new("text");
        for (nm, val) in atrs {
            txt.newattrib(nm, val);
        }
        txt.addattrib(Attrib::new("x", format!("{}", i * 20 + 10)));
        txt.value = ch.to_string();
        suji.addchild(txt);
    }
    ret.addchild(suji);

    // dan numbers
    let mut dan = Tag::new("g");
    dan.newattrib("transform", "translate(183,0)");
    for (i, ch) in "一二三四五六七八九".chars().enumerate() {
        let atrs = [("x", "0"), ("font-size", "10px"), ("text-anchor", "left")];
        let mut txt = Tag::new("text");
        for (nm, val) in atrs {
            txt.newattrib(nm, val);
        }
        txt.addattrib(Attrib::new("y", format!("{}", i * 20 + 13)));
        txt.value = ch.to_string();
        dan.addchild(txt);
    }
    ret.addchild(dan);
    ret
}

#[derive(Debug)]
pub struct LastMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub koma: sfen::Koma,
    pub promote: sfen::Promotion,
    pub dir: String,
}

impl LastMove {
    pub fn new() -> LastMove {
        LastMove {
            from: (0, 0),
            to: (0, 0),
            koma: Koma::from(' ', Promotion::None),
            promote: sfen::Promotion::None,
            dir: String::new(),
        }
    }
    // 7776FUPNLRAHCY
    pub fn read(txt: &str) -> Result<LastMove, String> {
        let mut lm = LastMove {
            from: (0, 0),
            to: (0, 0),
            koma: sfen::Koma::from(' ', sfen::Promotion::None),
            promote: sfen::Promotion::None,
            dir: String::new(),
        };
        let re =
            regex::Regex::new("(\\d\\d)(\\d\\d)([a-zA-Z][a-zA-Z])([PN]?)([LRAUHSCY]*)").unwrap();
        match re.captures(txt) {
            Some(cap) => {
                let frm: usize = cap.get(1).map_or("", |s| s.as_str()).parse().unwrap();
                lm.from = (frm / 10, frm % 10);
                let to: usize = cap.get(2).map_or("", |s| s.as_str()).parse().unwrap();
                lm.to = (to / 10, to % 10);
                match sfen::Koma::fromcsa(cap.get(3).map_or("", |s| s.as_str())) {
                    Some(k) => lm.koma = k,
                    None => {
                        return Err(format!("\"{}\" is invalid lastmove about koma.", txt));
                    }
                }
                match cap.get(4).map_or("", |s| s.as_str()) {
                    "P" => {
                        lm.promote = Promotion::Promoted;
                    }
                    "N" => {
                        lm.promote = Promotion::NotPromoted;
                    }
                    _ => {}
                }
                lm.dir = cap.get(5).map_or("", |s| s.as_str()).to_string();

                Ok(lm)
            }
            None => Err(format!("\"{}\" is invalid lastmove.", txt)),
        }
    }
    pub fn is_ok(&self) -> bool {
        self.to.0 > 0 && self.to.1 > 0
    }
    pub fn topos(&self) -> Option<(usize, usize)> {
        if self.is_ok() {
            Some(self.to)
        } else {
            None
        }
    }
    pub fn is_from_komadai(&self) -> bool {
        self.from.0 == 0 && self.from.1 == 0
    }
    pub fn to_string(&self) -> Result<String, String> {
        if !self.is_ok() {
            return Ok(String::new());
        }
        const INVALID_MSG: &str = "invalid last move.";
        let mut ret = String::new();
        ret += ["１", "２", "３", "４", "５", "６", "７", "８", "９"][self.to.0 - 1];
        ret += ["一", "二", "三", "四", "五", "六", "七", "八", "九"][self.to.1 - 1];
        match self.koma.to_kstring() {
            Some(k) => {
                ret += &k;
            }
            None => return Err(String::from(INVALID_MSG)),
        }
        if self.is_from_komadai() {
            ret += "打";
            if !self.dir.is_empty() || self.promote.is_promoted() {
                return Err(String::from(INVALID_MSG));
            }
        } else {
            for e in self.dir.chars() {
                ret += match e {
                    'R' => "右",
                    'L' => "左",
                    'A' => "上",
                    'U' => "上",
                    'H' => "引",
                    'S' => "下",
                    'D' => "下",
                    'Y' => "寄",
                    'C' => "直",
                    _ => return Err(format!("{} is not supported in LastMove.", e)),
                };
                // println!("[{} in {}]", e, self.dir);
            }
            ret += &self.promote.to_string();
        }
        Ok(ret + "まで")
    }
}
