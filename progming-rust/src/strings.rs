#[cfg(test)]
mod strings {
    
    #[test]
    fn classifiying_chars() {
        /* ---------- IS NUMERIC --------------- */
        // Funciona para caracteres ASCII
        assert!('4'.is_numeric());

        // E tambem funciona para caracteres UNICODE (cuidado!)
        assert!('ᛮ'.is_numeric());
        assert!('⑧'.is_numeric());

        /* ---------- IS ALPHABETIC ------------ */
        // Funciona para caracteres ASCII
        assert!('q'.is_alphabetic());

        // E tambem funciona para caracteres UNICODE (cuidado!)
        assert!('七'.is_alphabetic());

        /* ---------IS ALPHANUMERIC ------------ */
        assert!('9'.is_alphanumeric());
        assert!('饂'.is_alphanumeric());
        assert!(!'*'.is_alphanumeric());

        /* IS_WHITESPACE */
        assert!(' '.is_whitespace());
        assert!('\n'.is_whitespace());
        assert!('\u{A0}'.is_whitespace());


        /* IS_CONTROL */
        assert!('\n'.is_control());
        assert!('\u{85}'.is_control());

        /* ASCII_ESPECIFIC */
        assert!('Q'.is_ascii_graphic());
        assert!('~'.is_ascii_graphic());
        assert!(!' '.is_ascii_graphic());

        assert!('z'.is_ascii_lowercase());
        assert!('Z'.is_ascii_uppercase());

        assert!(' '.is_ascii_whitespace());
        assert!('\n'.is_ascii_whitespace());
        assert!(!'\u{A0}'.is_ascii_whitespace()); // Esse eh whitespace em UNICODE

        /* Metodos ASCII tambem estao disponiveis em u8 */
        assert!(32u8.is_ascii_whitespace());
        assert!(b'9'.is_ascii_digit());
    }

    #[test]
    fn handling_digits() {
        let v = '9';
        assert_eq!(v.to_digit(10), Some(9));

        let v = 'B';
        assert_eq!(v.to_digit(16), Some(11));

        assert_eq!(char::from_digit(13, 16), Some('d'));
    }

    #[test]
    fn case_conversion() {
        // Em geral, to_uppercase / lowercase -> retorna um iterador que vai produzir os caracteres
        let mut upper = 's'.to_uppercase();
        assert_eq!(upper.next(), Some('S'));
        assert_eq!(upper.next(), None);

        // Excecoes no UNICODE
        let mut upper = 'ß'.to_uppercase();
        assert_eq!(upper.next(), Some('S'));
        assert_eq!(upper.next(), Some('S'));
        assert_eq!(upper.next(), None);

        let mut lower = 'İ'.to_lowercase();
        assert_eq!(lower.next(), Some('i'));
        assert_eq!(lower.next(), Some('\u{307}'));
        assert_eq!(lower.next(), None);
    }

    #[test]
    fn conversions_to_n_from_integers() {
        // Na conversão pra inteiro, dependendo da quantidade de bits, é feito o truncamento
        assert_eq!('B' as u32, 66);
        assert_eq!('饂' as u8, 66);
        assert_eq!('二' as i8, -116);

        // A conversão reversa produz uma Option
        assert_eq!(char::from(66), 'B');
        assert_eq!(char::from_u32(0x9942), Some('饂'));
        assert_eq!(char::from_u32(0xd800), None);
    }

    #[test]
    fn string_inspection() {
        let full = "bookkeeping";
        assert_eq!(&full[..4], "book");
        assert_eq!(&full[5..], "eeping");
        assert_eq!(&full[2..4], "ok");
        assert_eq!(full[..].len(), 11);
        assert_eq!(full[5..].contains("boo"), false);
        let (begin, end) = full.split_at(4);
        assert_eq!(begin, "book");
        assert_eq!(end, "keeping");
    }

