+++
title = "Requisitos y modelado con UML"
summary = "Requisitos funcionales y no funcionales, casos de uso, y los diagramas UML de clases, secuencia, actividad y estados."
branch = "Ingeniería de software"
order = 3
level = "Intermedio"
tags = ["requisitos", "UML", "casos de uso", "diagrama de clases", "modelado"]
+++

## Ingeniería de requisitos

Un **requisito** es algo que el sistema debe hacer o una cualidad que debe tener. Equivocarse aquí es el error más caro: se construye bien el producto equivocado.

| Tipo | Describe | Ejemplo |
|---|---|---|
| **Funcional** | qué hace el sistema | «El usuario puede buscar temas por palabra» |
| **No funcional** | qué cualidades tiene | «La búsqueda responde en menos de 200 ms» |
| **Restricción** | límites impuestos | «Debe funcionar sin conexión» |

### Atributos de calidad

| Atributo | Pregunta |
|---|---|
| Rendimiento | ¿qué tan rápido responde? |
| Escalabilidad | ¿soporta más carga? |
| Disponibilidad | ¿qué porcentaje del tiempo funciona? |
| Seguridad | ¿resiste accesos indebidos? |
| Usabilidad | ¿es fácil de aprender y usar? |
| Mantenibilidad | ¿es fácil de modificar? |
| Portabilidad | ¿corre en otros entornos? |
| Accesibilidad | ¿pueden usarlo personas con discapacidad? |

Suelen entrar en conflicto (más seguridad, menos usabilidad): hay que priorizar.

### Actividades

| Actividad | Qué se hace |
|---|---|
| **Obtención** | entrevistas, observación, talleres, prototipos, análisis de sistemas existentes |
| **Análisis** | resolver conflictos, priorizar, detectar omisiones |
| **Especificación** | escribirlos con claridad |
| **Validación** | confirmar con los interesados que son los correctos |
| **Gestión** | controlar los cambios y mantener la trazabilidad |

Los **interesados** (*stakeholders*) son todos los que afectan o son afectados por el sistema: usuarios, clientes, operadores, reguladores.

### Un buen requisito

| Cualidad | Significado |
|---|---|
| Claro | una sola interpretación |
| Verificable | se puede comprobar con una prueba |
| Necesario | responde a una necesidad real |
| Factible | se puede construir |
| Consistente | no contradice a otros |
| Trazable | se sabe de dónde viene |

> [!NOTE]
> - Malo: «El sistema debe ser rápido y fácil de usar».
> - Bueno: «El 95 % de las búsquedas debe responder en menos de 200 ms con 1000 usuarios simultáneos».

