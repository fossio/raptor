# Auditoria de Solução — ofx (visão executiva)

| Campo | Valor |
|---|---|
| Data | 2026-07-20 |
| Escopo | Consistência da proposta de solução como produto viável — não é auditoria de código nem de decisão de ADR (essas estão em `analise-critica-adr.md`) |
| Ótica | Dois auditores: **CPO** (produto, mercado, adoção, valor) e **CTO** (viabilidade técnica, risco de execução, sustentabilidade) |
| Base | Discovery/ADR fechado, documento de evolução, handoff de UX, benchmark de mercado |
| Achados | A#1–A#33 (rodada 1: A#1–18 estratégia/execução; rodada 2: A#19–33 operação, borda e segunda ordem) |
| Veredito | Ao final (§4) |
| **Status** | **Todos os 33 achados resolvidos.** Este documento é o registro histórico das objeções originais — as respostas e decisões estão em `resolucao-auditoria.md`. Não editar este arquivo para refletir resoluções; é o "antes", não o "depois". |

Este documento faz o papel de banca crítica: assume que a proposta é boa o suficiente para merecer escrutínio sério, e procura onde ela quebra sob pressão de um comitê que precisa decidir se investe tempo/capital. As objeções são deliberadamente adversariais — é o serviço que uma auditoria presta.

---

## 1. A ótica do CPO — produto, mercado, valor

### A#1 [Crítico] — Qual é o problema que dói o bastante para alguém trocar de ferramenta?

A tese ("analytics de ferramenta com privacidade de função local") é elegante, mas a pergunta do comitê é: **a privacidade é uma dor ativa ou uma preferência latente?** O usuário que hoje usa Mobills/Organizze entregou os dados e não parece incomodado — a adoção massiva desses apps é a evidência contrária. O projeto aposta que existe um segmento que *se recusa* a entregar o extrato, mas não dimensiona esse segmento. Sem isso, o produto pode ser tecnicamente superior e comercialmente irrelevante.

**O que falta:** uma hipótese de segmento-alvo com tamanho estimado e um sinal de disposição — mesmo qualitativo (fóruns, comunidades de privacidade, r/personalfinance-BR, usuários de GnuCash que reclamam de fricção). O documento de evolução cita "soberania de dados" como tese, mas tese não é demanda validada.

**Contra-argumento a favor do projeto:** é FLOSS, mantenedor único, sem pressão de retorno de capital. O custo de estar errado sobre o tamanho do mercado é baixo — não há investidor a decepcionar. Isso *enfraquece a severidade* para este projeto específico, mas o CPO registraria: se algum dia buscar tração além do uso pessoal, esta é a primeira pergunta a responder.

### A#2 [Alto] — O produto entrega valor recorrente ou é uma visita única?

A persona importa a fatura, vê o gasto, e... volta quando? Todo o valor hoje é retrospectivo (o que já aconteceu). Uma pessoa não exporta OFX toda semana — o ato de exportar é fricção real. O risco é o produto ser um "extrato bonito de uso esporádico", não um hábito. Apps concorrentes resolvem isso com sync automático (o que este projeto proíbe por design) — então o projeto **abre mão do mecanismo de recorrência do mercado sem propor um substituto**.

**Mitigação já no roadmap:** os horizontes H1/H2 do documento de evolução (recorrência detectada, forecast, orçamento) são exatamente a resposta — dão motivo pra voltar. Mas eles estão em horizonte distante, e o MVP (Fases 0–5) entrega só o retrovisor. **O CPO questionaria a sequência:** o produto pode morrer de irrelevância antes de chegar ao H2 que o salvaria.

### A#3 [Alto] — Fricção de aquisição do próprio insumo

Para usar o produto, a pessoa precisa saber que o banco dela exporta OFX, encontrar onde, exportar, e arrastar o arquivo. Cada passo é abandono. A persona declarada é "leiga em tecnologia" — e exportar OFX do internet banking não é tarefa de leigo. **Há uma tensão não resolvida entre a persona (leiga) e o insumo que ela precisa produzir (exige saber navegar o banco).** O handoff de UX não endereça o onboarding *antes* do arquivo existir — só a partir do drag-and-drop.

**Recomendação:** o produto precisa de conteúdo/guia "como exportar do seu banco" por instituição — é parte do produto, não suporte. E possivelmente aceitar formatos que a pessoa consegue obter mais fácil (CSV, já previsto em H3) mais cedo do que o roadmap sugere.

### A#4 [Médio] — O diferencial "privacidade" é comunicável para a persona leiga?

