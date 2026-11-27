+++
title = "Procesador y lenguaje ensamblador"
summary = "Registros, instrucciones de máquina, modos de direccionamiento, la pila y las llamadas a función, segmentación (pipeline) y arquitecturas RISC y CISC."
branch = "Arquitectura de computadoras"
order = 2
level = "Intermedio"
tags = ["procesador", "ensamblador", "registros", "pipeline", "RISC"]
+++

## Del código a la máquina

Un procesador solo ejecuta **instrucciones de máquina**: números binarios que codifican operaciones muy simples. El **lenguaje ensamblador** es su representación legible, con una línea por instrucción.

```text
código fuente (C, Rust)  →  compilador  →  ensamblador  →  código máquina
   x = a + b;                               add x0, x1, x2     10001011...
```

Cada familia de procesadores tiene su propio conjunto de instrucciones (**ISA**, *Instruction Set Architecture*): un programa compilado para una no funciona en otra.

| ISA | Tipo | Dónde se usa |
|---|---|---|
| x86-64 | CISC | computadoras de escritorio, servidores |
| ARM (AArch64) | RISC | teléfonos, Apple Silicon, sistemas embebidos |
| RISC-V | RISC | abierta; docencia, microcontroladores |

## Registros

Los **registros** son las posiciones de almacenamiento más rápidas: están dentro del procesador y son pocos (entre 16 y 32 de uso general).

| Registro | Función |
|---|---|
| De uso general | operandos y resultados |
| Contador de programa (PC) | dirección de la próxima instrucción |
| Puntero de pila (SP) | tope de la pila |
| Registro de estado (banderas) | resultado de la última operación: cero, negativo, acarreo, desbordamiento |

En un procesador de 64 bits, cada registro guarda 64 bits.

## Tipos de instrucciones

| Categoría | Ejemplos (RISC-V) | Qué hace |
|---|---|---|
| Aritméticas y lógicas | `add`, `sub`, `and`, `or`, `sll` | operan entre registros |
| Con constante | `addi x1, x1, 4` | usan un valor inmediato |
| Carga y almacenamiento | `ld`, `sd` | mueven datos entre memoria y registros |
| Saltos condicionales | `beq`, `bne`, `blt` | cambian el PC si se cumple una condición |
| Saltos y llamadas | `jal`, `jalr` | saltan y guardan la dirección de retorno |

> [!IMPORTANT]
> No existen `if`, `while` ni `for` en el nivel de máquina: todo el control de flujo son **comparaciones y saltos**.

### Un bucle traducido

```c
int suma = 0;
for (int i = 0; i < n; i++) {
    suma += v[i];
}
```

```text
        li   t0, 0            # suma = 0
        li   t1, 0            # i = 0
bucle:  bge  t1, a1, fin      # si i >= n, salir
        slli t2, t1, 2        # t2 = i * 4 (cada int ocupa 4 bytes)
        add  t2, a0, t2       # t2 = dirección de v[i]
        lw   t3, 0(t2)        # t3 = v[i]
        add  t0, t0, t3       # suma += v[i]
        addi t1, t1, 1        # i++
        j    bucle
fin:
```

Acceder a `v[i]` es calcular una dirección: base más índice por tamaño del elemento.

## Modos de direccionamiento

Formas de indicar dónde está un operando:

| Modo | Ejemplo | El operando es |
|---|---|---|
| Inmediato | `addi x1, x0, 5` | la constante 5 |
| Registro | `add x1, x2, x3` | el contenido de un registro |
| Base más desplazamiento | `ld x1, 16(x2)` | la memoria en `x2 + 16` |
| Relativo al PC | `beq x1, x2, etiqueta` | una dirección a cierta distancia del PC |

El modo base más desplazamiento sirve para campos de estructuras y variables locales.

## La pila y las llamadas

La **pila** es una zona de memoria que crece y decrece con las llamadas a función. Cada llamada crea un **marco** (*stack frame*) con:

- la dirección de retorno;
- los registros que hay que preservar;
- las variables locales.

```text
direcciones altas
┌──────────────────────┐
│ marco de main        │
├──────────────────────┤
│ marco de f           │
├──────────────────────┤
│ marco de g           │ ← SP (tope)
└──────────────────────┘
direcciones bajas (la pila crece hacia abajo)
```

Una **convención de llamada** fija las reglas que comparten todos los programas:

| Aspecto | Regla típica |
|---|---|
| Argumentos | en ciertos registros; el resto, en la pila |
| Valor de retorno | en un registro determinado |
| Registros preservados | la función llamada debe restaurarlos |
| Registros temporales | la función llamada puede pisarlos |

> [!NOTE]
> Una recursión sin caso base agota la pila: es el **desbordamiento de pila** (*stack overflow*). Y escribir fuera de un arreglo local puede pisar la dirección de retorno, la base de muchos ataques clásicos.

## Formato de las instrucciones

En RISC-V, todas las instrucciones ocupan **32 bits**, divididos en campos:

```text
 31      25 24  20 19  15 14 12 11   7 6      0
┌──────────┬──────┬──────┬─────┬──────┬────────┐
│ funct7   │ rs2  │ rs1  │ f3  │ rd   │ opcode │    add rd, rs1, rs2
└──────────┴──────┴──────┴─────┴──────┴────────┘
```

El **opcode** identifica la operación; `rs1` y `rs2` son los registros de origen y `rd`, el de destino. Con 5 bits por registro se direccionan $2^5 = 32$ registros.

## RISC y CISC

| | RISC | CISC |
|---|---|---|
| Instrucciones | pocas y simples | muchas, algunas complejas |
| Longitud | fija | variable |
| Acceso a memoria | solo con cargas y almacenamientos | en casi cualquier instrucción |
| Decodificación | simple | compleja |
| Ejemplos | ARM, RISC-V | x86-64 |

Los x86 modernos traducen internamente sus instrucciones a **microoperaciones** de estilo RISC: la diferencia hoy es más de interfaz que de implementación.

## Segmentación (pipeline)

Ejecutar una instrucción lleva varias etapas. La **segmentación** las solapa, como una línea de montaje:

| Etapa | Qué hace |
|---|---|
| IF | busca la instrucción en memoria |
| ID | la decodifica y lee los registros |
| EX | ejecuta en la ALU |
| MEM | accede a memoria |
| WB | escribe el resultado en un registro |

```text
ciclo:    1    2    3    4    5    6    7
instr 1:  IF   ID   EX   MEM  WB
instr 2:       IF   ID   EX   MEM  WB
instr 3:            IF   ID   EX   MEM  WB
```

Cada instrucción sigue tardando cinco ciclos, pero termina **una por ciclo**.

### Riesgos

| Riesgo | Causa | Solución |
|---|---|---|
| De datos | una instrucción necesita un resultado aún no escrito | adelantamiento (*forwarding*), o una burbuja |
| De control | no se sabe qué instrucción sigue a un salto | predicción de saltos |
| Estructural | dos etapas necesitan el mismo recurso | duplicar el recurso |

### Predicción de saltos

El procesador **adivina** el resultado de cada salto y sigue ejecutando. Si acierta (más del 95 % de las veces), no pierde tiempo; si falla, descarta el trabajo especulativo y paga una penalización de 10 a 20 ciclos.

> [!TIP]
> Por eso el código con saltos **impredecibles** es lento: procesar un arreglo ordenado puede ser varias veces más rápido que el mismo arreglo desordenado, aunque el algoritmo sea idéntico.

## Procesadores modernos

| Técnica | Idea |
|---|---|
| Superescalar | varias instrucciones por ciclo |
| Ejecución fuera de orden | ejecuta las instrucciones cuando sus datos están listos, no en el orden escrito |
| Ejecución especulativa | avanza por el camino predicho de un salto |
| SIMD | una instrucción opera sobre varios datos a la vez (SSE, AVX, NEON) |
| Múltiples núcleos | varios procesadores en un mismo chip |
| *Hyper-threading* | un núcleo aparenta ser dos |

## Rendimiento

$$t_{\text{CPU}} = N_{\text{instr}} \times \text{CPI} \times t_{\text{ciclo}}$$

| Factor | Depende de |
|---|---|
| $N_{\text{instr}}$: instrucciones ejecutadas | el algoritmo, el compilador, la ISA |
| CPI: ciclos por instrucción | la microarquitectura, los fallos de caché y de predicción |
| $t_{\text{ciclo}} = 1/f$ | la tecnología de fabricación |

> [!NOTE]
> **Ejemplo.** Un programa ejecuta $2 \times 10^9$ instrucciones con CPI = 1,5 en un procesador de 3 GHz:
> $$t = \frac{2 \times 10^9 \times 1{,}5}{3 \times 10^9} = 1\ \text{s}$$

La frecuencia dejó de crecer hacia 2005 por el calor disipado; desde entonces, las mejoras vienen de más núcleos y mejor microarquitectura.

### Ley de Amdahl

Si una fracción $p$ del programa se acelera $s$ veces, la aceleración total es:

$$S = \frac{1}{(1 - p) + \dfrac{p}{s}}$$

> [!IMPORTANT]
> La parte que **no** se mejora limita el resultado. Si el 90 % del programa se paraleliza perfectamente, la aceleración máxima es $1/0{,}1 = 10$, sin importar cuántos núcleos haya.

## Interrupciones

Una **interrupción** desvía la ejecución hacia una rutina del sistema operativo:

| Tipo | Origen | Ejemplo |
|---|---|---|
| De hardware | un dispositivo | llegó un paquete de red, se pulsó una tecla, venció un temporizador |
| Excepción | la instrucción en curso | división por cero, fallo de página |
| Llamada al sistema | el programa la pide | leer un archivo |

El procesador guarda el PC, pasa a **modo núcleo**, ejecuta el manejador y luego retoma el programa. Así el sistema operativo recupera el control sin que los programas cooperen.

> [!TIP]
> Para ver el ensamblador que genera un compilador: `gcc -S -O2 archivo.c`, `cargo rustc --release -- --emit asm`, o el sitio Compiler Explorer (godbolt.org).