**Priorización MoSCoW**: lo que **debe** tener (*Must*), **debería** (*Should*), **podría** (*Could*) y **no tendrá** por ahora (*Won't*).

## Casos de uso

Un **caso de uso** describe una interacción entre un **actor** (una persona u otro sistema) y el sistema para lograr un objetivo.

| Campo | Ejemplo |
|---|---|
| Nombre | Extraer dinero |
| Actor | Cliente |
| Precondición | El cliente insertó una tarjeta válida |
| Flujo principal | 1. Ingresa el PIN. 2. Elige el monto. 3. El sistema verifica el saldo. 4. Entrega el dinero. 5. Imprime el comprobante. |
| Flujos alternativos | 3a. Saldo insuficiente: informa y vuelve al paso 2. |
| Poscondición | El saldo se actualizó |

Los flujos alternativos y de error son donde se esconden los requisitos olvidados.

## UML

El **Lenguaje Unificado de Modelado** (UML) es una notación gráfica estándar para describir sistemas de software.

| Grupo | Diagramas | Muestran |
|---|---|---|
| **Estructurales** | clases, componentes, despliegue, objetos, paquetes | qué partes hay y cómo se relacionan |
| **De comportamiento** | casos de uso, secuencia, actividad, estados | qué pasa a lo largo del tiempo |

> [!IMPORTANT]
> Un modelo es una simplificación con un propósito: comunicar. Un diagrama que nadie lee o que no se mantiene al día no sirve.

### Diagrama de casos de uso

Muestra los actores (figuras de palitos), los casos de uso (óvalos) y el límite del sistema (un rectángulo). Da una vista general de **qué** ofrece el sistema.

- **Include**: un caso siempre incorpora a otro («Extraer dinero» incluye «Validar PIN»).
- **Extend**: un caso agrega comportamiento opcional a otro.

### Diagrama de clases

Es el más usado. Cada clase es un rectángulo con tres secciones:

```text
┌──────────────────────┐
│       Cuenta         │
├──────────────────────┤
│ - saldo: Decimal     │
│ - titular: Cliente   │
├──────────────────────┤
│ + depositar(monto)   │
│ + extraer(monto)     │
│ + consultarSaldo()   │
└──────────────────────┘
```

Visibilidad: `+` público, `-` privado, `#` protegido.

| Relación | Significado | Notación |
|---|---|---|
| Asociación | una clase conoce a otra | línea |
| Agregación | «tiene», las partes viven por su cuenta | rombo hueco |
| Composición | «tiene», las partes no existen sin el todo | rombo lleno |
| Herencia | «es un» | flecha con triángulo hueco |
| Realización | implementa una interfaz | línea punteada con triángulo hueco |
| Dependencia | usa temporalmente | flecha punteada |

La **multiplicidad** indica cuántos objetos participan:

| Notación | Significado |
|---|---|
| `1` | exactamente uno |
| `0..1` | cero o uno |
| `*` o `0..*` | cero o más |
| `1..*` | uno o más |

> [!NOTE]
> «Un cliente tiene una o más cuentas; cada cuenta pertenece a un solo cliente»: Cliente `1` —— `1..*` Cuenta.

### Diagrama de secuencia

Muestra **cómo colaboran los objetos en el tiempo**. Cada participante tiene una **línea de vida** vertical; los mensajes son flechas horizontales, ordenadas de arriba hacia abajo.

```text
Cliente        Cajero          Banco
   │   PIN       │               │
   │────────────>│  validar(PIN) │
   │             │──────────────>│
   │             │      ok       │
   │             │<- - - - - - - │
   │   menú      │               │
   │<- - - - - - │               │
```

Flecha llena: llamada. Flecha punteada: respuesta. Los fragmentos `alt`, `opt` y `loop` representan alternativas, opciones y repeticiones.

Es ideal para documentar un caso de uso o una interacción entre servicios.

### Diagrama de actividad

Es un diagrama de flujo: muestra los **pasos** de un proceso, con decisiones (rombos), actividades en paralelo (barras) y **calles** (*swimlanes*) que indican quién hace cada cosa.

### Diagrama de estados

Muestra los **estados** por los que pasa un objeto y los **eventos** que provocan cada transición.

```text
            pagar              enviar             entregar
[Pendiente] ─────> [Pagado] ─────────> [Enviado] ─────────> [Entregado]
     │                 │
     │ cancelar        │ cancelar
     v                 v
 [Cancelado] <─────────┘
```

Ayuda a detectar transiciones no contempladas: ¿se puede cancelar un pedido ya enviado?

### Diagramas de componentes y de despliegue

- **Componentes**: los módulos del sistema y sus interfaces.
- **Despliegue**: en qué máquinas o contenedores corre cada parte.

## Otros modelos

| Modelo | Para qué |
|---|---|
| Entidad-relación | estructura de una base de datos |
| Modelo C4 | arquitectura en cuatro niveles: contexto, contenedores, componentes y código |
| Prototipos y *wireframes* | validar la interfaz antes de construirla |
| Diagramas como código (Mermaid, PlantUML) | diagramas en texto, versionados junto al código |

## Cuánto modelar

| Situación | Nivel de modelado |
|---|---|
| Discutir una idea | un boceto en el pizarrón |
| Documentar una decisión importante | un diagrama cuidado y una explicación |
| Sistema crítico o regulado | modelos formales y completos |

En equipos ágiles se modela **lo justo**: lo necesario para entenderse, y no más.

## Verificación y validación

| | Pregunta |
|---|---|
| **Verificación** | ¿estamos construyendo **bien** el producto? (cumple la especificación) |
| **Validación** | ¿estamos construyendo el producto **correcto**? (sirve al usuario) |

> [!WARNING]
> Los usuarios suelen pedir una solución («quiero un botón para exportar a Excel») en lugar de contar su problema («necesito presentar este informe a fin de mes»). Preguntar siempre **para qué**.

> [!TIP]
> Si un requisito no puede convertirse en una prueba que diga «pasa» o «no pasa», todavía no está bien escrito.
