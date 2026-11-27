+++
title = "Compiladores e intérpretes"
summary = "Fases de un compilador: análisis léxico, sintáctico y semántico, representación intermedia, optimización y generación de código; intérpretes, máquinas virtuales y compilación JIT."
branch = "Teoría de la computación"
order = 3
level = "Avanzado"
tags = ["compiladores", "análisis léxico", "análisis sintáctico", "AST", "optimización", "JIT"]
+++

## Traducir programas

| Herramienta | Qué hace | Ejemplos |
|---|---|---|
| **Compilador** | traduce todo el programa a otro lenguaje antes de ejecutarlo | C, C++, Rust, Go |
| **Intérprete** | ejecuta el programa directamente | Python, Ruby |
| Compilador a *bytecode* + máquina virtual | traduce a un código intermedio que ejecuta una máquina virtual | Java, C#, Python |
| Compilador **JIT** | compila a código máquina durante la ejecución | JavaScript (V8), Java (HotSpot) |
| Transpilador | traduce entre lenguajes de alto nivel | TypeScript → JavaScript |

## Fases

```text
código fuente
     │  análisis léxico
     ▼
  componentes léxicos (tokens)
     │  análisis sintáctico
     ▼
  árbol de sintaxis abstracta (AST)
     │  análisis semántico
     ▼
  AST anotado con tipos
     │  generación de código intermedio
     ▼
  representación intermedia (IR)
     │  optimización
     ▼
  IR optimizada
     │  generación de código
     ▼
código máquina
```

Las primeras fases forman el *front end* (dependen del lenguaje fuente) y las últimas, el *back end* (dependen de la máquina). Una **representación intermedia** común permite combinar $m$ lenguajes con $n$ arquitecturas escribiendo $m + n$ componentes en lugar de $m \times n$. Es el diseño de **LLVM**.

## Análisis léxico

Agrupa los caracteres en **componentes léxicos** (*tokens*) y descarta espacios y comentarios.

```text
posicion = inicial + velocidad * 60
```

| Lexema | Categoría |
|---|---|
| `posicion` | identificador |
| `=` | asignación |
| `inicial` | identificador |
| `+` | operador |
| `velocidad` | identificador |
| `*` | operador |
| `60` | número |

Cada categoría se describe con una **expresión regular**, y el analizador es un autómata finito.

```python
import re

PATRON = re.compile(r"\s*(?:(\d+)|([A-Za-z_]\w*)|(.))")

def tokens(texto):
    for numero, nombre, simbolo in PATRON.findall(texto):
        if numero:
            yield ("NUM", int(numero))
        elif nombre:
            yield ("ID", nombre)
        else:
            yield ("OP", simbolo)
```

## Análisis sintáctico

Comprueba que los componentes siguen la **gramática** y construye el árbol.

```text
expresion → termino (("+" | "-") termino)*
termino   → factor (("*" | "/") factor)*
factor    → NUM | ID | "(" expresion ")"
```

El **árbol de sintaxis abstracta** conserva la estructura y descarta los detalles (paréntesis, puntos y coma):

```text
        =
       / \
posicion  +
         / \
   inicial  *
           / \
   velocidad  60
```

### Métodos

| Método | Idea | Gramáticas |
|---|---|---|
| **Descendente** recursivo | una función por cada regla; empieza por la raíz | LL |
| **Ascendente** | reduce desde las hojas hacia la raíz, con una tabla y una pila | LR, LALR |

El descendente recursivo se escribe a mano con facilidad:

```python
class Analizador:
    def __init__(self, tokens):
        self.tokens = list(tokens)
        self.i = 0

    def actual(self):
        return self.tokens[self.i] if self.i < len(self.tokens) else ("FIN", None)

    def avanzar(self):
        token = self.actual()
        self.i += 1
        return token

    def expresion(self):
        nodo = self.termino()
        while self.actual() in (("OP", "+"), ("OP", "-")):
            op = self.avanzar()[1]
            nodo = (op, nodo, self.termino())
        return nodo

    def termino(self):
        nodo = self.factor()
        while self.actual() in (("OP", "*"), ("OP", "/")):
            op = self.avanzar()[1]
            nodo = (op, nodo, self.factor())
        return nodo

    def factor(self):
        tipo, valor = self.avanzar()
        if tipo in ("NUM", "ID"):
            return valor
        if (tipo, valor) == ("OP", "("):
            nodo = self.expresion()
            self.avanzar()                     # consume ")"
            return nodo
        raise SyntaxError(f"inesperado: {valor}")
```

`Analizador(tokens("2 + 3 * 4")).expresion()` devuelve `('+', 2, ('*', 3, 4))`.

> [!WARNING]
> Una regla **recursiva por izquierda** (`E → E + T`) hace que un analizador descendente se llame a sí mismo sin fin. Se reescribe con repetición, como arriba.

Herramientas como Yacc, Bison y ANTLR generan el analizador a partir de la gramática.

## Análisis semántico

Verifica lo que la gramática no puede expresar:

