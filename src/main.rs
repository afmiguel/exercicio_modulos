//! src/main.rs - Versão Inicial Expandida com Documentação (Não Modular)
//!
//! Este arquivo contém uma coleção de funções matemáticas abrangendo
//! aritmética básica e avançada, geometria 2D e estatística descritiva,
//! juntamente com uma função `main` que demonstra o uso de cada uma delas.
//! O objetivo deste estado inicial é servir como ponto de partida para um
//! exercício de refatoração, onde este código monolítico será organizado
//! em uma estrutura modular.

use std::collections::HashMap;
use std::f64::consts::PI; // Necessário para 'mode'

// --- Aritmética ---

/// Adiciona dois inteiros.
///
/// # Parameters
/// * `a`: O primeiro operando.
/// * `b`: O segundo operando.
///
/// # Returns
/// A soma de `a` e `b`.
fn adicionar(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtrai o segundo inteiro do primeiro.
///
/// # Parameters
/// * `a`: O minuendo.
/// * `b`: O subtraendo.
///
/// # Returns
/// A diferença entre `a` e `b`.
fn subtrair(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplica dois inteiros.
///
/// # Parameters
/// * `a`: O primeiro fator.
/// * `b`: O segundo fator.
///
/// # Returns
/// O produto de `a` e `b`.
fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

/// Divide dois números de ponto flutuante.
///
/// Retorna `None` se o divisor `b` for zero.
///
/// # Parameters
/// * `a`: O dividendo.
/// * `b`: O divisor.
///
/// # Returns
/// `Some(resultado)` se a divisão for bem-sucedida, `None` caso contrário.
fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

/// Calcula o resto da divisão inteira.
///
/// # Parameters
/// * `a`: O dividendo.
/// * `b`: O divisor.
///
/// # Returns
/// O resto de `a` dividido por `b`.
fn modulo(a: i32, b: i32) -> i32 {
    a % b
}

/// Calcula a potência de um número base elevado a um expoente.
///
/// Utiliza números de ponto flutuante para permitir expoentes não inteiros.
///
/// # Parameters
/// * `base`: O número base.
/// * `exp`: O expoente.
///
/// # Returns
/// `base` elevado à potência `exp`.
fn power(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

/// Calcula o fatorial de um número inteiro não negativo.
///
/// O fatorial de 0 é 1.
///
/// # Parameters
/// * `n`: O número para o qual calcular o fatorial (deve ser não negativo).
///
/// # Returns
/// O fatorial de `n`. Pode ocorrer overflow para valores grandes de `n`.
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Verifica se um número inteiro não negativo é primo.
///
/// Um número primo é um número natural maior que 1 que não tem divisores
/// positivos além de 1 e ele mesmo.
///
/// # Parameters
/// * `n`: O número a ser verificado.
///
/// # Returns
/// `true` se `n` for primo, `false` caso contrário.
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    // Otimização: verificar divisores apenas até a raiz quadrada de n
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// --- Geometria ---

/// Representa um ponto no espaço 2D com coordenadas de ponto flutuante.
#[derive(Debug, Clone, Copy)]
struct Ponto {
    /// A coordenada no eixo X.
    x: f64,
    /// A coordenada no eixo Y.
    y: f64,
}

/// Calcula a distância euclidiana entre dois pontos no espaço 2D.
///
/// # Parameters
/// * `p1`: O primeiro ponto.
/// * `p2`: O segundo ponto.
///
/// # Returns
/// A distância entre `p1` e `p2`.
fn distancia_entre_pontos(p1: &Ponto, p2: &Ponto) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

/// Calcula a área de um retângulo.
///
/// # Parameters
/// * `largura`: A largura do retângulo.
/// * `altura`: A altura do retângulo.
///
/// # Returns
/// A área do retângulo.
fn area_retangulo(largura: f64, altura: f64) -> f64 {
    largura * altura
}

/// Calcula a área de um círculo.
///
/// # Parameters
/// * `raio`: O raio do círculo.
///
/// # Returns
/// A área do círculo.
fn area_circulo(raio: f64) -> f64 {
    PI * raio.powi(2)
}

/// Calcula o perímetro de um retângulo.
///
/// # Parameters
/// * `width`: A largura do retângulo.
/// * `height`: A altura do retângulo.
///
/// # Returns
/// O perímetro do retângulo.
fn perimeter_rectangle(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

/// Calcula o perímetro (circunferência) de um círculo.
///
/// # Parameters
/// * `radius`: O raio do círculo.
///
/// # Returns
/// A circunferência do círculo.
fn perimeter_circle(radius: f64) -> f64 {
    2.0 * PI * radius
}

/// Translada um ponto por um vetor de deslocamento (dx, dy).
///
/// # Parameters
/// * `p`: O ponto original.
/// * `dx`: O deslocamento no eixo X.
/// * `dy`: O deslocamento no eixo Y.
///
/// # Returns
/// Um novo `Ponto` resultante da translação.
fn translate_point(p: &Ponto, dx: f64, dy: f64) -> Ponto {
    Ponto {
        x: p.x + dx,
        y: p.y + dy,
    }
}

// --- Estatística ---

/// Calcula a média aritmética de um slice de números de ponto flutuante.
///
/// Retorna `None` se o slice estiver vazio.
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(media)` se o slice não estiver vazio, `None` caso contrário.
fn media(numeros: &[f64]) -> Option<f64> {
    if numeros.is_empty() {
        None
    } else {
        Some(numeros.iter().sum::<f64>() / numeros.len() as f64)
    }
}

/// Calcula a mediana de um slice de números de ponto flutuante.
///
/// A mediana é o valor que separa a metade superior da metade inferior de um conjunto de dados.
/// Retorna `None` se o slice estiver vazio.
/// **Importante:** Esta função ordena o slice de entrada `numeros` no local.
///
/// # Parameters
/// * `numeros`: Um slice mutável contendo os números. Será ordenado por esta função.
///
/// # Returns
/// `Some(mediana)` se o slice não estiver vazio, `None` caso contrário.
fn mediana(numeros: &mut [f64]) -> Option<f64> {
    if numeros.is_empty() {
        return None;
    }
    // Ordena o slice no local para encontrar a mediana
    numeros.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numeros.len() / 2;
    if numeros.len() % 2 == 0 {
        // Média dos dois elementos centrais para tamanho par
        media(&[numeros[mid - 1], numeros[mid]])
    } else {
        // Elemento central para tamanho ímpar
        Some(numeros[mid])
    }
}

/// Calcula a variância populacional de um slice de números de ponto flutuante.
///
/// A variância mede a dispersão dos dados em relação à média.
/// Retorna `None` se o slice estiver vazio.
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(variancia)` se o slice não estiver vazio, `None` caso contrário.
fn variance(numeros: &[f64]) -> Option<f64> {
    // Calcula a média dos quadrados das diferenças em relação à média
    media(numeros).map(|m| {
        media(
            &numeros
                .iter()
                .map(|x| (x - m).powi(2))
                .collect::<Vec<f64>>(),
        )
        .unwrap_or(0.0)
    })
}

/// Calcula o desvio padrão populacional de um slice de números de ponto flutuante.
///
/// O desvio padrão é a raiz quadrada da variância.
/// Retorna `None` se o slice estiver vazio.
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(desvio_padrao)` se o slice não estiver vazio, `None` caso contrário.
fn standard_deviation(numeros: &[f64]) -> Option<f64> {
    variance(numeros).map(|v| v.sqrt())
}

/// Encontra a moda (ou modas) de um slice de números de ponto flutuante.
///
/// A moda é o valor que aparece com mais frequência no conjunto de dados.
/// Pode haver mais de uma moda (conjunto multimodal).
/// Retorna `None` se o slice estiver vazio ou se não houver uma moda clara (todos os elementos são únicos ou têm a mesma frequência baixa).
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(Vec<f64>)` contendo a(s) moda(s) se existir(em), `None` caso contrário.
fn mode(numeros: &[f64]) -> Option<Vec<f64>> {
    if numeros.is_empty() {
        return None;
    }
    let mut counts = HashMap::new();
    // Conta a frequência de cada número.
    // Usar representação de string para chaves f64 é uma solução alternativa,
    // pois f64 não implementa Eq/Hash diretamente devido a NaN.
    numeros
        .iter()
        .for_each(|&n| *counts.entry(n.to_string()).or_insert(0) += 1);

    // Encontra a frequência máxima
    let max_count = counts.values().max().copied().unwrap_or(0);

    // Se a contagem máxima for 1 (e houver mais de um elemento), não há moda clara
    if max_count <= 1 && numeros.len() > 1 {
        return None;
    }

    // Coleta todos os números que têm a frequência máxima
    Some(
        counts
            .into_iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(val_str, _)| {
                val_str
                    .parse::<f64>()
                    .expect("Falha ao converter string de volta para f64")
            }) // Usar expect aqui, pois a string veio de um f64
            .collect(),
    )
}

/// Ponto de entrada principal da aplicação.
///
/// Demonstra o uso das funções matemáticas definidas neste arquivo.
fn main() {
    println!("--- Demonstração Inicial Expandida (Todas as Funções Usadas) ---");

    // --- Seção de Demonstração: Aritmética ---
    println!("\n** Aritmética **");
    let a_int = 15;
    let b_int = 4;
    let a_float = 15.0;
    let b_float = 4.0;
    let base_pow = 2.0;
    let exp_pow = 10.0;

    println!(
        "Adição: {} + {} = {}",
        a_int,
        b_int,
        adicionar(a_int, b_int)
    );
    println!(
        "Subtração: {} - {} = {}",
        a_int,
        b_int,
        subtrair(a_int, b_int)
    );
    println!(
        "Multiplicação: {} * {} = {}",
        a_int,
        b_int,
        multiplicar(a_int, b_int)
    );
    match dividir(a_float, b_float) {
        Some(res) => println!("Divisão: {} / {} = {}", a_float, b_float, res),
        None => println!("Divisão por zero!"),
    }
    match dividir(a_float, 0.0) {
        // Teste de divisão por zero
        Some(res) => println!("Divisão: {} / {} = {}", a_float, 0.0, res), // Não deve acontecer
        None => println!("Tentativa de Divisão por zero!"),
    }
    println!("Módulo: {} % {} = {}", a_int, b_int, modulo(a_int, b_int));
    println!(
        "Potência: {}^{} = {}",
        base_pow,
        exp_pow,
        power(base_pow, exp_pow)
    );
    println!("Fatorial de 5: {}", factorial(5));
    println!("Verificação Primo (17): {}", is_prime(17));
    println!("Verificação Primo (18): {}", is_prime(18));

    // --- Seção de Demonstração: Geometria ---
    println!("\n** Geometria **");
    let ponto_a = Ponto { x: 1.0, y: 1.0 };
    let ponto_b = Ponto { x: 4.0, y: 5.0 };
    let ponto_c = translate_point(&ponto_a, 3.0, 4.0); // Ponto A transladado
    let largura_rect = 5.0;
    let altura_rect = 3.0;
    let raio_circ = 2.5;

    println!("Ponto A: {:?}", ponto_a);
    println!("Ponto B: {:?}", ponto_b);
    println!("Ponto C (A transladado): {:?}", ponto_c);
    println!(
        "Distância (A -> B): {:.2}",
        distancia_entre_pontos(&ponto_a, &ponto_b)
    );
    println!(
        "Área Retângulo ({:.1}x{:.1}): {:.2}",
        largura_rect,
        altura_rect,
        area_retangulo(largura_rect, altura_rect)
    );
    println!(
        "Perímetro Retângulo ({:.1}x{:.1}): {:.2}",
        largura_rect,
        altura_rect,
        perimeter_rectangle(largura_rect, altura_rect)
    );
    println!(
        "Área Círculo (raio {:.1}): {:.2}",
        raio_circ,
        area_circulo(raio_circ)
    );
    println!(
        "Perímetro Círculo (raio {:.1}): {:.2}",
        raio_circ,
        perimeter_circle(raio_circ)
    );

    // --- Seção de Demonstração: Estatística ---
    println!("\n** Estatística **");
    // 'dados_stats' precisa ser mutável para a função 'mediana' que ordena no local.
    let mut dados_stats = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0, 1.0];
    println!("Dados originais: {:?}", dados_stats);

    if let Some(m) = media(&dados_stats) {
        println!("Média: {:.2}", m);
    }
    // A chamada a 'mediana' modifica 'dados_stats' ordenando-o.
    if let Some(med) = mediana(&mut dados_stats) {
        println!("Mediana: {:.2}", med);
    }
    println!(
        "Dados após cálculo da mediana (ordenados): {:?}",
        dados_stats
    );
    if let Some(var) = variance(&dados_stats) {
        // Usa os dados já ordenados
        println!("Variância: {:.2}", var);
    }
    if let Some(dev_std) = standard_deviation(&dados_stats) {
        // Usa os dados já ordenados
        println!("Desvio Padrão: {:.2}", dev_std);
    }
    if let Some(moda_vals) = mode(&dados_stats) {
        // Usa os dados já ordenados
        println!("Moda: {:?}", moda_vals);
    }

    println!("\n--- Fim da Demonstração ---");
}
