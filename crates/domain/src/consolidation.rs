//! Consolidação multi-fonte (ADR-04, §11): combinar N reimportações de
//! extratos/faturas — o banco nunca entrega o histórico completo de uma vez
//! (janela de 60 dias) — numa única série ordenada cronologicamente, sem
//! `FITID` duplicado entre fontes.
//!
//! Escopo desta versão: chave de deduplicação é `Fitid` puro (reusa
//! [`crate::dedupe::dedupe_by_fitid`]), sem ainda escopar por conta
//! (`(AccountId, Fitid)`) nem distinguir cópia idêntica de conflito real de
//! payload — essa evolução (achados #65/#53/#77) é escopo da issue #14
//! (Milestone 3), que depende deste código já estar no repositório.
//! Partição por moeda (achado #51) também fica para essa issue.

use chrono::NaiveDate;

use crate::dedupe::{dedupe_by_fitid, DedupeOutcome, Fitid};

/// Rótulo identificando de qual fonte (arquivo importado) um evento veio —
/// alimenta `ExternalInput::ConsolidatedFrom` quando uma métrica de
/// portfólio é calculada sobre o resultado (achado #14 de `metric`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceLabel(String);

impl SourceLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SourceLabel {
    fn from(value: &str) -> Self {
        SourceLabel::new(value)
    }
}

/// Resultado da consolidação: série ordenada cronologicamente, contagem de
/// duplicatas removidas e as fontes combinadas.
///
/// É exatamente o objeto que a fronteira wasm serializa como `Portfolio`
/// (ADR-13) — mesma estrutura, dois nomes por camada (Rust/JS).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsolidationOutcome<T> {
    pub events: Vec<T>,
    pub duplicates_removed: usize,
    pub sources: Vec<SourceLabel>,
}

