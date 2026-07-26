# Handoff — ofx para UX, Design e Frontend

| Campo | Valor |
|---|---|
| Fonte | Discovery/ADR completo (`discovery-ofx-rust-wasm.md`) — este documento traduz, não substitui |
| Público | UX, Design de produto, Frontend |
| Estado da biblioteca | Discovery fechado, zero decisões pendentes; código ainda não escrito além de `ofx-domain` |
| Propósito | Dar contexto suficiente para desenhar jornadas, personas, telas, componentes, copy e acessibilidade sem precisar ler o Discovery técnico inteiro |

---

## 1. O que este produto é, em uma frase

Uma página (ou app) que lê o extrato/fatura bancário do usuário — inteiramente no navegador dele, sem enviar o arquivo a servidor nenhum — e devolve análise de gasto que hoje só apps com acesso aos seus dados oferecem. **Privacidade não é uma feature secundária, é a proposta central**: todo o processamento acontece no dispositivo do usuário. Isso deve aparecer no design como trust signal explícito, não como detalhe técnico enterrado num rodapé.

O motor por trás (parsing + cálculo) é uma biblioteca Rust compilada para WASM. Vocês não constroem essa parte — vocês constroem a interface que a consome. Este documento existe para que a interface reflita as decisões que já foram tomadas sobre o que a biblioteca calcula, como ela reage a erro, e o que ela nunca vai fazer.

---

## 2. Persona

### Primária — a pessoa para quem tudo é desenhado primeiro

Leiga em finanças e em tecnologia. Tem um ou mais cartões de crédito. Exporta o OFX da fatura pelo site do banco (ou já tem os arquivos salvos) e quer entender o próprio gasto — não sabe o que é TWR, VaR ou Sharpe, e não precisa saber. As perguntas dela, na ordem em que provavelmente as faz:

1. "Quanto eu gastei?"
2. "Com o quê?"
3. "É normal esse valor?" / "Tem algo estranho aí?"
4. "Quanto ainda falta pagar do que parcelei?"
5. "Esse arquivo tá certo? Posso confiar nesses números?"

Ela pode ter mais de um cartão, e pode ter um arquivo de banco com mais de uma conta dentro (corrente + poupança no mesmo OFX). Ela **não** vai entender um erro técnico. Toda mensagem de erro ou aviso precisa ser traduzida para linguagem de "o que aconteceu com o meu dinheiro", nunca para "o que aconteceu com o parsing".

### Secundária — modo avançado

Usuário com conhecimento financeiro que quer métricas de investimento (retorno, risco, volatilidade) sobre um extrato de corretora. Este modo **não é o padrão** — fica atrás de um toggle explícito, ativado pela pessoa (decidido: nunca detecção automática de conteúdo). O toggle reconfigura a UI inteira, como um "modo desenvolvedor" — ver §3 e Jornada 6.

---

## 3. Princípios que restringem o design (não são sugestões)

Estes vêm de decisões arquiteturais fechadas — o design precisa respeitá-los, não contorná-los:

- **Nada sai do navegador.** Nenhuma tela deve sugerir upload, sincronização com servidor, login, ou conta de usuário. Não há "esqueci minha senha" porque não há conta. Se o produto precisar de persistência entre sessões (lembrar arquivos já importados), isso é `localStorage`/`IndexedDB` do lado do app — decisão de implementação, não da lib.
- **Consolidar múltiplos arquivos é sempre uma ação explícita do usuário, nunca automática.** A lib nunca junta arquivos sozinha. Isso tem que virar um botão/confirmação visível ("ver gasto combinado dos dois cartões"), não um comportamento invisível.
- **Todo número vem acompanhado de proveniência.** Cada métrica carrega de onde veio (janela de tempo, quantas transações entraram, parâmetros usados). Isso é ativo de confiança para a persona leiga — o design deveria expor isso de forma leve (ex.: um "i" expansível), não escondido, porque é exatamente o que resolve a pergunta 5 da persona ("posso confiar nesse número?").
- **Múltiplas moedas nunca são somadas.** Se o usuário importa um cartão em BRL e outro em USD, a consolidação mostra os dois totais separados, nunca um número só. O design precisa de um padrão visual para "isto é mais de uma moeda" que não pareça bug.
- **Todo mês/gasto anômalo/erro tem uma explicação em português simples pronta.** O Discovery já escreveu vários exemplos de copy (ver §6) — são ponto de partida, não texto final, mas o tom (curto, sem jargão, sem culpar o usuário) é intencional.
- **O modo avançado é uma reconfiguração completa da UI/UX, não um toggle raso.** Decisão fechada: ativar o modo avançado troca o esqueleto de navegação, vocabulário e telas disponíveis inteiro — como um "modo desenvolvedor" — nunca é a mesma tela da persona leiga com alguns campos extras aparecendo. As duas personas (leiga de cartão e investidor avançado) têm jornadas incompatíveis; misturar os dois vocabulários numa tela só é exatamente o que degrada as duas experiências. Isso é decisão de estrutura de navegação (quais telas existem em cada modo), não de visibilidade de campo — pensem nos dois modos como dois apps que compartilham o mesmo motor de dados, não como uma tela com um switch.
- **Toda chamada à biblioteca é assíncrona, e nada roda na thread principal.** O motor (WASM) vive dentro de um Web Worker — decisão fechada, não negociável. Consequência prática pro frontend: toda função da API retorna `Promise`, sem exceção, inclusive as que "parecem" instantâneas. Consequência pro design: **todo número na tela tem um estado de carregamento**, mesmo que na maioria das vezes ele apareça rápido demais pra ser visto. Arquivos grandes ou consolidação de muitos meses podem levar segundos — a UI precisa continuar respondendo (scroll, navegação, cancelar) enquanto isso acontece. Não desenhem telas que assumem dado disponível na hora que a tela monta.
- **Nenhuma telemetria de produto, em nenhuma forma, nem anônima.** Diferente do canal de reporte de arquivo problemático (que é opt-in e sobre a estrutura de um arquivo específico, nunca sobre uso), não existe contagem de uso, dashboard de features tocadas, ou qualquer "ping" de analytics de produto. Nenhuma tela deve incluir esse tipo de instrumentação, mesmo com o argumento de "é só anônimo".

---

## 4. Jornadas de usuário

