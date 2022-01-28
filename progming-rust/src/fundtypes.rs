#[cfg(test)]
mod tests {
    #[test]
    fn test_direct_typecast_operator() {
        assert_eq!(10_i8 as u16, 10_u16);
        assert_eq!(2525_u16 as i16, 2525_i16);
        assert_eq!(-1_i16 as i32, -1_i32);
        assert_eq!(65535_u16 as i32, 65535_i32);
    }

    #[test]
    fn test_truncated_typecast_operator() {
        assert_eq!(1000_i16 as u8, 232_u8);
        assert_eq!(65535_u32 as i16, -1_i16);
        assert_eq!(-1_i8 as u8, 255_u8);
        assert_eq!(255_u8 as i8, -1_i8);
    }

    #[test]
    #[should_panic]
    fn test_avoid_integer_overflow() {
        // Example to avoid 
        let mut value: usize = 1; 
        loop {
            value = value.checked_mul(10).expect("Multiplication Overflow");
        }
    }

    #[test]
    fn test_checked_operations() {
        assert_eq!(10_u8.checked_add(20), Some(30));
        assert_eq!(100_u8.checked_add(200), None);
        assert_eq!((-128_i16).checked_div(-1), Some(128));
        assert_eq!((-128_i8).checked_div(-1), None);
    }

    #[test]
    fn test_wrapping_operations() {
        assert_eq!(100_u16.wrapping_mul(200), 20000);

        // Sem overflow seria 250000
        assert_eq!(500_u16.wrapping_mul(500), 53392); 
        
        // Tipos com sinal podem gerar valores negativos
        assert_eq!(500_i16.wrapping_mul(500), -12144);

        // Operações bitwise shift, a distância é normalizada
        // para o tamanho do inteiro. (19 % 16 = 3) | 5 << 3 = 40
        assert_eq!(5_i16.wrapping_shl(19), 40);
    }

    #[test]
    fn test_saturating_operations() {
        // Saturating -> Valor é truncado ao máximo ou mínimo caso ocorra overflow
        assert_eq!(32760_i16.saturating_add(10), 32767);
        assert_eq!((-32760_i16).saturating_sub(10), -32768);
    }

    #[test]
    fn test_overflowing_operations() {
        // Overflowing -> O overflow ocorre mas também retorna uma flag para indicar
        assert_eq!(255_u8.overflowing_sub(2), (253, false));
        assert_eq!(255_u8.overflowing_add(2), (1, true));
        
        // Operações bitwise shift, a distância é normalizada
        // para o tamanho do inteiro. (19 % 16 = 3) | 5 << 3 = 40
        assert_eq!(5_i16.overflowing_shl(19), (40, true));
    }

    #[test]
    fn test_floating_point() {
        assert!((-1. / f32::INFINITY).is_sign_negative());
        assert_eq!(-f32::MIN, f32::MAX);
        assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5.0);
        assert_eq!((-1.01_f64).floor(), -2.0);
    }

    #[test]
    fn test_chars() {
        assert_eq!('*'.is_alphabetic(), false);
        assert_eq!('β'.is_alphabetic(), true);
        assert_eq!('8'.to_digit(10), Some(8));
        assert_eq!('ಠ'.len_utf8(), 3);
        assert_eq!(std::char::from_digit(2, 10), Some('2'));
    }

    #[test]
    fn test_tuples() {
        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");
    }

    #[test]
    fn test_arrays() {
        let numbers = [1, 2, 6, 7, 12, 17, 19];
        let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
        assert_eq!(numbers[3], 7);
        assert_eq!(taxonomy.len(), 3);

        let mut chaos = [77, 11, 23, 101, 5, 39];
        chaos.sort();
        assert_eq!(chaos, [5, 11, 23, 39, 77, 101]);
    }

    #[test]
    fn test_vectors_playground() {
        // Usando um vec! macro
        {
            let mut primes = vec![2, 3, 5, 7];
            assert_eq!(primes.iter().product::<i32>(), 210);

            primes.push(11);
            primes.push(13);

            assert_eq!(primes.iter().product::<i32>(), 30030);
        }

        // Usando variáveis para definir o tamanho de um vector
        {
            let (width, height) = (600, 800);
            let screen = vec![0; width * height];
            assert_eq!(screen.len(), 600 * 800);
        }

        // Usando Vec::new()
        {
            let mut pal = Vec::new();
            pal.push("step");
            pal.push("on");
            pal.push("no");
            pal.push("pets");
            assert_eq!(pal, vec!["step", "on", "no", "pets"]);
        }

        // Criando um vetor a partir de um iterador
        {
            let v: Vec<i32> = (0..5).collect();
            assert_eq!(v, [0, 1, 2, 3, 4]);
            assert_eq!(v, vec![0, 1, 2, 3, 4]);
        }

        // É possível usar "sclice methods" em vetores
        {
            let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
            palindrome.reverse();
            assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
        }

        // Usando Vec::with_capacity e suas implicações
        {
            let mut v = Vec::with_capacity(2);
            assert_eq!(v.len(), 0);
            assert_eq!(v.capacity(), 2);

            v.push(1);
            v.push(2);

            assert_eq!(v.len(), 2);
            assert_eq!(v.capacity(), 2);

            v.push(3); // Nesse ponto, ocorre alocação de mais memória
            assert_eq!(v.len(), 3);
            assert!(v.capacity() > 2); // Usualmente sairia 4 ...
        }

        // Pegando itens do final de um vetor usando .pop()
        {
            let mut v = vec!["Snow puff", "Glass gem"];
            assert_eq!(v.pop(), Some("Glass gem"));
            assert_eq!(v.pop(), Some("Snow puff"));
            assert_eq!(v.pop(), None);
        }
    }

    #[test]
    fn test_strings_playground() {
        assert!("ONE".to_lowercase() == "one");
        assert!("peanut".contains("nut"));
        assert_eq!("ಠ_ಠ".replace("ಠ", "o"), "o_o");
        assert_eq!("    clean\n".trim(), "clean");

        for word in "veni, vidi, vici".split(", "){
            assert!(word.starts_with("v"));
        }
    }
}
