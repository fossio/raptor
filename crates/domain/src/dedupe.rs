//! Deduplicação por FITID (achado #10): reimportar extratos sobrepostos
//! pode repetir `FITID` — a mesma transação aparecendo em duas exportações
//! porque a janela de 60 dias que o banco impõe se sobrepõe à anterior.
//!
//! `Transaction`/`Account` ainda não existem em código (Milestone 0, issue
//! #40), então `dedupe_by_fitid` opera sobre qualquer `T` via extratores —
//! nenhuma dependência do modelo de domínio completo.
//!
//! Escopo desta versão: chave de deduplicação é `Fitid` puro. A evolução
//! para chave `(AccountId, Fitid)` e para diferenciar cópia idêntica de
//! conflito real de payload (achados #65/#53) é escopo da issue #14
//! (Milestone 3, "Consolidação multi-fonte"), que depende deste código já
//! estar no repositório.

use chrono::NaiveDate;
use std::collections::HashMap;

/// Identificador único de transação dentro do escopo de uma conta emissora
/// (`<FITID>` do spec OFX).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fitid(String);

impl Fitid {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Fitid {
    fn from(value: &str) -> Self {
        Fitid::new(value)
    }
}

/// Resultado de uma deduplicação: os eventos sobreviventes e quantos foram
/// descartados por repetição de `FITID`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DedupeOutcome<T> {
    pub events: Vec<T>,
    pub duplicates_removed: usize,
}

/// Remove entradas com `FITID` repetido, mantendo a ocorrência de **data
/// mais antiga** — é o registro original; a reimportação posterior é a
/// cópia. Empate de data mantém a primeira ocorrência encontrada.
///
/// A ordem relativa das entradas sobreviventes é preservada (primeira
/// ocorrência de cada `FITID`, na posição em que apareceu) — não faz parte
/// desta função ordenar cronologicamente a série inteira; isso é
/// responsabilidade de [`crate::consolidation::consolidate_by_fitid`].
///
/// ```
/// use raptor_domain::dedupe::{dedupe_by_fitid, Fitid};
/// use chrono::NaiveDate;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct Event { fitid: &'static str, date: NaiveDate }
///
/// let events = vec![
///     Event { fitid: "A1", date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap() },
///     Event { fitid: "A1", date: NaiveDate::from_ymd_opt(2026, 1, 5).unwrap() }, // reimportação, mesma transação
///     Event { fitid: "A2", date: NaiveDate::from_ymd_opt(2026, 1, 12).unwrap() },
/// ];
///
/// let outcome = dedupe_by_fitid(events, |e| Fitid::from(e.fitid), |e| e.date);
/// assert_eq!(outcome.duplicates_removed, 1);
/// assert_eq!(outcome.events.len(), 2);
/// assert_eq!(outcome.events[0].date, NaiveDate::from_ymd_opt(2026, 1, 5).unwrap());
/// ```
pub fn dedupe_by_fitid<T, F, D>(events: Vec<T>, fitid_of: F, date_of: D) -> DedupeOutcome<T>
where
    F: Fn(&T) -> Fitid,
    D: Fn(&T) -> NaiveDate,
{
    let mut kept: Vec<T> = Vec::new();
    let mut index_of: HashMap<Fitid, usize> = HashMap::new();
    let mut duplicates_removed = 0usize;

    for event in events {
        let fitid = fitid_of(&event);
        match index_of.get(&fitid) {
            None => {
                index_of.insert(fitid, kept.len());
                kept.push(event);
            }
            Some(&idx) => {
                if date_of(&event) < date_of(&kept[idx]) {
                    kept[idx] = event;
                }
                duplicates_removed += 1;
            }
        }
    }

    DedupeOutcome {
        events: kept,
        duplicates_removed,
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

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn sem_repeticao_preserva_todos() {
        let events = vec![
            Event {
                fitid: "A1",
                date: date(2026, 1, 1),
            },
            Event {
                fitid: "A2",
                date: date(2026, 1, 2),
            },
        ];
        let outcome = dedupe_by_fitid(events, |e| Fitid::from(e.fitid), |e| e.date);
        assert_eq!(outcome.duplicates_removed, 0);
        assert_eq!(outcome.events.len(), 2);
    }

    #[test]
    fn fitid_repetido_mantem_ocorrencia_mais_antiga() {
        let events = vec![
            Event {
                fitid: "A1",
                date: date(2026, 1, 10),
            },
            Event {
                fitid: "A1",
                date: date(2026, 1, 5),
            },
        ];
        let outcome = dedupe_by_fitid(events, |e| Fitid::from(e.fitid), |e| e.date);
        assert_eq!(outcome.duplicates_removed, 1);
        assert_eq!(
            outcome.events,
            vec![Event {
                fitid: "A1",
                date: date(2026, 1, 5)
            }]
        );
    }

    #[test]
    fn fitid_repetido_com_nova_ocorrencia_mais_antiga_depois() {
        // A ocorrência mais antiga aparece DEPOIS na sequência de entrada —
        // ainda assim deve prevalecer.
        let events = vec![
            Event {
                fitid: "A1",
                date: date(2026, 1, 5),
            },
            Event {
                fitid: "A1",
                date: date(2026, 1, 1),
            },
        ];
        let outcome = dedupe_by_fitid(events, |e| Fitid::from(e.fitid), |e| e.date);
        assert_eq!(
            outcome.events,
            vec![Event {
                fitid: "A1",
                date: date(2026, 1, 1)
            }]
        );
    }

    #[test]
    fn empate_de_data_mantem_primeira_ocorrencia() {
        let events = vec![
            Event {
                fitid: "A1",
                date: date(2026, 1, 1),
            },
            Event {
                fitid: "A1",
                date: date(2026, 1, 1),
            },
        ];
        let outcome = dedupe_by_fitid(events, |e| Fitid::from(e.fitid), |e| e.date);
        assert_eq!(outcome.duplicates_removed, 1);
        assert_eq!(outcome.events.len(), 1);
    }

    #[test]
    fn tres_ocorrencias_do_mesmo_fitid_conta_duas_duplicatas() {
        let events = vec![
            Event {
                fitid: "A1",
                date: date(2026, 1, 10),
            },
            Event {
                fitid: "A1",
                date: date(2026, 1, 5),
            },
            Event {
                fitid: "A1",
                date: date(2026, 1, 20),
            },
        ];
        let outcome = dedupe_by_fitid(events, |e| Fitid::from(e.fitid), |e| e.date);
        assert_eq!(outcome.duplicates_removed, 2);
        assert_eq!(
            outcome.events,
            vec![Event {
                fitid: "A1",
                date: date(2026, 1, 5)
            }]
        );
    }

    #[test]
    fn lista_vazia_nao_gera_duplicatas() {
        let outcome: DedupeOutcome<Event> =
            dedupe_by_fitid(Vec::new(), |e| Fitid::from(e.fitid), |e| e.date);
        assert_eq!(outcome.duplicates_removed, 0);
        assert!(outcome.events.is_empty());
    }
}
