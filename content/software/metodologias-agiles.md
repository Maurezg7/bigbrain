+++
title = "Ciclo de vida y metodologías ágiles"
summary = "Modelos de ciclo de vida del software, el manifiesto ágil, Scrum, Kanban, programación extrema y estimación."
branch = "Ingeniería de software"
order = 2
level = "Básico"
tags = ["ágil", "Scrum", "Kanban", "ciclo de vida", "cascada"]
+++

## Ciclo de vida del software

Todo proyecto pasa, de alguna forma, por estas actividades:

| Actividad | Pregunta que responde |
|---|---|
| Requisitos | ¿qué necesita el usuario? |
| Diseño | ¿cómo se va a construir? |
| Implementación | escribir el código |
| Pruebas | ¿funciona y hace lo pedido? |
| Despliegue | ponerlo en uso |
| Mantenimiento | corregir, adaptar y mejorar |

El mantenimiento suele llevarse la mayor parte del costo total: el software exitoso se modifica durante años.

## Modelos

| Modelo | Idea | Conviene cuando |
|---|---|---|
| **Cascada** | las fases se hacen una vez, en secuencia | los requisitos son estables y conocidos |
| **En V** | cada fase de desarrollo tiene su fase de prueba | sistemas críticos y regulados |
| **Incremental** | se entrega por partes funcionales | se puede dividir el producto |
| **Iterativo** | se repite el ciclo refinando el producto | los requisitos se van descubriendo |
| **Espiral** | iteraciones guiadas por el análisis de riesgos | proyectos grandes y riesgosos |
| **Ágil** | iteraciones cortas con retroalimentación continua | los requisitos cambian |

El problema de la cascada: los errores de requisitos se descubren al final, cuando corregirlos es más caro, y el cliente ve el producto recién al terminar.

## El manifiesto ágil

En 2001, un grupo de desarrolladores resumió sus valores:

> [!IMPORTANT]
> - **Individuos e interacciones** sobre procesos y herramientas.
> - **Software funcionando** sobre documentación extensiva.
> - **Colaboración con el cliente** sobre negociación contractual.
> - **Respuesta ante el cambio** sobre seguir un plan.

No dice que lo de la derecha no importe, sino que lo de la izquierda importa más.

Algunos de sus doce principios:

- Entregar software funcionando con frecuencia, en semanas.
- Aceptar cambios de requisitos, incluso tarde.
- Trabajo conjunto y diario entre negocio y desarrolladores.
- El software funcionando es la medida principal del progreso.
- Ritmo sostenible.
- Simplicidad: maximizar el trabajo **no** hecho.
- El equipo reflexiona regularmente sobre cómo mejorar.

## Scrum

Es el marco ágil más difundido. Organiza el trabajo en **sprints**: iteraciones de duración fija, de una a cuatro semanas, que terminan con un incremento utilizable.

### Roles

| Rol | Responsabilidad |
|---|---|
| **Product Owner** | define **qué** construir y en qué orden; maximiza el valor |
| **Scrum Master** | ayuda al equipo a aplicar Scrum y quita impedimentos |
| **Desarrolladores** | deciden **cómo** construir y hacen el trabajo; se autoorganizan |

### Artefactos

| Artefacto | Qué es |
|---|---|
| **Product Backlog** | lista ordenada de todo lo que podría hacerse |
| **Sprint Backlog** | lo elegido para este sprint, más el plan para lograrlo |
| **Incremento** | el producto funcionando al final del sprint |

La **definición de terminado** (*Definition of Done*) es la lista de condiciones que un elemento debe cumplir para considerarse completo: probado, revisado, integrado, documentado.

### Eventos

| Evento | Cuándo | Para qué |
|---|---|---|
| Planificación del sprint | al inicio | elegir el objetivo y el trabajo |
| Scrum diario | cada día, 15 minutos | sincronizar y detectar impedimentos |
| Revisión del sprint | al final | mostrar el incremento y recibir comentarios |
| Retrospectiva | al final | mejorar la forma de trabajar |

## Historias de usuario