O §8 do handoff já levanta isso como questão aberta — corretamente. Mas do ponto de vista de produto é mais grave: se a persona leiga não *entende* por que processamento local importa, o único diferencial do produto é invisível pra ela, e aí ele compete de igual pra igual com Mobills na única dimensão que sobra (features), onde perde. **O diferencial só é diferencial se for percebido.** Isso é risco de posicionamento, não só de copy.

### A#5 [Médio] — Priorização cartão-primeiro está certa, mas cria dívida de escopo

A ADR-13 prioriza a persona de cartão. Coerente. Mas o domínio modela 5 Message Sets (Banking, CC, Investment, Bill Pay, Taxes) desde a Fase 0 — o CPO questionaria: **por que carregar o custo de modelar Bill Pay e Taxes agora se a persona-alvo não os usa?** Há uma inconsistência entre a priorização de produto (cartão) e a ambição de completude do domínio (tudo). Modelar os 5 é decisão de engenharia (completude), não de produto (foco) — e o comitê de produto veria isso como escopo não-justificado por valor de usuário.

**Nuance:** o custo de modelar structs de dados que não são usados por analytics ainda é baixo (são tipos, não features). Mas é dívida latente — cada Message Set modelado é superfície que precisa de teste, corpus e manutenção.

### A#6 [Médio] — Ausência de métrica de sucesso

Nenhum documento define como se mede se o produto deu certo. Para um projeto pessoal FLOSS isso é aceitável, mas o CPO registraria a ausência: sem uma definição de sucesso (nº de bancos suportados sem erro? nº de usuários? redução de tempo pra entender a fatura?), não há como priorizar entre horizontes nem saber quando parar de investir num deles.

---

## 2. A ótica do CTO — viabilidade, execução, sustentabilidade

### A#7 [Crítico] — Mantenedor único vs. ambição do escopo é o risco de execução dominante

O roadmap tem 7 fases + 4 horizontes de evolução. O benchmark mostra que os concorrentes maduros (ofxtools, LibOFX) têm anos de trabalho e comunidade. **Um mantenedor único entregando parsing resiliente de todos os dialetos + 20 métricas + fronteira WASM + toda a camada de evolução é um horizonte de muitos anos.** O risco não é a qualidade das decisões (que é alta) — é o projeto estagnar na Fase 2 e nunca chegar ao analytics que o justifica.

**O que o Discovery acerta:** o §7 do documento de evolução já reconhece isso e ordena os horizontes para que "cada corte seja um produto completo". Essa é a mitigação certa. **O que o CTO exigiria:** que essa disciplina de "cada fase é entregável e para-able" seja verificada fase a fase, não só afirmada — a Fase 0 entrega algo usável sozinha? (Hoje: workspace + domain + parser XML — não é usável por ninguém ainda; o primeiro corte usável é a Fase 3, o que é tarde.)

### A#8 [Crítico] — A resiliência a dialeto é afirmada, não provada, e é o risco técnico nº 1