/// Combina múltiplas fontes numa única série ordenada por data crescente,
/// removendo `FITID` duplicado entre elas.
///
/// Regra de precedência: em `FITID` repetido, vence a ocorrência de **data
/// mais antiga** (é o registro original; a reimportação posterior é a
/// cópia — ver [`dedupe_by_fitid`]). Empate de data desempata pela ordem de
/// `sources`: a primeira fonte da lista vence, então passar a fonte mais
/// confiável primeiro importa nesse caso de borda.
///
/// A saída sai ordenada por data crescente — pronta para analytics de
/// janela (TWR, MDD, σ dependem dessa ordem) sem o chamador reordenar.
///
/// ```
/// use raptor_domain::consolidation::{consolidate_by_fitid, SourceLabel};
/// use raptor_domain::dedupe::Fitid;
/// use chrono::NaiveDate;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct Event { fitid: &'static str, date: NaiveDate }
///
/// fn d(y: i32, m: u32, day: u32) -> NaiveDate { NaiveDate::from_ymd_opt(y, m, day).unwrap() }
///
/// let fatura_janeiro = vec![
///     Event { fitid: "A1", date: d(2026, 1, 5) },
///     Event { fitid: "A2", date: d(2026, 1, 20) },
/// ];
/// let fatura_fevereiro = vec![
///     Event { fitid: "A2", date: d(2026, 1, 20) }, // reimportado, mesma transação
///     Event { fitid: "A3", date: d(2026, 2, 3) },
/// ];
///
/// let outcome = consolidate_by_fitid(
///     vec![
///         (SourceLabel::from("fatura-janeiro.ofx"), fatura_janeiro),
///         (SourceLabel::from("fatura-fevereiro.ofx"), fatura_fevereiro),
///     ],
///     |e| Fitid::from(e.fitid),
///     |e| e.date,
/// );
///
/// assert_eq!(outcome.duplicates_removed, 1);
/// assert_eq!(outcome.events.len(), 3);
/// assert_eq!(outcome.events.iter().map(|e| e.fitid).collect::<Vec<_>>(), vec!["A1", "A2", "A3"]);
/// ```
pub fn consolidate_by_fitid<T, F, D>(
    sources: Vec<(SourceLabel, Vec<T>)>,
    fitid_of: F,
    date_of: D,
) -> ConsolidationOutcome<T>
where
    F: Fn(&T) -> Fitid,
    D: Fn(&T) -> NaiveDate,
{
    let source_labels: Vec<SourceLabel> = sources.iter().map(|(label, _)| label.clone()).collect();
    let combined: Vec<T> = sources.into_iter().flat_map(|(_, events)| events).collect();

    let DedupeOutcome {
        mut events,
        duplicates_removed,
    } = dedupe_by_fitid(combined, |e| fitid_of(e), |e| date_of(e));

    // Sort estável: empate de data preserva a ordem de `sources` herdada do
    // `flat_map` acima.
    events.sort_by_key(|e| date_of(e));

    ConsolidationOutcome {
        events,
        duplicates_removed,
        sources: source_labels,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Event {
        fitid: &'static str,
        date: NaiveDate,
    }

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn fonte_unica_sem_duplicata_so_ordena() {
        let source = vec![
            Event {
                fitid: "A2",
                date: d(2026, 1, 20),
            },
            Event {
                fitid: "A1",
                date: d(2026, 1, 5),
            },
        ];
        let outcome = consolidate_by_fitid(
            vec![(SourceLabel::from("fatura.ofx"), source)],
            |e| Fitid::from(e.fitid),
            |e| e.date,
        );
        assert_eq!(outcome.duplicates_removed, 0);
        assert_eq!(
            outcome.events.iter().map(|e| e.fitid).collect::<Vec<_>>(),
            vec!["A1", "A2"]
        );
    }

    #[test]
    fn fitid_repetido_entre_fontes_deduplica() {
        let jan = vec![Event {
            fitid: "A1",
            date: d(2026, 1, 5),
        }];
        let fev = vec![Event {
            fitid: "A1",
            date: d(2026, 1, 5),
        }];
        let outcome = consolidate_by_fitid(
            vec![
                (SourceLabel::from("jan.ofx"), jan),
                (SourceLabel::from("fev.ofx"), fev),
            ],
            |e| Fitid::from(e.fitid),
            |e| e.date,
        );
        assert_eq!(outcome.duplicates_removed, 1);
        assert_eq!(outcome.events.len(), 1);
    }

    #[test]
    fn saida_ordenada_cronologicamente() {
        let jan = vec![Event {
            fitid: "A1",
            date: d(2026, 1, 25),
        }];
        let fev = vec![Event {
            fitid: "A2",
            date: d(2026, 2, 1),
        }];
        let outcome = consolidate_by_fitid(
            vec![
                (SourceLabel::from("jan.ofx"), fev), // fonte "jan" carrega o evento de fevereiro de propósito
                (SourceLabel::from("fev.ofx"), jan),
            ],
            |e| Fitid::from(e.fitid),
            |e| e.date,
        );
        assert_eq!(
            outcome.events.iter().map(|e| e.date).collect::<Vec<_>>(),
            vec![d(2026, 1, 25), d(2026, 2, 1)]
        );
    }

    #[test]
    fn empate_de_data_desempata_pela_ordem_das_fontes() {
        let confiavel = vec![Event {
            fitid: "A1",
            date: d(2026, 1, 10),
        }];
        let menos_confiavel = vec![Event {
            fitid: "A2",
            date: d(2026, 1, 10),
        }];
        let outcome = consolidate_by_fitid(
            vec![
                (SourceLabel::from("confiavel.ofx"), confiavel),
                (SourceLabel::from("menos-confiavel.ofx"), menos_confiavel),
            ],
            |e| Fitid::from(e.fitid),
            |e| e.date,
        );
        // Mesma data para A1/A2 — a fonte passada primeiro vence o empate.
        assert_eq!(
            outcome.events.iter().map(|e| e.fitid).collect::<Vec<_>>(),
            vec!["A1", "A2"]
        );
    }

    #[test]
    fn registra_todas_as_fontes_mesmo_sem_duplicata() {
        let outcome = consolidate_by_fitid(
            vec![
                (
                    SourceLabel::from("a.ofx"),
                    vec![Event {
                        fitid: "A1",
                        date: d(2026, 1, 1),
                    }],
                ),
                (
                    SourceLabel::from("b.ofx"),
                    vec![Event {
                        fitid: "B1",
                        date: d(2026, 1, 2),
                    }],
                ),
            ],
            |e| Fitid::from(e.fitid),
            |e| e.date,
        );
        assert_eq!(
            outcome.sources,
            vec![SourceLabel::from("a.ofx"), SourceLabel::from("b.ofx")]
        );
    }

    #[test]
    fn nenhuma_fonte_produz_saida_vazia() {
        let outcome: ConsolidationOutcome<Event> =
            consolidate_by_fitid(Vec::new(), |e| Fitid::from(e.fitid), |e| e.date);
        assert!(outcome.events.is_empty());
        assert_eq!(outcome.duplicates_removed, 0);
        assert!(outcome.sources.is_empty());
    }
}
