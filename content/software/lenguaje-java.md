+++
title = "Java"
summary = "La máquina virtual, tipos, clases e interfaces, genéricos, colecciones, excepciones, streams y las novedades del Java moderno."
branch = "Lenguajes"
order = 3
level = "Intermedio"
tags = ["Java", "JVM", "colecciones", "genéricos", "streams"]
+++

## Características

**Java** es un lenguaje orientado a objetos, de tipado **estático**, con recolector de basura. Se compila a **bytecode**, que ejecuta la **máquina virtual de Java** (JVM): el mismo programa corre en cualquier sistema que tenga una JVM.

| Fortalezas | Debilidades |
|---|---|
| ecosistema enorme y maduro | verboso |
| muy buen rendimiento una vez en marcha | arranque lento y mayor uso de memoria |
| compatibilidad hacia atrás durante décadas | |
| excelentes herramientas | |

Domina el software empresarial y los sistemas bancarios, y fue el lenguaje original de Android.

```java
public class Hola {
    public static void main(String[] args) {
        System.out.println("Hola, mundo");
    }
}
```

```bash
javac Hola.java     # compila a Hola.class (bytecode)
java Hola           # lo ejecuta en la JVM
```

| Sigla | Qué es |
|---|---|
| JVM | la máquina virtual que ejecuta el bytecode |
| JRE | la JVM más las bibliotecas |
| JDK | el JRE más el compilador y las herramientas |

La JVM compila a código máquina, mientras el programa corre, las partes que más se usan (**compilación JIT**). Otros lenguajes también corren sobre ella: Kotlin, Scala, Clojure.

## Tipos

### Primitivos

| Tipo | Tamaño | Ejemplo |
|---|---|---|
| `int` | 32 bits | `42` |
| `long` | 64 bits | `42L` |
| `double` | 64 bits | `3.14` |
| `boolean` | — | `true` |
| `char` | 16 bits | `'a'` |
| `byte`, `short`, `float` | 8, 16, 32 bits | |

### Referencias

Todo lo demás (objetos, arreglos, `String`) se maneja por **referencia**. Una referencia puede ser `null`; usarla lanza `NullPointerException`.

Cada primitivo tiene una clase **envoltorio** (`Integer`, `Double`, `Boolean`), necesaria en las colecciones. La conversión es automática (*autoboxing*).

```java
int edad = 20;
String nombre = "Ana";
var lista = new ArrayList<String>();   // inferencia de tipo local
final double PI = 3.14159;             // no se puede reasignar
```

> [!WARNING]
> Las cadenas y los objetos se comparan con `.equals()`. El operador `==` compara si son **el mismo objeto**, no si tienen el mismo contenido.

Los `String` son inmutables. Para concatenar muchas veces en un bucle se usa `StringBuilder`.

## Control de flujo

```java
if (nota >= 7) {
    estado = "promociona";
} else if (nota >= 4) {
    estado = "regulariza";
} else {
    estado = "libre";
}

for (int i = 0; i < 3; i++) { ... }

for (String n : nombres) {              // for-each
    System.out.println(n);
}

String tipo = switch (dia) {            // switch como expresión
    case "sábado", "domingo" -> "fin de semana";
    default -> "día hábil";
};
```

## Clases

```java
public class Cuenta {
    private final String titular;
    private double saldo;

    public Cuenta(String titular) {            // constructor
        this.titular = titular;
    }

    public void depositar(double monto) {
        if (monto <= 0) {
            throw new IllegalArgumentException("monto inválido");
        }
        saldo += monto;
    }

    public double getSaldo() {
        return saldo;
    }
}
```

| Modificador | Visible desde |
|---|---|
| `public` | cualquier clase |
| `protected` | el paquete y las subclases |
| (ninguno) | el mismo paquete |
| `private` | solo la clase |

- `static`: el miembro pertenece a la clase, no a cada instancia.
- `final`: una variable no se reasigna; un método no se redefine; una clase no se hereda.

Las clases se organizan en **paquetes** (`package com.ejemplo.banco;`) y se usan con `import`.

### Herencia e interfaces

Una clase hereda de **una sola** superclase (`extends`) y puede implementar **varias** interfaces (`implements`).

```java
public interface Figura {
    double area();

    default String describir() {               // método por defecto
        return "Figura de área " + area();
    }
}

public class Circulo implements Figura {
    private final double radio;

    public Circulo(double radio) { this.radio = radio; }

    @Override
    public double area() {
        return Math.PI * radio * radio;
    }
}
```

`@Override` pide al compilador que verifique que el método realmente redefine uno heredado.

Una clase `abstract` no se puede instanciar y puede dejar métodos sin implementar.

### Registros y enumeraciones

Un **record** es una clase inmutable de datos: genera el constructor, los accesores, `equals`, `hashCode` y `toString`.

```java
public record Punto(double x, double y) {
    public double distanciaAlOrigen() {
        return Math.sqrt(x * x + y * y);
    }
}

public enum Estado { PENDIENTE, PAGADO, ENVIADO }
```

## Genéricos

Permiten escribir clases y métodos que funcionan con cualquier tipo, verificados al compilar:

```java
public class Caja<T> {
    private T contenido;
    public void guardar(T valor) { contenido = valor; }
    public T obtener() { return contenido; }
}

Caja<String> caja = new Caja<>();

public static <T extends Comparable<T>> T maximo(List<T> lista) { ... }
```

## Colecciones

| Interfaz | Implementación habitual | Características |
|---|---|---|
| `List` | `ArrayList` | ordenada, con índice, admite repetidos |
| | `LinkedList` | inserciones rápidas en los extremos |
| `Set` | `HashSet` | sin repetidos, sin orden |
| | `TreeSet` | sin repetidos, ordenado |
| `Map` | `HashMap` | pares clave-valor |
| | `TreeMap` | ordenado por clave |
| `Deque` | `ArrayDeque` | pila y cola |

```java
List<String> nombres = new ArrayList<>();
nombres.add("Ana");
nombres.get(0);

Map<String, Integer> edades = new HashMap<>();
edades.put("Ana", 20);
edades.getOrDefault("Luis", 0);

for (Map.Entry<String, Integer> e : edades.entrySet()) {
    System.out.println(e.getKey() + ": " + e.getValue());
}

List<Integer> fija = List.of(1, 2, 3);      // inmutable
```

> [!IMPORTANT]
> Declarar las variables con el tipo de la **interfaz** (`List`, `Map`), no con el de la implementación: permite cambiarla sin tocar el resto.

Si una clase se usa como clave de un `HashMap` o en un `HashSet`, debe redefinir `equals` **y** `hashCode` de forma coherente.

## Excepciones

```java
try (BufferedReader lector = Files.newBufferedReader(Path.of("datos.txt"))) {
    String linea;
    while ((linea = lector.readLine()) != null) {
        System.out.println(linea);
    }
} catch (IOException e) {
    System.err.println("No se pudo leer: " + e.getMessage());
}
```

El **try con recursos** cierra el archivo automáticamente.

| Tipo | Hereda de | ¿Obligatorio manejarla? | Ejemplo |
|---|---|---|---|
| Verificada (*checked*) | `Exception` | sí: capturarla o declararla con `throws` | `IOException` |
| No verificada | `RuntimeException` | no | `NullPointerException`, `IllegalArgumentException` |
| Error | `Error` | no se manejan | `OutOfMemoryError` |

## Lambdas y streams

Una **expresión lambda** es una función anónima. Un **stream** procesa una colección con operaciones encadenadas.

```java
List<String> aprobados = estudiantes.stream()
    .filter(e -> e.nota() >= 6)
    .sorted(Comparator.comparing(Estudiante::nombre))
    .map(Estudiante::nombre)
    .toList();

double promedio = estudiantes.stream()
    .mapToDouble(Estudiante::nota)
    .average()
    .orElse(0);

Map<String, List<Estudiante>> porCarrera = estudiantes.stream()
    .collect(Collectors.groupingBy(Estudiante::carrera));
```

Los streams son **perezosos**: nada se ejecuta hasta la operación final (`toList`, `collect`, `count`).

`Optional<T>` representa un valor que puede faltar, sin usar `null`:

```java
Optional<Estudiante> mejor = estudiantes.stream()
    .max(Comparator.comparing(Estudiante::nota));

String nombre = mejor.map(Estudiante::nombre).orElse("nadie");
```

## Java moderno

| Novedad | Versión | Qué aporta |
|---|---|---|
| Lambdas y streams | 8 | estilo funcional |
| `var` | 10 | inferencia de tipo local |
| `switch` como expresión | 14 | menos código |
| Bloques de texto (`"""`) | 15 | cadenas de varias líneas |
| Records | 16 | clases de datos |
| Clases selladas | 17 | jerarquías cerradas |
| Coincidencia de patrones | 21 | `switch` sobre tipos |
| Hilos virtuales | 21 | concurrencia liviana y masiva |

```java
sealed interface Forma permits Circulo, Rectangulo {}
record Circulo(double radio) implements Forma {}
record Rectangulo(double base, double altura) implements Forma {}

double area(Forma f) {
    return switch (f) {
        case Circulo c -> Math.PI * c.radio() * c.radio();
        case Rectangulo r -> r.base() * r.altura();
    };
}
```

Las versiones con soporte extendido (LTS) son 8, 11, 17, 21 y 25.

## Concurrencia

```java
try (var ejecutor = Executors.newVirtualThreadPerTaskExecutor()) {
    Future<String> resultado = ejecutor.submit(() -> descargar(url));
    System.out.println(resultado.get());
}
```

`synchronized`, los tipos atómicos y las colecciones concurrentes (`ConcurrentHashMap`) protegen los datos compartidos entre hilos.

## Herramientas

| Herramienta | Uso |
|---|---|
| Maven, Gradle | compilación y dependencias |
| JUnit, Mockito | pruebas |
| IntelliJ IDEA, Eclipse, VS Code | entornos de desarrollo |
| Spring Boot, Quarkus, Micronaut | frameworks |

Convenciones: clases en `PascalCase`, métodos y variables en `camelCase`, constantes en `MAYUSCULAS_CON_GUIONES`, paquetes en minúsculas.

> [!TIP]
> Preferir objetos **inmutables** (records, campos `final`, `List.of`): son más simples de razonar y seguros entre hilos.
