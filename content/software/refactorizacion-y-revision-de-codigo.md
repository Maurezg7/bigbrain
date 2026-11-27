+++
title = "Refactorización y revisión de código"
summary = "Malos olores en el código, técnicas de refactorización, deuda técnica, trabajo con código heredado y revisión entre pares."
branch = "Ingeniería de software"
order = 6
level = "Intermedio"
tags = ["refactorización", "code review", "deuda técnica", "código limpio", "código heredado"]
+++

## Qué es refactorizar

**Refactorizar** es cambiar la estructura interna del código para hacerlo más fácil de entender y de modificar, **sin cambiar su comportamiento** observable.

No es arreglar errores ni agregar funciones: es ordenar.

> [!IMPORTANT]
> Regla de los dos sombreros: en cada momento, o se **agrega funcionalidad** o se **refactoriza**. Nunca las dos cosas en el mismo cambio.

**Por qué**: el código se lee muchas más veces de las que se escribe. Sin mantenimiento, cada cambio lo desordena un poco, hasta que cualquier modificación se vuelve lenta y riesgosa.

**Cuándo**:

- Antes de agregar una función, si el diseño actual la dificulta: «primero hacer fácil el cambio, después hacer el cambio fácil».
- Al entender un código confuso, para dejar escrita esa comprensión.
- **Regla del boy scout**: dejar el código un poco mejor de lo que se encontró.
- **Regla de tres**: la primera vez se escribe; la segunda, se tolera la duplicación; la tercera, se refactoriza.

## El requisito: pruebas

Sin pruebas automáticas no hay forma de saber si el comportamiento cambió.

El ciclo:

1. Asegurarse de que las pruebas pasan.
2. Hacer **un** cambio pequeño.
3. Correr las pruebas.
4. Guardar (*commit*) y repetir.

Pasos chicos: si algo se rompe, la causa está en las últimas líneas tocadas.

## Malos olores

Un **mal olor** (*code smell*) es un síntoma de un problema de diseño.

| Olor | Síntoma | Remedio habitual |
|---|---|---|
| Código duplicado | la misma lógica en varios lugares | extraer función |
| Función larga | no entra en una pantalla | extraer función |
| Clase grande | demasiadas responsabilidades | extraer clase |
| Lista larga de parámetros | más de tres o cuatro | introducir un objeto parámetro |
| Nombres confusos | `d`, `tmp2`, `procesar()` | renombrar |
| Números mágicos | `if dias > 30` | constante con nombre |
| Condicionales anidados | flechas de `if` dentro de `if` | cláusulas de guarda |
| `switch` sobre un tipo repetido | el mismo `if tipo ==` en muchos lugares | polimorfismo |
| Envidia de funcionalidad | un método usa más los datos de otra clase que los propios | mover método |
| Cirugía de escopeta | un cambio obliga a tocar muchos archivos | agrupar lo que cambia junto |
| Código muerto | funciones que nadie llama | borrar |
| Comentarios que explican el qué | el código no se entiende solo | renombrar, extraer |

## Técnicas

### Extraer función

```python
# Antes
def imprimir_factura(factura):
    total = 0
    for item in factura.items:
        total += item.precio * item.cantidad
    print(f"Cliente: {factura.cliente}")
    print(f"Total: {total}")

# Después
def total_de(factura):
    return sum(item.precio * item.cantidad for item in factura.items)

def imprimir_factura(factura):
    print(f"Cliente: {factura.cliente}")
    print(f"Total: {total_de(factura)}")
```

### Renombrar

```python
# Antes
def calc(d, t):
    return d / t

# Después
def velocidad_media(distancia_km, tiempo_h):
    return distancia_km / tiempo_h
```

Es la refactorización más barata y de las más valiosas.

### Reemplazar números mágicos

```python
# Antes
if edad >= 18 and saldo > 50000:

# Después
MAYORIA_DE_EDAD = 18
SALDO_MINIMO = 50_000

if edad >= MAYORIA_DE_EDAD and saldo > SALDO_MINIMO:
```

### Cláusulas de guarda

Tratar primero los casos especiales y salir, en lugar de anidar:

```python
# Antes
def precio_final(cliente, precio):
    if cliente is not None:
        if cliente.activo:
            if cliente.es_socio:
                return precio * 0.9
            else:
                return precio
        else:
            raise ValueError("cliente inactivo")
    else:
        raise ValueError("falta el cliente")

# Después
def precio_final(cliente, precio):
    if cliente is None:
        raise ValueError("falta el cliente")
    if not cliente.activo:
        raise ValueError("cliente inactivo")
    if cliente.es_socio:
        return precio * 0.9
    return precio
```

### Reemplazar condicional por polimorfismo

