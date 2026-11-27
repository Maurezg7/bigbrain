+++
title = "Linux y la línea de comandos"
summary = "Sistema de archivos de Linux, comandos esenciales, tuberías y redirecciones, permisos, procesos, paquetes, SSH y guiones de shell."
branch = "Sistemas operativos"
order = 4
level = "Básico"
tags = ["Linux", "terminal", "bash", "shell", "SSH"]
+++

## Por qué la terminal

Linux domina en servidores, supercomputadoras, la nube y los sistemas embebidos. La **línea de comandos** es su interfaz universal: funciona igual en una notebook, en un servidor remoto o dentro de un contenedor, y todo lo que se escribe puede **automatizarse**.

El **shell** es el programa que interpreta los comandos. El más común es **bash**; **zsh** y **fish** son alternativas.

```text
usuario@equipo:~/proyecto$ comando -opciones argumentos
```

## El árbol de directorios

Todo cuelga de una única raíz, `/`.

| Directorio | Contenido |
|---|---|
| `/home` | carpetas personales de los usuarios |
| `/etc` | configuración del sistema |
| `/usr`, `/bin` | programas instalados |
| `/var` | datos variables: registros, bases de datos |
| `/tmp` | archivos temporales |
| `/dev` | dispositivos |
| `/proc`, `/sys` | información del núcleo, como archivos virtuales |
| `/root` | carpeta del administrador |

| Símbolo | Significa |
|---|---|
| `~` | la carpeta personal |
| `.` | el directorio actual |
| `..` | el directorio padre |
| `-` | el directorio anterior (con `cd`) |

Los nombres distinguen **mayúsculas de minúsculas**, y los archivos que empiezan con punto están ocultos.

## Moverse y manipular archivos

| Comando | Qué hace |
|---|---|
| `pwd` | muestra el directorio actual |
| `ls -la` | lista, con detalles y archivos ocultos |
| `cd ruta` | cambia de directorio |
| `mkdir -p a/b/c` | crea directorios, incluidos los intermedios |
| `cp -r origen destino` | copia (con `-r`, directorios) |
| `mv origen destino` | mueve o renombra |
| `rm archivo`, `rm -r carpeta` | borra |
| `touch archivo` | crea un archivo vacío |
| `ln -s destino nombre` | crea un enlace simbólico |
| `find . -name "*.rs"` | busca archivos |
| `du -sh carpeta`, `df -h` | espacio usado; espacio libre en disco |

> [!WARNING]
> `rm` **no tiene papelera**: lo borrado no se recupera. Con `rm -rf` y una ruta equivocada se puede perder todo. Revisar siempre la ruta antes de ejecutar.

### Comodines

| Patrón | Coincide con |
|---|---|
| `*` | cualquier cadena |
| `?` | un carácter |
| `[abc]`, `[0-9]` | uno del conjunto |
| `{a,b}` | cada alternativa: `cp archivo.{txt,bak}` |

## Ver y procesar texto

| Comando | Qué hace |
|---|---|
| `cat archivo` | muestra el contenido |
| `less archivo` | lo muestra paginado (`q` para salir, `/` para buscar) |
| `head -n 20`, `tail -n 20` | las primeras o últimas líneas |
| `tail -f registro.log` | sigue el archivo mientras crece |
| `grep -rn "texto" .` | busca texto en archivos |
| `wc -l` | cuenta líneas |
| `sort`, `uniq -c` | ordena; agrupa líneas repetidas contándolas |
| `cut -d, -f2` | extrae columnas |
| `sed 's/viejo/nuevo/g'` | reemplaza texto |
| `awk '{print $1}'` | procesa por campos |
| `diff a b` | compara dos archivos |

## Tuberías y redirecciones

Cada programa tiene tres flujos:

| Flujo | Número | Por defecto |
|---|---|---|
| Entrada estándar (stdin) | 0 | el teclado |
| Salida estándar (stdout) | 1 | la pantalla |
| Salida de error (stderr) | 2 | la pantalla |

| Operador | Efecto |
|---|---|
| `>` | envía la salida a un archivo (lo sobrescribe) |
| `>>` | la agrega al final |
| `<` | toma la entrada de un archivo |
| `2>` | redirige los errores |
| `2>&1` | une los errores con la salida |
| <code>&#124;</code> | **tubería**: la salida de un comando es la entrada del siguiente |

```bash
# Las 5 direcciones IP con más pedidos en un registro web
cut -d' ' -f1 acceso.log | sort | uniq -c | sort -rn | head -5

# Guardar salida y errores en un archivo
cargo build > compilacion.log 2>&1
```

> [!IMPORTANT]
> Es la **filosofía Unix**: programas pequeños que hacen una sola cosa bien y se combinan mediante tuberías de texto.

| Operador | Efecto |
|---|---|
| `a && b` | ejecuta `b` solo si `a` tuvo éxito |
| <code>a &#124;&#124; b</code> | ejecuta `b` solo si `a` falló |
| `a ; b` | ejecuta ambos, en orden |
| `a &` | ejecuta `a` en segundo plano |
| `$(comando)` | sustituye por la salida del comando |

## Permisos y usuarios

```bash
ls -l guion.sh
-rwxr-xr--  1 ana  docentes  512  mar 10 09:30  guion.sh
```