    #[test]
    fn string_iterator() {
        let spacey = "man hat tan";
        let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(spaceless, "manhattan")
    }

    #[test]
    fn inserting_text_into_strings() {
        {
            // Extend
            let mut text = "con".to_string();
            text.extend("tri but ion".split_whitespace());
            assert_eq!(text, "contribution")
        }

        {
            // Insert a char
            let mut text = "Hello  welcome to our hotel.".to_string();
            text.insert(6, 'K');
            assert_eq!(text, "Hello K welcome to our hotel.");

            // Insert a slice
            let mut text = "Hello  welcome to our hotel.".to_string();
            text.insert_str(6, "Customer");
            assert_eq!(text, "Hello Customer welcome to our hotel.");

            // Obs: performance ruim
        }

        {
            // Usando fmt::Write
            // Obs: escrever em strings eh infalivel
            use std::fmt::Write;

            let mut letter = String::new();
            writeln!(letter, "Quem {} esses mafagafos?", "amafagafou").unwrap();
            writeln!(letter, "Este provavelmente sera um bom amafagafiador;").unwrap();
            assert_eq!(letter, "Quem amafagafou esses mafagafos?\n\
                                Este provavelmente sera um bom amafagafiador;\n");
        }

        {
            // Add & AddAssign
            let left = "partners".to_string();
            let mut right = "crime".to_string();
            assert_eq!(left + " in " + &right, "partners in crime");

            right += " doesn't pay";
            assert_eq!(right, "crime doesn't pay");
        }
    }

    #[test]
    fn removing_text_from_strings() {
        {
            // Clear
            let mut weather_report = "Today the sky is clean and there is no chance of rain".to_string();
            weather_report.clear();
            assert_eq!(weather_report, "")
        }

        {
            // Truncate
            let mut weather_report = "Today the sky is clean and there is no chance of rain".to_string();
            weather_report.truncate(22);
            assert_eq!(weather_report, "Today the sky is clean")
        }

        {
            // Pop from the last
            let mut alphabet = "ABCDE".to_string();
            assert_eq!(alphabet.pop(), Some('E'));
            assert_eq!(alphabet.pop(), Some('D'));
            assert_eq!(alphabet.pop(), Some('C'));
            assert_eq!(alphabet.pop(), Some('B'));
            assert_eq!(alphabet.pop(), Some('A'));
            assert_eq!(alphabet.pop(), None);
        }

        {
            // Remove (oposto ao insert)
            let mut text = "Hello K welcome to our hotel.".to_string();
            text.remove(6);
            assert_eq!(text, "Hello  welcome to our hotel.");

            // Obs: performance ruim
        }

        {
            // Drain -> 1º cria um "iterator" no slice extraido
            let mut city = "Itaquaquecetuba".to_string();
            assert_eq!(city.drain(5..11).collect::<String>(), "aquece");

            // 2º o restante da string eh empurrada pra frente na variavel antiga
            assert_eq!(city, "Itaqutuba");

            let mut president = "Churchill".to_string();
            president.drain(2..6);
            assert_eq!(president, "Chill");
        }

        {
            // Replace range
            let mut message = "Pela manhã te encontro".to_string();
            message.replace_range(5..11, "tarde");
            assert_eq!(message, "Pela tarde te encontro");
        }
    }

    #[test]
    fn searching_patterns() {
        let haystack = "Um belo dia, no meio da noite...";
        assert_eq!(haystack.find(','), Some(11)); // Ocorrência de caracteres
        assert_eq!(haystack.find("noite"), Some(24)); // Slices de string
        assert_eq!(haystack.find(char::is_whitespace), Some(2)); // Ou uma função -> bool

        // Em geral todas as funções de busca suportam esses 3 tipos de "padrões"
        let code = "\t      function noodle() { ";
        assert_eq!(code.trim_start_matches([' ', '\t'].as_ref()), "function noodle() { ")
    }