O benchmark mostra que resiliência a dialeto é onde bibliotecas OFX vivem ou morrem (goofx existe só pra isso; ofxparse depende de amostras da comunidade). O projeto aposta em corpus sintético por dialeto (ADR-08, achado #4) — decisão correta de privacidade, **mas há um risco que o próprio Discovery admite: corpus sintético é gerado a partir do que o autor *sabe* que os bancos fazem de errado.** Os dialetos que quebram parsers reais são justamente os que ninguém antecipou. Corpus sintético não descobre o desconhecido-desconhecido; só codifica o conhecido.

**Tensão de fundo:** a decisão de privacidade (nunca usar extrato real) está em conflito direto com a necessidade de robustez (que se alimenta de arquivos reais e estranhos). O projeto escolheu privacidade — legítimo — mas **não resolveu como vai descobrir os dialetos que não conhece sem ver arquivos reais.** Possível caminho não explorado: um mecanismo opt-in onde o usuário reporta um arquivo que falhou, localmente anonimizado antes de qualquer envio, ou um "cole aqui a tag que quebrou" sem o arquivo inteiro. Hoje isso é um ponto cego.

### A#9 [Alto] — Orçamento de binário WASM é uma aposta não medida que pode invalidar a UX

O ADR-07 tem disciplina de tamanho (opt-level=z, twiggy, features mínimas), mas o próprio documento admite que o tamanho real só é medível na Fase 5. **`rust_decimal` + `encoding_rs` + `quick-xml` + o domínio inteiro + serde na fronteira** — se o `.wasm` resultante for grande (vários MB), a promessa de "página que carrega rápido e roda offline" quebra, especialmente em mobile, que é onde a persona brasileira está. O CTO registraria: **a viabilidade da tese de UX (rápido, offline, no navegador) depende de um número que ninguém tem ainda** e que só aparece muito tarde no roadmap. É risco tardio — o pior tipo, porque descobrir na Fase 5 que o binário é inviável significa retrabalho de decisões da Fase 0 (ADR-03 sobre `rust_decimal`).

**Recomendação:** antecipar uma medição de binário mínima já na Fase 0/1 — compilar só `domain` para WASM e medir, antes de investir no resto. O Discovery já tem a "ação imediata" de compilar para wasm32; falta acoplar a isso uma medição de tamanho como gate.

### A#10 [Alto] — Performance de série grande na fronteira sem handle é risco conhecido e adiado

A ADR-06 reverteu para "documento serializado por chamada, sem handle" — e admite explicitamente que isso re-serializa o documento inteiro a cada chamada de métrica. A justificativa (arquivos da persona são pequenos) é razoável, **mas o modo avançado de investimento (ADR-13) opera sobre séries que podem ser grandes, e a consolidação multi-fonte junta N arquivos.** O caso "usuário avançado com histórico longo de corretora consolidado" é exatamente onde a decisão de não ter handle dói — e é um caso que o produto declara suportar. A decisão está registrada como reavaliável, mas o CTO notaria que **o gatilho de reavaliação (Fase 5, medição real) é tardio e o caso de dor não é da persona primária, então tende a ser despriorizado justamente por quem sente a dor não ser o alvo.**

### A#11 [Alto] — Dependência de decisões não-tomadas na camada de evolução

O documento de evolução (H1/H2) propõe categorização por regras, orçamento, forecast — e afirma que tudo cabe no padrão "função pura". **Mas categorização e orçamento pressupõem que o usuário fornece as regras/envelopes, e não há decisão sobre como esse estado do usuário entra e persiste** sem violar o anti-roadmap ("storage é do consumidor"). Há uma tensão real: as features de maior valor recorrente (H2) exigem estado do usuário, e a arquitetura proíbe estado na lib. A resposta ("o consumidor gerencia") empurra a complexidade toda para a camada de UI/app, que ainda não tem dono nem design. **O CTO questionaria se a divisão lib-pura / estado-no-consumidor sobrevive ao contato com as features que realmente prendem usuário** — ou se em algum ponto a lib vai precisar de um conceito de "sessão/perfil" que hoje o anti-roadmap veta.

### A#12 [Médio] — Testabilidade do analytics financeiro depende de golden values que alguém precisa calcular à mão

A estratégia de eval fraco→forte (§8) é excelente em princípio: cada métrica testada contra um valor conhecido. **Mas quem produz o valor conhecido?** IRR de uma anuidade, Sharpe de uma série, TWR com sub-períodos — os golden values precisam ser calculados por uma fonte independente e confiável (planilha de referência, outra biblioteca, cálculo manual auditado). Isso é trabalho especializado e volumoso, e o Discovery não diz de onde vêm esses valores de referência. **Sem uma fonte de verdade para os golden values, os testes fortes provam consistência interna, não correção** — a métrica pode estar sistematicamente errada e passar em todos os testes se o golden foi calculado com a mesma premissa errada.

### A#13 [Médio] — MPL-2.0 recém-decidida muda a análise de adoção que motivou o próprio projeto

A troca LGPL→MPL-2.0 (achado #58) foi bem fundamentada tecnicamente. Mas o CTO notaria uma consequência de segunda ordem: MPL-2.0 é copyleft por arquivo, o que a torna *mais* permissiva para consumo comercial que LGPL. Isso **facilita que um concorrente comercial (um Mobills da vida) incorpore o parser** sem precisar abrir o app dele. Ou seja: a decisão de licença que resolveu o atrito de adoção Rust também baixou a barreira para o cenário competitivo do §7 do documento de evolução (concorrente captura o nicho). Não é erro — é um trade-off que o comitê deveria ver explicitado, não descoberto depois.

### A#14 [Médio] — Ausência de estratégia de dados de teste realistas bloqueia CI significativo

Ligado ao A#8 e A#12: sem corpus real e sem golden values de fonte independente, o CI valida que o código faz o que o autor achou que os bancos fazem e calcula o que o autor achou que a métrica deveria dar. **É um sistema fechado que se valida contra as próprias premissas.** Para um produto que lida com o dinheiro de pessoas, isso é insuficiente a longo prazo — em algum momento precisa de validação contra realidade externa (arquivos reais anonimizados, valores de referência de terceiros). O Discovery não tem essa ponte.

### A#15 [Baixo] — `k_d` e outros símbolos ainda não definidos

Já registrado como achado #74 na auditoria de ADR; o CTO só confirma que símbolos de métrica sem definição no documento são risco de implementação divergente quando a Fase 4 chegar.

### A#16 [Baixo] — O crate `ofx-rs` concorrente pode tornar metade do trabalho redundante

Se o objetivo é o pacote (parsing + analytics + WASM), e já existe um parser Rust tipado publicado, o CTO perguntaria se não é mais rápido *consumir* `ofx-rs` como front-end de parsing e focar 100% no diferencial (analytics + WASM), em vez de reescrever o parser. O Discovery registra isso como opção teórica (A ADR-01 mantém `domain` como ativo desacoplável), mas não a avalia seriamente como acelerador de roadmap. **Para um mantenedor único com risco de execução A#7, terceirizar o parsing pode ser a diferença entre chegar ou não ao analytics.** Contra-argumento: perde-se o controle sobre a resiliência a dialeto (A#8), que é diferencial. É um trade-off que merece uma decisão explícita, não silêncio.

---

## 3. Consistência interna da proposta — auditoria cruzada

Onde os documentos entre si (Discovery, evolução, handoff) concordam e onde tensionam:

### A#17 [Médio] — O handoff de UX promete recorrência que o roadmap ainda não entrega

O handoff descreve jornadas ricas (aprofundar no mês, comparar cartões, parcelas em aberto) como se fossem o produto. Mas várias dessas visões dependem de fases distantes (parcelas = Fase 3; consolidação madura = Fase 4). **Se o time de UX desenhar o produto completo do handoff, vai projetar telas para capacidades que não existem por anos.** O handoff deveria marcar, por jornada, qual fase a habilita — senão cria expectativa de produto que o roadmap não sustenta no curto prazo. É a mesma classe do achado #17 da auditoria de ADR (visões da tabela vs. fase que as entrega), agora entre documentos.

### A#18 [Baixo] — Coerência forte onde importa

Registro do lado positivo, porque auditoria honesta mede os dois: a espinha dorsal é consistente entre todos os documentos. Privacidade estrutural, `Money` com moeda, proveniência em toda métrica, persona cartão-primeiro, anti-roadmap explícito — esses aparecem alinhados no Discovery, na evolução e no handoff sem contradição. As decisões pendentes foram todas fechadas na entrevista. A qualidade de *raciocínio* documentado (cada reversão com custo nomeado, cada trade-off explícito) está acima da média de discovery de produto. **O risco do projeto não é qualidade de decisão — é volume de execução e validação contra realidade externa.**

---

## 3.5. Segunda rodada — operação, borda e efeitos de segunda ordem

A primeira rodada cobriu estratégia e execução. Esta ataca o que um comitê experiente pergunta na segunda reunião: como isso opera depois de lançado, o que acontece nas bordas do fluxo feliz, e quais efeitos de segunda ordem as decisões já tomadas produzem.

### A ótica do CPO — continuação

#### A#19 [Alto] — Sem loop de feedback, o produto não sabe onde está falhando

Consequência direta da privacidade estrutural: **se nada sai do navegador, nada sai do navegador — inclusive o sinal de que um banco quebrou o parser.** O produto não tem telemetria, não tem relatório de erro automático, não sabe quais bancos seus usuários usam nem onde falha. Para um app SaaS, o dashboard de erros é como o time descobre o dialeto novo. Aqui, o autor só descobre que o Banco X quebrou se um usuário se der ao trabalho de reportar manualmente — e a persona é leiga, então não vai reportar com detalhe técnico útil. **O mecanismo de melhoria contínua do produto está estruturalmente amputado pela própria tese.** É o A#8 (descobrir dialeto) visto do lado de produto: não é só como testar antes, é como saber depois. Precisa de um canal de reporte deliberado e desenhado (opt-in, local-first), senão o produto fica cego para a própria qualidade em produção.

#### A#20 [Alto] — "Entender o gasto" é resultado, e o produto entrega dados, não entendimento

A persona quer *entender* o próprio gasto. O produto entrega total, categoria, evolução, anomalia — mas isso é matéria-prima de entendimento, não entendimento. Um número "você gastou R$ 4.200" não responde a pergunta emocional por trás ("tô gastando demais? com o quê que eu nem percebo?"). Os apps de mercado resolvem isso com comparação ("você gastou 30% mais que mês passado"), com metas, com narrativa. **O produto, como especificado, para no dado e deixa o salto para o insight por conta do usuário leigo — que é exatamente quem menos consegue dar esse salto sozinho.** A visão de anomalia é o embrião do insight, mas está enquadrada como "transação estatisticamente fora do padrão", que é linguagem de dado, não de entendimento. Risco de produto: ser percebido como "planilha mais bonita", não como "finalmente entendi minha fatura".

#### A#21 [Médio] — O modo avançado é um produto diferente disfarçado de toggle

Investidor de corretora e leigo de cartão não são a mesma pessoa em modos diferentes — são dois produtos com personas, vocabulários e jornadas incompatíveis, compartilhando só o parser. Empacotá-los no mesmo app com um toggle cria risco de os dois ficarem medianos: a UI leiga contaminada por conceitos avançados que vazam, e o investidor frustrado com um app que foi desenhado para quem não sabe o que é VaR. **A decisão de sequência (leigo primeiro) está certa; a decisão de que são o mesmo produto não foi auditada.** Talvez o modo avançado devesse ser um consumidor *separado* da mesma lib, não um modo do mesmo app — o que a arquitetura permite e o produto não considerou.

#### A#22 [Médio] — Não há tratamento do caso "o usuário discorda do número"

A persona leiga vai, em algum momento, olhar o total e pensar "isso tá errado, eu não gastei tudo isso". Pode ser categorização errada, transferência contada como gasto, parcela mal interpretada, ou o próprio usuário esquecido. **O produto não tem, em lugar nenhum, o fluxo de "por que esse número é esse" navegável até a transação individual.** A proveniência (que o Discovery valoriza) responde "de onde veio o cálculo", mas não "quais transações somaram esse total" de forma que o leigo audite e confie. Sem drill-down até o lançamento, o primeiro número que discordar da intuição do usuário destrói a confiança no produto inteiro. Confiança é binária para leigo: um número errado percebido contamina todos os outros.

### A ótica do CTO — continuação

#### A#23 [Alto] — Superfície de ataque de parser sobre input hostil é subestimada

O produto processa arquivos de terceiros — e o corpus inclui arquivos deliberadamente malformados. O SECURITY.md está previsto, e o achado #19 (billion laughs no XML) foi tratado. Mas o CTO nota que **um parser SGML com tabela de aridade + fallback heurístico + recuperação de tags é precisamente o tipo de código onde vivem os bugs de segurança de memória** — e embora Rust elimine a classe de memory-safety, não elimina panics, loops infinitos, ou exaustão de recursos por entrada adversária construída. O `max_bytes` (achado #9) cobre tamanho bruto, mas não profundidade de aninhamento (stack overflow por recursão), nem uma entrada pequena que expande em processamento (o análogo do billion laughs no *lado SGML*, não só XML). A auditoria de segurança do parser SGML sob fuzzing é mencionada? Não. **Fuzzing (cargo-fuzz/AFL) do parser deveria ser entregável, não boa intenção** — é a única forma de encontrar o input que trava antes que um usuário o encontre.

#### A#24 [Alto] — WASM single-thread + arquivo grande = navegador congelado, sem mitigação desenhada

O executor WASM é single-thread (a §5 do Discovery reconhece: `spawn_local`, sem `Send`). Parsing e cálculo rodam na main thread do browser por padrão. **Um arquivo grande ou um cálculo pesado (IRR iterativo, consolidação de N fontes) congela a UI inteira** — a aba trava, o usuário acha que quebrou. A solução padrão (Web Worker, rodar o WASM fora da main thread) não é mencionada em lugar nenhum do Discovery nem do handoff. Isso é decisão de arquitetura de fronteira que precisa existir antes da Fase 5, e tem implicação de API (a fronteira vira assíncrona, mensagens em vez de chamadas diretas) — potencialmente revertendo parte da ADR-06. **Risco de descobrir tarde que a API síncrona da ADR-06 é incompatível com a UX responsiva que exige Web Worker.**

#### A#25 [Médio] — Versionamento e migração de dados persistidos não tem dono

O anti-roadmap põe persistência no consumidor. Mas se o app (a demo, ou qualquer consumidor) persiste documentos parseados ou regras de categorização em `IndexedDB`, e a estrutura do `Document` muda entre versões da lib (nova variante de enum, campo novo — como `installment` que acabou de ser adicionado), **o dado persistido na versão antiga quebra ou é mal interpretado pela nova.** Quem versiona o schema serializado? Como migra? A ADR-06 fala de `Serialize`/`Deserialize` mas não de evolução de schema ao longo do tempo. É problema clássico e adiado — e como cada consumidor persiste por conta própria, cada um vai reinventar (mal) a migração. A lib deveria ao menos versionar o formato serializado que emite.

#### A#26 [Médio] — Precisão de `f64` na fronteira estatística não foi cercada por invariante testável

A ADR-03 é rigorosa com `Decimal` para dinheiro, e o achado #56 fechou que `Decimal` cruza a fronteira como string. Mas as métricas estatísticas (Sharpe, VaR, volatilidade) usam `f64` por necessidade, e o resultado `f64` atravessa a fronteira. **Duas execuções do mesmo cálculo em `f64` podem divergir no último bit dependendo da ordem de operações** (associatividade de ponto flutuante), o que colide com o requisito de determinismo do §3. Para um número exibido, o último bit não importa — mas para um golden test (§8) que compara `f64` por igualdade exata, importa muito, e para o requisito de "mesmo input, mesmo output byte a byte", também. Falta decidir a tolerância de comparação e garantir ordem de redução estável nas agregações `f64`. É sutil e é exatamente o tipo de coisa que passa despercebida até um teste ficar intermitente.

#### A#27 [Médio] — Estratégia de build/release do `.wasm` + pacote npm é complexa e frágil para mantenedor único

O RELEASING.md previsto reconhece que a ordem de publicação é não-trivial: `domain` → `parse`/`analytics` → `wasm` no crates.io, depois npm via wasm-bindgen. **Cada release cross-registry (crates.io + npm) com quatro crates interdependentes é uma coreografia que erra fácil** — versão dessincronizada entre o crate e o pacote npm gerado, `Cargo.lock` vs. lockfile npm, o `.wasm` publicado não bater com a versão do JS glue. Para mantenedor único sem CI de release automatizado, isso é fonte recorrente de release quebrada. Merece automação (GitHub Actions de release) tratada como entregável de infra, não como processo manual documentado.

#### A#28 [Médio] — Reprodutibilidade do build WASM ao longo do tempo

Ligado ao A#9 e A#27: o orçamento de binário depende de versões exatas de `wasm-opt`, da toolchain Rust, do wasm-bindgen. **Daqui a dois anos, recompilar o mesmo commit pode produzir um `.wasm` de tamanho diferente** (otimizador mudou, toolchain mudou). Para um projeto cujo diferencial inclui "binário enxuto", a ausência de um ambiente de build fixado (container com versões pinadas) significa que o número que valida o ADR-07 não é reprodutível. Não é bloqueante hoje, mas é dívida de infra que cresce silenciosa.

### Efeitos de segunda ordem e consistência — continuação

#### A#29 [Alto] — A soma das decisões "corretas" produz um MVP que não é usável por ninguém cedo

Cada decisão isolada é defensável, mas o CTO soma o vetor: privacidade estrutural (sem servidor para iterar), corpus sintético (sem dado real para robustez), mantenedor único (sem paralelismo de execução), lib-pura (estado empurrado para um consumidor que não existe), primeiro corte usável só na Fase 3. **O conjunto dessas escolhas — cada uma ótima — compõe um caminho onde nada é demonstrável a usuário real por muito tempo, e todo o aprendizado de mercado fica represado até tarde.** É o risco mais insidioso porque não aparece em nenhuma decisão individual, só na integral delas. A mitigação não é reverter nenhuma decisão — é forçar um "corte fino vertical" cedo: um só banco, um só dialeto, uma só métrica, ponta a ponta no browser, na Fase 1, só para ter algo real na frente de um usuário real. O roadmap horizontal (camada por camada) atrasa isso; um slice vertical o antecipa.

#### A#30 [Médio] — Privacidade estrutural vs. a realidade de que apps precisam de telemetria de produto

Tensão de segunda ordem não reconhecida: o autor vai querer saber se o produto é usado, quantos arquivos foram processados, quais features são tocadas — telemetria de produto básica (nada a ver com o módulo de cálculos financeiros `ofx-analytics` — termos deliberadamente distintos para não colidir). **A tese "nada sai do navegador" proíbe até a telemetria de uso mais anônima.** Ou o projeto aceita ser cego para o próprio uso (coerente com a tese, mas operacionalmente duro), ou vai haver pressão para uma exceção ("só um ping anônimo") que é o primeiro furo na tese inteira — e a tese é binária, um furo a descaracteriza. Melhor decidir agora, explicitamente, que é cego por princípio, do que erodir depois sob pressão prática. O anti-roadmap veta telemetria de erro; deveria vetar (ou permitir explicitamente) telemetria de uso também, para não ficar como zona cinzenta.

#### A#31 [Médio] — O documento de evolução assume que formato (Open Finance) é intercambiável, mas o modelo de domínio é OFX-formado

H3.3 propõe Open Finance como "só mais um formato" que mapeia para o mesmo `Document`. Mas o `Document` foi modelado a partir do vocabulário OFX (`TRNAMT`, `FITID`, Message Sets, `INVPOS`) — é Ubiquitous Language do OFX, como a §5 admite com orgulho. **Open Finance tem modelo de dados próprio, mais rico e diferente (categorização nativa, dados de recebível, granularidade distinta).** Forçá-lo no `Document` OFX-formado ou perde informação que o OFX não tem, ou distorce o modelo. A afirmação "o domínio é agnóstico de formato" é aspiracional — o domínio é agnóstico de *sintaxe* (SGML vs XML), mas é *semanticamente* OFX. O CTO marcaria a promessa de intercambiabilidade de formato como não comprovada para formatos com modelo semântico diferente.

#### A#32 [Baixo] — Nomenclatura "ofx" limita o que o projeto declara querer ser

Se o horizonte inclui CSV e Open Finance (H3), o nome "ofx" no projeto, nas crates e no domínio vira contra a ambição — o `Document` chamado por um projeto `ofx` fica estranho recebendo dados de Open Finance. É a mesma lição que o próprio projeto aplicou ao renomear `bindings`→`wasm` e `aggregation`→`ledger` (nomes conceituais, não tecnológicos): pelo próprio critério do autor, "ofx" é nome de tecnologia/formato, não de conceito. Se a ambição multi-formato é séria, o nome já nasceu com prazo de validade. Baixa severidade porque é renomeável (`git mv`), mas é dívida conceitual que o projeto se impôs contra a própria regra.

#### A#33 [Baixo] — Coerência da disciplina de anti-escopo (achado positivo)

Registro positivo desta rodada: o anti-roadmap é a peça mais madura da proposta. A maioria dos projetos morre de escopo inflado; este tem uma lista explícita e fundamentada do que *não* fará (sync, persistência na lib, ML embarcado, UI de produto, multiusuário). Essa disciplina é rara e é o que dá credibilidade à afirmação de que os cortes de roadmap são reais. Vários achados desta auditoria (A#11, A#25, A#30) tensionam o anti-roadmap — mas o fato de existir uma linha clara para tensionar já é sinal de maturidade acima da média.



**CPO:** A proposta é internamente coerente e diferenciada, mas repousa sobre uma hipótese de mercado não validada (A#1) e um mecanismo de recorrência que só chega tarde (A#2). Para uso pessoal/FLOSS, aprovada — o custo de estar errado é baixo e o valor de aprendizado é alto. Para qualquer ambição de tração, condicionada a: validar o segmento que se importa com privacidade, resolver a fricção de aquisição do insumo (A#3), e antecipar as features de recorrência (H1) para mais perto do MVP.

**CTO:** Arquitetura sólida, decisões bem fundamentadas, disciplina de escopo real. Mas três riscos técnicos são subestimados ou adiados demais: resiliência a dialeto sem caminho para descobrir o desconhecido (A#8), tamanho de binário como aposta não medida com invalidação tardia (A#9), e a tensão lib-pura vs. estado-do-usuário nas features que dão recorrência (A#11). Aprovada para começar, condicionada a: medição de binário como gate já na Fase 1 (não Fase 5), uma estratégia de validação contra realidade externa (arquivos e golden values de fonte independente, A#12/A#14), e uma decisão explícita sobre consumir `ofx-rs` vs. reescrever o parser (A#16), dado o risco de execução de mantenedor único.

**Consolidado:** o projeto merece existir e está bem pensado. Os achados críticos não são "isto está errado" — são "isto é uma aposta cujo resultado você só vai conhecer tarde demais para reagir barato". A recomendação transversal da banca é **antecipar os pontos de descoberta**: medir binário cedo, validar dialeto cedo, testar a hipótese de recorrência cedo. Mover as três incertezas mais caras para o começo do roadmap, onde ainda é barato mudar de ideia, é a única mudança estrutural que a banca exigiria antes de comprometer tempo significativo.

A segunda rodada endurece essa conclusão num único ponto (A#29): o problema não está em nenhuma decisão isolada, mas na **integral** delas — a soma de escolhas individualmente ótimas (sem servidor, sem dado real, mantenedor único, lib pura, primeiro corte usável na Fase 3) produz um caminho onde nada é demonstrável a usuário real por muito tempo. A banca converge numa recomendação que subsume as anteriores: **um slice vertical mínimo — um banco, um dialeto, uma métrica, ponta a ponta no browser, na Fase 1** — em vez do avanço horizontal camada por camada. Isso ataca de uma vez A#2 (recorrência), A#8/A#19 (dialeto e feedback), A#9 (binário) e A#29 (demonstrabilidade), porque força o produto inteiro a existir em miniatura antes de existir em completude. Duas decisões técnicas de fronteira (A#24 Web Worker e A#26 determinismo de `f64`) precisam ser tomadas antes desse slice, porque ambas podem reverter parte da ADR-06 se descobertas depois.

**Adendo do CTO após a segunda rodada:** o parser sobre input hostil precisa de fuzzing como entregável (A#23), não como intenção; e a operação em produção sob privacidade estrutural (A#19, A#30) precisa de uma decisão explícita — o produto é deliberadamente cego para o próprio uso e erro, ou terá um canal opt-in local-first? Deixar isso como zona cinzenta é convite à erosão da tese sob a primeira pressão prática.

---

## Apêndice — Achados por severidade

| # | Severidade | Ótica | Uma linha |
|---|---|---|---|
| A#1 | Crítico | CPO | Segmento que valoriza privacidade não é dimensionado |
| A#7 | Crítico | CTO | Mantenedor único vs. escopo de anos — risco de estagnação |
| A#8 | Crítico | CTO | Resiliência a dialeto sem caminho para o desconhecido-desconhecido |
| A#2 | Alto | CPO | Valor recorrente só chega no H2, distante |
| A#3 | Alto | CPO | Persona leiga vs. fricção de exportar o próprio OFX |
| A#9 | Alto | CTO | Tamanho de binário: aposta não medida, invalidação tardia |
| A#10 | Alto | CTO | Fronteira sem handle dói no caso avançado que o produto suporta |
| A#11 | Alto | CTO | Features de recorrência exigem estado que o anti-roadmap veta |
| A#4 | Médio | CPO | Diferencial de privacidade pode ser invisível pra persona |
| A#5 | Médio | CPO | 5 Message Sets vs. foco cartão — escopo não-justificado por valor |
| A#6 | Médio | CPO | Sem métrica de sucesso definida |
| A#12 | Médio | CTO | Golden values sem fonte de verdade independente |
| A#13 | Médio | CTO | MPL-2.0 baixa a barreira para concorrente comercial |
| A#14 | Médio | CTO | CI se valida contra as próprias premissas |
| A#17 | Médio | Cross | Handoff promete recorrência que o roadmap não entrega cedo |
| A#15 | Baixo | CTO | Símbolos de métrica (`k_d`) sem definição |
| A#16 | Baixo | CTO | Consumir `ofx-rs` vs. reescrever não foi decidido explicitamente |
| A#18 | Baixo | Cross | Coerência forte da espinha dorsal (achado positivo) |
| A#29 | Crítico | Cross | A integral das decisões ótimas → nada demonstrável cedo |
| A#19 | Alto | CPO | Privacidade estrutural amputa o loop de feedback de qualidade |
| A#20 | Alto | CPO | Entrega dado, não entendimento — persona leiga não dá o salto sozinha |
| A#23 | Alto | CTO | Parser sobre input hostil sem fuzzing como entregável |
| A#24 | Alto | CTO | WASM single-thread congela a UI; Web Worker não desenhado (pode reverter ADR-06) |
| A#21 | Médio | CPO | Modo avançado é outro produto disfarçado de toggle |
| A#22 | Médio | CPO | Sem drill-down até a transação, o primeiro número em que o usuário discorda quebra a confiança |
| A#25 | Médio | CTO | Migração de schema serializado sem dono |
| A#26 | Médio | CTO | Determinismo de `f64` na fronteira não cercado por invariante |
| A#27 | Médio | CTO | Release cross-registry (crates.io+npm) frágil para solo |
| A#28 | Médio | CTO | Build WASM não reprodutível ao longo do tempo |
| A#30 | Médio | Cross | Tese "nada sai" proíbe até telemetria de uso — decidir, não erodir |
| A#31 | Médio | CTO | Domínio é semanticamente OFX; Open Finance como "só um formato" não comprovado |
| A#32 | Baixo | Cross | Nome "ofx" viola a própria regra de nome conceitual, dada a ambição multi-formato |
| A#33 | Baixo | Cross | Disciplina de anti-escopo é a peça mais madura (achado positivo) |