| Comprobación | Ejemplo de error |
|---|---|
| Declaración antes del uso | usar una variable no declarada |
| **Tipos** | sumar un número y una cadena |
| Cantidad y tipo de argumentos | llamar con un argumento de menos |
| Ámbito | usar una variable local fuera de su bloque |
| Flujo | una función que no devuelve valor por algún camino |

Usa una **tabla de símbolos**: para cada identificador, su tipo, su ámbito y su ubicación.

| Sistema de tipos | Cuándo se comprueba | Ejemplos |
|---|---|---|
| Estático | al compilar | Rust, Java, C, TypeScript |
| Dinámico | al ejecutar | Python, JavaScript |

La **inferencia de tipos** deduce los tipos sin que el programador los escriba.

## Representación intermedia

El **código de tres direcciones** usa instrucciones con, a lo sumo, un operador:

```text
t1 = velocidad * 60
t2 = inicial + t1
posicion = t2
```

En la forma **SSA** (asignación estática única) cada variable se asigna exactamente una vez, lo que simplifica las optimizaciones. El programa se divide en **bloques básicos** (tramos sin saltos), que forman el **grafo de flujo de control**.

## Optimización

Transforma el programa en otro equivalente y más eficiente.

| Optimización | Antes | Después |
|---|---|---|
| Plegado de constantes | `x = 60 * 60` | `x = 3600` |
| Propagación de constantes | `a = 5; b = a + 1` | `b = 6` |
| Eliminación de código muerto | código cuyo resultado no se usa | se elimina |
| Subexpresiones comunes | `a*b + a*b` | `t = a*b; t + t` |
| Reducción de fuerza | `x * 2` | `x << 1` |
| Expansión en línea (*inlining*) | llamada a una función corta | el cuerpo de la función |
| Extracción de invariantes | cálculo constante dentro de un bucle | se calcula una vez, fuera |
| Vectorización | bucle sobre un arreglo | instrucciones SIMD |

> [!IMPORTANT]
> Una optimización debe **preservar el comportamiento observable** del programa. El compilador solo la aplica cuando puede demostrar que es segura.

| Nivel | Significado |
|---|---|
| `-O0` | sin optimizar; compila rápido, fácil de depurar |
| `-O2` | optimización estándar |
| `-O3` | más agresiva |
| `-Os` | prioriza el tamaño |

## Generación de código

| Tarea | Descripción |
|---|---|
| Selección de instrucciones | elegir las instrucciones de la máquina para cada operación |
| **Asignación de registros** | decidir qué variables viven en los pocos registros disponibles; equivale a colorear un grafo |
| Planificación | ordenar las instrucciones para aprovechar la segmentación |

Para la sentencia del ejemplo, en RISC-V:

```text
li   t0, 60
mul  t1, a1, t0        # velocidad * 60
add  a0, a2, t1        # inicial + ...
```

Después, el **enlazador** combina los archivos objeto y las bibliotecas en un ejecutable, resolviendo las referencias entre ellos.

## Intérpretes y máquinas virtuales

Un intérprete de **recorrido del árbol** evalúa el AST directamente:

```python
def evaluar(nodo, entorno):
    if isinstance(nodo, int):
        return nodo
    if isinstance(nodo, str):
        return entorno[nodo]
    op, izq, der = nodo
    a, b = evaluar(izq, entorno), evaluar(der, entorno)
    return {"+": a + b, "-": a - b, "*": a * b, "/": a / b}[op]
```

Es simple, pero lento. Lo habitual es compilar a ***bytecode***, un código compacto para una **máquina virtual**:

```text
CARGAR inicial
CARGAR velocidad
CONSTANTE 60
MULTIPLICAR
SUMAR
GUARDAR posicion
```

Es una **máquina de pila**: los operandos se apilan y cada operación consume los del tope.

### Compilación JIT

Una máquina virtual con **JIT** (*just in time*) empieza interpretando, detecta el código que se ejecuta mucho y lo compila a código máquina, usando información que solo existe al ejecutar (los tipos reales de los valores).

| | Compilación anticipada | JIT |
|---|---|---|
| Arranque | rápido | más lento (calentamiento) |
| Rendimiento sostenido | alto | alto |
| Información para optimizar | estática | del comportamiento real |
| Portabilidad | un binario por plataforma | el mismo *bytecode* en todas |

## Gestión de memoria

| Estrategia | Lenguajes | Característica |
|---|---|---|
| Manual | C, C++ | control total; riesgo de fugas y accesos inválidos |
| **Recolección de basura** | Java, Go, Python, JavaScript | automática; introduce pausas |
| Propiedad y préstamos | Rust | verificada al compilar; sin recolector |
| Conteo de referencias | Swift, Python | libera al llegar a cero; los ciclos necesitan tratamiento aparte |

Un recolector de **marcado y barrido** parte de las raíces (pila, variables globales), marca todo lo alcanzable y libera el resto. Los recolectores **generacionales** aprovechan que la mayoría de los objetos muere joven.

> [!TIP]
> Escribir un intérprete para un lenguaje pequeño (una calculadora con variables y funciones) es uno de los mejores ejercicios para entender cómo funcionan los lenguajes. El libro *Crafting Interpreters*, de Robert Nystrom, es una guía gratuita y muy clara.
