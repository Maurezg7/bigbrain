+++
title = "Programación orientada a objetos"
summary = "Clases y objetos, encapsulamiento, herencia, polimorfismo, abstracción, interfaces y composición."
branch = "Fundamentos de programación"
order = 3
level = "Intermedio"
tags = ["POO", "clases", "herencia", "polimorfismo", "encapsulamiento"]
+++

## La idea

La **programación orientada a objetos** (POO) organiza el programa en **objetos**: unidades que reúnen **datos** (atributos) y el **comportamiento** que opera sobre ellos (métodos).

- Una **clase** es el molde: define qué atributos y métodos tendrán sus objetos.
- Un **objeto** o **instancia** es un ejemplar concreto creado a partir de la clase.

```python
class CuentaBancaria:
    def __init__(self, titular, saldo=0):    # constructor
        self.titular = titular
        self.saldo = saldo

    def depositar(self, monto):
        self.saldo += monto

cuenta = CuentaBancaria("Ana", 1000)         # instancia
cuenta.depositar(500)
print(cuenta.saldo)                          # 1500
```

`self` (en otros lenguajes, `this`) es la referencia al objeto sobre el que se llama el método.

## Los cuatro pilares

| Pilar | Idea |
|---|---|
| **Encapsulamiento** | ocultar el estado interno y exponer solo operaciones seguras |
| **Abstracción** | mostrar qué hace un objeto, no cómo |
| **Herencia** | crear una clase a partir de otra, reutilizando su código |
| **Polimorfismo** | tratar de la misma forma a objetos de distintas clases |

## Encapsulamiento

Los atributos no se modifican libremente desde afuera: se accede a través de métodos que mantienen las **invariantes** (condiciones que siempre deben cumplirse).

```python
class CuentaBancaria:
    def __init__(self, titular):
        self.titular = titular
        self._saldo = 0                # el guion bajo indica «uso interno»

    @property
    def saldo(self):
        return self._saldo

    def extraer(self, monto):
        if monto <= 0:
            raise ValueError("el monto debe ser positivo")
        if monto > self._saldo:
            raise ValueError("saldo insuficiente")
        self._saldo -= monto
```

Así ningún código externo puede dejar el saldo en negativo.

| Visibilidad | Quién accede | Java, C++ | Python |
|---|---|---|---|
| Pública | cualquiera | `public` | sin prefijo |
| Protegida | la clase y sus subclases | `protected` | `_nombre` (convención) |
| Privada | solo la clase | `private` | `__nombre` |

## Herencia

Una **subclase** hereda los atributos y métodos de su **superclase** y puede agregar o **redefinir** (*override*) comportamiento. Expresa una relación «**es un**».

```python
class Empleado:
    def __init__(self, nombre, sueldo):
        self.nombre = nombre
        self.sueldo = sueldo

    def pago_mensual(self):
        return self.sueldo

class Gerente(Empleado):
    def __init__(self, nombre, sueldo, bono):
        super().__init__(nombre, sueldo)     # llama al constructor de la superclase
        self.bono = bono

    def pago_mensual(self):                  # redefinición
        return self.sueldo + self.bono
```

## Polimorfismo

El mismo mensaje produce comportamientos distintos según la clase del objeto.

```python
empleados = [Empleado("Luis", 1000), Gerente("Eva", 2000, 500)]

for e in empleados:
    print(e.nombre, e.pago_mensual())    # 1000 y 2500
```

El bucle no necesita saber qué tipo concreto es cada uno. Agregar una clase nueva no obliga a tocarlo.

> [!IMPORTANT]
> El polimorfismo reemplaza cadenas de `if tipo == ...`: cada clase sabe cómo hacer lo suyo.

## Abstracción e interfaces

Una **clase abstracta** define métodos sin implementarlos; no se puede instanciar y obliga a las subclases a completarlos.

```python
from abc import ABC, abstractmethod
import math

class Figura(ABC):
    @abstractmethod
    def area(self): ...

class Circulo(Figura):
    def __init__(self, radio):
        self.radio = radio

    def area(self):
        return math.pi * self.radio ** 2

class Rectangulo(Figura):
    def __init__(self, base, altura):
        self.base, self.altura = base, altura

    def area(self):
        return self.base * self.altura
```

Una **interfaz** es un contrato: una lista de métodos que una clase se compromete a ofrecer, sin datos ni implementación.

| | Clase abstracta | Interfaz |
|---|---|---|
| Puede tener atributos | sí | en general no |
| Puede tener código | sí | en general no (algunos lenguajes admiten métodos por defecto) |
| Cuántas se pueden usar | una (en Java) | varias |
| Expresa | «es un» | «puede hacer» |

## Composición

Un objeto puede **contener** a otros. Expresa una relación «**tiene un**».

```python
class Motor:
    def arrancar(self):
        return "motor en marcha"

class Auto:
    def __init__(self):
        self.motor = Motor()          # un auto tiene un motor

    def arrancar(self):
        return self.motor.arrancar()
```

> [!IMPORTANT]
> **Preferir la composición a la herencia.** La herencia acopla fuertemente la subclase a los detalles de la superclase; la composición permite combinar y cambiar piezas con libertad.

Usar herencia solo cuando la subclase realmente **es** un caso particular de la superclase y puede usarse en su lugar sin romper nada (**principio de sustitución de Liskov**).

> [!NOTE]
> ¿Un `Cuadrado` debe heredar de `Rectangulo`? Si `Rectangulo` permite cambiar la base sin cambiar la altura, un cuadrado no puede cumplir ese contrato. La herencia, aunque parezca natural, es incorrecta.

## Miembros de clase

- **Atributo de instancia**: cada objeto tiene el suyo.
- **Atributo de clase** (estático): lo comparten todas las instancias.
- **Método estático**: pertenece a la clase y no usa ninguna instancia.

```python
class Circulo:
    cantidad = 0                      # atributo de clase

    def __init__(self, radio):
        self.radio = radio            # atributo de instancia
        Circulo.cantidad += 1
```

## Igualdad e identidad

| Comparación | Pregunta | Python | Java |
|---|---|---|---|
| Identidad | ¿son el mismo objeto? | `is` | `==` |
| Igualdad | ¿tienen el mismo valor? | `==` | `equals()` |

## Diagrama de clases

En UML, una clase se dibuja como un rectángulo con tres partes: nombre, atributos y métodos.

| Relación | Significado | Símbolo |
|---|---|---|
| Herencia | es un | flecha con triángulo hueco |
| Implementación | cumple una interfaz | línea punteada con triángulo hueco |
| Asociación | usa o conoce | línea |
| Agregación | tiene (las partes existen por separado) | rombo hueco |
| Composición | tiene (las partes no existen sin el todo) | rombo lleno |

## Críticas y alternativas

La POO no es la única forma de organizar código. Las jerarquías profundas de herencia son rígidas y el estado mutable compartido dificulta la concurrencia. Lenguajes como Rust y Go no tienen herencia de clases: usan composición con **traits** o interfaces. Muchos lenguajes modernos combinan objetos con ideas funcionales.

> [!TIP]
> Al diseñar, buscar los **sustantivos** del problema (candidatos a clases) y los **verbos** (candidatos a métodos), y dar a cada clase una sola responsabilidad.