    #[test]
    fn search_and_replacing() {
        {
            // Searching 
            assert!("2017".starts_with(char::is_numeric));
            let quip = "We also know there are known unknowns";
            assert_eq!(quip.find("know"), Some(8));
            assert_eq!(quip.rfind("know"), Some(31));
            assert_eq!(quip.find("ya know"), None);
            assert_eq!(quip.rfind(char::is_uppercase), Some(0));
        }
        {
            // Replacing
            let frase = "A única coisa que temos que temer é o próprio medo.";
            assert_eq!(frase.replace("medo", "giro"), 
                "A única coisa que temos que temer é o próprio giro.");

            assert_eq!("`Borrow` and `BorrowMut`".replace(|x: char| !x.is_alphanumeric(), ""), 
                "BorrowandBorrowMut");
            
            assert_eq!("cabababababbage".replace("aba", "***"), "c***b***babbage");
        }
    }

    #[test]
    fn iterating_over() {
        {
            // Indices or chars or bytes
            assert_eq!("élan".char_indices().collect::<Vec<_>>(), 
                vec![(0, 'é'), (2, 'l'), (3, 'a'), (4, 'n')]);
        }

        {
            // Splices
            let expression = "bête noir d'Arrrgh";
            let mut parts = expression.split(' '); 
            assert_eq!(parts.next(), Some("bête"));
            assert_eq!(parts.next(), Some("noir"));
            //assert_eq!(parts.next(), Some(""));
            assert_eq!(parts.next(), Some("d'Arrrgh"));
            //assert_eq!(parts.next(), Some(""));
            assert_eq!(parts.next(), None);
        }

        {
            // Split different than whitespace
            assert_eq!("jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(), 
                vec!["jimb", "1000", "Jim Blandy", ""]);

            // 
            assert_eq!("127.0.0.1   localhost\n\
                        127.0.0.1   www.reddit.com\n"
                        .split_terminator('\n').collect::<Vec<_>>(), 
                        vec!["127.0.0.1   localhost", "127.0.0.1   www.reddit.com"]);
        }       
    }

    #[test]
    fn trimming() {
        assert_eq!("\t*.rs  ".trim(), "*.rs");
        assert_eq!("\t*.rs  ".trim_start(), "*.rs  ");
        assert_eq!("\t*.rs  ".trim_end(), "\t*.rs");
        assert_eq!("001990".trim_start_matches('0'), "1990");
    }

    #[test]
    fn parsing_from_strings() {
        use std::str::FromStr;
        use std::net::IpAddr;

        assert_eq!(usize::from_str("3628800"), Ok(3628800));
        assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
        assert_eq!(bool::from_str("true"), Ok(true));

        assert!(f64::from_str("1,2312").is_err());
        assert!(bool::from_str("TRUE").is_err());

        assert_eq!(char::from_str("é"), Ok('é'));
        assert!(char::from_str("abcdeg").is_err());

        let addr = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
        assert_eq!(addr, IpAddr::from([0xfe80, 0,0,0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]));

        // Alternative
        let addr = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap();
        assert_eq!(addr, IpAddr::from([0xfe80, 0,0,0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]));
    }

