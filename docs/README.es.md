# AutoStarter

<!-- language-switcher -->
[English](../README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · **Español** · [العربية](README.ar.md) · [Português](README.pt.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [日本語](README.ja.md) · [हिन्दी](README.hi.md)

![AutoStarter: tus programas, tu orden de inicio. Un gestor para Windows creado con Tauri y Rust.](images/hero.svg)

**Gestor de inicio de programas para Windows con configuración del comportamiento de las ventanas y soporte de bandeja del sistema.**

AutoStarter (AutoStart Manager en la interfaz) inicia las aplicaciones seleccionadas cuando él mismo arranca. Añade los programas que necesites, configura su comportamiento y activa el inicio automático del propio gestor: así tu conjunto de aplicaciones se iniciará al entrar en Windows.

El proyecto está construido con **Tauri 1, Rust, TypeScript y Vite 5**.

> **Solo Windows.** La aplicación usa el registro y la API Win32; la ejecución y compilación de la versión de escritorio en Linux y macOS no está admitida en la implementación actual.

## Contenido

- [Características](#características)
- [Inicio rápido](#inicio-rápido)
- [Modos de inicio de los programas](#modos-de-inicio-de-los-programas)
- [Ajustes del gestor](#ajustes-del-gestor)
- [Instalación y compilación desde el código fuente](#instalación-y-compilación-desde-el-código-fuente)
- [Almacenamiento de datos e inicio automático de Windows](#almacenamiento-de-datos-e-inicio-automático-de-windows)
- [Preguntas frecuentes](#preguntas-frecuentes)
- [Estructura del proyecto](#estructura-del-proyecto)
- [Contribuir](#contribuir)
- [Licencia](#licencia)

## Características

- **Tu propia lista de programas:** añade archivos `.exe`, accesos directos `.lnk` y scripts `.bat` / `.cmd` mediante el diálogo de selección de archivos.
- **Activación independiente:** excluye temporalmente un programa del inicio sin eliminarlo de la lista.
- **Control de ventanas:** inicio normal, minimizado u oculto, además del envío de un comando de cierre tras detectar el proceso.
- **Inicio al entrar en Windows:** arranque automático del gestor para el usuario actual.
- **Soporte de bandeja:** ocultar la ventana principal, restaurarla y salir desde el menú contextual del icono.
- **Modo sin interfaz:** inicia los programas activados y cierra el gestor a los 30 segundos.
- **Ajustes locales:** la lista de programas y los parámetros se guardan en archivos JSON.
- **10 idiomas de interfaz:** ruso, inglés, chino, español, árabe, portugués, francés, alemán, japonés e hindi.

## Inicio rápido

![Tres pasos: añade programas y activa sus tarjetas, configura los modos y el inicio automático del gestor, entra en Windows. Iniciar el gestor manualmente también ejecuta las entradas activadas.](images/workflow.svg)

Si la aplicación aún no está compilada, sigue primero las [instrucciones de compilación](#instalación-y-compilación-desde-el-código-fuente).

1. Ejecuta `tauri-app.exe`: así se llama actualmente el ejecutable del proyecto.
2. Abre los ajustes con el botón de la esquina inferior derecha y elige **Language → Español**.
3. Pulsa **«+ Añadir»** y selecciona un programa o script.
4. Si lo necesitas, marca **«Minimizado»**, **«Oculto»** o **«Cerrar»** en la tarjeta del programa.
5. Pon el interruptor de la tarjeta en **«On»**. Las entradas nuevas están desactivadas por defecto.
6. En los ajustes activa **«Inicio automático»** si quieres que el gestor y los programas seleccionados se inicien al entrar en Windows.

Los cambios se guardan automáticamente. El botón de eliminar solo quita la entrada de la lista: el archivo del programa permanece en el disco.

> **Importante:** los programas activados se ejecutan cada vez que arranca un nuevo proceso de AutoStarter, incluido el arranque manual. Añadir una entrada o ponerla en «On» no la ejecuta de inmediato. Para comprobar la configuración, cierra completamente el gestor con **Quit** en la bandeja y vuelve a abrirlo. Las aplicaciones que ya se estén ejecutando pueden iniciarse de nuevo.

## Modos de inicio de los programas

| Modo en la tarjeta | Comportamiento |
| --- | --- |
| Sin marcas adicionales | Inicio normal mediante `cmd /c start`. |
| **Minimizado** | Inicio mediante `cmd /c start /min`, solicitando una ventana minimizada. |
| **Oculto** | Inicio mediante PowerShell `Start-Process -WindowStyle Hidden`. Es una solicitud de ocultar la ventana, no una garantía de que el programa aparezca en la bandeja. |
| **Cerrar** | Inicio normal y, a continuación, búsqueda del proceso por nombre de archivo durante unos 30 segundos. Tras detectarlo, el gestor espera otros 0,5 segundos y envía el comando de cierre del sistema a las ventanas visibles del proceso. |
| **Off / On** | Excluye o incluye la entrada en el arranque la próxima vez que se inicie el gestor. |

**Particularidades y limitaciones:**

- «Minimizado» y «Oculto» son mutuamente excluyentes en la interfaz.
- «Cerrar» tiene prioridad sobre ambos modos y no significa forzar la terminación del proceso. La aplicación de destino puede cerrarse, minimizarse en su propia bandeja, mostrar un diálogo o ignorar la solicitud.
- La búsqueda para «Cerrar» se realiza por nombre de archivo, no por la ruta completa ni por el identificador de la instancia iniciada. Por eso el comando puede afectar a un proceso con el mismo nombre que ya esté en ejecución. Para accesos directos y scripts, el nombre del archivo seleccionado normalmente no coincide con el del proceso real.
- Algunas aplicaciones gestionan sus propias ventanas y pueden ignorar el inicio oculto o minimizado.
- La interfaz no permite configurar argumentos de línea de comandos, directorio de trabajo ni retardos individuales.

Añade solo programas y scripts de confianza. No uses «Cerrar» con aplicaciones que tengan datos sin guardar sin comprobarlo antes.

## Ajustes del gestor

| Ajuste | Qué hace |
| --- | --- |
| **Idioma** | Cambia el idioma de la interfaz; por defecto está seleccionado el inglés. |
| **Iniciar en bandeja** | Oculta la ventana principal al arrancar la versión normal y muestra el icono en el área de notificación. |
| **Inicio automático** | Añade AutoStarter al inicio automático de Windows para el usuario actual. Desactivado por defecto. |
| **Cerrar tras ejecutar** | En el inicio automático de Windows añade el parámetro `--close-after-30`: el gestor ejecuta los programas activados, espera 30 segundos y termina sin crear la interfaz ni el icono de bandeja. Solo disponible con «Inicio automático» activado. |

**«Cerrar» en la tarjeta y «Cerrar tras ejecutar» en los ajustes son funciones distintas:** la primera afecta a las ventanas del programa seleccionado, la segunda al propio gestor. El intervalo de 30 segundos es fijo y no implica esperar a que terminen todas las aplicaciones iniciadas.

Un arranque manual normal sin el parámetro abre el gestor según el ajuste «Iniciar en bandeja», incluso si «Cerrar tras ejecutar» está activado.

### Control desde la bandeja

- **La cruz de la ventana** oculta el gestor en la bandeja en lugar de cerrarlo.
- **Un clic en el icono** o la opción **Show** devuelve la ventana principal.
- **Clic derecho → Quit** cierra el gestor por completo.

Los nombres de los elementos del menú de bandeja se muestran actualmente en inglés, independientemente del idioma de la interfaz.

## Instalación y compilación desde el código fuente

### Requisitos

Para ejecutar la versión gráfica se necesita **Microsoft Edge WebView2 Runtime**. Un entorno de compilación práctico es Windows 10/11 con lo siguiente instalado:

- [Node.js](https://nodejs.org/) **20 o superior** y npm; se recomienda la versión LTS actual.
- [Rust](https://rustup.rs/): toolchain stable actual para Windows MSVC. En `Cargo.toml` se indica un mínimo de 1.70, pero las dependencias del archivo lock pueden requerir una versión más reciente.
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) con los componentes **Desktop development with C++**, MSVC y Windows SDK.
- [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).
- [Git](https://git-scm.com/) para clonar el repositorio.

Más información: [preparación del entorno Tauri 1 para Windows](https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-windows).

### Obtener el código fuente

En PowerShell ejecuta:

```powershell
git clone https://github.com/BORGERone/AutoStarter.git
cd AutoStarter
npm ci
```

`npm ci` instala las dependencias según `package-lock.json`. La primera compilación también requiere acceso a la red para descargar las dependencias de Rust y las herramientas de empaquetado.

### Modo de desarrollo

```powershell
npm run tauri-dev
```

O ejecuta `dev.bat` desde el Explorador. Tauri levanta automáticamente Vite en el puerto **5174** y abre la ventana de escritorio.

> `npm run dev` inicia solo la interfaz web. En un navegador normal no están disponibles las funciones de Tauri: selección de archivos mediante el diálogo nativo, guardado de ajustes, control de la bandeja y del registro. Usa `npm run tauri-dev` para una comprobación completa de la aplicación.

### Compilación con instaladores

```powershell
npm run tauri-build
```

El comando compila la interfaz, la aplicación Rust y los paquetes definidos por la configuración de Tauri. Los resultados se encuentran en:

- `src-tauri/target/release/tauri-app.exe`: el ejecutable;
- `src-tauri/target/release/bundle/`: los paquetes de instalación creados por el empaquetador.

### Compilación sin instalador

```powershell
.\build.bat
```

El script ejecuta secuencialmente `npm run build` y `cargo build --release` en el directorio `src-tauri`. El resultado es `src-tauri/target/release/tauri-app.exe`. La interfaz gráfica sigue necesitando WebView2 Runtime.

Ejecución manual del modo sin interfaz:

```powershell
.\src-tauri\target\release\tauri-app.exe --close-after-30
```

### Comprobación de la interfaz

```powershell
npm run build
```

El comando ejecuta la comprobación de TypeScript y la compilación de producción de Vite. No comprueba el código Rust ni el comportamiento de la API Win32. Actualmente no hay un script de pruebas automatizadas independiente en `package.json`.

## Almacenamiento de datos e inicio automático de Windows

Los ajustes del usuario actual se guardan en el directorio:

```text
%APPDATA%\tauri-launcher\
├── programs.json    # Lista de programas, rutas y modos de inicio
└── settings.json    # Idioma y ajustes del gestor
```

Para hacer una copia de seguridad, cierra completamente el gestor y copia ambos archivos. Al trasladarlos a otro equipo, revisa las rutas de los programas. Los parámetros de inicio automático del registro no forman parte de esta copia: tras el traslado, activa de nuevo el inicio automático desde la instancia de la aplicación que quieras.

Al activar «Inicio automático» se crea el valor **`AutoStartManager`** en la clave:

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
```

Allí se guarda la ruta al ejecutable actual del gestor y, si procede, el parámetro `--close-after-30`. Se trata de un arranque **al iniciar sesión el usuario actual**, no de un servicio del sistema. No se crean entradas separadas para cada programa añadido; el gestor no administra el inicio automático existente de otras aplicaciones.

**Desactiva «Inicio automático» antes de mover o eliminar el ejecutable.** Tras moverlo, abre el gestor desde la nueva ubicación y vuelve a activarlo. Para un restablecimiento completo, desactiva primero el inicio automático, cierra la aplicación con Quit y luego elimina el directorio `%APPDATA%\tauri-launcher`: esto borrará la lista guardada y los ajustes.

## Preguntas frecuentes

**He añadido un programa, pero no se inicia.**

Comprueba el interruptor «On» en su tarjeta y que el archivo exista en la ruta indicada. Reinicia el gestor por completo: modificar la lista por sí solo no inicia nada. Para el arranque al entrar en Windows también debe estar activado «Inicio automático» en los ajustes del gestor.

**Las aplicaciones se abren dos veces.**

AutoStarter no comprueba si un programa ya se está ejecutando. Revisa su propia opción de inicio automático y la lista de inicio de Windows. Reiniciar el propio gestor también vuelve a ejecutar todas las entradas activadas.

**Tras pulsar la cruz, el gestor sigue funcionando.**

Es el comportamiento esperado. Busca el icono de AutoStart Manager en el área de notificación, despliega los iconos ocultos si hace falta, y elige Quit.

**La aplicación no aparece tras entrar en Windows.**

Comprueba «Iniciar en bandeja» y «Cerrar tras ejecutar»: en el primer caso la ventana está oculta, en el segundo no se crea ninguna interfaz. Para cambiar los ajustes, ejecuta el archivo manualmente sin el parámetro `--close-after-30`.

**La compilación no encuentra el enlazador o el Windows SDK.**

Revisa la instalación de C++ Build Tools y del toolchain MSVC de Rust, y reinicia la terminal. Si hay problemas de visualización de la ventana, comprueba que esté WebView2 Runtime.

**El modo de desarrollo indica que el puerto está ocupado.**

Libera el puerto 5174, por ejemplo cerrando la instancia anterior de Vite. El puerto se usa tanto en la configuración de Vite como en la de Tauri.

## Estructura del proyecto

```text
AutoStarter/
├── index.html                 # Marcado de la ventana principal y los ajustes
├── src/
│   ├── main.ts                # Interfaz, traducciones y llamadas a Tauri
│   └── styles.css             # Estilos de la aplicación
├── src-tauri/
│   ├── src/main.rs            # Inicio de programas, JSON, registro y bandeja Win32
│   ├── Cargo.toml             # Dependencias y parámetros de Rust
│   ├── Cargo.lock             # Dependencias de Rust fijadas
│   ├── build.rs               # Script de compilación de Tauri
│   └── tauri.conf.json        # Ventana, permisos y empaquetado
├── icon.ico                   # Icono de la aplicación y de la bandeja
├── package.json               # Comandos npm y dependencias de la interfaz
├── package-lock.json          # Dependencias npm fijadas
├── vite.config.ts             # Configuración de Vite
├── build.bat                  # Compilación release sin instalador
└── dev.bat                    # Arranque del modo de desarrollo
```

## Contribuir

Los informes de errores y las sugerencias se pueden enviar en [Issues](https://github.com/BORGERone/AutoStarter/issues). Para un informe reproducible, indica la versión de Windows, el modo de ejecución o compilación, los pasos y el comportamiento esperado. Antes de publicar registros o archivos JSON, elimina los datos personales y las rutas privadas.

Para cambios en la interfaz ejecuta `npm run build`; los cambios en el inicio, el registro y la bandeja deben comprobarse además en la versión de escritorio en Windows. Es recomendable acompañar el pull request con una descripción de los cambios y de las comprobaciones realizadas.

## Licencia

En `src-tauri/Cargo.toml` se indica la licencia **MIT**. Todavía no hay en el repositorio un archivo `LICENSE` independiente con el texto completo de la licencia.
