use std::char;
use std::cmp::{max, min};


fn main() {
    println!("Hello, world!");
    println!("Result: {}", karacybi("1685287499328328297814655639278583667919355849391453456921116729",
                            "7114192848577754587969744626558571536728983167954552999895348492"))
}


fn equalize_length(x: &str, y: &str) -> (Box<String>, Box<String>) {
    let diff = max(x.len(), y.len()) - min(x.len(), y.len());
    let filler = (0..diff).map(|_| '0');

    return if x.len() > y.len() {
        (Box::new(x.to_string()), 
         Box::new(filler.chain(y.chars()).collect::<String>()))
    }
    else {
        (Box::new(filler.chain(x.chars()).collect::<String>()),
         Box::new(y.to_string()))
    };
}

#[test]
fn test_qualize_lengths() {
    let (x, y) = equalize_length("24", "2");
    assert_eq!("24", x.as_ref());
    assert_eq!("02", y.as_ref());

    let (x, y) = equalize_length("2", "1234567890");
    assert_eq!("0000000002", x.as_ref());
    assert_eq!("1234567890", y.as_ref());
}


// precondition: length x & y should be equal
fn mod2length(x: &str, y:&str) -> (Box<String>, Box<String>) {
    let (mut new_x, mut new_y) = equalize_length(x, y);

    if (new_x.len() % 2) == 0 {
        return (new_x.clone(), new_y.clone());
    } 

    let diff = (new_x.len() >> 1 << 2) as u8 - new_x.len() as u8;
    for _ in 0..diff {
        new_x.insert(0, '0');
        new_y.insert(0, '0');
    }

    return (new_x, new_y);
}


#[test]
fn test_mod2length() {
    {
        let (x, y) = mod2length("070", "131");
        assert_eq!(x.as_ref(), "0070");
        assert_eq!(y.as_ref(), "0131");
    }
    {
        let (x, y) = mod2length("0070", "1131");
        assert_eq!(x.as_ref(), "0070");
        assert_eq!(y.as_ref(), "1131")
    }
}


fn mul_10n(x: &str, n: usize) -> Box<String> {
    if x == "0" {
        return Box::new(String::from("0"));
    }

    let mut new_x = Box::new(x.to_string());
    for _ in 0..n {
        new_x.push('0');
    }
    return new_x;
}

// x - y
// precondition: x > y
fn sub(x: &str, y: &str) -> Box<String> {
    // println!("{} - {}", x, y);
    let (first, second) = equalize_length(x, y);
    let mut carry: i32 = 0;
    let mut result = Box::new(String::new());
    
    for (a, b) in first.chars().rev().zip(second.chars().rev()) {
        let b_digit = b.to_digit(10).unwrap() as i32;
        let a_digit = a.to_digit(10).unwrap() as i32;

        let tmp = if (a_digit - carry) < b_digit {
            a_digit + 10 - carry - b_digit
        }
        else {
            a_digit - carry - b_digit
        };

        carry = if (a_digit - carry) < b_digit {1} else {0};

        result.insert(0, char::from_digit(tmp as u32, 10).unwrap());
    }

    return Box::new(result.trim_left_matches('0').to_string());
}

#[test]
fn test_sub() {
    assert_eq!(sub("4", "1").as_ref(), "3");
    assert_eq!(sub("24", "6").as_ref(), "18");
    assert_eq!(sub("24", "22").as_ref(), "2");
    assert_eq!(sub("1024", "36").as_ref(), "988");
}