    #[test]
    fn converting_to_string() {
        use std::net::IpAddr;
        use std::str::FromStr;

        assert_eq!(format!("{}, wow", "doge"), "doge, wow");
        assert_eq!(format!("{}", true), "true");
        assert_eq!(format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0)/2.0), "(0.500, 0.866)");

        let addr = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
        let formatted_addr = format!("{}", addr);
        assert_eq!(formatted_addr, "fe80::3ea9:f4ff:fe34:7a50");

        let addresses = vec![addr, IpAddr::from_str("192.168.0.1").unwrap()];
        assert_eq!(format!("{:?}", addresses), "[fe80::3ea9:f4ff:fe34:7a50, 192.168.0.1]");
    }

    #[test]
    fn producing_utf8_text() {
        let good_utf8 = vec![0xe9, 0x8c, 0x86];
        assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

        let bad_utf8 = vec![0x9f, 0xf0, 0xa6, 0x80];
        let res = String::from_utf8(bad_utf8);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().into_bytes(), vec![0x9f, 0xf0, 0xa6, 0x80]);
    }

    #[test]
    fn formatting_text() {
        let template = "bookends";

        assert_eq!(format!("{}", template), "bookends");
        assert_eq!(format!("{:4}", template), "bookends"); // min = 4
        assert_eq!(format!("{:12}", template), "bookends    "); // min = 12
        assert_eq!(format!("{:.4}", template), "book"); // max = 4
        assert_eq!(format!("{:.12}", template), "bookends"); // max = 12
        assert_eq!(format!("{:12.20}", template), "bookends    "); // min = 12, max = 20
        assert_eq!(format!("{:4.20}", template), "bookends"); // min = 4, max = 20
        assert_eq!(format!("{:4.6}", template), "booken"); // min = 4, max = 6
        assert_eq!(format!("{:6.4}", template), "book  "); // min = 6, max = 4 (LoL)
        assert_eq!(format!("{:<12}", template), "bookends    "); // left allign
        assert_eq!(format!("{:^12}", template), "  bookends  "); // center allign
        assert_eq!(format!("{:>12}", template), "    bookends"); // right allign
        assert_eq!(format!("{:=^12}", template), "==bookends=="); // pad with center allign
        assert_eq!(format!("{:*>12.4}", template), "********book"); // pad with width limit
    }

    #[test]
    fn formatting_integers() {
        let template = 1234;

        assert_eq!(format!("{}", template), "1234");
        assert_eq!(format!("{:+}", template), "+1234");
        assert_eq!(format!("{:12}", template), "        1234");
        assert_eq!(format!("{:2}", template), "1234");
        assert_eq!(format!("{:+12}", template), "       +1234");
        assert_eq!(format!("{:012}", template), "000000001234");
        assert_eq!(format!("{:+012}", template), "+00000001234");
        assert_eq!(format!("{:<12}", template), "1234        ");
        assert_eq!(format!("{:^12}", template), "    1234    ");
        assert_eq!(format!("{:>12}", template), "        1234");
        assert_eq!(format!("{:<+12}", template), "+1234       ");
        assert_eq!(format!("{:^+12}", template), "   +1234    ");
        assert_eq!(format!("{:>+12}", template), "       +1234");
        assert_eq!(format!("{:=^12}", template), "====1234====");

        // other notations
        assert_eq!(format!("{:b}", template), "10011010010");
        assert_eq!(format!("{:12o}", template), "        2322");
        assert_eq!(format!("{:+12x}", template), "        +4d2");
        assert_eq!(format!("{:+12X}", template), "        +4D2");
        assert_eq!(format!("{:+#12x}", template), "      +0x4d2");
        assert_eq!(format!("{:+#012x}", template), "+0x0000004d2");
        assert_eq!(format!("{:+#06x}", template), "+0x4d2");
    }

    #[test]
    fn formatting_floats() {
        let template = 1234.5678;
        assert_eq!(format!("{}", template), "1234.5678");
        assert_eq!(format!("{:.2}", template), "1234.57"); // rounded up
        assert_eq!(format!("{:.6}", template), "1234.567800");
        assert_eq!(format!("{:12}", template), "   1234.5678");
        assert_eq!(format!("{:12.2}", template), "     1234.57");
        assert_eq!(format!("{:12.6}", template), " 1234.567800");
        assert_eq!(format!("{:e}", template), "1.2345678e3");
        assert_eq!(format!("{:.3e}", template), "1.235e3");
        assert_eq!(format!("{:12.3e}", template), "     1.235e3");
        assert_eq!(format!("{:12.3E}", template), "     1.235E3");
    }
}