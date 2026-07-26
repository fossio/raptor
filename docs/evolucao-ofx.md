# Evolução — Visão de longo prazo do projeto ofx

| Campo | Valor |
|---|---|
| Data | 2026-07-21 (revisado — Open Finance deprioritizado, correção de referência a licença) |
| Natureza | **Visão proposta, não decisão.** Nenhum item aqui é vinculante até virar ADR no Discovery. O §10 do Discovery é o roadmap tático (Fases −1 a 6); este documento é o macro pós-Fase 6 |
| Método | Análise da intenção do projeto + pesquisa de mercado (jul/2026) sobre bibliotecas OFX concorrentes e ferramentas que importam o formato; benchmark funcional por documentação pública; benchmark de performance fica como metodologia proposta (§6) — medir sem harness seria número inventado |
| Relação com invariantes | Todo item de horizonte referencia as invariantes do Discovery que preserva — a visão não pode contradizer o que o projeto é |

---

## 1. Análise da intenção do projeto

O Discovery declara o *quê* (parsing multi-versão + 20 métricas + WASM offline). A intenção por trás tem três camadas, e a ordem importa porque cada camada restringe a de cima:

**Camada de superfície — o problema técnico.** Parsear todas as versões de um protocolo hostil (SGML 1.x sem fechamento de tags, dialetos por emissor, charsets legados) com resiliência de nível industrial. Esse problema já foi atacado várias vezes pelo ecossistema — é resolvível, e a existência de LibOFX/ofxtools/ofxgo prova que sozinho ele não diferencia ninguém.

**Camada do meio — a lacuna estrutural do mercado.** O levantamento do §2 mostra um padrão sem exceção: **toda biblioteca OFX existente para no parsing**. Nenhuma entrega uma única métrica financeira. E o levantamento do §3 mostra o espelho: **toda ferramenta que entrega analytics exige que o usuário confie seus dados a um aplicativo** — SaaS (Mobills, Organizze), servidor self-hosted (Firefly III) ou desktop com banco próprio (GnuCash). Não existe hoje o quadrante "analytics de nível de ferramenta com confiança de nível de função local". O ofx é a única proposta nesse quadrante: a inteligência das ferramentas, empacotada como biblioteca pura que roda no navegador de quem tem o arquivo.

