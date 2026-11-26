+++
title = "Control de versiones con Git"
summary = "Guardar la historia de un proyecto, trabajar en ramas y colaborar sin pisarse."
branch = "Ingeniería de software"
order = 1
level = "Básico"
tags = ["git", "commits", "ramas", "merge", "GitHub"]
+++

## Para qué sirve

Un **sistema de control de versiones** registra cada cambio hecho a un proyecto: qué cambió, quién lo hizo, cuándo y por qué. Permite volver a cualquier estado anterior, trabajar en paralelo sin interferir y combinar el trabajo de varias personas. **Git** es el estándar de la industria.

## Las tres zonas

![Las tres zonas de Git: directorio de trabajo, área de preparación y repositorio](/media/software/git-zonas.svg)

*Un cambio pasa por tres zonas antes de quedar en la historia, y luego se comparte con el remoto.*

- **Directorio de trabajo**: los archivos que estás editando.
- **Área de preparación** (*staging*): los cambios elegidos para el próximo commit.
- **Repositorio**: la historia de commits, guardada en la carpeta `.git`.

Un **commit** es una fotografía del proyecto en un momento dado, con un mensaje, un autor y un identificador único.

## Flujo básico

```bash
git init                      # crear un repositorio en la carpeta actual
git status                    # ver qué cambió
git add informe.md            # preparar un archivo
git commit -m "Agrega la introducción del informe"
git log --oneline             # ver la historia
```

## Ramas

Una **rama** es una línea de desarrollo independiente. Permite trabajar en una funcionalidad sin afectar la rama principal (`main`) hasta que esté lista.

```bash
git switch -c grafico-resultados   # crear una rama y pasarse a ella
# ... editar, add, commit ...
git switch main                    # volver a main
git merge grafico-resultados       # integrar los cambios
```

Si dos ramas modificaron las mismas líneas, Git no puede decidir solo y marca un **conflicto**. Se resuelve editando el archivo para dejar la versión correcta, y luego `git add` y `git commit`.

## Trabajar con un remoto

Un **remoto** es una copia del repositorio alojada en un servidor (GitHub, GitLab), que sirve de punto de encuentro del equipo.

| Comando | Qué hace |
|---|---|
| `git clone <url>` | descarga un repositorio completo |
| `git pull` | trae los commits nuevos del remoto y los integra |
| `git push` | sube tus commits al remoto |
| `git fetch` | trae los commits nuevos sin integrarlos |

El flujo de colaboración habitual: crear una rama, subirla y abrir un **pull request** para que otra persona revise los cambios antes de integrarlos a `main`.

## Deshacer

| Situación | Comando |
|---|---|
| Descartar cambios no preparados de un archivo | `git restore archivo` |
| Sacar un archivo del área de preparación | `git restore --staged archivo` |
| Corregir el último commit (todavía sin subir) | `git commit --amend` |
| Revertir un commit ya compartido | `git revert <id>` |

> [!WARNING]
> No reescribas historia que ya subiste y que otros usan (`commit --amend`, `rebase`, `push --force`). Para deshacer algo compartido, usá `git revert`, que agrega un commit nuevo en lugar de borrar uno viejo.

## Buenas prácticas

- Commits **pequeños y frecuentes**, cada uno con un solo propósito.
- Mensajes que expliquen **qué y por qué**, no "cambios varios".
- Un archivo `.gitignore` para excluir archivos generados, dependencias y, sobre todo, **contraseñas y claves**.
- `git pull` antes de empezar a trabajar, para partir de la última versión.
