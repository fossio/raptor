//! `Metric<V>` — o tipo de retorno de toda métrica de `analytics` (ADR-05).
//!
//! Nunca o valor cru: sempre acompanhado de [`Provenance`], o canal de
//! auditoria que substitui `println!`/`eprintln!` (proibidos no core) para
//! registrar janela, contagem de observações e entradas externas usadas.

use rust_decimal::Decimal;

/// Resultado de uma métrica: o valor calculado e sua proveniência.
///
/// ```
/// use raptor_domain::metric::{Metric, Provenance};
///
/// let m = Metric::new(42, Provenance::new(50));
/// assert_eq!(m.value, 42);
/// assert!(m.provenance.diagnostics.is_empty()); // 50 >= 30, amostra suficiente
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Metric<V> {
    pub value: V,
    pub provenance: Provenance,
}

impl<V> Metric<V> {
    /// Constrói uma métrica a partir do valor e da proveniência já montada.
    pub fn new(value: V, provenance: Provenance) -> Self {
        Self { value, provenance }
    }
}

/// Proveniência de uma métrica: quantas observações entraram no cálculo,
/// quais entradas externas foram usadas, e diagnostics emitidos automaticamente.
///
/// Este `Diagnostic` é distinto do `Diagnostic`/`DiagnosticCode` de parsing
/// (que mora em `domain::diagnostic`, achado #48) — este é sobre a
/// confiabilidade estatística do cálculo, não sobre o arquivo importado.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Provenance {
    pub sample_size: usize,
    pub external_inputs: Vec<ExternalInput>,
    pub diagnostics: Vec<MetricDiagnostic>,
}

/// Abaixo deste tamanho de amostra, `Provenance::new` sinaliza baixa
/// significância estatística automaticamente (achado #15).
const LOW_SAMPLE_THRESHOLD: usize = 30;

impl Provenance {
    /// Cria a proveniência a partir do tamanho de amostra, emitindo
    /// automaticamente [`MetricDiagnostic::LowSampleSignificance`] quando
    /// `sample_size < 30` — a chamadora nunca precisa lembrar de checar isso
    /// à parte (achado #15).
    ///
    /// ```
    /// use raptor_domain::metric::{Provenance, MetricDiagnostic};
    ///
    /// let small = Provenance::new(5);
    /// assert_eq!(
    ///     small.diagnostics,
    ///     vec![MetricDiagnostic::LowSampleSignificance { sample_size: 5 }],
    /// );
    ///
    /// let large = Provenance::new(365);
    /// assert!(large.diagnostics.is_empty());
    /// ```
    pub fn new(sample_size: usize) -> Self {
        let mut diagnostics = Vec::new();
        if sample_size < LOW_SAMPLE_THRESHOLD {
            diagnostics.push(MetricDiagnostic::LowSampleSignificance { sample_size });
        }
        Self {
            sample_size,
            external_inputs: Vec::new(),
            diagnostics,
        }
    }

    /// Registra uma entrada externa consumida pela métrica, retornando
    /// `self` para encadear na construção.
    pub fn with_external_input(mut self, input: ExternalInput) -> Self {
        self.external_inputs.push(input);
        self
    }
}

/// Entrada externa (não derivada dos dados) que uma métrica consumiu —
/// registrada na proveniência para nunca ser "usada e descartada"
/// silenciosamente (achado #14). Novas variantes entram conforme as
/// métricas que as consomem forem implementadas (`risk`, `trend`, etc.).
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalInput {
    /// Taxa livre de risco usada em métricas de retorno/risco (ex.: Sharpe).
    RiskFreeRate(Decimal),
    /// Nível de confiança usado em métricas de risco (ex.: VaR).
    ConfidenceLevel(Decimal),
    /// Entrada externa nomeada sem variante dedicada ainda.
    Named { name: &'static str, value: String },
}

/// Diagnostic emitido automaticamente sobre a confiabilidade estatística de
/// uma métrica.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricDiagnostic {
    /// Amostra pequena (`n < 30`) — resultado estatisticamente frágil.
    LowSampleSignificance { sample_size: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amostra_pequena_gera_diagnostic_automatico() {
        let provenance = Provenance::new(10);
        assert_eq!(provenance.sample_size, 10);
        assert_eq!(
            provenance.diagnostics,
            vec![MetricDiagnostic::LowSampleSignificance { sample_size: 10 }]
        );
    }

    #[test]
    fn amostra_no_limiar_nao_gera_diagnostic() {
        assert!(Provenance::new(30).diagnostics.is_empty());
    }

    #[test]
    fn amostra_grande_nao_gera_diagnostic() {
        assert!(Provenance::new(365).diagnostics.is_empty());
    }

    #[test]
    fn with_external_input_acumula_entradas() {
        let provenance = Provenance::new(100)
            .with_external_input(ExternalInput::RiskFreeRate(Decimal::new(5, 2)))
            .with_external_input(ExternalInput::ConfidenceLevel(Decimal::new(95, 2)));

        assert_eq!(provenance.external_inputs.len(), 2);
        assert_eq!(
            provenance.external_inputs[0],
            ExternalInput::RiskFreeRate(Decimal::new(5, 2))
        );
    }

    #[test]
    fn metric_carrega_valor_e_provenance_juntos() {
        let m = Metric::new(Decimal::new(150, 2), Provenance::new(40));
        assert_eq!(m.value, Decimal::new(150, 2));
        assert_eq!(m.provenance.sample_size, 40);
    }
}