**Camada de fundo — a tese.** Privacidade não como feature, mas como restrição arquitetural (ADR-09) é uma posição sobre soberania de dados financeiros: o extrato é do usuário, a análise deve acontecer onde o extrato está, e a garantia deve ser estrutural (sem dependência de rede) em vez de contratual (política de privacidade). Isso é coerente com todo o resto do perfil do projeto — MPL-2.0 (copyleft por arquivo, decidido após revisão do atrito de linkagem estática — achado #58/A#13), convenções GNOME, corpus sintético por respeito a dado de pessoa identificável, proveniência em cada métrica. A tese em uma frase: **"seu extrato nunca sai da sua máquina, e ainda assim você tem análise melhor do que a de quem pede seus dados."**

Consequência prática para este documento: qualquer item de evolução que exija rede, servidor ou estado persistente na lib não é evolução — é outro projeto. O anti-roadmap (§5) existe para isso.

---

## 2. Benchmark de bibliotecas OFX concorrentes

Benchmark **funcional**, por documentação pública em jul/2026. Performance/binário exigem harness (§6).

| Biblioteca | Linguagem | SGML 1.x | XML 2.x | Message Sets | Resiliência a dialeto | Escreve OFX | Client Direct Connect | WASM/browser | Analytics | Precisão monetária |
|---|---|---|---|---|---|---|---|---|---|---|
| **LibOFX** | C/C++ | ✔ | ✔ | Banking, CC, Investment | Madura (décadas de arquivos reais via GnuCash) | Parcial (requests) | Via consumidores | ✖ | ✖ | Double (float) |
| **ofxtools** | Python | ✔ (1.6) | ✔ (2.03) | Completo incl. Investment (foco declarado) e produção de OFX; Taxes no to-do | Alta — mantenedor reporta 10+ anos de arquivos reais no parser | ✔ | ✔ (`ofxget`) | ✖ | ✖ | `Decimal` |
| **ofxparse** | Python | ✔ | ✔ | Bank, CC, Investment | Baseada em amostras contribuídas; manutenção esparsa | ✖ | ✖ | ✖ | ✖ | `Decimal` |
| **ofxgo** | Go | ✔ | ✔ | Amplo (não completo, por admissão do autor) | Média; foco em request+parse | ✔ (requests) | ✔ | Possível (tinygo), não é alvo | ✖ | Tipos próprios |
| **goofx** | Go | ✔ (recupera tags faltantes) | ✔ | Básico | **É o produto**: existe para consertar arquivos que violam o spec | ✖ | ✖ | ✖ | ✖ | — |
| **ofx-rs** (crate 2026) | Rust | ✔ | ✔ | Tipado, entry point único | Declarada ("previne bugs de dado financeiro em compile time"); puro, sem I/O, sem unsafe | ✖ | ✖ | Não é alvo declarado | ✖ | Tipada |
| **ofx (este projeto)** | Rust | ✔ (planejado, Fase 1) | ✔ (planejado, Fase 0/1) | 5 Message Sets incl. Bill Pay e Taxes (ADR-04) | Diagnostics estruturados + corpus por dialeto + parsing parcial (ADR-08) | ✖ (candidato H3) | ✖ **por design** (ADR-09) | **✔ alvo primário** | **✔ — o diferencial** | `Decimal` + moeda obrigatória |

### Leitura do benchmark

1. **A coluna "Analytics" é unânime em ✖.** É a evidência empírica da lacuna do §1 — parsing é onde todo o ecossistema para. O diferencial do ofx não é disputável nessa coluna hoje.
2. **A coluna WASM também.** Nenhuma trata browser como alvo primário; a combinação analytics+WASM+offline não tem ocupante.
3. **O surgimento do crate `ofx-rs` (publicado ~mar/2026) muda o tabuleiro.** "Parser OFX tipado em Rust, puro, 1.x+2.x, sem I/O" deixou de ser diferencial — já existe. A consequência estratégica: o valor defensável do ofx é o **pacote** (parsing resiliente com diagnostics + analytics com proveniência + fronteira WASM + corpus de dialeto brasileiro), nunca o parser isolado. Isso reforça a priorização da ADR-13 (ledger/integrity antes de tudo) e sugere não gastar diferenciação em completude de parsing além do necessário.
4. **ofxtools é a régua de completude.** Investment message set completo, produção de OFX, décadas de arquivos reais. Quando o ofx medir cobertura de modelo, é contra ofxtools que se mede — inclusive na decisão H3 de escrever OFX (round-trip), que ofxtools prova ser viável e útil.
5. **goofx é a prova de que resiliência a dialeto é dor de mercado real** — uma biblioteca inteira existe só para fechar tags que emissores esquecem. A aposta do ofx em diagnostics estruturados + tabela de aridade + fallback heurístico (ADR-02/08) ataca a mesma dor com mais rastreabilidade.
6. **Precisão monetária separa o campo em dois.** LibOFX expõe valores em ponto flutuante; as bibliotecas modernas usam decimal. O ofx é o único com moeda obrigatória no tipo (`Money`), o que nenhum concorrente tem — invariante do achado #2 como diferencial de corretude.

---

## 3. Funcionalidades das ferramentas de mercado que importam OFX

O que os *consumidores* do formato constroem em cima dele — é o mapa do que usuários esperam fazer com um extrato importado.

| Capacidade | GnuCash | Firefly III | Actual Budget | Mobills / Organizze | MMEX / HomeBank / KMyMoney |
|---|---|---|---|---|---|
| Contabilidade de partidas dobradas | ✔ (núcleo) | ✔ (núcleo) | ✖ (envelope) | ✖ | Parcial |
| Categorização na importação | Manual + matcher | **Motor de regras** (condições compostas, auto-categoriza no import) | Regras | Manual na tela de conciliação; automática via Open Finance | Regras simples |
| Conciliação / matching de duplicata | ✔ (matcher com aprendizado, reconcilia contra saldo declarado) | ✔ | ✔ | Tela de conciliação no import OFX | ✔ |
| Detecção de transferência entre contas próprias | ✔ | ✔ | ✔ | Parcial | Parcial |
| Recorrência (assinaturas, contas fixas) | Agendadas | **Recorrentes + detecção de bills + forecast** | Agendadas | Lançamentos recorrentes | ✔ |
| Orçamento por categoria | ✔ | ✔ (com rollover e auto-budget) | **✔ núcleo** (zero-based) | ✔ (metas por categoria) | ✔ |
| Projeção / forecast de fluxo | Relatórios | ✔ (bills previstos, gráfico de forecast) | Parcial | Parcial | Parcial |
| Patrimônio líquido (net worth) em série temporal | ✔ | ✔ | ✔ | Parcial (investimentos no painel) | ✔ |
| Multi-moeda | ✔ | ✔ (orçamento só na moeda principal) | Parcial | ✖ na prática | ✔ |
| Investimentos | ✔ (via OFX Investment) | Básico | ✖ | Carteira ao lado das despesas | ✔ |
| Restrições reveladoras do import OFX | — | — | Import manual por arquivo (postura deliberada) | Não importa arquivo misto banco+cartão; janela de ~6 meses; sinal negativo obrigatório nos gastos; parcelas de fatura não importáveis por planilha | — |

### Três leituras que dirigem o roadmap

**A hierarquia de valor percebido é: categorizar → conciliar → recorrência → orçamento → projetar.** Categorização é a feature nº 1 de todo app brasileiro de finanças — é a primeira coisa que Mobills e Organizze pedem na tela de conciliação do OFX importado. O ofx hoje não tem nada nessa camada; `group_by_payee` é o embrião, não o órgão.

**As visões da ADR-13 são retrovisor; o mercado maduro vende para-brisa.** Tudo que o ofx entrega hoje responde "o que aconteceu" (total, evolução, anomalias, saúde do arquivo). Firefly III e GnuCash respondem também "o que vem" (bills previstos, forecast, orçamento contra realizado). A metade preditiva do ofx (`BR`/`runway`) é o único habitante desse lado — e é exatamente o lado onde a persona leiga sente valor recorrente (voltar toda semana, não só no dia que importa a pilha).

**As restrições do Mobills são um mapa de dores não resolvidas** — e cada uma é uma oportunidade direta do ofx: arquivo misto banco+cartão rejeitado (o ofx aceita — multi-conta é lista plana da ADR-04); janela de 6 meses (o ofx não tem limite — consolidação histórica é o caso de uso central); sinal sem `-` quebra o import (é literalmente o achado #72 — normalização com diagnostic); parcelas não importáveis (achado #76). O corpus de dialeto brasileiro do ofx pode nascer dessas dores documentadas publicamente.

---

## 4. Roadmap macro por horizontes

Cada item: o que é, racional, pré-requisito e a invariante que preserva. Regra de admissão em todos os horizontes — o **teste da função pura**: se a capacidade não puder ser expressa como `fn(dados, parâmetros_externos) -> Metric/Report` com proveniência, ela não pertence à lib (pertence à borda, ao consumidor, ou ao anti-roadmap).

### H0 — Consolidação da base (pós-Fase 6, caminho até 1.0)

**Itens:** aplicar as rodadas de auditoria pendentes (achados #48–#85); corpus por dialeto dos principais emissores brasileiros, semeado pelas dores públicas do §3 (sinal sem `-`, arquivos mistos, parcelamento); paridade sgml/xml (#55); parcelamento como campo estruturado (#76); separação estrutural do Discovery.

**Racional:** analytics sobre parse não confiável é castelo na areia — e o §2 mostra que resiliência a dialeto é onde bibliotecas vivem ou morrem (goofx existe só para isso; ofxparse pede amostras da comunidade; ofxtools cita 10+ anos de arquivos reais como credencial). A credencial equivalente do ofx será o corpus brasileiro sintético por dialeto — nenhum concorrente tem foco BR.

**Invariantes:** todas — H0 é literalmente o fechamento delas.

### H1 — Paridade contábil local ("completar o retrovisor")

**Itens, em ordem:**

1. **Motor de categorização por regras** (`rules`, módulo novo). Regras declarativas do usuário (payee normalizado contém X, valor entre A e B, `TRNTYPE` = Y → categoria Z), avaliadas como função pura sobre a série; o *conjunto de regras* é dado do consumidor (JSON via fronteira), a *avaliação* é da lib. Racional: é a capacidade nº 1 do §3, e regras — não ML — preservam determinismo (§3 do Discovery) e proveniência ("categorizado pela regra R" é auditável; "o modelo achou" não é). Firefly III prova que motor de regras com condições compostas cobre a esmagadora maioria dos casos reais.
2. **Detecção de transferência entre contas próprias.** Par débito/crédito de mesmo valor, datas próximas, contas distintas do mesmo `Portfolio` → marcado como transferência interna, excluído de "gasto" no consolidado. Racional: sem isso, o escopo consolidado da ADR-13 conta o pagamento da fatura duas vezes (saída na conta + entrada no cartão) — é a extensão natural do achado #52 para multi-conta, e toda ferramenta do §3 tem.
3. **Detecção de recorrência** (`recurrence`, família de `analytics`). Payee normalizado + valor estável (tolerância) + cadência regular (mensal/semanal) → assinaturas e contas fixas identificadas estatisticamente, com confiança na proveniência. Racional: pré-requisito de todo o H2; puramente estatístico, zero ML, cabe no padrão `Metric`/`Provenance` existente.
4. **Patrimônio em série temporal** (`networth`). Saldos reconstruídos (#69) + posições de investimento avaliadas → série de patrimônio. Racional: presente em todas as ferramentas maduras do §3; no ofx é composição de peças já planejadas (ledger + Investments do ADR-04).

**Invariantes:** pureza (tudo é `fn(série, params)`), proveniência, `Money` com moeda (categorias somam por moeda, #51), storage sempre do consumidor (regras entram pela fronteira como parâmetro — nenhuma persistência na lib, coerente com Hexagonal §5).

### H2 — Inteligência local ("construir o para-brisa")

**Itens:**

1. **Projeção de fluxo e de fatura.** Recorrências do H1 + parcelas em andamento (#76) → forecast da próxima fatura e do saldo em N dias, **como intervalo com premissas na proveniência**, nunca ponto seco. Racional: é a resposta local-first ao forecast do Firefly III; parcelamento dá ao ofx uma vantagem estrutural no caso brasileiro — parcelas futuras são o componente *determinístico* do forecast de fatura, e nenhuma ferramenta do §3 explora isso bem.
2. **Orçamento como avaliação.** O envelope (categoria → teto mensal) é dado do consumidor; a lib avalia realizado vs. teto, ritmo de consumo do envelope no mês e projeção de estouro (com a recorrência do H1). Racional: mesmo padrão da categorização — a lib nunca guarda o orçamento, só o avalia; é o que permite a Actual Budget-experience sem servidor.
3. **Simulação.** "E se eu cortar a categoria X / quitar o parcelamento Y" → reprojeção do fluxo. Função pura sobre série + hipótese; a hipótese vai inteira pra proveniência.
4. **Score de saúde financeira.** Composição de `integrity` (dado confiável?) + CU + BR/`runway` + estouro de orçamento num indicador para a persona leiga, com decomposição auditável (nunca número mágico).

**Invariantes:** as mesmas do H1 — o teste da função pura segura este horizonte inteiro; nenhum item exige rede, relógio além do port previsto na §5, nem estado.

### H3 — Plataforma e formatos

**Itens, condicionais e em ordem de custo/benefício:**

1. **Escrita/geração de OFX** (round-trip). Racional: ofxtools prova utilidade (testes de round-trip fortalecem o corpus; migração entre ferramentas; "exportar visão consolidada como um OFX" torna o ofx interoperável com todo o §3 como *produtor*, não só leitor). Reverte um não-objetivo atual — por isso é decisão de ADR, não continuidade.
2. **CSV/planilha como formato de entrada.** Racional: realidade brasileira — parte dos emissores não fornece OFX (limitação reconhecida pelos próprios apps do §3, que aceitam planilha como fallback). Entra como terceiro parser independente (`csv::to_document` com perfil de mapeamento por dialeto) — e é o gatilho registrado na ADR-02/§11 para reabrir a pergunta do AST neutro com 3 formatos.
3. **[Fora de escopo por ora — confirmado pelo autor, A#31/RA-31] Open Finance Brasil como formato, jamais como integração.** Análise preservada como referência caso o item seja revisitado — não faz mais parte do roadmap vivo; o projeto segue baseado 100% em OFX. Racional original, mantido para contexto: o ecossistema passou de 100 milhões de contas conectadas e o dado transacional (extrato, fatura, investimentos) flui por APIs padronizadas do BCB — no longo prazo, é o Open Finance, não o OFX, o formato dominante de dado financeiro no Brasil. A lib **não** chama API (ADR-09 é inegociável); mas um dump JSON das APIs Open Finance que o usuário obtenha por qualquer meio seria um arquivo local como outro qualquer — `openfinance::to_document` o mapearia pro mesmo `Document`. **Ressalva registrada na auditoria de solução (A#31):** essa promessa de "é só mais um formato" não estava comprovada — o `Document` é semanticamente OFX (`TRNAMT`, `FITID`, Message Sets), enquanto Open Finance tem modelo mais rico (categoria estruturada, CNPJ/CPF de contraparte, parcelamento estruturado) que provavelmente exigiria campos novos no domínio central, não só um mapeador novo. Se o item for revisitado, essa ressalva é o primeiro ponto a endereçar antes de prometer novamente que é "só um formato".
4. **FDX**, mantido como registrado no §11 do Discovery — reavaliar com sinal real de demanda.

**Invariantes:** `domain` como núcleo estável que nenhum formato novo toca (ADR-01/04 — é exatamente o cenário para o qual a arquitetura foi desenhada); privacidade estrutural (formatos entram, rede nunca).

### Sequência entre horizontes — o racional da ordem

H0 antes de tudo porque confiança no parse é multiplicador de todo o resto. H1 antes de H2 porque forecast sem recorrência detectada e sem transferências filtradas projeta ruído. H3 por último porque **formato novo multiplica o valor do que se faz com o dado** — adicionar CSV/Open Finance quando o analytics é raso multiplica pouco; quando H1/H2 existem, cada formato novo herda categorização, recorrência, forecast e orçamento de graça. É a mesma lógica que fez a ADR-13 priorizar `ledger`/`integrity` sobre `risk`/`returns`, aplicada em escala maior.

---

## 5. Anti-roadmap — o que deliberadamente não entra

| Não entra | Por quê |
|---|---|
| Sync bancário, OFX Direct Connect, chamadas a APIs Open Finance | Viola a ADR-09 (rede) — é a linha que separa biblioteca local de agregador. ofxtools/ofxgo têm client Direct Connect; o ofx **não ter** é posição, não lacuna. Quem quiser, implementa na borda e entrega bytes |
| Persistência embutida (banco, cache em disco) | Storage é do consumidor via port (§5 do Discovery); a lib com estado deixa de ser função |
| Categorização por ML embarcado | Quebra determinismo (§3), explode o orçamento de binário (ADR-07) e degrada proveniência a "porque o modelo disse". Regras declarativas primeiro; ML, se algum dia, como port externo opcional na borda — nunca dentro do core |
| UI de produto | `web/` é demo de validação; interface é dos consumidores da lib |
| Contas multiusuário, nuvem, telemetria de produto (uso, contagem de arquivos, features tocadas) — **sem exceção, nem anônima** (A#30 da auditoria de solução, fechado) | Contradizem a tese do §1 na raiz; a tese é tratada como binária — nenhum "só um ping anônimo" é aceito, porque abriria o precedente que a erodiria com o tempo |

O anti-roadmap é tão vinculante quanto o roadmap: cada item aqui é uma pressão previsível que vai aparecer ("por que não sincroniza direto?") e cuja resposta já está decidida.

---

## 6. Benchmark de performance — metodologia proposta

Números sem harness seriam inventados; o que se fixa agora é **como medir**, para a Fase 5/6 executar:

**Harness.** Corpus sintético paramétrico (1k / 10k / 100k transações × dialetos × 1.x/2.x), gerado pelo mesmo gerador do achado #4. Medições: (a) parse nativo com `criterion`, comparando no mesmo corpus contra LibOFX (via `ofxdump`), ofxtools, ofxgo e o crate `ofx-rs` — a comparação cross-language mede ordem de grandeza, não vitória por nanosegundo; (b) tamanho do `.wasm` (bruto / `wasm-opt -Oz` / gzip) com `twiggy` decompondo por crate — valida o orçamento do ADR-07 com dado real e responde a condição de revisão do ADR-03 (`rust_decimal` no binário); (c) custo do round-trip de serialização da fronteira (ADR-06) em função do tamanho do documento — é a medição que a própria ADR-06 declara como gatilho de reavaliação; (d) pico de memória linear no parse do maior arquivo do corpus (valida o `max_bytes` do achado #9 com base real em vez de chute).

**Hipóteses a validar (não promessas):** parse de fatura típica (centenas de transações) em milissegundos de dígito único nativo; `.wasm` gzip na casa de poucas centenas de KB; round-trip da fronteira imperceptível (<5 ms) no tamanho de arquivo da persona. Se qualquer hipótese falhar, ela aponta a decisão a reabrir (ADR-03, 06 ou 07) — o benchmark existe para alimentar decisões, não para marketing.

**Publicação.** Resultados versionados no repo como página de transparência, com o harness reproduzível — a credencial "medido, não afirmado" diferencia num ecossistema em que nenhuma lib do §2 publica benchmark.

---

## 7. Riscos da visão

**`ofx-rs` capturar o nicho Rust primeiro.** Mitigação: não competir em parsing — o diferencial declarado (§2) é o pacote analytics+WASM+dialeto BR, que o concorrente não sinaliza perseguir. Acompanhar o crate; se ele amadurecer muito, consumi-lo como front-end alternativo é teoricamente possível (o `domain` do ofx é o ativo, ADR-01) — opção registrada, não plano.

**OFX minguar no Brasil.** O Open Finance já é o canal dominante de dado transacional automático; apps brasileiros tratam o import de arquivo como fallback. Mitigação estrutural originalmente prevista (H3.3) está pausada por decisão do autor (A#31/RA-31, foco 100% OFX por ora) — o risco fica **reconhecido e sem mitigação ativa** enquanto isso durar, não resolvido. Contra-tendência a favor, que segue valendo independente da mitigação: o arquivo exportado manualmente continua sendo o único canal que **não exige consentimento contínuo a um agregador** — exatamente o usuário da tese do §1. Se o risco se materializar de forma perceptível (queda real de demanda por OFX), H3.3 é o primeiro item a reabrir.

**H1/H2 virarem "app disfarçado de lib".** É o risco de escopo mais provável — orçamento e categorização são features de app por instinto. Guarda-corpo: o teste da função pura do §4 aplicado item a item em revisão de ADR; qualquer item que precise guardar estado do usuário dentro da lib falha o teste e migra pra borda.

**Mantenedor único vs. ambição do roadmap.** Mitigação honesta: os horizontes são ordenados para que **cada corte seja um produto completo** — parar no H0 entrega a melhor lib de parsing BR; parar no H1 entrega o retrovisor completo; H2/H3 são upside, não dívida. Nenhum horizonte deixa o anterior pela metade.

---

## Fontes consultadas (jul/2026)

Documentação e repositórios: ofxtools (GitHub/PyPI/ReadTheDocs), ofxparse (GitHub), ofxgo (GitHub/pkg.go.dev), goofx (pkg.go.dev), crate ofx-rs (lib.rs), LibOFX via GnuCash (wiki e código do importador OFX), Firefly III (docs oficiais e comparativos 2026), Actual Budget (comparativos 2026), Mobills e Organizze (centrais de ajuda oficiais sobre importação OFX), Open Finance Brasil (portal oficial e cobertura de mercado 2026).
