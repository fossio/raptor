# Resolução da Auditoria de Solução — ofx

| Campo | Valor |
|---|---|
| Data | 2026-07-21 (concluído) |
| Documento-fonte | `auditoria-solucao.md` (achados A#1–A#33) |
| Natureza | Respostas do autor às objeções da banca, transformadas em decisão ou plano de ação; cada resolução aponta o(s) achado(s) que fecha |
| Estado | **Concluída — 33 de 33 achados resolvidos.** Última pendência (A#15, `k_d`) fechada com o handoff técnico original anexado pelo autor |
| Convenção | Cada seção: decisão tomada · o que muda no projeto · achados fechados/mitigados · o que fica em aberto |

Este documento é a contrapartida da auditoria: onde a banca levantou objeções, aqui ficam as decisões. Nem toda objeção vira mudança — algumas são respondidas com "aceito o risco, e aqui está o porquê", que é uma resolução válida desde que explícita.

---

## R1 — Identidade do projeto: tooling FLOSS para o padrão OFX, standalone por definição

**Resposta do autor (item 1):** o projeto é um projeto FLOSS que implementa os padrões OFX e facilita o tooling em torno dele — gerenciador do arquivo, das mesclas e dos cálculos matemáticos e análises financeiras, tudo standalone, sem qualquer serviço externo.

**O que isso resolve — reenquadra A#1 e A#4 (a objeção "mercado de privacidade não dimensionado").** A banca de produto tratou privacidade como *proposta de valor a validar contra um mercado*. A resposta reposiciona: privacidade/standalone não é um *pitch de marketing competindo com Mobills* — é uma **propriedade de engenharia do que um tooling FLOSS de padrão aberto deve ser**. O par correto de comparação não é Mobills/Organizze (produtos SaaS de finanças pessoais), é **LibOFX, ofxtools, ofxparse** (bibliotecas de tooling do formato). Nesse universo, "roda local, sem serviço externo" não é diferencial a vender — é o padrão da categoria, e o diferencial real é o que o benchmark já apontou: ser o único tooling da categoria que vai além do parsing e entrega os cálculos.

Consequência: **A#1 e A#4 deixam de ser riscos de produto e viram não-objetivos explícitos.** O projeto não precisa dimensionar "o mercado de quem se importa com privacidade" porque não está competindo por esse mercado — está construindo a ferramenta de referência de um padrão aberto, e a audiência primária inclui o próprio autor e a comunidade de tooling (desenvolvedores, power users, contadores, quem constrói sobre a lib), não o consumidor leigo de app de banco disputado pelo mercado SaaS.

**Isso não anula a persona leiga** — ela continua sendo o alvo da *demo de referência* e do design de UX (o handoff segue válido). O que muda é o enquadramento: a persona leiga é *um* consumidor da lib, o mais visível, mas o produto é a lib + tooling, não o app. Isso está coerente com a ADR-13, que já separa "a lib computa" de "a interface consome".

**Esclarecimento terminológico (trava para o resto dos documentos):** "analytics", em toda a extensão do Discovery e desta resolução, significa exclusivamente o módulo de **cálculos matemáticos e análises financeiras sobre a estrutura de dados do OFX** — a crate `ofx-analytics` (ledger, risk, returns, credit, anomaly, predictive, integrity). Não se refere, em nenhum momento, a telemetria de uso do serviço/lib/site. Essa segunda coisa — saber quantas pessoas usam, quais features são tocadas — é um conceito totalmente diferente, hoje vetado pelo anti-roadmap, e discutido à parte no achado A#30 da auditoria. Para não repetir a colisão de termo que já apareceu no resumo de questões em aberto (onde usei "analytics de uso" bem ao lado de "analytics" no sentido financeiro), a partir daqui A#30 passa a ser chamado por **"telemetria de produto"**, nunca "analytics", justamente para manter os dois conceitos visualmente distintos em qualquer leitura futura.

**Registrar no Discovery:** adicionar aos não-objetivos (§2) que o projeto não compete no mercado de apps SaaS de finanças pessoais; seu par de categoria é o tooling de formato (LibOFX/ofxtools), e standalone é propriedade herdada da categoria, não proposta a validar.

---

## R2 — Recorrência é objetivo de produto, não efeito colateral

**Resposta do autor (item 2):** busca-se recorrência, para o usuário acompanhar a própria evolução ao longo do tempo.

**O que isso resolve — endereça A#2 (valor recorrente só chega no H2 distante) e A#20 (entrega dado, não entendimento).** A recorrência agora é objetivo declarado, o que a promove de "horizonte distante" para **vetor de priorização do roadmap** (ver R5/R7 — a reestruturação move as capacidades de acompanhamento temporal para mais perto do início). "Acompanhar a evolução" é precisamente o que transforma dado em entendimento (A#20): a resposta à pergunta "tô gastando demais?" não é um número absoluto, é a comparação com o próprio histórico ("você gastou 18% mais que a média dos últimos 3 meses"). Isso reposiciona a `period_series` e a família `trend` (SMA/EMA) de "métricas entre outras" para **espinha dorsal da proposta de valor**.

**Tensão honesta que permanece (liga a A#25 — versionamento de dados persistidos):** acompanhar evolução ao longo do tempo exige que *algo* persista entre sessões — o histórico consolidado dos arquivos já importados. O anti-roadmap mantém persistência fora da lib (é do consumidor), mas a recorrência como objetivo torna a **estratégia de persistência do consumidor de referência** uma peça de primeira classe, não um detalhe adiável. Fica registrado para resolução na leva que tratar A#11/A#25: a demo de referência precisa de um modelo de persistência local (IndexedDB) versionado, e a lib precisa versionar o formato serializado que emite. Não muda o anti-roadmap (a lib segue sem estado); muda a prioridade de desenhar o contrato de persistência cedo.

---

## R3 — Fluxo de exportação OFX nos 5 principais bancos brasileiros + Nubank

**Resposta ao item 3.** Levantamento do caminho de exportação de extrato OFX no internet banking, com o dado operacional que impacta o produto. Fonte: documentação pública de dezenas de ferramentas de conciliação (jul/2026); caminhos de UI mudam com frequência, então o valor durável aqui são os **padrões transversais**, não o clique exato. Nubank entra à parte dos 5 tradicionais por pedido explícito — é o maior banco digital do país e, como o levantamento mostra, tem um perfil de dialeto genuinamente diferente dos bancos com agência física.

| Banco | Caminho (Internet Banking) | Formato ofertado | Janela histórica | Observação de produto |
|---|---|---|---|---|
| **Banco do Brasil** | Conta corrente → Extrato → ícone de download → "Money 2000+ (OFX)" | OFX (rótulo "Money 2000+") | Apenas os últimos 60 dias | Janela curta força exportação frequente — favorável à recorrência (R2) |
| **Bradesco** | Saldos e Extratos → Extrato Mensal/Por Período → selecionar conta → Buscar → Salvar como arquivo → "OFX (Money 2000 em diante)" | OFX (rótulo "Money 2000 em diante") | Apenas os últimos 60 dias | Exige selecionar conta explicitamente antes de exportar |
| **Itaú** | Conta corrente → Extrato/consultar por período → "Salvar em outros formatos" → "OFX / Money 2000" | OFX (rótulo "OFX/Money 2000") | ~60 dias (padrão do setor) | Passo extra de "salvar em outros formatos" esconde a opção |
| **Caixa** | Internet Banking → extrato → exportar OFX | OFX | ~60 dias | Documentação pública menos detalhada; validar no corpus |
| **Santander** | Conta Corrente → Extrato (Money) → escolher período → Exibir → Exportar → "Money 2000 ou superior" | OFX (rótulo "Money 2000+") | ~60 dias | Rótulo "Money" no menu, não "OFX" — confunde o leigo |
| **Nubank (PF)** | App Nu → menu "Faturas" → escolher período → "Exportar OFX+" (cartão); extrato de conta: site descontinuado para PF, geração fica no app, arquivo chega por e-mail cadastrado (OFX + PDF) | OFX (rótulo "OFX+") | Por período/mês escolhido, sem teto de 60 dias documentado | **Só app, sem web** — quebra o pressuposto de "baixar direto no computador"; entrega pode ser por e-mail, não download imediato; existe extensão de terceiro (Chrome) só pra exportar a fatura, sinal direto de fricção real (reforça A#3) |
| **Nubank (PJ)** | App/site PJ → Conta PJ → Extrato → Exportar → OFX | OFX (rótulo "Money 100 e 102") | Por mês; até 5 MB, ordem crescente de datas, sem lançamentos do dia (D-1 ou anteriores) | Terceiro rótulo de formato distinto no corpus ("Money 100/102"); restrições de tamanho/ordem/data são regras de dialeto a validar |

**Sete padrões transversais que viram requisito de produto** (os 5 primeiros vêm dos bancos tradicionais; 6 e 7 vêm da inclusão do Nubank):

1. **O rótulo do formato quase nunca é "OFX" — é "Money 2000+" ou "Money 2000 em diante".** Herança do Microsoft Money. A persona leiga procura "OFX" e não acha; procura "exportar" e encontra um menu que diz "Money". **Isso vira conteúdo de onboarding obrigatório (resolve parte de A#3):** o guia "como exportar do seu banco" precisa dizer "procure por *Money 2000* ou *OFX*, é a mesma coisa".

2. **Janela de 60 dias é a norma, não exceção.** BB e Bradesco confirmam explicitamente; o resto segue o padrão. **Consequência dupla:** (a) reforça a recorrência (R2) — o usuário é *obrigado* pelo banco a exportar a cada ~2 meses, o que naturaliza o hábito que o produto quer criar; (b) a consolidação multi-arquivo (ADR-04) não é feature avançada, é **necessidade estrutural** — nenhum usuário terá histórico longo num arquivo só, todos terão uma pilha de janelas de 60 dias para juntar. Isso eleva a prioridade da consolidação no roadmap.

3. **Todos exigem selecionar período/conta antes de exportar** — ou seja, o arquivo que chega ao produto é sempre um recorte, e faturas de cartão vêm separadas de extrato de conta. Coerente com o modelo multi-fonte já decidido.

4. **A fricção de exportação é real e por-banco** (Itaú esconde atrás de "outros formatos"; Santander rotula como "Money"). **Confirma A#3 como problema legítimo** e define a forma da solução: guia visual por banco, não instrução genérica.

5. **A janela de 60 dias + exportação recorrente é o mecanismo de recorrência que o produto não tinha (responde à objeção central de A#2).** A banca perguntou "por que o usuário voltaria?". Resposta parcial do próprio ecossistema: porque o banco só dá 60 dias, então a cada ~2 meses ele precisa exportar de novo — e se o produto guarda o histórico consolidado, cada nova exportação é uma visita com valor incremental (a evolução cresce). A recorrência não depende só das features de H2; ela é induzida pela limitação dos bancos, desde que o produto persista o histórico.

6. **Bancos digitais têm um perfil de dialeto qualitativamente diferente dos bancos com agência — Nubank confirma isso, não é exceção isolada.** Três diferenças estruturais: (a) **sem web para pessoa física** — a exportação depende do app, e a entrega pode ser por e-mail em vez de download direto, o que quebra o pressuposto implícito de "arrasta o arquivo que acabou de baixar" das jornadas do handoff de UX (precisa de um passo "baixe o anexo do e-mail" antes de chegar no produto); (b) **rótulo de formato ainda diferente** — nem "Money 2000" nem simplesmente "OFX": Nubank PF usa "OFX+", Nubank PJ usa "Money 100/102" — um terceiro e quarto rótulo distintos, reforçando o padrão 1 (o formato quase nunca se chama "OFX" na tela); (c) **existe uma extensão de terceiro só para resolver a exportação de fatura do Nubank** (`nubank-ofx`, Chrome), e há reclamação registrada na comunidade oficial do Nubank sobre a dificuldade de exportar fatura em OFX para PF depois que o site foi descontinuado. **É evidência direta e independente de A#3** (fricção de exportar o próprio insumo é real) — na mesma categoria do achado sobre o `ofxparse2` (R4): quando a comunidade cria ferramenta paralela só para exportar/converter, é o sinal mais forte possível de que o fluxo oficial não é suficiente.

7. **Achado técnico lateral, relevante para o achado #66:** a extensão `nubank-ofx` documenta que a fatura do Nubank pode ser exportada com a data da transação **ou** a data em que ela entrou na fatura, e que as duas diferem tipicamente em ~1 dia. Isso não é o mesmo eixo do achado #66 (UTC vs. horário local), mas é uma segunda fonte de ambiguidade de data específica de dialeto de cartão — **candidata a entrar no corpus de teste da Fase de parsing** (R4) como caso de dialeto Nubank, não como mudança na decisão já fechada do achado #66.

**Nota de escopo remanescente:** Inter, C6 e demais bancos digitais/neobanks continuam como dialetos a mapear no corpus quando a Fase de parsing chegar (R4/R5) — o padrão revelado por Nubank (app-only, rótulo variante, terceiro preenchendo lacuna) é a hipótese de trabalho razoável para eles também, a confirmar caso a caso.

---

## R4 — Resiliência a dialeto: resolver via pesquisa pública + corpus, com canal de reporte local-first

**Respostas do autor (itens 4, 8 e 9 convergem aqui):** resolver o problema do descobrir-o-desconhecido (A#8/A#19); apoiar-se em pesquisa pública profunda dos problemas conhecidos da comunidade e replicá-los em testes do corpus (item 8); e seguir a recomendação da banca sobre o loop de feedback (item 9 → A#19).

**A objeção A#8 era:** corpus sintético só codifica o que o autor já sabe; não descobre o dialeto desconhecido. **A resolução tem três camadas:**

**Camada 1 — mineração da dor pública já documentada (item 8).** A comunidade OFX documentou os modos de falha por 15+ anos; eles não são desconhecidos, estão espalhados em listas do GnuCash, issues de bibliotecas e ferramentas de validação. Pesquisa preliminar já confirma que os principais são conhecidos e recorrentes:

- **`FITID` em branco, `"unknown"` ou constante** é a dor nº 1, documentada desde 2010. A spec exige FITID único por conta, mas arquivos reais trazem muitos em branco, e a primeira transação de ID vazio toma posse do ID vazio; qualquer importação seguinte com FITID em branco é analisada como duplicata. Isso **valida empiricamente o achado #77** (qualidade degenerada de FITID) — não era hipótese, é a falha mais reportada do formato.
- **`FITID` reusado/reciclado entre exportações** quebra a reimportação: se o banco reusa IDs de transação OFX, não é possível importar mais de uma vez sem colisão. Valida a chave `(AccountId, Fitid)` do achado #65 e a precedência cronológica da consolidação.
- **Data malformada ou valor inválido faz parsers pularem transações silenciosamente**, e a discrepância só aparece na conciliação de fim de mês; **header obrigatório ausente, tag SGML não fechada ou encoding inválido abortam a importação inteira**, às vezes com erro que não identifica o campo. Valida a estratégia de parsing parcial + diagnostics (ADR-08) e o `FatalParseError` estruturado (achado #83).
- **Falta de `INTU.BID`/`ORG`** faz o Quicken rejeitar o arquivo como "download error" — dialeto conhecido de campos institucionais ausentes.
- **Bancos brasileiros especificamente:** já existe um fork do ofxparse (`ofxparse2`) criado por causa de melhor compatibilidade com bancos brasileiros, detecção automática de encoding e robustez para variações reais de OFX; trata headers malformados ou parcialmente inválidos de certos bancos brasileiros. **Isto é ouro para o corpus:** confirma que encoding e header malformado BR são dores reais e já catalogadas por alguém que viu os arquivos.

**Ação:** o corpus sintético é semeado a partir desta mineração — cada modo de falha documentado publicamente vira um gerador de dialeto com um caso de teste. A lista acima já é o backlog inicial do corpus. **Isso fecha A#8 no eixo do conhecido:** não estamos adivinhando o que os bancos erram, estamos catalogando 15 anos de reclamação pública e replicando em teste sintético — sem tocar em nenhum arquivo real.

**Camada 2 — canal de reporte local-first (item 9, resolve A#19).** Seguindo a recomendação da banca, o produto ganha um mecanismo opt-in onde, quando o parser encontra algo que não sabe tratar, o usuário pode **gerar um relatório de dialeto anonimizado** — não o arquivo, mas a estrutura do problema (a tag que quebrou, a forma da aridade inesperada, o padrão do FITID degenerado), com valores monetários e identificadores removidos localmente antes de qualquer coisa sair. O usuário revê o que será compartilhado e decide. Isso dá ao projeto o loop de feedback que a privacidade estrutural amputava (A#19), **sem furar a tese** — o que sai é metadado de estrutura anonimizado por escolha explícita, nunca o extrato. É a diferença entre telemetria automática (vetada) e um "reportar arquivo problemático" que o usuário aciona.

**Camada 3 — fuzzing como entregável (antecipa A#23).** Já que o parser é o coração do risco, fuzzing (`cargo-fuzz`) do tokenizer SGML e do reader XML entra como entregável da fase de parser, não como intenção — é o método que encontra o input que trava antes que um usuário encontre. Detalhamento na leva que fechar A#23; registrado aqui porque nasce da mesma resposta.

**O que permanece em aberto:** o desconhecido-desconhecido *genuíno* (um dialeto que nunca ninguém reportou) só aparece com uso real — a Camada 2 é a rede que o captura em produção. Aceita-se que o corpus nunca será completo; a arquitetura de parsing parcial + diagnostics (ADR-08) é o que garante que um dialeto novo degrade em aviso, não em crash.

---

## R5 + R7 — Reestruturação do roadmap em épicos/stories, guiada pelas prioridades reais

**Respostas do autor (itens 5 e 7):** reestruturar a sequência dos itens conforme as prioridades e o rumo que o produto tende a seguir (item 5), estruturando como épico → user story → tasks para que o reagrupamento faça sentido (item 7).

**O que muda em relação ao roadmap atual (§10 do Discovery):** o roadmap hoje é *horizontal por camada técnica* (primeiro todo o parsing, depois todo o analytics, depois a fronteira). A auditoria (A#29) mostrou que isso represa a demonstrabilidade — nada é usável por ninguém até a Fase 3. A reestruturação adota **fatia vertical guiada por valor**, conforme a recomendação da banca (A#29) e os novos vetores de prioridade (recorrência R2, consolidação como necessidade estrutural R3, dor de dialeto conhecida R4).

**Novo vetor de priorização, em ordem:**
1. **Demonstrabilidade cedo** (A#29) — um caminho fim-a-fim no browser o quanto antes.
2. **Recorrência** (R2) — acompanhamento temporal e consolidação como espinha, não como extra.
3. **Robustez sobre a dor conhecida** (R4) — os modos de falha catalogados tratados desde o primeiro parser.
4. **Persona leiga de cartão primeiro** (ADR-13, mantido).

### Estrutura épico → story (primeira versão para validação)

Abaixo, os épicos reordenados. Cada um traz as user stories principais; as tasks técnicas ficam para o detalhamento por épico (esta é a visão de reagrupamento que o item 7 pediu, não o backlog completo). Ordem de execução é a ordem de leitura.

**Épico 0 — Fundação verificável (was Fase −1/0)**
*Já parcialmente entregue (`ofx-domain`).*
- Como mantenedor, quero o workspace multi-crate e o domínio tipado compilando para WASM, para ter a base sobre a qual tudo assenta.
- Como mantenedor, quero medir o tamanho do `.wasm` do `domain` sozinho já aqui, para saber cedo se o orçamento de binário é viável (**antecipa A#9 — medição como gate, não como descoberta de Fase 5**).

**Épico 1 — Fatia vertical mínima: Nubank como piloto (NOVO — resposta direta a A#29; piloto trocado por decisão do autor)**
*Um banco, um dialeto, uma métrica, ponta a ponta no browser.*
- **Spike primeiro, antes de qualquer código de parser:** inspecionar um arquivo real de fatura Nubank e confirmar o header (`OFXHEADER:100/DATA:OFXSGML` → dialeto 1.x, ou `<?xml?>`/`<?OFX OFXHEADER="200"?>` → dialeto 2.x). Pesquisa pública não confirmou qual dos dois o Nubank emite — decisão de qual parser mínimo (SGML ou XML) construir primeiro depende deste spike, não é suposição.
- Como usuária, quero arrastar o OFX da minha fatura Nubank (exportado via app, menu "Faturas" → "Exportar OFX+" — R3) e ver o total gasto, para ter valor real na primeira interação usando o banco que eu de fato uso.
- Como mantenedor, quero esse caminho rodando no browser via WASM com o parser mínimo (versão confirmada pelo spike acima), para ter algo demonstrável e medível de verdade antes de investir em completude.
- Como usuária, quero que a ambiguidade de data do Nubank (data da transação vs. data de entrada na fatura, ~1 dia de diferença — R3, ponto 7) seja tratada de forma explícita nesta fatia, já que é o dialeto piloto, não adiada para o Épico 2.
- *Por que primeiro:* é o corte que ataca A#29, A#2 e A#9 de uma vez — existe produto real, medível e demonstrável antes da completude. (O canal de reporte de dialeto que endereça A#19 entra no Épico 2, junto da resiliência — este primeiro corte ainda não lida com arquivo malformado.) BB/Bradesco (candidatos originais, dialeto mais simples) passam a ser o *segundo* alvo, dentro do Épico 2, junto dos demais bancos tradicionais.

**Épico 2 — Parsing resiliente sobre a dor conhecida (was Fase 1, reordenado)**
- Como usuária de qualquer um dos 5 bancos tradicionais (BB, Bradesco, Itaú, Caixa, Santander), quero que meu arquivo seja lido mesmo com os defeitos conhecidos (FITID degenerado, encoding BR, header malformado, vírgula decimal, timezone), para não ver um erro em vez do meu extrato.
- Como usuária Nubank, quero que meu arquivo continue funcionando mesmo quando eu exportar um extrato de conta (PJ, rótulo "Money 100/102") ou uma fatura malformada — o Épico 1 cobriu meu caminho feliz de fatura PF; aqui ganha robustez a arquivo problemático, igual aos demais bancos.
- Como mantenedor, quero o corpus semeado pela mineração pública (R4) e fuzzing do parser, para que os modos de falha catalogados sejam teste, não surpresa.
- Como usuária, quero reportar um arquivo que não funcionou de forma anonimizada e opt-in (R4 camada 2), para ajudar a melhorar sem expor meus dados.
- *Inclui:* parser SGML 1.x, tabela de aridade, ambos os mapeadores, diagnostics estruturados.

**Épico 3 — Recorrência e consolidação (PROMOVIDO — era efeito de Fases 3–4)**
- Como usuária, quero juntar minhas exportações de 60 em 60 dias num histórico contínuo (consolidação, ADR-04), porque o banco nunca me dá tudo de uma vez (R3).
- Como usuária, quero ver minha evolução mês a mês e comparar com meus próprios meses anteriores (`period_series`, `trend`), para *entender* se estou gastando mais (resolve A#20).
- Como usuária, quero que o produto lembre o que já importei (persistência local versionada da demo de referência — liga R2/A#25), para não recomeçar do zero a cada visita.
- *Por que aqui:* recorrência (R2) + janela de 60 dias (R3) tornam isto o coração do valor, não um horizonte distante.

**Épico 4 — Analytics de gasto da persona leiga (was Fase 3)**
- Como usuária leiga, quero gasto por estabelecimento, ticket médio, extremos, anomalias, utilização de limite e parcelas em aberto, para entender minha fatura.
- *Inclui:* `ledger` completo, `credit`, `anomaly`, `integrity`, `predictive`.

**Épico 5 — Fronteira WASM madura e demo de produto (was Fase 5)**
- Como consumidora da lib, quero a API completa do Grupo 1–3 (ADR-13) rodando dentro de um Web Worker (**A#24, decidido — RA-24**), para uma UI que não congela mesmo em consolidação grande ou cálculo pesado.
- *Inclui:* implementar a fronteira assíncrona já especificada na ADR-06 revisada (worker + `postMessage` + RPC fino); atualizar a sintaxe dos diagramas de §7/§12 de síncrona para `await`.

**Épico 6 — Analytics avançado de investimento (was Fase 4)**
- Como usuário avançado, quero retorno e risco (TWR, IRR, Sharpe, VaR, MDD) sobre extrato de corretora, em modo separado.
- *Nota de A#21:* avaliar se este épico é um consumidor separado da lib, não um modo do mesmo app.

**Épico 7 — Hardening e lançamento público (was Fase 6)**
- Corpus por dialeto completo, otimização de binário, PWA, artefatos de comunidade (ADR-10), decisão sobre release cross-registry automatizado (**A#27**).

**Diferenças-chave vs. o roadmap antigo, e o achado que cada uma fecha:**
- Fatia vertical (Épico 1) inserida antes da completude → **A#29**.
- Recorrência/consolidação promovida de efeito tardio para Épico 3 → **A#2, A#20, R2, R3**.
- Medição de binário movida para o Épico 0 como gate → **A#9**.
- Corpus e reporte local-first embutidos no Épico 2 → **A#8, A#19, R4**.
- Fuzzing e decisão de Web Worker viram entregáveis nomeados (Épicos 2 e 5) → **A#23, A#24**.

**Nota histórica:** o detalhamento de tasks por épico ficou para o `roadmap-github.md` (já feito). Os achados A#11/A#25, A#16, A#21 e A#24, citados aqui como pendentes numa versão anterior deste parágrafo, foram todos resolvidos em rodadas posteriores (RA-11/25, RA-16, RA-21, RA-24 — ver seções próprias acima e o placar final).

---

## RA-24 — Fronteira assíncrona via Web Worker, decidida agora

**Resposta do autor:** Web Worker já no Épico 5 — a fronteira vira assíncrona, e a ADR-06 é revisada agora, não só planejada para depois.

**O que muda — fecha A#24.** O `.wasm` passa a rodar dentro de um Web Worker, nunca na main thread; toda a API do Grupo 1–3 (ADR-13) vira `Promise` do lado do consumidor. **Já apliquei a revisão na ADR-06 do Discovery** — a decisão está registrada lá com o exemplo de código (worker + `postMessage` + RPC fino), e é explícita sobre um ponto que importa: isso é **ortogonal** à decisão de "sem handle residente" (a outra metade da ADR-06) — o worker continua sem guardar `Document` vivo entre chamadas, só muda *onde* a serialização atravessa a fronteira. Nenhuma das duas decisões invalida a outra.

**O que fica pendente, registrado no próprio Discovery:** os diagramas de sequência do §7 e os sete Casos de Uso do §12 ainda mostram as chamadas como síncronas. A ordem lógica não muda com o worker — só a forma (`await`). É ajuste de sintaxe nos diagramas, não decisão em aberto; fica para uma passada cosmética futura.

---

## RA-11/25 — Versionamento de schema serializado: adiado conscientemente

**Resposta do autor (após esclarecimento):** adiar — só decidir quando alguém construir de fato um app que guarda dado entre sessões.

**O que isso fecha e o que não fecha.** A#11/A#25 perguntava: quando o `Document` ganha um campo novo (como `installment`, que acabou de acontecer), quem avisa um consumidor que persistiu dados antigos que eles ficaram desatualizados? A resposta é **C — não decidir agora**. Isso não é omissão: é reconhecer que desenhar um mecanismo de versionamento sem um consumidor real que persista dados é desenhar no escuro — a forma certa da solução (carimbo de versão no envelope? migração automática? invalidação simples?) só fica clara com um caso de uso real na mão.

**Risco aceito e documentado** — mesmo padrão do achado #66 (mantive só UTC): registrei uma linha nova na tabela de riscos do Discovery (§9) explicitando que isso é decisão consciente de adiar, não lacuna esquecida, com a condição de reabertura amarrada ao Épico 5 (quando a demo de referência de fato persistir algo entre sessões).

**Nada muda no anti-roadmap:** a lib continua sem estado; o que fica em aberto é só se a lib vai ou não *ajudar* o consumidor com um carimbo de versão — pergunta adiada, não respondida com "nunca".

---

## RA-16 — Parser próprio, não `ofx-rs`

**Resposta do autor:** manter parser próprio (ADR-02) — controle total sobre resiliência a dialeto é o diferencial declarado.

**Fecha A#16 sem ambiguidade.** Nenhuma dependência de terceiro para parsing; a ADR-02 (dois parsers independentes, `sgml`/`xml`, sem AST neutro) permanece exatamente como estava — não precisa de nenhuma edição no Discovery.

**Racional que confirma a decisão, amarrando com R1 e R4:** depender de `ofx-rs` significaria herdar as decisões de resiliência de outro projeto sobre exatamente os problemas que R4 identificou como a dor real do formato (FITID degenerado, encoding BR, aridade SGML malformada). Um projeto cujo diferencial declarado (R1) é ser o tooling de referência do formato — não um consumidor de tooling alheio — não terceiriza a parte que mais precisa controlar. O esforço do Épico 2 continua maior do que seria consumindo terceiro; isso é aceito como preço do controle, não como custo escondido.

---

## RA-7 — Mantenedor único vs. escopo do roadmap: risco aceito conscientemente

**Resposta do autor:** aceitar o risco conscientemente — é projeto pessoal/FLOSS, sem pressão de prazo, nenhuma mudança de escopo.

**O que isso fecha e o que não fecha.** A#7 era o achado crítico de execução: 8 milestones, dezenas de issues, um mantenedor. A resposta não muda uma vírgula do roadmap — nem corta escopo, nem busca contribuidor, nem impõe cadência formal. É a mesma classe de resolução do achado #66 (manter status quo, risco documentado, não mitigado): o roadmap continua ambicioso como está, e o risco de estagnar num milestone no meio do caminho fica **nomeado e aceito**, não escondido atrás de otimismo de planejamento.

**Coerência com R1:** faz sentido com o reenquadramento já feito — sendo tooling FLOSS sem pressão de tração (R1) nem métrica de sucesso definida (R6), não há prazo externo para o risco de execução ameaçar. O único custo real de aceitar sem mitigar é que, se o projeto de fato estagnar num milestone, não haverá rede de segurança (comunidade, cadência, escopo reduzido) preparada de antemão — e isso é precisamente o trade-off que o autor escolheu.

**Registrado no Discovery** (§9, tabela de riscos) no mesmo padrão do achado #66 e do A#11/A#25 — risco nomeado, mitigação "nenhuma por escolha", condição de reabertura em aberto (não há gatilho definido, porque não há mudança de escopo condicionada a nada).

---

## RA-10 — Fronteira sem handle mantida para todos os escopos; cache é problema do consumidor

**Resposta do autor:** manter sem handle pra todo mundo — se o modo avançado doer, quem resolve é cache do lado do consumidor (JS), não a lib.

**Fecha A#10.** A ADR-06 (documento serializado por chamada, sem handle residente) não ganha uma exceção para o modo avançado de investimento — vale igual para os dois escopos (persona leiga e avançada). Se uma consolidação grande de séries de preço tornar a reserialização repetida perceptível, a responsabilidade de mitigar (cache, memoização do lado JS, evitar rechamadas desnecessárias) é do consumidor que constrói o Milestone 6 — nunca da lib.

**Racional:** mantém a simplicidade de consumo que a ADR-06 já buscava (zero gerência de ciclo de vida, nenhum `free()` a lembrar) como propriedade universal da fronteira, em vez de bifurcar a API em "modo simples" e "modo com handle" — o que criaria dois contratos para o mesmo tipo de dado e duplicaria a superfície de manutenção. **Continua valendo a condição de reabertura já registrada na ADR-06:** se o binário real (Fase de fronteira madura) mostrar que o custo é inaceitável mesmo com cache do consumidor, a decisão volta à mesa — mas o ônus da prova é de um caso medido, não hipotético.

**Ação prática para o Milestone 6:** a issue de "modo avançado" (`roadmap-github.md`, `#6.1`/`#6.2`) ganha uma nota de que cache de série consolidada é responsabilidade do app consumidor, não da lib — evita que alguém "resolva" isso reabrindo a ADR-06 sem necessidade real.

---

## RA-12/14 — Golden values contra biblioteca madura (numpy-financial / LibreOffice)

**Resposta do autor:** comparar contra outra biblioteca madura (numpy-financial, Excel/LibreOffice) para as métricas padrão de mercado (IRR, TWR).

**Fecha A#12 e A#14 juntos — eram duas faces do mesmo problema.** A#12 perguntava de onde vêm os valores de referência dos testes fortes (§8 do Discovery); A#14 apontava que, sem fonte externa, o CI só prova consistência interna, não correção matemática. A resposta resolve as duas: métricas com definição de mercado amplamente implementada (IRR/XIRR, TWR, e por extensão Sharpe/CAGR onde aplicável) usam **`numpy-financial` ou a função equivalente do LibreOffice Calc/Excel como fonte de verdade externa** — calcula-se o valor de referência numa dessas ferramentas, documenta-se a entrada exata usada, e o teste forte do Rust compara contra esse número dentro da tolerância já definida (`1e-6` para IRR, conforme §8).

**Por que essa fonte e não planilha manual:** `numpy-financial`/LibreOffice já são implementações amplamente auditadas dessas fórmulas — calcular à mão introduziria o mesmo risco que se está tentando eliminar (erro humano não verificado). É uma segunda implementação independente, exatamente o que uma comparação de correção exige.

**O que fica de fora, por escopo:** métricas que não têm equivalente direto em `numpy-financial`/planilha (ex.: `runway`/`BR`, `z_score` com janela específica do domínio, `credit::utilization`) continuam no padrão de golden value calculado a partir da definição matemática direta, documentado no teste — não há biblioteca de mercado madura pra comparar essas contra. **Registrado no §8 do Discovery** como a fonte de verdade para os golden values de IRR/TWR; e como task nova no `roadmap-github.md` (Milestone 6, já que é onde `returns`/`risk` são implementados).

---

## RA-21 — Modo avançado como reconfiguração total de UI/UX, não app separado nem toggle superficial

**Resposta do autor:** um toggle que muda a interface inteira, como um "modo desenvolvedor" — reconfigura toda a estrutura de UI/UX conforme a persona, dentro do mesmo app.

**Isso não é nenhuma das três opções que ofereci — é uma quarta, e melhor que as três.** Não é o toggle superficial que eu descrevi (mostrar/esconder alguns números extras na mesma tela) — o problema desse toggle raso é exatamente o que a auditoria apontou: vocabulário e jornada vazando entre os dois públicos. Também não é o consumidor/app separado — que resolveria a contaminação às custas de duplicar toda a camada de apresentação e o esforço de manutenção. A resposta do autor fecha o meio-termo certo: **um único app, uma única lib consumida, mas o toggle é uma reconfiguração de primeira classe** — navegação, vocabulário, telas disponíveis e ordem de informação mudam por completo com o modo, não é a mesma tela com campos a mais. É o padrão de apps que têm "modo simples" vs. "modo avançado" como dois esqueletos de UI distintos sobre o mesmo dado (o próprio autor usou a analogia certa: modo desenvolvedor).

**Fecha A#21.** Nenhuma mudança na lib — `ofx-analytics`/ADR-13 já eram agnósticas de UI; a decisão é inteiramente do lado do consumidor de referência (a demo). **Registrado no handoff de UX** (§8, que tinha isso como questão em aberto) com a resposta definitiva: não é "toggle sempre visível vs. sugestão automática" — é "toggle que troca o esqueleto de UI inteiro", o que muda a pergunta de design de "onde colocar um botão" para "como estruturar duas jornadas completas compartilhando o mesmo motor".

---

## RA-22 — Drill-down até a transação: prioridade alta, Milestone 4

**Resposta do autor:** prioridade alta — vira story explícita já no Milestone 4 (analytics de gasto), junto do resto.

**Fecha A#22.** O problema identificado era estrutural, não só de UI: a `Provenance` de cada `Metric` hoje carrega janela temporal, contagem de inputs e faixa de datas (ADR-05) — mas **não a lista dos identificadores de transação que de fato compuseram o número**. Sem isso, não existe drill-down possível nem no backend nem na UI, por mais que a tela queira oferecer. **Decisão de API, registrada na ADR-05 do Discovery:** `Provenance` (ou um acessor irmão) passa a carregar os `Fitid` (ou `(AccountId, Fitid)`, coerente com a chave do achado #65) das transações que entraram no cálculo — não como obrigação de toda métrica (uma agregação sobre milhares de transações não precisa listar todas), mas como capacidade que `ledger::total_spent`/`extremes`/`group_by_payee` (as métricas mais prováveis de gerar dúvida da persona leiga) devem expor.

**Story adicionada ao `roadmap-github.md`, Milestone 4.**

---

## RA-30 — Cego por princípio, para sempre: nenhuma telemetria de produto

**Resposta do autor:** cego por princípio, pra sempre — nenhuma telemetria de produto, nunca, nem anônima.

**Fecha A#30 de forma definitiva, sem meio-termo.** Diferente do canal de reporte de dialeto (R4, Camada 2) — que é opt-in, acionado pelo usuário, sobre a estrutura de um arquivo que falhou — telemetria de produto (uso, contagem de arquivos processados, features tocadas) **não existe em nenhuma forma**, nem como exceção anônima. A tese "nada sai do navegador" é tratada como binária, exatamente como a auditoria recomendou: zero exceção evita a erosão gradual que um "só um ping" abriria.

**Consequência prática:** o autor aceita conscientemente ficar cego para o próprio uso do produto — não há dashboard de quantas pessoas usam, nenhuma forma de saber se uma feature específica é tocada. Isso é coerente com R1 (tooling FLOSS sem pressão de tração) e R6 (métrica de sucesso fora de escopo) — os três juntos fecham um mesmo eixo: este projeto não se mede pelo uso, se mede pela correção e completude do que entrega.

**Registrado no Discovery (ADR-09) e no anti-roadmap do documento de evolução** — telemetria de produto entra explicitamente na lista do que nunca será construído, ao lado de sync bancário e persistência embutida.

---

## RA-5 — Bill Pay e Taxes ganham visão leiga, não ficam como tipos dormentes

**Resposta do autor:** podemos adicionar esses Message Sets para a visão leiga — em vez de reduzir escopo ou só documentar como aposta de completude, usar os dados.

**Essa resposta é melhor que as três opções que ofereci** — não é "manter dormente" nem "cortar escopo", é "dar propósito ao que já foi modelado". Mas os dois Message Sets não têm o mesmo grau de certeza, e é importante não prometer os dois igual:

**Bill Pay — candidato real, ligado à recorrência (R2).** O Message Set Bill Pay representa pagamentos agendados configurados no serviço de pagamento de contas do próprio banco (payee, cronograma, valor). Se o banco popular esse bloco no export (nem todos fazem — precisa verificar no corpus), a visão natural é **"Contas agendadas/recorrentes"**: uma lista das contas fixas que o usuário já configurou pra pagamento automático, direto do dado estruturado do OFX — sem precisar da detecção estatística de recorrência que o H1 do documento de evolução propõe (que infere recorrência a partir do padrão de transações, quando o banco *não* fornece o dado estruturado). As duas abordagens são complementares: Bill Pay quando o dado existe, detecção estatística quando não existe.

**Taxes — candidato incerto, precisa de verificação antes de prometer.** O Message Set Taxes do spec OFX foi desenhado em torno de conceitos fiscais americanos (formulários 1099 — juros, dividendos — para declaração ao IRS). Bancos brasileiros **provavelmente não populam esse bloco** — a declaração de IR no Brasil usa documentos totalmente diferentes (informe de rendimentos, DIRPF), sem relação com o Tax Message Set do OFX. Antes de desenhar qualquer visão sobre isso, é preciso confirmar no corpus (ou em documentação de banco brasileiro) se esse Message Set aparece populado em algum export real. **Decisão prática:** entra como spike de pesquisa, não como visão prometida — se o corpus confirmar que nenhum banco BR popula, o Message Set continua modelado (por completude do spec, ADR-04) mas sem visão associada, e isso não é um fracasso da decisão, é o resultado esperado de verificar antes de prometer.

**Registrado na ADR-13 do Discovery** (nova visão candidata "Contas agendadas", função `ledger::scheduled_payments` / `getScheduledPayments(doc)`, condicionada a Bill Pay populado) **e no `roadmap-github.md`** (story `#4.7` no Milestone 4 para Bill Pay; spike `#4.8` separado para confirmar/descartar Taxes antes de qualquer story).

---

## RA-13 — MPL-2.0: trade-off de concorrente comercial aceito, sem reabrir a licença

**Resposta do autor:** aceitar o trade-off como está — MPL-2.0 já foi decidida, o benefício de adoção supera o risco de concorrente comercial usar.

**Fecha A#13.** Nenhuma mudança de licença. O que a auditoria pedia não era reconsiderar MPL-2.0 — já registrado como pesado o suficiente na troca do achado #58 — era só deixar **explícito** que copyleft por arquivo é mais permissivo que o LGPL original pra consumo comercial, e que isso foi visto, não descoberto depois. **Registrado na ADR-10 do Discovery**, amarrado a R1: um concorrente comercial usando o parser não é ameaça ao projeto porque o projeto não compete nesse mercado (R1) — é, se algo, adoção do padrão que o projeto quer ver difundido.

---

## RA-31 — Open Finance deprioritizado: foco 100% em OFX por ora

**Resposta do autor:** ignorar Open Finance por enquanto, seguir baseado 100% em OFX.

**Fecha A#31 por escopo, não pela resposta técnica que eu tinha oferecido.** A pergunta original era "o domínio aguenta a extensão pra Open Finance sem reescrita, ou precisa de campos novos desde já?" — a resposta do autor torna a pergunta não aplicável agora: Open Finance sai do horizonte ativo, então a tensão entre "`Document` semanticamente OFX" e "Open Finance com modelo mais rico" não precisa ser resolvida nem mitigada hoje. É mais brando que a opção 3 que eu tinha oferecido ("desistir da ambição") — "por enquanto" sinaliza pausa de escopo, não abandono permanente da ideia registrada no documento de evolução.

**O que muda:**
- `evolucao-ofx.md`, item H3.3 (Open Finance como formato) passa de horizonte ativo para "fora de escopo por ora" — a análise já escrita (a diferença semântica entre os dois modelos) permanece registrada como referência, caso o item seja revisitado no futuro, mas deixa de fazer parte do roadmap vivo.
- **CSV (H3.2) é uma pergunta distinta**, não tocada por esta resposta — resolve uma dor diferente (bancos que não oferecem OFX, obrigando fallback) e o autor não se referiu a ele. Fica como item separado; se o "foco 100% OFX" também deve valer pra CSV, é uma decisão própria, ainda em aberto.
- Nenhuma mudança na Discovery em si: `domain`/ADR-04 já são inteiramente centrados em OFX por natureza — a questão só existiria se Open Finance fosse de fato perseguido, o que deixou de ser o caso agora.

---

## RA-26 — Determinismo com tolerância explícita para métricas f64

**Resposta do autor:** definir tolerância explícita (epsilon) nos testes de métricas f64, sem garantir ordem de operação bit-a-bit.

**Fecha A#26.** O requisito de determinismo do §3 ganhou a ressalva que faltava: byte a byte vale para `Decimal`/`Money` (achado #56), mas para `f64` (Sharpe, VaR, volatilidade, EMA) "mesmo input, mesmo output" significa dentro de epsilon explícito por métrica — ponto flutuante não é associativo, então variar a ordem de redução entre execuções não é regressão. **Registrado no §3 e no §8 do Discovery**, ao lado da decisão RA-12/14 (golden values contra `numpy-financial`/LibreOffice) — são a mesma categoria de decisão de teste, então ficaram no mesmo lugar do documento.

---

## RA-27 — Release automatizada via GitHub Actions desde o Milestone 7

**Resposta do autor:** automatizar via GitHub Actions já no Milestone 7, usando ferramenta pronta pra workspace Rust (ex.: `release-plz`).

**Fecha A#27.** A issue `#7.5` do roadmap deixou de ser spike `needs:decision` e virou task concreta: pipeline de release cobrindo a ordem `domain` → `parse`/`analytics` → `wasm` → pacote npm, disparado por tag, sem passo manual de versão/lockfile. **Registrado na ADR-10 do Discovery** — `RELEASING.md` agora descreve o pipeline automatizado, não um checklist manual.

---

## RA-28 — Toolchain fixada desde já, não adiada para quando o binário for medido

**Resposta do autor:** fixar versões (toolchain pinada) desde já.

**Fecha A#28 — e a resposta é mais rigorosa que as três opções que ofereci.** Não escolheu "aceitar a deriva" nem "adiar pro Milestone 5/7" (que eu tinha oferecido como opções, alinhadas com quando o binário é de fato medido) — escolheu fixar a toolchain **antes** de qualquer medição, no Milestone 0. Isso é mais conservador e mais barato: fixar cedo custa uma linha de configuração; descobrir depois que o número histórico não é reproduzível custa re-auditoria de decisões que dependeram dele (ADR-03, ADR-07). **Registrado na ADR-07 do Discovery** e como task nova (`#0.3`) no `roadmap-github.md`, Milestone 0 — junto da medição de binário (`#0.2`), que agora tem base reproduzível desde a primeira medição.

---

## RA-15 — `k_d` = Custo Implícito da Dívida, confirmado contra o handoff técnico original

**Resposta do autor:** anexou o handoff técnico original das 20 operações — o mesmo documento que fundamentou o Discovery, que eu nunca tinha visto diretamente nesta conversa.

**Fecha A#15 com fonte primária, não mais hipótese.** O handoff confirma: `k_d` = **Custo Implícito da Dívida**, categoria Analítica, fórmula `k_d = Σ I_paid / B̄_debt` (juros pagos sobre saldo médio em dívida), tags `TRNAMT (INT)` e `BALAMT`. Minha hipótese anterior (custo do rotativo do cartão) estava na direção certa — é essencialmente isso, calculado empiricamente a partir dos juros de fato cobrados na fatura em vez de uma taxa nominal — mas agora é fato confirmado contra a fonte, não interpretação.

**Ressalva que a fórmula em si expõe, e que fica registrada pra Fase 2 (Credit Card):** a fórmula pressupõe transações com `TRNTYPE=INT` representando os juros cobrados. `INT` no spec OFX é documentado principalmente pra **juros recebidos** (conta corrente/poupança rendendo) — não está confirmado que emissores de cartão brasileiros usam essa mesma tag pra **juros cobrados** no rotativo. É plausível que apareça como `FEE`/`SRVCHG` genérico ou só como texto em `NAME`/`MEMO` ("JUROS ROTATIVO", "ENCARGOS FINANCEIROS"), no mesmo padrão de dialeto que já motivou os achados #6/#18. Isso não bloqueia o Discovery agora — é verificação de corpus da Fase 2, registrada explicitamente pra não ser esquecida.

**Nota sobre o processo:** um outro chat deste mesmo projeto já tinha chegado a uma avaliação equivalente (mesma hipótese, mesma ressalva sobre a tag), rastreando contra o histórico da sessão original — não pude verificar diretamente essa alegação de rastreamento (não tenho acesso àquela conversa), mas a substância da análise se confirma agora contra a fonte primária que o autor anexou aqui, o que é evidência mais forte que qualquer reconstrução de memória de sessão.

**Registrado no Discovery, ADR-05** — linha `credit` do mapa de módulos.

---

## RA-17 — Handoff editado: cada jornada agora aponta o Milestone que a habilita

**Resposta do autor:** editar o handoff agora — cada jornada ganha nota "habilitada a partir do Milestone X".

**Fecha A#17.** As sete jornadas de `handoff-ux-design-frontend.md` (§4) ganharam anotação explícita de disponibilidade, mapeada contra os Milestones do `roadmap-github.md`: Jornada 1 (onboarding) e Jornada 7 (entradas inválidas) desde o Milestone 1; Jornada 4 (múltiplas contas) parcialmente desde o Milestone 1, madura no 4; Jornada 5 (resiliência) no Milestone 2; Jornada 3 (múltiplos cartões) no Milestone 3; Jornada 2 (aprofundamento) no Milestone 4; Jornada 6 (modo avançado) no Milestone 6. Isso evita que o time de design desenhe telas completas para capacidades que só existem muitos milestones depois.

---

## RA-32 — Nome "ofx" mantido

**Resposta do autor:** manter o nome como está — renomear é barato depois (`git mv`), cedo demais pra mudar agora.

**Fecha A#32.** Nenhuma mudança de nome. Coerente com a própria observação que motivou a pergunta: a severidade da objeção já tinha caído bastante depois do A#31 (Open Finance, a maior ameaça semântica ao nome, foi pausado) — CSV sozinho (H3.2, ainda não tocado) não é mudança de identidade grande o suficiente pra justificar o custo de decidir um nome novo agora sem necessidade real.

---

## R6 — Métrica de sucesso: fora de escopo por ora (item 6)

**Resposta do autor (item 6):** não é importante para este momento do projeto.

**Resolução — A#6 aceito e adiado explicitamente.** Coerente com R1: sendo tooling FLOSS sem pressão de tração, definir KPI de sucesso agora seria cerimônia sem função. Registrado como decisão consciente, não como omissão. Quando/se o projeto buscar audiência além do uso próprio, a métrica natural — sugerida aqui só para não se perder — é *cobertura de dialeto*: quantos dos bancos do corpus parseiam sem `FatalParseError`. É a métrica que mede o que a categoria (tooling de formato) de fato entrega, alinhada a R1. Nenhuma ação agora.

---

## R10 — Item 10 do autor: mapeamento superado pela resolução achado-a-achado

O item 10 da primeira leva ("Resolva") pretendia mapear pra um achado A# específico da entrevista original — mas a numeração 1–10 das respostas do autor nunca bateu 1:1 com a numeração A# da banca, e esta seção ficou reservada, sem fechamento, por várias rodadas. **Superada na prática:** todos os 33 achados (incluindo os ~19 que esta seção listava como "ainda não endereçados" quando foi escrita) foram resolvidos individualmente nas levas RA-5 a RA-32 — ver placar completo abaixo. A ambiguidade de mapeamento do item 10 original não precisa mais de solução, porque cada achado foi endereçado diretamente pelo próprio número A#, não através do mapeamento pendente que esta seção esperava.

---

## Placar de achados após esta leva

| Achado | Estado | Onde |
|---|---|---|
| A#1 | Reenquadrado (não-objetivo) | R1 |
| A#2 | Endereçado | R2, R3, R5/R7 |
| A#3 | Endereçado (guia por banco) | R3 |
| A#4 | Reenquadrado (não-objetivo) | R1 |
| A#6 | Aceito e adiado | R6 |
| A#8 | Resolvido (conhecido) + rede para o desconhecido | R4 |
| A#9 | Mitigado (medição vira gate no Épico 0) | R5/R7 |
| A#19 | Resolvido (reporte local-first) | R4 |
| A#20 | Endereçado (evolução = entendimento) | R2, R5/R7 |
| A#23 | Planejado (fuzzing como entregável) | R4, R5/R7 |
| A#24 | Resolvido (Web Worker, ADR-06 revisada agora) | RA-24 |
| A#29 | Endereçado (fatia vertical) | R5/R7 |
| A#11 / A#25 | Adiado conscientemente (risco documentado em §9) | RA-11/25 |
| A#16 | Resolvido (parser próprio, não `ofx-rs`) | RA-16 |
| A#7 | Aceito conscientemente, sem mudança de escopo (risco documentado em §9) | RA-7 |
| A#10 | Resolvido (sem handle universal; cache é do consumidor) | RA-10 |
| A#12 / A#14 | Resolvido (golden values contra numpy-financial/LibreOffice) | RA-12/14 |
| A#21 | Resolvido (toggle reconfigura UI/UX inteira, um só app) | RA-21 |
| A#22 | Resolvido (drill-down, prioridade alta, Milestone 4) | RA-22 |
| A#30 | Resolvido (cego por princípio, para sempre, sem exceção) | RA-30 |
| A#5 | Resolvido (Bill Pay ganha visão "contas agendadas"; Taxes vira spike de verificação) | RA-5 |
| A#13 | Resolvido (trade-off aceito, registrado na ADR-10, sem mudar licença) | RA-13 |
| A#31 | Resolvido (Open Finance deprioritizado, foco 100% OFX por ora) | RA-31 |
| A#26 | Resolvido (epsilon explícito por métrica f64, não bit-a-bit) | RA-26 |
| A#27 | Resolvido (release automatizada via GitHub Actions, Milestone 7) | RA-27 |
| A#28 | Resolvido (toolchain fixada desde já, Milestone 0) | RA-28 |
| A#17 | Resolvido (handoff editado, jornadas apontam Milestone) | RA-17 |
| A#32 | Resolvido (nome mantido, renomear é barato depois) | RA-32 |
| A#15 | Resolvido (k_d = Custo Implícito da Dívida, confirmado contra fonte primária) | RA-15 |

**Auditoria concluída — 33 de 33 achados endereçados** (31 resolvidos/reenquadrados/aceitos individualmente + A#18/A#33, achados positivos que nunca precisaram de resolução). Nenhum item aberto.