```python
# Antes
def area(figura):
    if figura.tipo == "circulo":
        return 3.14159 * figura.radio ** 2
    elif figura.tipo == "rectangulo":
        return figura.base * figura.altura

# Después: cada clase sabe calcular su área
class Circulo:
    def area(self): return 3.14159 * self.radio ** 2

class Rectangulo:
    def area(self): return self.base * self.altura
```

### Otras

| Técnica | Qué hace |
|---|---|
| Extraer variable | da nombre a una expresión compleja |
| Insertar en línea (*inline*) | elimina una función o variable que no aporta |
| Mover función | la lleva a la clase o módulo donde pertenece |
| Extraer clase | separa una responsabilidad |
| Introducir objeto parámetro | agrupa datos que viajan juntos |
| Separar consulta de modificación | una función o devuelve un valor o cambia estado, no ambas |

Los entornos de desarrollo automatizan muchas (renombrar, extraer, mover) de forma segura.

## Deuda técnica

Es el costo acumulado de las soluciones rápidas. Como una deuda, cobra **intereses**: cada cambio futuro es más lento.

| Tipo | Ejemplo |
|---|---|
| Deliberada y prudente | «entregamos así para llegar a la fecha, y lo corregimos después» |
| Deliberada e imprudente | «no hay tiempo para diseñar» |
| Inadvertida | «no sabíamos hacerlo mejor» |

Se gestiona: se registra, se prioriza según cuánto estorba, y se paga de a poco, en las zonas que más se modifican. Un código feo que nadie toca no cobra intereses.

> [!WARNING]
> «Reescribir todo desde cero» casi siempre fracasa: se pierden años de correcciones acumuladas y el sistema viejo sigue cambiando mientras tanto. Es preferible mejorar de forma incremental.

## Código heredado

Una definición práctica: **código sin pruebas**. Estrategia:

1. Escribir **pruebas de caracterización**: registran lo que el código hace **hoy**, sea correcto o no.
2. Encontrar **costuras**: puntos donde se puede cambiar el comportamiento sin editar ese lugar (por ejemplo, pasar una dependencia por parámetro).
3. Refactorizar en pasos pequeños, bajo la protección de esas pruebas.

El patrón del **estrangulador** reemplaza un sistema viejo de a partes: lo nuevo crece alrededor y va tomando funciones hasta que lo viejo se puede apagar.

## Revisión de código

En la **revisión entre pares**, otra persona lee un cambio antes de integrarlo. Suele hacerse sobre un *pull request*.

Sirve para:

- Encontrar errores antes de que lleguen a producción.
- Mantener la calidad y la coherencia.
- **Compartir conocimiento**: nadie es el único que entiende una parte.

### Para quien envía

| Práctica | Por qué |
|---|---|
| Cambios **pequeños** | se revisan mejor y más rápido |
| Un solo propósito por cambio | no mezclar refactorización con funcionalidad |
| Descripción clara: qué, por qué, cómo probarlo | da contexto |
| Revisar el propio cambio antes de enviarlo | evita lo obvio |
| Pruebas incluidas y tubería en verde | respeta el tiempo ajeno |

### Para quien revisa

Qué mirar, en orden de importancia:

1. **Corrección**: ¿hace lo que dice? ¿Y los casos borde?
2. **Diseño**: ¿encaja con el resto del sistema?
3. **Legibilidad**: ¿se entiende sin explicación?
4. **Pruebas**: ¿cubren lo importante?
5. **Seguridad y rendimiento**, donde corresponda.

El estilo y el formato se dejan a las herramientas automáticas.

| Hacer | Evitar |
|---|---|
| preguntar: «¿qué pasa si la lista está vacía?» | ordenar: «esto está mal» |
| comentar sobre el código | comentar sobre la persona |
| explicar el porqué de cada sugerencia | imponer gustos personales |
| distinguir lo bloqueante de lo opcional | bloquear por detalles menores |
| reconocer lo que está bien | solo señalar defectos |
| responder pronto | dejar el cambio esperando días |

> [!NOTE]
> En lugar de «Esto es ineficiente», escribir: «Este bucle consulta la base por cada elemento. ¿Podríamos traerlos todos en una sola consulta? Con 1000 pedidos serían 1000 consultas».

### Programación en pareja

Dos personas trabajan sobre el mismo código al mismo tiempo: una escribe y la otra revisa y piensa el siguiente paso. Es una revisión continua.

## Código limpio en resumen

| Principio | Significado |
|---|---|
| Nombres que revelan intención | el nombre dice qué es y para qué |
| Funciones pequeñas con una tarea | |
| DRY | no repetirse |
| KISS | mantenerlo simple |
| YAGNI | no construir lo que todavía no hace falta |
| Menor sorpresa | el código hace lo que parece |

> [!TIP]
> Un comentario que explica **qué** hace el código suele poder reemplazarse por un buen nombre. Los comentarios valiosos explican **por qué**.
