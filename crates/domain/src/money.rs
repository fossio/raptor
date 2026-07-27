//! Representação monetária com moeda obrigatória (ADR-03, achado #2).
//!
//! `Money` nunca soma silenciosamente entre moedas diferentes: o tipo não
//! implementa `std::ops::Add` de propósito, para que somar dois valores sem
//! checar a moeda seja um erro de compilação, não um bug de runtime. Soma
//! passa por [`Money::try_add`] ou [`sum_homogeneous`], que retornam
//! `Err(CurrencyMismatch)` em vez de misturar moedas.

use std::fmt;

use rust_decimal::Decimal;

/// Código de moeda ISO 4217 (três letras maiúsculas, ex.: `BRL`, `USD`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CurrencyCode([u8; 3]);

impl CurrencyCode {
    /// Moeda comum usada em testes e exemplos: Real brasileiro.
    pub const BRL: CurrencyCode = CurrencyCode(*b"BRL");
    /// Moeda comum usada em testes e exemplos: Dólar americano.
    pub const USD: CurrencyCode = CurrencyCode(*b"USD");

    /// Retorna o código como string (ex.: `"BRL"`).
    pub fn as_str(&self) -> &str {
        // Invariante do construtor garante ASCII válido.
        std::str::from_utf8(&self.0).expect("CurrencyCode sempre contém ASCII válido")
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Erro ao construir um [`CurrencyCode`] a partir de uma string arbitrária.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidCurrencyCode {
    input: String,
}

impl fmt::Display for InvalidCurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "código de moeda inválido: {:?} (esperado 3 letras ASCII maiúsculas)",
            self.input
        )
    }
}

impl std::error::Error for InvalidCurrencyCode {}

impl TryFrom<&str> for CurrencyCode {
    type Error = InvalidCurrencyCode;

    /// ```
    /// use raptor_domain::money::CurrencyCode;
    ///
    /// let brl = CurrencyCode::try_from("BRL").unwrap();
    /// assert_eq!(brl.as_str(), "BRL");
    /// assert!(CurrencyCode::try_from("R$").is_err());
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() == 3 && bytes.iter().all(u8::is_ascii_uppercase) {
            Ok(CurrencyCode([bytes[0], bytes[1], bytes[2]]))
        } else {
            Err(InvalidCurrencyCode {
                input: value.to_string(),
            })
        }
    }
}

/// Um valor monetário exato (`Decimal`, nunca `f64`) com moeda obrigatória.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    amount: Decimal,
    currency: CurrencyCode,
}

impl Money {
    /// Constrói um valor monetário.
    ///
    /// ```
    /// use raptor_domain::money::{Money, CurrencyCode};
    /// use rust_decimal::Decimal;
    ///
    /// let m = Money::new(Decimal::new(1050, 2), CurrencyCode::BRL); // R$ 10,50
    /// assert_eq!(m.currency(), CurrencyCode::BRL);
    /// ```
    pub fn new(amount: Decimal, currency: CurrencyCode) -> Self {
        Self { amount, currency }
    }

    /// O valor numérico exato.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// A moeda do valor.
    pub fn currency(&self) -> CurrencyCode {
        self.currency
    }

    /// Soma dois valores monetários, falhando se as moedas divergirem.
    ///
    /// Deliberadamente não é `std::ops::Add`: o objetivo é que somar em
    /// moedas diferentes seja um `Result` a tratar no call-site, nunca um
    /// operador que alguém usa sem pensar (achado #2).
    pub fn try_add(self, other: Money) -> Result<Money, CurrencyMismatch> {
        if self.currency != other.currency {
            return Err(CurrencyMismatch {
                expected: self.currency,
                found: other.currency,
            });
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }
}

/// Erro retornado ao tentar somar valores em moedas diferentes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrencyMismatch {
    pub expected: CurrencyCode,
    pub found: CurrencyCode,
}

impl fmt::Display for CurrencyMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "moedas incompatíveis: esperado {}, encontrado {}",
            self.expected, self.found
        )
    }
}

impl std::error::Error for CurrencyMismatch {}

/// Soma uma sequência de valores monetários, exigindo que todos compartilhem
/// a mesma moeda. É o ponto de entrada que agregações financeiras (NCF, ADB,
/// AT) usam em vez de somar `Decimal` cru.
///
/// Retorna `None` para uma sequência vazia — não há moeda a inferir.
///
/// ```
/// use raptor_domain::money::{sum_homogeneous, Money, CurrencyCode};
/// use rust_decimal::Decimal;
///
/// let values = vec![
///     Money::new(Decimal::new(100, 0), CurrencyCode::BRL),
///     Money::new(Decimal::new(250, 0), CurrencyCode::BRL),
/// ];
/// let total = sum_homogeneous(values).unwrap().unwrap();
/// assert_eq!(total.amount(), Decimal::new(350, 0));
/// ```
pub fn sum_homogeneous(
    values: impl IntoIterator<Item = Money>,
) -> Result<Option<Money>, CurrencyMismatch> {
    let mut iter = values.into_iter();
    let Some(first) = iter.next() else {
        return Ok(None);
    };
    iter.try_fold(first, |acc, next| acc.try_add(next))
        .map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_add_same_currency_soma_exata() {
        let a = Money::new(Decimal::new(1000, 2), CurrencyCode::BRL);
        let b = Money::new(Decimal::new(250, 2), CurrencyCode::BRL);
        let total = a.try_add(b).unwrap();
        assert_eq!(total.amount(), Decimal::new(1250, 2));
        assert_eq!(total.currency(), CurrencyCode::BRL);
    }

    #[test]
    fn try_add_moedas_diferentes_retorna_erro() {
        let a = Money::new(Decimal::new(1000, 2), CurrencyCode::BRL);
        let b = Money::new(Decimal::new(1000, 2), CurrencyCode::USD);
        let err = a.try_add(b).unwrap_err();
        assert_eq!(err.expected, CurrencyCode::BRL);
        assert_eq!(err.found, CurrencyCode::USD);
    }

    #[test]
    fn sum_homogeneous_vazio_retorna_none() {
        assert_eq!(sum_homogeneous(Vec::<Money>::new()).unwrap(), None);
    }

    #[test]
    fn sum_homogeneous_moeda_unica_soma_tudo() {
        let values = vec![
            Money::new(Decimal::new(100, 0), CurrencyCode::BRL),
            Money::new(Decimal::new(200, 0), CurrencyCode::BRL),
            Money::new(Decimal::new(300, 0), CurrencyCode::BRL),
        ];
        let total = sum_homogeneous(values).unwrap().unwrap();
        assert_eq!(total.amount(), Decimal::new(600, 0));
    }

    #[test]
    fn sum_homogeneous_moeda_mista_retorna_erro() {
        let values = vec![
            Money::new(Decimal::new(100, 0), CurrencyCode::BRL),
            Money::new(Decimal::new(200, 0), CurrencyCode::USD),
        ];
        assert!(sum_homogeneous(values).is_err());
    }

    #[test]
    fn currency_code_rejeita_string_invalida() {
        assert!(CurrencyCode::try_from("brl").is_err()); // minúsculo
        assert!(CurrencyCode::try_from("R$").is_err()); // não-ASCII/tamanho errado
        assert!(CurrencyCode::try_from("BRLL").is_err()); // tamanho errado
    }

    #[test]
    fn currency_code_aceita_iso4217_valido() {
        let code = CurrencyCode::try_from("EUR").unwrap();
        assert_eq!(code.as_str(), "EUR");
    }
}
