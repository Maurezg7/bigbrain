+++
title = "Patrones de diseño"
summary = "Qué es un patrón y los más usados: creacionales, estructurales y de comportamiento, con ejemplos."
branch = "Ingeniería de software"
order = 4
level = "Avanzado"
tags = ["patrones de diseño", "singleton", "factory", "observer", "strategy"]
+++

## Qué es un patrón

Un **patrón de diseño** es una solución probada y con nombre a un problema de diseño que se repite. No es código para copiar, sino una idea que se adapta a cada caso.

Fueron catalogados en 1994 en el libro *Design Patterns*, de cuatro autores conocidos como la «banda de los cuatro» (GoF), que describe 23 patrones.

Sirven para:

- **Reutilizar** experiencia en lugar de reinventar.
- Tener un **vocabulario común**: decir «acá va un Observer» comunica un diseño entero.

| Categoría | Se ocupa de | Ejemplos |
|---|---|---|
| **Creacionales** | cómo se crean los objetos | Factory Method, Abstract Factory, Builder, Singleton, Prototype |
| **Estructurales** | cómo se componen | Adapter, Decorator, Facade, Composite, Proxy |
| **De comportamiento** | cómo se comunican y reparten responsabilidades | Strategy, Observer, Command, State, Iterator, Template Method |

Dos principios detrás de casi todos:

> [!IMPORTANT]
> - **Programar contra una interfaz**, no contra una implementación.
> - **Preferir la composición** de objetos a la herencia de clases.

## Creacionales

### Factory Method

**Problema**: el código necesita crear objetos sin atarse a una clase concreta.

**Solución**: delegar la creación en una función o método que decide qué clase instanciar.

```python
class NotificadorEmail:
    def enviar(self, mensaje): print(f"Email: {mensaje}")

class NotificadorSMS:
    def enviar(self, mensaje): print(f"SMS: {mensaje}")

def crear_notificador(canal):
    canales = {"email": NotificadorEmail, "sms": NotificadorSMS}
    return canales[canal]()

crear_notificador("sms").enviar("Hola")
```

Agregar un canal nuevo no obliga a tocar el código que lo usa.

### Builder

**Problema**: construir un objeto con muchos parámetros, varios opcionales.

**Solución**: un objeto constructor que se configura paso a paso.

```java
Pizza pizza = new Pizza.Builder()
    .tamanio("grande")
    .conQueso()
    .conAceitunas()
    .build();
```

### Singleton

**Problema**: debe existir una única instancia de una clase, accesible globalmente (una configuración, un registro).

> [!WARNING]
> El Singleton es el patrón más criticado: es estado global encubierto, oculta las dependencias y dificulta las pruebas. Casi siempre es mejor crear una sola instancia al inicio y **pasarla** a quien la necesite (inyección de dependencias).

## Estructurales

### Adapter

**Problema**: se quiere usar una clase cuya interfaz no coincide con la que el código espera.

**Solución**: un envoltorio que traduce una interfaz a otra, como un adaptador de enchufe.

```python
class PasarelaVieja:
    def hacer_pago(self, centavos): ...

class AdaptadorPasarela:
    def __init__(self, pasarela):
        self.pasarela = pasarela

    def pagar(self, pesos):                      # la interfaz esperada
        self.pasarela.hacer_pago(int(pesos * 100))
```

### Decorator

**Problema**: agregar responsabilidades a un objeto sin modificar su clase ni crear una subclase por cada combinación.

**Solución**: envolverlo en otro objeto con la misma interfaz que añade comportamiento antes o después.

```python
import time

def medir_tiempo(funcion):
    def envoltorio(*args, **kwargs):
        inicio = time.perf_counter()
        resultado = funcion(*args, **kwargs)
        print(f"{funcion.__name__}: {time.perf_counter() - inicio:.3f} s")
        return resultado
    return envoltorio

@medir_tiempo
def procesar(datos): ...
```

Los *middlewares* de los frameworks web siguen esta idea.

### Facade

**Problema**: un subsistema complejo es difícil de usar.

**Solución**: una clase que ofrece una interfaz simple y oculta los detalles.

```python
class Pedidos:
    def __init__(self, inventario, pagos, envios):
        self.inventario, self.pagos, self.envios = inventario, pagos, envios

    def comprar(self, cliente, producto):
        self.inventario.reservar(producto)
        self.pagos.cobrar(cliente, producto.precio)
        self.envios.despachar(cliente, producto)
```

