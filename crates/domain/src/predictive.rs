//! Família derivada de métricas (achados #1/#7, ADR-05): `burn_rate` é
//! primária (`fn(&[Money], ...) -> Metric<Money>`), `runway` é derivada
//! (`fn(Money, &Metric<Money>) -> Metric<Decimal>`) — consome o resultado
//! de `burn_rate`, não dados brutos. Forçar as duas na mesma assinatura
//! "toda métrica é pura sobre dados crus" era o próprio bug de modelagem
//! que o achado #7 identificou.

use rust_decimal::Decimal;

use crate::metric::{Metric, Provenance};
use crate::money::{sum_homogeneous, CurrencyMismatch, Money};

/// Erro ao calcular uma métrica preditiva.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredictiveError {
    /// `burn_rate` não aceita uma sequência vazia — não há período a medir.
    EmptyInput,
    /// Os valores fornecidos não compartilham a mesma moeda.
    CurrencyMismatch(CurrencyMismatch),
    /// `runway` exige que `burn_rate` seja negativo (queimando caixa); um
    /// ritmo zero ou positivo (net inflow) não define uma "pista" finita.
    NotBurning,
}

/// Taxa de Queima (Burn Rate): fluxo de caixa líquido médio por período.
///
/// Cada elemento de `net_cashflow_per_period` é o fluxo líquido de um
/// período (mês, tipicamente) na convenção de sinal do OFX — negativo
/// significa queima de caixa naquele período. O resultado é a média
/// simples desses valores.
///
/// ```
/// use raptor_domain::predictive::burn_rate;
/// use raptor_domain::money::{Money, CurrencyCode};
/// use rust_decimal::Decimal;
///
/// let months = vec![
///     Money::new(Decimal::new(-100000, 2), CurrencyCode::BRL), // -R$ 1.000,00
///     Money::new(Decimal::new(-200000, 2), CurrencyCode::BRL), // -R$ 2.000,00
/// ];
/// let rate = burn_rate(&months).unwrap();
/// assert_eq!(rate.value.amount(), Decimal::new(-150000, 2)); // -R$ 1.500,00
/// ```
pub fn burn_rate(net_cashflow_per_period: &[Money]) -> Result<Metric<Money>, PredictiveError> {
    if net_cashflow_per_period.is_empty() {
        return Err(PredictiveError::EmptyInput);
    }
    let total = sum_homogeneous(net_cashflow_per_period.iter().copied())
        .map_err(PredictiveError::CurrencyMismatch)?
        .expect("checado não-vazio acima");
    let count = Decimal::from(net_cashflow_per_period.len());
    let average = Money::new(total.amount() / count, total.currency());
    Ok(Metric::new(
        average,
        Provenance::new(net_cashflow_per_period.len()),
    ))
}

/// Pista de Sobrevivência (Runway): quantos períodos o caixa disponível
/// dura no ritmo de queima informado por `burn`.
///
/// Família derivada: consome o *resultado* de [`burn_rate`], não dados
/// brutos — é o caso central do achado #7.
///
/// ```
/// use raptor_domain::predictive::{burn_rate, runway};
/// use raptor_domain::money::{Money, CurrencyCode};
/// use rust_decimal::Decimal;
///
/// let months = vec![Money::new(Decimal::new(-100000, 2), CurrencyCode::BRL)]; // -R$ 1.000,00/mês
/// let rate = burn_rate(&months).unwrap();
/// let available = Money::new(Decimal::new(500000, 2), CurrencyCode::BRL); // R$ 5.000,00
/// let survival = runway(available, &rate).unwrap();
/// assert_eq!(survival.value, Decimal::new(5, 0)); // 5 meses de pista
/// ```
pub fn runway(available: Money, burn: &Metric<Money>) -> Result<Metric<Decimal>, PredictiveError> {
    if available.currency() != burn.value.currency() {
        return Err(PredictiveError::CurrencyMismatch(CurrencyMismatch {
            expected: available.currency(),
            found: burn.value.currency(),
        }));
    }
    if burn.value.amount() >= Decimal::ZERO {
        return Err(PredictiveError::NotBurning);
    }
    let periods = available.amount() / (-burn.value.amount());
    Ok(Metric::new(
        periods,
        Provenance::new(burn.provenance.sample_size),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money::CurrencyCode;

    fn brl(cents: i64) -> Money {
        Money::new(Decimal::new(cents, 2), CurrencyCode::BRL)
    }

    #[test]
    fn burn_rate_vazio_retorna_erro() {
        assert_eq!(burn_rate(&[]), Err(PredictiveError::EmptyInput));
    }

    #[test]
    fn burn_rate_moeda_mista_retorna_erro() {
        let values = vec![
            brl(-1000),
            Money::new(Decimal::new(-1000, 2), CurrencyCode::USD),
        ];
        assert!(matches!(
            burn_rate(&values),
            Err(PredictiveError::CurrencyMismatch(_))
        ));
    }

    #[test]
    fn burn_rate_calcula_media_simples() {
        let values = vec![brl(-10000), brl(-20000), brl(-30000)];
        let rate = burn_rate(&values).unwrap();
        assert_eq!(rate.value.amount(), Decimal::new(-20000, 2));
        assert_eq!(rate.provenance.sample_size, 3);
    }

    #[test]
    fn runway_calcula_periodos_restantes() {
        let rate = burn_rate(&[brl(-50000)]).unwrap(); // -R$ 500,00/mês
        let available = brl(200000); // R$ 2.000,00
        let survival = runway(available, &rate).unwrap();
        assert_eq!(survival.value, Decimal::new(4, 0));
    }

    #[test]
    fn runway_com_burn_rate_zero_retorna_erro() {
        let rate = Metric::new(brl(0), Provenance::new(1));
        assert_eq!(runway(brl(100000), &rate), Err(PredictiveError::NotBurning));
    }

    #[test]
    fn runway_com_burn_rate_positivo_retorna_erro() {
        let rate = Metric::new(brl(1000), Provenance::new(1)); // net inflow, não queima
        assert_eq!(runway(brl(100000), &rate), Err(PredictiveError::NotBurning));
    }

    #[test]
    fn runway_moeda_incompativel_retorna_erro() {
        let rate = burn_rate(&[brl(-1000)]).unwrap();
        let available = Money::new(Decimal::new(100000, 2), CurrencyCode::USD);
        assert!(matches!(
            runway(available, &rate),
            Err(PredictiveError::CurrencyMismatch(_))
        ));
    }

    #[test]
    fn runway_propaga_sample_size_da_provenance_do_burn() {
        let rate = burn_rate(&[brl(-1000), brl(-2000)]).unwrap();
        let survival = runway(brl(30000), &rate).unwrap();
        assert_eq!(survival.provenance.sample_size, 2);
    }
}