Describen una necesidad desde el punto de vista de quien la usa:

> **Como** estudiante, **quiero** marcar un tema como leído **para** saber qué me falta estudiar.

Cada una lleva **criterios de aceptación** verificables:

- Al marcar un tema, aparece con una tilde en la lista del área.
- La marca se conserva al cerrar y volver a abrir el navegador.

Una buena historia cumple **INVEST**: Independiente, Negociable, Valiosa, Estimable, Pequeña (*Small*) y Comprobable (*Testable*).

Las historias grandes se llaman **épicas** y se dividen.

## Estimación

Estimar en horas es poco confiable. Se suele usar una medida **relativa**: los **puntos de historia**, que comparan el esfuerzo de una historia con el de otras.

- **Planning poker**: cada integrante elige en secreto una carta (1, 2, 3, 5, 8, 13...) y se discuten las diferencias.
- **Velocidad**: puntos que el equipo completa por sprint. Sirve para planificar, no para comparar equipos ni evaluar personas.

> [!NOTE]
> Un equipo con velocidad promedio de 30 puntos y un backlog de 150 puntos necesita unos 5 sprints. Con sprints de dos semanas, cerca de 10 semanas.

Un **gráfico de trabajo pendiente** (*burndown*) muestra el trabajo restante día por día.

## Kanban

Visualiza el flujo de trabajo en un **tablero** con columnas:

| Por hacer | En curso (máx. 3) | En revisión (máx. 2) | Hecho |
|---|---|---|---|

Sus principios:

1. **Visualizar** todo el trabajo.
2. **Limitar el trabajo en curso** (WIP): no empezar algo nuevo hasta terminar lo que hay.
3. **Gestionar el flujo**: medir cuánto tarda cada tarea y eliminar los cuellos de botella.
4. Mejorar de forma continua.

> [!IMPORTANT]
> Limitar el trabajo en curso obliga a **terminar** antes de empezar. Diez tareas a medio hacer no valen nada; dos terminadas, sí.

| | Scrum | Kanban |
|---|---|---|
| Cadencia | sprints de duración fija | flujo continuo |
| Roles | tres definidos | no prescribe |
| Cambios | no durante el sprint | en cualquier momento |
| Límite | lo que entra en el sprint | trabajo en curso por columna |
| Conviene para | desarrollo de producto | soporte, operaciones, mantenimiento |

## Programación extrema

La **programación extrema** (XP) aporta las prácticas técnicas:

| Práctica | En qué consiste |
|---|---|
| Desarrollo guiado por pruebas (TDD) | escribir la prueba antes que el código |
| Programación en pareja | dos personas, un teclado |
| Integración continua | integrar y probar varias veces por día |
| Refactorización | mejorar el diseño constantemente |
| Diseño simple | lo mínimo que resuelve el problema de hoy |
| Entregas pequeñas | versiones frecuentes |
| Propiedad colectiva | cualquiera puede mejorar cualquier parte |

## Lean y el producto mínimo viable

El pensamiento **Lean** busca eliminar el **desperdicio**: funciones que nadie usa, esperas, trabajo a medio hacer, retrabajo.

Un **producto mínimo viable** (MVP) es la versión más pequeña que permite aprender de usuarios reales. El ciclo es **construir, medir, aprender**.

## Deuda técnica

Es el costo futuro de las soluciones rápidas de hoy. Como una deuda financiera, genera **intereses**: cada cambio posterior cuesta más. A veces conviene endeudarse para llegar a una fecha, pero hay que pagarla.

## Errores frecuentes

| Error | Consecuencia |
|---|---|
| Hacer las ceremonias sin los valores | «ágil» de nombre, cascada en la práctica |
| Usar la velocidad como objetivo | se inflan las estimaciones |
| No tener definición de terminado | trabajo «casi listo» que nunca cierra |
| Saltar las retrospectivas | los problemas se repiten |
| Ágil sin prácticas técnicas | el código se degrada y el equipo se frena |

> [!TIP]
> La pregunta que resume el enfoque ágil: ¿cuál es lo más pequeño que podemos entregar para aprender algo?
