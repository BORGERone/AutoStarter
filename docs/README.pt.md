# AutoStarter

<!-- language-switcher -->
[English](../README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · [Español](README.es.md) · [العربية](README.ar.md) · **Português** · [Français](README.fr.md) · [Deutsch](README.de.md) · [日本語](README.ja.md) · [हिन्दी](README.hi.md)

![AutoStarter — os seus programas, a sua ordem de arranque. Um gestor para Windows criado com Tauri e Rust.](images/hero.svg)

**Gestor de arranque de programas para Windows com configuração do comportamento das janelas e suporte à bandeja do sistema.**

O AutoStarter (AutoStart Manager na interface) inicia as aplicações selecionadas sempre que ele próprio arranca. Adicione os programas de que precisa, configure o comportamento deles e ative o arranque automático do próprio gestor — assim o seu conjunto de aplicações será iniciado ao entrar no Windows.

O projeto foi construído com **Tauri 1, Rust, TypeScript e Vite 5**.

> **Apenas Windows.** A aplicação usa o registo e a API Win32; executar e compilar a versão de ambiente de trabalho em Linux e macOS não é suportado na implementação atual.

## Conteúdo

- [Funcionalidades](#funcionalidades)
- [Início rápido](#início-rápido)
- [Modos de arranque dos programas](#modos-de-arranque-dos-programas)
- [Definições do gestor](#definições-do-gestor)
- [Instalação e compilação a partir do código-fonte](#instalação-e-compilação-a-partir-do-código-fonte)
- [Armazenamento de dados e arranque automático do Windows](#armazenamento-de-dados-e-arranque-automático-do-windows)
- [Perguntas frequentes](#perguntas-frequentes)
- [Estrutura do projeto](#estrutura-do-projeto)
- [Contribuir](#contribuir)
- [Licença](#licença)

## Funcionalidades

- **A sua própria lista de programas:** adicione ficheiros `.exe`, atalhos `.lnk` e scripts `.bat` / `.cmd` através da caixa de diálogo de seleção de ficheiros.
- **Ativação independente:** exclua temporariamente um programa do arranque sem o remover da lista.
- **Controlo de janelas:** arranque normal, minimizado ou oculto, além do envio de um comando de fecho depois de detetar o processo.
- **Arranque ao entrar no Windows:** início automático do gestor para o utilizador atual.
- **Suporte à bandeja:** ocultar a janela principal, restaurá-la e sair através do menu de contexto do ícone.
- **Modo sem interface:** inicia os programas ativados e encerra o gestor após 30 segundos.
- **Definições locais:** a lista de programas e os parâmetros são guardados em ficheiros JSON.
- **10 idiomas de interface:** russo, inglês, chinês, espanhol, árabe, português, francês, alemão, japonês e hindi.

## Início rápido

![Três passos: adicione programas e ative os respetivos cartões, configure os modos e o arranque automático do gestor, entre no Windows. Iniciar o gestor manualmente também executa as entradas ativadas.](images/workflow.svg)

Se a aplicação ainda não estiver compilada, siga primeiro as [instruções de compilação](#instalação-e-compilação-a-partir-do-código-fonte).

1. Execute `tauri-app.exe` — é este o nome atual do executável do projeto.
2. Abra as definições com o botão no canto inferior direito e escolha **Language → Português**.
3. Clique em **«+ Adicionar»** e selecione um programa ou script.
4. Se necessário, marque **«Minimizado»**, **«Oculto»** ou **«Fechar»** no cartão do programa.
5. Coloque o interruptor do cartão em **«On»**. As novas entradas estão desativadas por predefinição.
6. Nas definições ative **«Arranque automático»** se quiser iniciar o gestor com os programas selecionados ao entrar no Windows.

As alterações são guardadas automaticamente. O botão de eliminar remove apenas a entrada da lista — o ficheiro do programa permanece no disco.

> **Importante:** os programas ativados são executados a cada novo arranque do processo AutoStarter, incluindo o arranque manual. Adicionar uma entrada ou colocá-la em «On» não a executa imediatamente. Para verificar a configuração, encerre totalmente o gestor através de **Quit** na bandeja e abra-o de novo. As aplicações já em execução podem ser iniciadas outra vez.

## Modos de arranque dos programas

| Modo no cartão | Comportamento |
| --- | --- |
| Sem marcações adicionais | Arranque normal através de `cmd /c start`. |
| **Minimizado** | Arranque através de `cmd /c start /min`, pedindo uma janela minimizada. |
| **Oculto** | Arranque através do PowerShell `Start-Process -WindowStyle Hidden`. É um pedido para ocultar a janela, não uma garantia de que o programa apareça na bandeja. |
| **Fechar** | Arranque normal e, em seguida, procura do processo pelo nome do ficheiro durante cerca de 30 segundos. Depois de o detetar, o gestor espera mais 0,5 segundos e envia o comando de fecho do sistema às janelas visíveis do processo. |
| **Off / On** | Exclui ou inclui a entrada no arranque no próximo início do gestor. |

**Particularidades e limitações:**

- «Minimizado» e «Oculto» excluem-se mutuamente na interface.
- «Fechar» tem prioridade sobre ambos os modos e não significa terminar o processo à força. A aplicação de destino pode fechar-se, minimizar-se na sua própria bandeja, mostrar uma caixa de diálogo ou ignorar o pedido.
- A procura para «Fechar» é feita pelo nome do ficheiro, não pelo caminho completo nem pelo identificador da instância iniciada. Por isso, o comando pode afetar um processo já em execução com o mesmo nome. No caso de atalhos e scripts, o nome do ficheiro selecionado normalmente não coincide com o nome do processo real.
- Algumas aplicações gerem as suas próprias janelas e podem ignorar o arranque oculto ou minimizado.
- A interface não permite configurar argumentos da linha de comandos, diretório de trabalho nem atrasos individuais.

Adicione apenas programas e scripts de confiança. Não use «Fechar» em aplicações com dados por guardar sem testar antes.

## Definições do gestor

| Definição | O que faz |
| --- | --- |
| **Idioma** | Altera o idioma da interface; o inglês está selecionado por predefinição. |
| **Iniciar na bandeja** | Oculta a janela principal ao arrancar a versão normal e mostra um ícone na área de notificação. |
| **Arranque automático** | Adiciona o AutoStarter ao arranque automático do Windows para o utilizador atual. Desativado por predefinição. |
| **Fechar após execução** | No arranque automático do Windows adiciona o parâmetro `--close-after-30`: o gestor executa os programas ativados, espera 30 segundos e termina sem criar a interface nem o ícone da bandeja. Disponível apenas com o «Arranque automático» ativado. |

**«Fechar» no cartão e «Fechar após execução» nas definições são funções diferentes:** a primeira diz respeito às janelas do programa selecionado, a segunda ao próprio gestor. O intervalo de 30 segundos é fixo e não significa esperar que todas as aplicações iniciadas terminem.

Um arranque manual normal sem o parâmetro abre o gestor de acordo com a definição «Iniciar na bandeja», mesmo que «Fechar após execução» esteja ativado.

### Controlo através da bandeja

- **O botão de fechar da janela** oculta o gestor na bandeja em vez de o encerrar.
- **Um clique no ícone** ou a opção **Show** devolve a janela principal.
- **Clique com o botão direito → Quit** encerra totalmente o gestor.

Os nomes dos itens do menu da bandeja são atualmente apresentados em inglês, independentemente do idioma da interface.

## Instalação e compilação a partir do código-fonte

### Requisitos

A versão gráfica requer o **Microsoft Edge WebView2 Runtime**. Um ambiente de compilação prático é o Windows 10/11 com o seguinte instalado:

- [Node.js](https://nodejs.org/) **20 ou superior** e npm; recomenda-se a versão LTS atual.
- [Rust](https://rustup.rs/) — toolchain stable atual para Windows MSVC. O `Cargo.toml` indica um mínimo de 1.70, mas as dependências do ficheiro lock podem exigir uma versão mais recente.
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) com os componentes **Desktop development with C++**, MSVC e Windows SDK.
- [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).
- [Git](https://git-scm.com/) para clonar o repositório.

Mais informações: [preparação do ambiente Tauri 1 para Windows](https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-windows).

### Obter o código-fonte

No PowerShell execute:

```powershell
git clone https://github.com/BORGERone/AutoStarter.git
cd AutoStarter
npm ci
```

O `npm ci` instala as dependências de acordo com o `package-lock.json`. A primeira compilação também precisa de acesso à rede para descarregar as dependências de Rust e as ferramentas de empacotamento.

### Modo de desenvolvimento

```powershell
npm run tauri-dev
```

Ou execute o `dev.bat` a partir do Explorador. O Tauri levanta automaticamente o Vite na porta **5174** e abre a janela de ambiente de trabalho.

> O `npm run dev` inicia apenas a interface web. Num navegador normal não estão disponíveis as funcionalidades do Tauri: seleção de ficheiros pela caixa de diálogo nativa, guardar definições, controlo da bandeja e do registo. Use `npm run tauri-dev` para uma verificação completa da aplicação.

### Compilação com instaladores

```powershell
npm run tauri-build
```

O comando compila a interface, a aplicação Rust e os pacotes definidos pela configuração do Tauri. Os resultados encontram-se em:

- `src-tauri/target/release/tauri-app.exe` — o executável;
- `src-tauri/target/release/bundle/` — os pacotes de instalação criados pelo empacotador.

### Compilação sem instalador

```powershell
.\build.bat
```

O script executa sequencialmente `npm run build` e `cargo build --release` no diretório `src-tauri`. O resultado é `src-tauri/target/release/tauri-app.exe`. A interface gráfica continua a exigir o WebView2 Runtime.

Execução manual do modo sem interface:

```powershell
.\src-tauri\target\release\tauri-app.exe --close-after-30
```

### Verificação da interface

```powershell
npm run build
```

O comando executa a verificação de TypeScript e a compilação de produção do Vite. Não verifica o código Rust nem o comportamento da API Win32. Atualmente não existe um script de testes automáticos separado no `package.json`.

## Armazenamento de dados e arranque automático do Windows

As definições do utilizador atual são guardadas no diretório:

```text
%APPDATA%\tauri-launcher\
├── programs.json    # Lista de programas, caminhos e modos de arranque
└── settings.json    # Idioma e definições do gestor
```

Para uma cópia de segurança, encerre totalmente o gestor e copie ambos os ficheiros. Ao transferir para outro computador, verifique os caminhos dos programas. Os parâmetros de arranque automático no registo não fazem parte desta cópia: depois da transferência, ative novamente o arranque automático a partir da instância pretendida da aplicação.

Ao ativar o «Arranque automático» é criado o valor **`AutoStartManager`** na chave:

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
```

Aí é guardado o caminho para o executável atual do gestor e, se necessário, o parâmetro `--close-after-30`. Trata-se de um arranque **no início de sessão do utilizador atual**, e não de um serviço do sistema. Não são criadas entradas separadas para cada programa adicionado; o gestor não administra o arranque automático existente de outras aplicações.

**Desative o «Arranque automático» antes de mover ou eliminar o executável.** Depois de o mover, abra o gestor a partir da nova localização e ative-o novamente. Para uma reposição completa, desative primeiro o arranque automático, encerre a aplicação através de Quit e depois elimine o diretório `%APPDATA%\tauri-launcher` — isto remove a lista guardada e as definições.

## Perguntas frequentes

**Adicionei um programa, mas ele não arranca.**

Verifique o interruptor «On» no cartão respetivo e se o ficheiro existe no caminho indicado. Reinicie o gestor por completo: alterar a lista, por si só, não inicia nada. Para o arranque ao entrar no Windows, o «Arranque automático» também tem de estar ativado nas definições do gestor.

**As aplicações abrem duas vezes.**

O AutoStarter não verifica se um programa já está em execução. Verifique a definição de arranque automático do próprio programa e a lista de arranque do Windows. Reiniciar o próprio gestor também volta a executar todas as entradas ativadas.

**Depois de clicar no botão de fechar, o gestor continua a funcionar.**

É o comportamento esperado. Encontre o ícone do AutoStart Manager na área de notificação, expanda os ícones ocultos se necessário, e escolha Quit.

**A aplicação não aparece depois de entrar no Windows.**

Verifique «Iniciar na bandeja» e «Fechar após execução»: no primeiro caso a janela está oculta, no segundo não é criada qualquer interface. Para alterar as definições, execute o ficheiro manualmente sem o parâmetro `--close-after-30`.

**A compilação não encontra o linker ou o Windows SDK.**

Verifique a instalação das C++ Build Tools e da toolchain MSVC do Rust e reinicie o terminal. Para problemas de apresentação da janela, verifique a presença do WebView2 Runtime.

**O modo de desenvolvimento indica que a porta está ocupada.**

Liberte a porta 5174, por exemplo terminando a instância anterior do Vite. A porta é usada tanto na configuração do Vite como na do Tauri.

## Estrutura do projeto

```text
AutoStarter/
├── index.html                 # Marcação da janela principal e das definições
├── src/
│   ├── main.ts                # Interface, traduções e chamadas ao Tauri
│   └── styles.css             # Estilos da aplicação
├── src-tauri/
│   ├── src/main.rs            # Arranque de programas, JSON, registo e bandeja Win32
│   ├── Cargo.toml             # Dependências e parâmetros de Rust
│   ├── Cargo.lock             # Dependências de Rust fixadas
│   ├── build.rs               # Script de compilação do Tauri
│   └── tauri.conf.json        # Janela, permissões e empacotamento
├── icon.ico                   # Ícone da aplicação e da bandeja
├── package.json               # Comandos npm e dependências da interface
├── package-lock.json          # Dependências npm fixadas
├── vite.config.ts             # Definições do Vite
├── build.bat                  # Compilação release sem instalador
└── dev.bat                    # Arranque do modo de desenvolvimento
```

## Contribuir

Relatórios de erros e sugestões podem ser deixados nos [Issues](https://github.com/BORGERone/AutoStarter/issues). Para um relatório reproduzível, indique a versão do Windows, a forma de execução ou compilação, os passos e o comportamento esperado. Antes de publicar registos ou ficheiros JSON, remova dados pessoais e caminhos privados.

Para alterações na interface execute `npm run build`; as alterações ao arranque, ao registo e à bandeja devem ser adicionalmente verificadas na versão de ambiente de trabalho no Windows. É preferível acompanhar o pull request com uma descrição das alterações e das verificações efetuadas.

## Licença

No `src-tauri/Cargo.toml` está indicada a licença **MIT**. Um ficheiro `LICENSE` separado com o texto completo da licença ainda não existe no repositório.
