use std::{env, fs::File, str::FromStr};
use num::Complex;
use image::{ColorType, png::PNGEncoder};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprint!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprint!("Example: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("Falha na leitura das dimensões da imagem!");
    let upper_left = parse_complex(&args[3])
        .expect("Erro na leitura do ponto superior esquerdo!");
    let lower_right = parse_complex(&args[4])
        .expect("Erro na leitura do ponto inferior direito!");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    // Renderização single thread
    // render(&mut pixels, bounds, upper_left, lower_right);

    // Renderização em multithreads (na mão)
    // let threads = 8;
    // let rows_per_band = bounds.1 / threads + 1;
    // {
    //     let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
    //     crossbeam::scope(|spawner| {
    //         for (i, band) in bands.into_iter().enumerate() {
    //             let top = rows_per_band * i;
    //             let height = band.len() / bounds.0;
    //             let band_bounds = (bounds.0, height);
    //             let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
    //             let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
    //             spawner.spawn(move |_| {
    //                 render(band, band_bounds, band_upper_left, band_lower_right);
    //             });
    //         }
    //     }).unwrap();
    // }

    {
        // Renderização multithread usando rayon
        let bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut(bounds.0)
            .enumerate()
            .collect();
        
        bands.into_par_iter()
            .for_each(|(i, band)|{
                let top = i;
                let band_bounds = (bounds.0, 1);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
    }

    write_image(&args[1], &pixels, bounds) 
        .expect("Erro ao tentar escrever o arquivo PNG!");
}
/// Determina se o complexo 'c' está no conjunto de Mandelbrot, usando no máximo 
/// 'limit' iterações. 
/// 
/// Se 'c' não estiver no conjunto, retorna 'Some(i)', onde 'i' é o número de 
/// iterações necessárias para que 'c' sair do círculo de raio 2 centrado na origem.
/// Se 'c' estiver dentro do conjunto quando o limite for atingido (o que significa 
/// que 'c' deve estar no conjunto), retorna None
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Analisa a strnig 's' como um par ordenado, como "400x600" ou "1.5,0.5"
/// 
/// Especificamente, 's' deveria ter forma <esquerda><sep><direita>, onde <sep> 
/// deve ser um caractere ASCII
/// 
/// Se a string 's' tiver a forma apropriada, retorna 'Some<x, y>'. Se não, 
/// retorna None
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None, 
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)), 
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Analisa um par de pontos flutuantes separados por vírgula e retorna um complexo
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}), 
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), 
        Some( Complex{ re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Dadas as coordenadas de um pixel na imagem de saída, retorna o 
/// ponto correspondente no plano complexo. 
/// 
/// 'bounds' é um par ordenado indicando a altura e largura da imagem em pixels
/// 'pixel' é um par (x, y) indicando um pixel 
fn pixel_to_point(bounds: (usize, usize), 
                  pixel: (usize, usize), 
                  upper_left: Complex<f64>, 
                  lower_right: Complex<f64>)
                  -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, 
                                    upper_left.im - lower_right.im);
    Complex { 
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64, 
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64 
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175), 
                              Complex{ re: -1.0, im: 1.0}, 
                              Complex{ re: 1.0, im: -1.0}), 
                Complex{ re: -0.5, im: -0.75});
}

fn render(  pixels: &mut [u8], 
            bounds: (usize, usize), 
            upper_left: Complex<f64>, 
            lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point( bounds, (column, row), 
                                                    upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0, 
                Some(count) => count as u8
            };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}