Cada jornada abaixo corresponde a um cenário já mapeado tecnicamente no Discovery (com diagrama de sequência, se quiserem ver o fluxo de chamadas exato). Aqui, tradução para o que a pessoa vê e faz. **Cada uma traz a partir de qual Milestone do roadmap (`roadmap-github.md`) ela fica disponível de verdade** — desenhar a tela antes disso é desenhar pra capacidade que ainda não existe (A#17).

### Jornada 1 — Primeira fatura (onboarding funcional)
**Disponível a partir do Milestone 1** (fatia vertical mínima, piloto Nubank) — total gasto e fluxo líquido básicos existem aqui; a robustez a arquivo malformado (Jornada 5) só amadurece no Milestone 2.
A pessoa chega ao produto, arrasta ou seleciona o arquivo `.ofx` da fatura. Sem cadastro, sem espera longa. Resultado: total gasto no período, o fluxo líquido (o que inclui o pagamento da fatura — ver §7 sobre a diferença entre os dois números, que é sutil e fácil de confundir visualmente) e um indicador de saúde do arquivo. Este é o momento zero — a primeira impressão do produto inteiro se forma aqui, e ela precisa ser rápida e sem fricção.

### Jornada 2 — Aprofundando no mês
**Disponível a partir do Milestone 4** (analytics de gasto da persona leiga) — é onde `group_by_payee`, `period_series`, `extremes`, `anomaly`, `credit::utilization`, `open_installments`, `scheduled_payments` e o drill-down existem de fato; não desenhar esta tela completa antes disso.
Depois de ver o total, a pessoa quer detalhe: gasto por estabelecimento, evolução mês a mês, maior e menor lançamento, transações fora do padrão (anomalias), utilização do limite de crédito, parcelas em aberto (quanto já pagou, quanto falta de cada compra parcelada) e — quando o banco fornece o dado — contas agendadas para pagamento automático. Esta é provavelmente a tela mais densa de informação do produto — seis a oito blocos de dado diferentes sobre o mesmo arquivo. Vale pensar em hierarquia visual forte e progressive disclosure (não jogar tudo com o mesmo peso). **Cada número desta tela precisa ser clicável até os lançamentos que o compuseram** (drill-down, §5) — é a tela onde a confiança se ganha ou se perde.

### Jornada 3 — Múltiplos cartões
**Disponível a partir do Milestone 3** (recorrência e consolidação) — é onde `consolidate_by_fitid` e a comparação por conta existem.
A pessoa já importou duas ou mais faturas de cartões diferentes, separadamente. Em algum momento decide "quero ver os dois juntos" — ação explícita (botão, nunca automático). O sistema consolida, pode achar transações duplicadas (arquivo reimportado por engano) e pode achar conflitos genuínos (mesma referência, valores diferentes em duas faturas — precisa de um aviso visível e específico, não um erro genérico). Depois de consolidado, a pessoa pode comparar cartão a cartão lado a lado.

### Jornada 4 — Um arquivo, várias contas
**Disponível desde o Milestone 1**, no básico — `Document` já carrega lista plana de contas desde o domínio (ADR-04); mas o seletor de conta só fica realmente útil a partir do Milestone 4, quando há mais funções pra comparar entre contas além do total.
Alguns bancos exportam conta corrente e poupança no mesmo arquivo. A pessoa importa um único arquivo e precisa poder alternar entre "só a conta X", "só a conta Y" e "as duas juntas" — sem precisar de um segundo arquivo nem do fluxo de consolidação da Jornada 3 (são mecanismos diferentes por trás, mas a pessoa não precisa perceber a diferença; a UI deveria abstrair isso com o mesmo seletor de conta/cartão).

### Jornada 5 — Arquivo problemático
**Disponível a partir do Milestone 2** (parsing resiliente sobre a dor conhecida) — é onde a resiliência a dialeto, o corpus e os diagnostics estruturados existem; o Milestone 1 (piloto Nubank) só cobre o caminho feliz.
A pessoa importa um arquivo de um banco com formatação ruim (comum na prática). O sistema nunca aborta silenciosamente — ele processa o que consegue e expõe o que teve que corrigir ou não conseguiu confiar (ex.: "algumas transações desse banco tinham um identificador que não é confiável, então nada foi descartado por engano"). Isso precisa aparecer como parte do relatório de saúde do arquivo, não como uma barra de erro vermelha assustadora — o tom é "avisamos, mas cuidamos disso pra você".

### Jornada 6 — Modo avançado (investimento)
**Disponível a partir do Milestone 6** (analytics avançado de investimento) — é a última capacidade do roadmap principal a amadurecer; não é prioridade de design antes disso.
Perfil secundário. A pessoa tem um extrato de corretora e ativa o modo avançado — um toggle tipo "modo desenvolvedor" que reconfigura a interface inteira, não um switch que só libera campos extras na mesma tela (ver §3). A partir daí pede retorno (TWR) e risco (Sharpe, VaR) do período — com os parâmetros usados (taxa livre de risco, nível de confiança do VaR) sempre visíveis, porque mudam o resultado.

### Jornada 7 — Entradas inválidas
**Disponível desde o Milestone 1** — o teto de tamanho (`maxBytes`) e o erro estruturado já fazem parte da fronteira mínima (ADR-06); a segunda causa de rejeição (arquivo ilegível) amadurece junto da resiliência do Milestone 2.
Duas causas distintas de rejeição, e a mensagem para cada uma precisa ser diferente e acionável: (a) arquivo grande demais — mensagem sugere exportar um período menor; (b) arquivo não é um OFX válido — mensagem pede para confirmar o formato. Nenhuma das duas pode ser um erro genérico tipo "algo deu errado".

---

## 5. Arquitetura de informação — visões que a UI precisa cobrir

Estas são as unidades de informação que a biblioteca já sabe produzir. Cada uma é candidata a uma tela, seção ou componente — a decisão de agrupamento é do design, mas nenhuma pode ficar de fora:

| Visão | Escopo (por cartão / todos juntos) | Observação de design |
|---|---|---|
| Total gasto no período | ambos | Exclui pagamento da fatura anterior |
| Fluxo líquido do período | ambos | Inclui o pagamento — **diferente do total gasto**, risco real de confusão visual se os dois aparecerem próximos sem rótulo claro |
| Evolução mês a mês | ambos | Série temporal — gráfico de linha ou barras |
| Maior/menor lançamento, ticket médio | ambos | |
| Gasto por estabelecimento | ambos | Nomes de estabelecimento chegam "sujos" (prefixo de maquininha, caixa alta) — já vêm normalizados pela biblioteca, mas o texto original também fica disponível se o design quiser mostrar os dois |
| Transações anômalas | ambos | Precisa de indicador visual não-alarmista — é "fora do padrão", não necessariamente "errado" |
| Utilização do limite de crédito | só por cartão | Pode vir indisponível (arquivo não trouxe o limite) — tratar como estado "sem dado", nunca mostrar zero ou traço genérico |
| Comparação entre cartões | consolidado | Lado a lado, particionado por moeda quando houver mais de uma |
| Compras parceladas em andamento | ambos | Total, pago, restante — por compra |
| Contas agendadas/recorrentes | ambos | **Condicional** — só existe se o banco preencher esse bloco no arquivo (nem todos preenchem). Precisa de estado "seu banco não fornece essa informação", distinto de "você não tem contas agendadas" |
| Drill-down até a transação | ambos | **Não é uma tela própria, é um comportamento de toda tela numérica** — clicar em qualquer total, extremo ou grupo de estabelecimento navega até os lançamentos que o compuseram. É o que sustenta a confiança (§7); sem isso, o primeiro número que a pessoa achar errado derruba a confiança em todos os outros |
| Saúde do arquivo importado | qualquer arquivo | Score de confiança + lista de achados em linguagem simples |
| Retorno e risco (modo avançado) | por extrato de investimento | Sempre com os parâmetros usados visíveis |

---

## 6. Estados, erros e copy — ponto de partida

A biblioteca já expõe uma taxonomia de situações que precisam de tratamento visual e textual distintos. Nenhuma delas deveria virar um alerta genérico. Frases entre aspas abaixo são exemplos já escritos no Discovery — reescrevam à vontade, mas mantenham o tom (curto, sem jargão, sem culpar o usuário, sempre dizendo o que fazer a seguir quando aplicável):

**Rejeição antes mesmo de tentar ler o arquivo**
- Arquivo grande demais: *"Este arquivo passa do limite configurado — tente exportar um período menor."*
- Arquivo ilegível / não é um OFX: *"Não foi possível ler este arquivo — confirme que é um OFX válido."*

**Correções silenciosas que o sistema já fez sozinho** (não são erro, mas devem ficar rastreáveis em algum "ver detalhes")
- Separador decimal corrigido (vírgula → ponto)
- Fuso horário assumido quando o arquivo não informa
- Tag desconhecida do banco, ignorada com segurança

**Avisos que exigem atenção da pessoa, mas não travam nada**
- Campo obrigatório ausente numa transação específica
- *"Uma transação aparece com valores diferentes em duas faturas"* (conflito real de dado, não duplicata)
- *"2 transações apareceram em duas faturas e foram contadas uma vez só"* (duplicata detectada e resolvida)
- Identificador de transação não confiável nesse banco — nada foi descartado por precaução

**Dado ausente, não incorreto**
- Limite de crédito não veio no arquivo → mostrar "não informado", nunca inventar um número
- Métrica sem dado suficiente (ex.: poucas transações para calcular uma média significativa) → mostrar "dado insuficiente ainda", nunca `NaN`, `Infinity` ou um zero que pareça resultado real

**Ao redor de todos esses estados:** o relatório de saúde do arquivo é o lugar natural para agregar tudo isso num placar de confiança, para a persona leiga não precisar interpretar cada aviso técnico isoladamente.

---

## 7. Duas armadilhas de copy que merecem atenção deliberada

1. **"Total gasto" vs. "Fluxo líquido"** — dois números parecidos, calculados de propósito de forma diferente (um exclui o pagamento da fatura anterior, o outro inclui). Se aparecerem na mesma tela sem diferenciação clara de rótulo e possivelmente de hierarquia visual, o risco de a pessoa achar que o sistema tem um bug é real. Recomendo rótulos que não dependam só da palavra "líquido" (jargão contábil) — testar linguagem tipo "o que eu gastei" vs. "o que saiu da conta este mês".

2. **Comparação entre cartões multi-moeda** — quando há mais de uma moeda envolvida, o sistema nunca soma. Isso precisa aparecer como "R$ X · US$ Y" lado a lado, nunca como um total único, e o motivo ("não convertemos automaticamente porque não temos acesso à cotação em tempo real") pode virar um tooltip educativo em vez de parecer uma limitação escondida.

---

## 8. Questões em aberto para o time de design decidir

Estas não são decisões técnicas — são decisões de produto/UX que o Discovery deliberadamente deixou para quem desenha a interface:

- **Representação visual do score de confiança do arquivo:** semáforo (cores), nota numérica, texto narrativo, ou combinação? Evitar depender só de cor (ver acessibilidade, §9).
- **Onde a proveniência de cada métrica aparece:** sempre visível, ou expansível sob demanda (ex.: ícone "i")?
- **Persistência entre sessões:** o produto lembra os arquivos já importados na próxima visita (via armazenamento local do navegador) ou cada sessão começa do zero? Isso muda a estrutura de onboarding inteira.
- **Como comunicar "processamento 100% local"** de forma que a pessoa leiga entenda o valor sem um texto técnico sobre WASM/browser.

---

## 9. Acessibilidade — pontos específicos deste produto

- **Nunca comunicar estado (saudável / com aviso / com erro) só por cor.** O relatório de saúde do arquivo, as anomalias e os conflitos de transação precisam de texto e/ou ícone junto da cor, sempre.
- **Estados assíncronos precisam de anúncio para leitor de tela.** O parsing e o cálculo acontecem no navegador e podem ter latência perceptível em arquivos maiores — usar regiões `aria-live` para o momento em que o resultado fica pronto, em vez de depender só de mudança visual.
- **Números com proveniência expansível precisam ser navegáveis por teclado** e o conteúdo expandido precisa ser lido de forma que fique claro que é contexto adicional daquele número específico (associação programática, não só posição visual).
- **Mensagens de erro (tamanho excedido, arquivo inválido) precisam ser anunciadas no ponto de foco**, não aparecer silenciosamente em outro lugar da tela — a pessoa acabou de interagir com o input de arquivo, é ali que a atenção está.
- **Gráficos de série temporal (evolução mensal) precisam de alternativa em tabela/texto**, não só o gráfico visual, para leitores de tela e para exportação/impressão.
- **O seletor de conta/cartão (jornadas 3 e 4) precisa ser claramente rotulado por nome, nunca só por posição ou cor**, já que a lista pode ter dois ou mais itens parecidos (dois cartões do mesmo banco, por exemplo).

---

## 10. Glossário rápido (termos que vão aparecer em specs técnicas)

| Termo técnico | O que significa em produto |
|---|---|
| `Document` | Os dados de um arquivo importado — pode conter mais de uma conta |
| `Portfolio` | O resultado de juntar vários arquivos (consolidação explícita) |
| Escopo "produto" | Visão de um cartão/conta isolado |
| Escopo "consolidado" | Visão de vários cartões/contas juntos |
| `Diagnostic` | Qualquer aviso, correção ou erro que o sistema registrou ao ler o arquivo |
| Proveniência | De onde um número veio — período, quantidade de transações, parâmetros usados, e quais lançamentos o compuseram (é o que viabiliza o drill-down) |
| Modo avançado | Métricas de investimento (retorno, risco) — não é o padrão; reconfigura a interface inteira, não é um switch de campos |
| Web Worker | Onde o motor de cálculo roda — fora da thread principal, por isso toda chamada é assíncrona |
| Message Set | Bloco do formato OFX por tipo de produto (conta, cartão, investimento, contas agendadas). Nem todo banco preenche todos — daí visões condicionais |

---

## 11. Onde ir para mais detalhe

O Discovery técnico completo (`discovery-ofx-rust-wasm.md`) tem, na seção 12 ("Casos de uso"), sete diagramas de sequência mostrando exatamente quais chamadas acontecem em cada jornada — útil se alguém do time quiser entender o fluxo de dados por trás de uma tela específica antes de desenhá-la. A ADR-13 dentro do mesmo documento tem a tabela completa de visões e a API de consumo com todas as funções disponíveis.