| Comando | Qué hace |
|---|---|
| `chmod +x guion.sh` | da permiso de ejecución |
| `chmod 640 archivo` | dueño: leer y escribir; grupo: leer; otros: nada |
| `chown usuario:grupo archivo` | cambia el propietario |
| `sudo comando` | ejecuta como administrador |
| `whoami`, `id` | usuario actual y sus grupos |

El usuario **root** puede hacer cualquier cosa. Lo recomendable es trabajar como usuario común y usar `sudo` solo cuando haga falta.

## Procesos

| Comando | Qué hace |
|---|---|
| `ps aux` | lista todos los procesos |
| `top`, `htop` | monitor en tiempo real |
| `kill PID` | pide a un proceso que termine (SIGTERM) |
| `kill -9 PID` | lo termina por la fuerza (SIGKILL) |
| `Ctrl+C` | interrumpe el proceso en primer plano |
| `Ctrl+Z`, `bg`, `fg` | suspende; reanuda en segundo o primer plano |
| `jobs` | lista los trabajos del shell |
| `nohup comando &` | lo deja corriendo aunque se cierre la terminal |

Los servicios del sistema se administran con **systemd**:

```bash
sudo systemctl status nginx
sudo systemctl restart nginx
sudo systemctl enable nginx          # lo inicia en cada arranque
journalctl -u nginx -f               # sigue su registro
```

## Variables de entorno

```bash
echo $HOME
export PORT=8787              # la heredan los programas que se lancen
PORT=9000 ./servidor          # solo para ese comando
env                           # lista todas
```

`PATH` es la lista de directorios donde el shell busca los programas. La configuración personal del shell vive en `~/.bashrc` (o `~/.zshrc`).

## Paquetes

| Distribución | Gestor | Instalar |
|---|---|---|
| Debian, Ubuntu | `apt` | `sudo apt update && sudo apt install paquete` |
| Fedora, RHEL | `dnf` | `sudo dnf install paquete` |
| Arch | `pacman` | `sudo pacman -S paquete` |
| macOS | Homebrew | `brew install paquete` |

## Red y acceso remoto

| Comando | Qué hace |
|---|---|
| `ip addr` | muestra las interfaces y sus direcciones |
| `ping equipo` | comprueba la conectividad |
| `curl -i https://sitio` | hace una petición HTTP |
| `ss -tlnp` | muestra los puertos en escucha |
| `dig dominio` | consulta DNS |
| `ssh usuario@servidor` | abre una sesión remota |
| `scp archivo usuario@servidor:ruta` | copia archivos por SSH |
| `rsync -av origen destino` | sincroniza directorios |

**SSH** cifra toda la sesión. Conviene autenticarse con un **par de claves** en lugar de contraseña:

```bash
ssh-keygen -t ed25519                   # genera el par de claves
ssh-copy-id usuario@servidor            # instala la clave pública en el servidor
```

La clave privada (`~/.ssh/id_ed25519`) nunca se comparte.

## Comprimir

```bash
tar -czf copia.tar.gz carpeta/          # crear
tar -xzf copia.tar.gz                   # extraer
zip -r copia.zip carpeta/ && unzip copia.zip
```

## Guiones de shell

Un **guion** (*script*) es un archivo con comandos.

```bash
#!/usr/bin/env bash
set -euo pipefail                       # detenerse ante cualquier error

origen="${1:?uso: respaldo.sh <carpeta>}"
destino="respaldos/$(date +%F)"

mkdir -p "$destino"

for archivo in "$origen"/*.md; do
    if [[ -s "$archivo" ]]; then
        cp "$archivo" "$destino/"
        echo "copiado: $archivo"
    fi
done

echo "Total: $(ls "$destino" | wc -l) archivos"
```

| Elemento | Sintaxis |
|---|---|
| Variable | `nombre="valor"` (sin espacios alrededor del `=`); se usa como `"$nombre"` |
| Argumentos | `$1`, `$2`, ...; `$#` es la cantidad y `$@`, todos |
| Código de salida | `$?`: 0 es éxito; cualquier otro valor, error |
| Condición | `if [[ condición ]]; then ... fi` |
| Bucle | `for x in lista; do ... done` |
| Función | `nombre() { ... }` |

| Prueba | Verdadera si |
|---|---|
| `-f ruta`, `-d ruta` | existe el archivo; el directorio |
| `-z "$x"`, `-n "$x"` | la cadena está vacía; no lo está |
| `"$a" == "$b"` | las cadenas son iguales |
| `$a -eq $b`, `-lt`, `-gt` | comparación numérica |

> [!WARNING]
> Poner siempre las variables **entre comillas dobles** (`"$archivo"`): sin ellas, un nombre con espacios se parte en varios argumentos.

## Ayuda y productividad

| Recurso | Uso |
|---|---|
| `man comando` | el manual |
| `comando --help` | resumen de opciones |
| `Tab` | autocompleta |
| `Ctrl+R` | busca en el historial |
| `!!` | repite el último comando (`sudo !!`) |
| `alias ll='ls -la'` | crea un atajo |
| `tmux` | sesiones que sobreviven a la desconexión |

> [!TIP]
> En Windows, **WSL** ofrece un Linux completo integrado al sistema. Es la forma más simple de practicar sin instalar otro sistema operativo.