### Composite

Trata de la misma forma a objetos individuales y a grupos de objetos, formando un árbol. Ejemplo: archivos y carpetas, donde ambos responden a `tamanio()`.

### Proxy

Un sustituto que controla el acceso a otro objeto: para cargarlo recién cuando hace falta, verificar permisos, guardar resultados en caché o acceder a algo remoto.

## De comportamiento

### Strategy

**Problema**: un algoritmo tiene varias variantes y se elige una en tiempo de ejecución.

**Solución**: encapsular cada variante detrás de una interfaz común e inyectar la elegida.

```python
def envio_estandar(peso): return 500 + 100 * peso
def envio_express(peso):  return 1200 + 150 * peso
def retiro_en_local(peso): return 0

class Pedido:
    def __init__(self, peso, calcular_envio):
        self.peso = peso
        self.calcular_envio = calcular_envio

    def costo_envio(self):
        return self.calcular_envio(self.peso)

Pedido(2, envio_express).costo_envio()      # 1500
```

Reemplaza los `if/elif` que crecen con cada variante. En lenguajes con funciones de primera clase, una estrategia es simplemente una función.

### Observer

**Problema**: cuando un objeto cambia, otros deben enterarse, sin que el primero los conozca.

**Solución**: los interesados se **suscriben**; el sujeto los **notifica**.

```python
class Evento:
    def __init__(self):
        self.suscriptores = []

    def suscribir(self, funcion):
        self.suscriptores.append(funcion)

    def emitir(self, dato):
        for funcion in self.suscriptores:
            funcion(dato)

pedido_creado = Evento()
pedido_creado.suscribir(lambda p: print("Enviar email por", p))
pedido_creado.suscribir(lambda p: print("Actualizar stock por", p))
pedido_creado.emitir("pedido 42")
```

Es la base de los eventos en las interfaces gráficas, de la reactividad de los frameworks web y de los sistemas de mensajería.

### Command

Convierte una operación en un objeto. Permite ponerla en cola, registrarla y **deshacerla**.

### State

Un objeto cambia su comportamiento según su estado interno; cada estado es una clase. Reemplaza condicionales repetidos sobre una variable de estado.

### Iterator

Recorre una colección sin exponer su estructura interna. Está incorporado en casi todos los lenguajes (`for ... in`).

### Template Method

Una clase base define el esqueleto de un algoritmo y deja que las subclases completen algunos pasos.

## Patrones de arquitectura

Operan a mayor escala:

| Patrón | Idea |
|---|---|
| **MVC** | separa datos (modelo), presentación (vista) y coordinación (controlador) |
| **Capas** | presentación, lógica de negocio, acceso a datos |
| **Repository** | oculta el acceso a datos detrás de una interfaz de colección |
| **Inyección de dependencias** | un objeto recibe lo que necesita en lugar de crearlo |
| **Publicar-suscribir** | componentes desacoplados que se comunican por mensajes |

## Antipatrones

Son soluciones habituales que dan malos resultados:

| Antipatrón | Descripción |
|---|---|
| Objeto dios | una clase que sabe y hace todo |
| Código espagueti | flujo enredado, sin estructura |
| Martillo de oro | usar la misma herramienta para todo |
| Optimización prematura | complicar antes de medir |
| Copiar y pegar | duplicación que multiplica los errores |
| Sobreingeniería | abstracciones para necesidades que no existen |

## Cuándo usarlos

> [!WARNING]
> Los patrones agregan clases e indirección. Aplicarlos «por las dudas» complica el código sin beneficio. Primero, la solución simple; el patrón entra cuando aparece el problema que resuelve.

Señales de que un patrón puede ayudar:

| Síntoma | Patrón candidato |
|---|---|
| `if` sobre un tipo repetido en muchos lugares | Strategy, State, polimorfismo |
| Constructores con demasiados parámetros | Builder |
| Hay que usar una biblioteca con otra interfaz | Adapter |
| Varios componentes reaccionan a un cambio | Observer |
| Hace falta deshacer operaciones | Command |
| El mismo agregado (registro, caché) en muchas funciones | Decorator |

> [!TIP]
> Aprender los patrones sirve sobre todo para **reconocerlos**: al leer un framework, saber que algo es un Observer o un Decorator ahorra horas.