// x + y
fn sum(x: &str, y: &str) -> Box<String> {
    // println!("{} + {}", x, y);
    let (bigger, smaller) = if x.len() > y.len() {
        equalize_length(x, y)
    }
    else {
        equalize_length(y, x)
    };
    let mut carry: u32 = 0;
    let mut result = Box::new(String::new());

    for (a, b) in bigger.chars().rev().zip(smaller.chars().rev()) {
        let tmp = b.to_digit(10).unwrap() + a.to_digit(10).unwrap() + carry;

        carry = if tmp / 10 > 0 {
             tmp / 10
        } else {
             0
        };

        result.insert(0, char::from_digit(tmp % 10, 10).unwrap());
    }

    // if we have carry after latest calculations,
    // then we should insert it at head
    if carry > 0 {
        result.insert(0, char::from_digit(carry as u32, 10).unwrap());
    }

    return result;
}

#[test]
fn test_sum() {
    assert_eq!(sum("9234567890", "11111").as_ref(), "9234579001");
    assert_eq!(sum("11111", "9234567890").as_ref(), "9234579001");

    assert_eq!(sum("49", "82").as_ref(), "131");
    assert_eq!(sum("44", "26").as_ref(), "70");
}


// x + y
fn karacybi(x: &str, y: &str) -> Box<String> {
    let n = max(x.len(), y.len());
    if n < 2 {
        let res = Box::new((
            i16::from_str_radix(x, 10).unwrap_or(0) * 
            i16::from_str_radix(y, 10).unwrap_or(0)
        ).to_string());
        return res;
    }
    
    let (a, b) = x.split_at(x.len() - n / 2);
    let (c, d) = y.split_at(y.len() - n / 2);

    let ac = karacybi(&a, &c);
    let bd = karacybi(&b, &d);

    // (a + b) * (c + d) - ac - bd
    let ad_bc = sub(&sub(&karacybi(&sum(&a, &b), &sum(&c, &d)), &ac), &bd);

    // println!("{} * {}", x, y);
    // println!("a = {}", a);
    // println!("b = {}", b);
    // println!("c = {}", c);
    // println!("d = {}", d);
    println!("a*d+b*c = {}", ad_bc);

    // 10i64.pow(n) * ac + 10i64.pow(n / 2) * ad_bc + bd;
    return sum(
        &sum(
            &mul_10n(&ac, (n / 2) * 2),
            &mul_10n(&ad_bc, n / 2)
        ), 
        &bd
    );
}


#[test]
fn test1() {
    // _test("3", "2", "6");
}

#[test]
fn test2() {
    // _test("33", "22", "726");
    // _test("49", "44", "2156");
}

#[test]
fn test3mul2() {
    _test("13", "7", "91");
    _test("131", "70", "9170");
}


#[test]
fn test8() {
    _test("49823261", "44269423", "2205647016448403");
    assert!(false);
}


#[test]
fn test8_1() {
    _test("54761293", "65394884", "3581108403425012");
}


#[test]
fn test16() {
    _test("9313685456934674", "7658898761837539", "71332574014261268360454523927286");
}

#[test]
fn test16_1(){
    _test("3957322621234423", "7748313756335578", "30662577304368647842211393201494");

}

#[test]
fn test_32() {
    _test("34215432964249374812219364786397", "94541964835273822784327848699719", "3234794260129733170788831535430575611379062580407060392628922443")
}

#[test]
fn test_32_1() {
    _test("71611955325935479159397713213124", "93567726499788166681348352945366", "6700567850052179472481148730882040129649508491917840721086183384")
}

#[test]
fn test_64() {
    _test("8436939677358274975644341226884162349535836199962392872868456892", "3864264464372346883776335161325428226997417338413342945574123327", 
          "32602566183268675582196165592691544162522778809155901895617284287276672563976841699892789718741377833554298336397153444191119684")
}

#[test]
fn test_64_1() {
    _test("8711129198194917883527844183686122989894424943636426448417394566", "2924825637132661199799711722273977411715641477832758942277358764", 
          "25478534007255378799894857247961445544397925869179138904636157575535921570058983065006369481295619500406669960288667484926076424")
}



fn _test(x: &str, y: &str, expected: &str) {
    let actual = karacybi(x, y);
    assert_eq!(actual.as_ref(), expected)
}
