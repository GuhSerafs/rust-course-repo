#[cfg(test)]
mod tests {
    #[test]
    fn fold() {
        // Alternativa ao reduce do python/kotlin
        let n = 42;
        assert_eq!((1..=n).fold(0, |sum, item| sum + item), 42 * 43 / 2) // n*(n+1)/2
    }

    #[test]
    fn iter_and_iter_mut() {
        let v = vec![4, 20, 12, 8, 6];
        let mut iterator = v.iter();
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&12));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn path_iterator() {
        use std::ffi::OsStr;
        use std::path::Path;

        let path = Path::new("C:/Users/information/Downloads/manjaro.iso");
        let mut path_iter = path.iter();
        assert_eq!(path_iter.next(), Some(OsStr::new("C:")));
        assert_eq!(path_iter.next(), Some(OsStr::new("Users")));
        assert_eq!(path_iter.next(), Some(OsStr::new("information")));
        assert_eq!(path_iter.next(), Some(OsStr::new("Downloads")));
        assert_eq!(path_iter.next(), Some(OsStr::new("manjaro.iso")));
        assert_eq!(path_iter.next(), None);
    }

    #[test]
    fn into_iterator() {
        use std::collections::BTreeSet;

        let mut goat_musics = BTreeSet::new();
        goat_musics.insert("Stairway to Heaven".to_string());
        goat_musics.insert("Dark Chest of Wonders".to_string());
        goat_musics.insert("Living Dead Beat".to_string());
        goat_musics.insert("The Chosen Path".to_string());
        goat_musics.insert("Twilight of the Thunder God".to_string());

        let mut goat_iter = goat_musics.into_iter();
        assert_eq!(goat_iter.next(), Some("Dark Chest of Wonders".to_string()));
        assert_eq!(goat_iter.next(), Some("Living Dead Beat".to_string()));
        assert_eq!(goat_iter.next(), Some("Stairway to Heaven".to_string()));
        assert_eq!(goat_iter.next(), Some("The Chosen Path".to_string()));
        assert_eq!(
            goat_iter.next(),
            Some("Twilight of the Thunder God".to_string())
        );
        assert_eq!(goat_iter.next(), None);
    }

    #[test]
    fn from_fn_iterator() {
        use std::iter::from_fn;

        let mut s = 0;
        let count_list: Vec<usize> = from_fn(|| {
            s += 1;
            Some(s)
        })
        .take(100)
        .collect();

        for i in 1..=100 {
            assert_eq!(count_list[i - 1], i);
        }
    }

    #[test]
    fn succesor_iterator() {
        use std::iter::successors;
        let count_list: Vec<usize> = successors(Some(0), |&c| Some(c + 1)).take(8).collect();
        for i in 0..8 {
            assert_eq!(count_list[i], i);
        }

        let fibonacci: Vec<(usize, usize)> = successors(Some((0, 1)), |&c| Some((c.1, c.0 + c.1)))
            .take(8)
            .collect();

        assert_eq!(fibonacci[0].1, 1);
        assert_eq!(fibonacci[1].1, 1);
        assert_eq!(fibonacci[2].1, 2);
        assert_eq!(fibonacci[3].1, 3);
        assert_eq!(fibonacci[4].1, 5);
        assert_eq!(fibonacci[5].1, 8);
        assert_eq!(fibonacci[6].1, 13);
        assert_eq!(fibonacci[7].1, 21);
    }

    #[test]
    fn map_and_filter_iterators() {
        let text = "    ponies   \n    giraffes\niguanas    \nsquid".to_string();
        let v: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|s| *s != "iguanas")
            .collect();
        assert_eq!(v, ["ponies", "giraffes", "squid"]);
    }

    #[test]
    fn filter_map_iterator() {
        use std::str::FromStr;

        let text = "1\nfrond .25 289\n3.1415 estuary\n";
        let numbers: Vec<f64> = text
            .split_whitespace()
            .filter_map(|w| f64::from_str(w).ok())
            .collect();
        assert_eq!(numbers, [1 as f64, 0.25, 289 as f64, 3.1415]);
    }

    #[test]
    fn flat_map_iterator() {
        use std::collections::HashMap;

        let mut major_cities = HashMap::new();
        major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
        major_cities.insert("USA", vec!["Portland", "Nashville"]);
        major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
        major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
        major_cities.insert("Netherlands", vec!["Amsterdan", "Utrecht"]);

        // Variante 1
        {
            let countries = ["Japan", "Brazil", "Kenya"];
            let cities: Vec<&str> = countries.iter()
                .flat_map(|country| major_cities[country].clone())
                .collect();

            assert_eq!(cities, ["Tokyo", "Kyoto", "São Paulo", "Brasília", "Nairobi", "Mombasa"]);
        }
        
        // Variante 2
        {
            let countries = ["USA", "Netherlands"];
            let cities: Vec<&&str> = countries.iter()
                .flat_map(|country| &major_cities[country])
                .collect();
            assert_eq!(cities, [&"Portland", &"Nashville", &"Amsterdan", &"Utrecht"]);    
        }
    }

    #[test]
    fn take_n_skip_while_iterator() {
        let msg = "To: jimb\r\n\
                        From: superego <editor@oreilly.com>\r\n\
                        \r\n\
                        How this is going?\r\n\
                        When you'll finish the job?\r\n";
        {
            // take while
            let nonempty_lines: Vec<&str> = msg.lines().take_while(|l| !l.is_empty()).collect();
            assert_eq!(nonempty_lines, ["To: jimb", "From: superego <editor@oreilly.com>"]);
        }

        {
            // skip while
            let body_text: Vec<&str> = msg.lines().skip_while(|l| !l.is_empty()).skip(1).collect();
            assert_eq!(body_text, ["How this is going?", "When you'll finish the job?"]);
        }
    }

    #[test]
    fn peek_iterator() {
        use std::iter::Peekable;

        fn parse_number<I>(tokens: &mut Peekable<I>) -> u32 
            where I: Iterator<Item=char>
        {
            let mut n = 0;
            loop {
                match tokens.peek() {
                    // Match Guard inside
                    Some(r) if r.is_digit(10) => {
                        n = n * 10 + r.to_digit(10).unwrap();
                    }
                    _ => return n // Retornando aqui, ele nao consome o caractere seguinte...
                }
                tokens.next();
            }
        }

        let mut chars = "226153980,1666319045".chars().peekable();

        assert_eq!(parse_number(&mut chars), 226153980);
        assert_eq!(chars.next(), Some(','));
        assert_eq!(parse_number(&mut chars), 1666319045);
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn fuse_iterator() {
        struct Flaky(bool);

        impl Iterator for Flaky {
            type Item = &'static str;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 {
                    self.0 = false;
                    Some("That's the last item...")
                } else {
                    self.0 = true;
                    None
                }
            }
        }

        {
            // Flaky Case
            let mut flaky = Flaky(true);
            assert_eq!(flaky.next(), Some("That's the last item..."));
            assert_eq!(flaky.next(), None);
            assert_eq!(flaky.next(), Some("That's the last item..."));
            assert_eq!(flaky.next(), None);
            assert_eq!(flaky.next(), Some("That's the last item..."));
        }
        {
            // Fused Flaky
            let mut not_flaky = Flaky(true).fuse();
            assert_eq!(not_flaky.next(), Some("That's the last item..."));
            assert_eq!(not_flaky.next(), None);
            assert_eq!(not_flaky.next(), None);
        }
    }

    #[test]
    fn rev_and_next_back_iterators() {
        let insect_parts = ["head", "thorax", "abdomen"];
        {
            // next_back()
            let mut iter = insect_parts.iter();
            assert_eq!(iter.next(), Some(&"head"));
            assert_eq!(iter.next_back(), Some(&"abdomen"));
            assert_eq!(iter.next(), Some(&"thorax"));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);
        }

        {
            // rev iterator
            let mut rev_parts = insect_parts.iter().rev();
            assert_eq!(rev_parts.next(), Some(&"abdomen"));
            assert_eq!(rev_parts.next(), Some(&"thorax"));
            assert_eq!(rev_parts.next(), Some(&"head"));
            assert_eq!(rev_parts.next(), None);
        }
    }
    
    #[test]
    fn inspect_iterator() {
        let mut before = vec![];
        let mut after = vec![];
        let upper_cased: String = "größe".chars()
            .inspect(|c| before.push(c.clone()))
            .flat_map(|c| c.to_uppercase())
            .inspect(|c| after.push(c.clone()))
            .collect();
        
        assert_eq!(upper_cased, "GRÖSSE");
        assert_eq!(before.iter().collect::<String>(), "größe");
        assert_eq!(after.iter().collect::<String>(), "GRÖSSE");
    }

    #[test]
    fn chain_iterator() {
        let v: Vec<u32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
        assert_eq!(v, [40, 30, 20, 3, 2, 1]);
    }

    #[test]
    fn enumerate_and_zip_iterators() {
        {
            // Enumerated chars
            let enumerated_chars: Vec<(usize, char)> = "HUE".chars().enumerate().collect();
            assert_eq!(enumerated_chars, vec![(0, 'H'), (1, 'U'), (2, 'E')]);
        }
        {
            // Zipped chars
            let zipped_chars: Vec<(usize, char)> = (1..=3).zip("HUE".chars()).collect();
            assert_eq!(zipped_chars, vec![(1, 'H'), (2, 'U'), (3, 'E')]);
        }   
    }

    #[test]
    fn cycle_iterator() {
        let dirs = ["North", "South", "East", "West"];
        let mut spin = dirs.iter().cycle();
        for i in 0..10 {
            assert_eq!(spin.next(), Some(&dirs[i % 4]));
        }
    }

    #[test]
    fn simple_accumulation() {
        {
            // Sum accumulator
            fn triangle(n: u64) -> u64 {
                (1..=n).sum()
            }
            assert_eq!(triangle(20), 210);
        }
        {
            // Product accumulator
            fn factorial(n: u64) -> u64 {
                (1..=n).product()
            }
            assert_eq!(factorial(20), 2432902008176640000);
        }
        {
            // Max and Min accumulator
            assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
            assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
        }
    }

    #[test]
    #[should_panic]
    fn max_and_min_with_functions() {
        {
            use std::cmp::Ordering;

            fn my_cmp(lhs: &f64, rhs: &f64) -> Ordering {
                lhs.partial_cmp(rhs).unwrap()
            }

            let numbers = [1.0, 4.0, 2.0, -7.0, -5.0];
            assert_eq!(numbers.iter().copied().max_by(my_cmp), Some(4.0));
            assert_eq!(numbers.iter().copied().max_by(my_cmp), Some(-7.0));

            let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
            assert_eq!(numbers.iter().copied().max_by(my_cmp), Some(4.0)); // Panics
        }
    }

    #[test]
    fn max_and_min_with_closures() {
        use std::collections::HashMap;

        let mut populations = HashMap::new();
        populations.insert("Brasilandia", 242_039);
        populations.insert("Parque Sao Luis", 43_774);
        populations.insert("Perus", 101_553);
        populations.insert("Pirituba", 308_932);
        populations.insert("Lapa", 7_001);
        assert_eq!(populations.iter().max_by_key(|&(_nome, pop)| pop), Some((&"Pirituba", &308_932)));
        assert_eq!(populations.iter().min_by_key(|&(_nome, pop)| pop), Some((&"Lapa", &7_001)));
    }

    #[test]
    fn comparing_sequences() {
        let packed = "Helen of Troy";
        let spaced = "Helen    of    Troy";
        let obscure = "Helen of Sandusky";

        assert!(packed != spaced);
        assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

        // Verdadeiro, pois ' ' < 'o'
        assert!(spaced < obscure);

        // Verdadeiro, pois 'T' > 'S'
        assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
    }

    #[test]
    fn any_and_all() {
        let word = "Czasowstrzymywacze";

        assert!(word.chars().any(char::is_uppercase));
        assert!(!word.chars().all(char::is_uppercase));
    }

    #[test]
    fn positions_from_start_and_from_end() {
        {
            // From Start
            let text = "Xanadu";
            assert_eq!(text.chars().position(|c| c == 'a'), Some(1));
            assert_eq!(text.chars().position(|c| c == 'z'), None);
        }
        {
            // From End (Necessita implementar o ExactSizeIterator trait)
            let text = b"Xanadu";
            assert_eq!(text.iter().rposition(|&c| c == b'a'), Some(3));
            assert_eq!(text.iter().rposition(|&c| c == b'z'), None);
        }
    }

    #[test]
    fn fold_and_rfold() {
        {
            // fold simples
            // Obs -> nomenclatura!
            // n: acumulador
            // i: item
            let nums = [5, 6, 7, 8, 9, 10];
            assert_eq!(nums.iter().fold(0, |n, _| n+1), 6);
            assert_eq!(nums.iter().fold(0, |n, i| n+i), 45);
            assert_eq!(nums.iter().fold(1, |n, i| n*i), 151200);
        }
        {
            // fold & rfold simples
            // Obs -> nomenclatura!
            // line: acumulador
            // word: item
            let words = ["Oto", "come", "doce", "seco", "de", "mocotó"];
            let pangram = words.iter()
                .fold(String::new(), |line, word| line + word + " ");
            let weird_pangram = words.iter()
                .rfold(String::new(), |line, word| line + word + " ");
            
            assert_eq!(pangram, "Oto come doce seco de mocotó ");
            assert_eq!(weird_pangram, "mocotó de seco doce come Oto ");
        }
    }

    #[test]
    fn try_fold() {
        {
            let failed_results = [Some(6), Some(1), Some(0), None, Some(3)];

            let sum = failed_results
                .iter()
                .try_fold(0, |s, res| -> Option<i32> {
                    match res {
                        Some(v) => Some(s + v),
                        None => None
                    }
            });

            assert_eq!(sum, None);
        }
        {
            let success_results = [Some(6), Some(1), Some(0), Some(7), Some(3)];

            let sum = success_results
                .iter()
                .try_fold(0, |s, res| -> Option<i32> {
                    if let Some(v) = res {
                        Some(s + v)
                    } else {
                        None
                    }
                });
            assert_eq!(sum, Some(17));
        }
    }

    #[test]
    fn getting_the_nth_from_start_and_from_end() {
        {
            let mut sq = (0..10).map(|x| x*x);
            // Inicio | Temos [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
            assert_eq!(sq.nth(4), Some(16)); // Consome os 4 primeiros itens | Sobrou [25, 36, 49, 64, 81]
            assert_eq!(sq.nth(0), Some(25)); // Consome o próximo item | Sobrou [36, 49, 64, 81]
            assert_eq!(sq.nth(6), None); // Sexto item a partir de agora não existe...
        }
        {
            let mut sq = (0..10).map(|x| x*x);
            // Inicio | Temos [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
            assert_eq!(sq.nth_back(4), Some(25)); // Consome os 4 últimos itens | Sobrou [0, 1, 4, 9, 16]
            assert_eq!(sq.nth_back(0), Some(16)); // Consome o próximo item | Sobrou [0, 1, 4, 9]
            assert_eq!(sq.nth_back(6), None); // Sexto item a partir de agora não existe...
        }
        {
            // Last
            let sq = (0..10).map(|x| x*x);
            assert_eq!(sq.last(), Some(81)); // Consome os 4 últimos itens | Sobrou [0, 1, 4, 9, 16]
        }
    }

    #[test]
    fn find_iterators() {
        use std::collections::HashMap;

        let mut populations = HashMap::new();
        populations.insert("Brasilandia", 242_039);
        populations.insert("Parque Sao Luis", 43_774);
        populations.insert("Perus", 101_553);
        populations.insert("Pirituba", 308_932);
        populations.insert("Lapa", 7_001);
        assert_eq!(populations.iter().find(|&(_nome, &pop)| pop > 1_000_000), None);
        assert_eq!(populations.iter().find(|&(_nome, &pop)| pop > 300_000), Some((&"Pirituba", &308_932)));
    }
}
