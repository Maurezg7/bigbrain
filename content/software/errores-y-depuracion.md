+++
title = "Errores, excepciones y depuración"
summary = "Tipos de errores, manejo de excepciones, programación defensiva, pruebas y técnicas sistemáticas para encontrar fallas."
branch = "Fundamentos de programación"
order = 5
level = "Intermedio"
tags = ["excepciones", "depuración", "errores", "pruebas", "debugging"]
+++

## Tipos de errores

| Tipo | Cuándo aparece | Ejemplo |
|---|---|---|
| **De sintaxis** | al compilar o interpretar | falta un paréntesis |
| **De tipos** | al compilar (tipado estático) o al ejecutar | sumar un número y un texto |
| **En tiempo de ejecución** | durante la ejecución | dividir por cero, índice fuera de rango |
| **Lógicos** | nunca «saltan»: el resultado es incorrecto | usar `<` en lugar de `<=` |

Los lógicos son los más difíciles: el programa corre sin quejarse.

Errores típicos:

| Error | Descripción |
|---|---|
| *Off-by-one* | el bucle hace una iteración de más o de menos |
| Referencia nula | usar un valor que no existe (`None`, `null`) |
| Bucle infinito | la condición de salida nunca se cumple |
| Desbordamiento | un número excede el rango de su tipo |
| Condición de carrera | el resultado depende del orden entre hilos |
| Comparación de flotantes | `0.1 + 0.2 == 0.3` es falso |

## Excepciones

Una **excepción** interrumpe el flujo normal y sube por la pila de llamadas hasta que alguien la **captura**. Si nadie lo hace, el programa termina.

```python
def leer_edad(texto):
    try:
        edad = int(texto)
    except ValueError:
        print("No es un número válido")
        return None
    else:
        return edad            # solo si no hubo excepción
    finally:
        print("Fin")           # se ejecuta siempre
```

**Lanzar** una excepción:

```python
def raiz(x):
    if x < 0:
        raise ValueError("x no puede ser negativo")
    return x ** 0.5
```

**Definir** excepciones propias:

```python
class SaldoInsuficiente(Exception):
    pass
```

### Buenas prácticas

| Hacer | Evitar |
|---|---|
| capturar excepciones **específicas** | `except:` que atrapa todo |
| capturar donde se puede hacer algo útil | capturar y no hacer nada (silenciar) |
| mensajes con contexto (qué, con qué dato) | mensajes genéricos |
| liberar recursos siempre (`finally`, `with`) | dejar archivos o conexiones abiertos |
| excepciones para situaciones excepcionales | usarlas como control de flujo normal |

> [!WARNING]
> Un `except: pass` esconde los errores en lugar de resolverlos: el programa sigue con datos corruptos y falla más tarde, lejos de la causa.

```python
with open("datos.txt") as archivo:     # se cierra solo, aun si hay error
    contenido = archivo.read()
```

### Otras estrategias

| Estrategia | Lenguajes | Idea |
|---|---|---|
| Excepciones | Python, Java, C++, JavaScript | el error «salta» por la pila |
| Códigos de retorno | C | la función devuelve un código que hay que revisar |
| Valores de resultado | Rust (`Result`), Go (`error`) | el error es un valor que el llamador debe tratar |

## Programación defensiva

- **Validar las entradas** en los bordes del sistema (usuario, archivos, red).
- **Fallar pronto**: detectar el problema cerca de su causa.
- **Aserciones**: verifican supuestos internos que nunca deberían violarse.

```python
def promedio(valores):
    assert len(valores) > 0, "la lista no puede estar vacía"
    return sum(valores) / len(valores)
```

Las aserciones son para errores de programación; para datos externos inválidos se usan excepciones.

## Depuración

**Depurar** (*debugging*) es encontrar y corregir la causa de una falla. Método:

1. **Reproducir** el error de forma confiable.
2. **Aislar**: reducir el caso al mínimo que lo produce.
3. **Formular una hipótesis** sobre la causa.
4. **Comprobarla** con un experimento.
5. **Corregir** la causa, no el síntoma.
6. **Verificar**, y agregar una prueba para que no vuelva.

> [!IMPORTANT]
> Leer el mensaje de error **completo**. La traza indica el archivo, la línea y la cadena de llamadas.

```text
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    print(promedio([]))
  File "main.py", line 5, in promedio
    return sum(valores) / len(valores)
ZeroDivisionError: division by zero
```

Se lee de abajo hacia arriba: el tipo de error, dónde ocurrió y quién lo llamó.

### Técnicas

| Técnica | Cuándo usarla |
|---|---|
| Imprimir valores | rápida, para ver el flujo y los datos |
| **Depurador** | pausar, avanzar paso a paso e inspeccionar variables |
| **Registro** (*logging*) | programas que corren mucho tiempo o en producción |
| **Búsqueda binaria** | comentar la mitad del código para acotar dónde falla |
| `git bisect` | hallar el commit que introdujo el error |
| Explicarlo en voz alta | el «pato de goma»: al explicar, aparece el supuesto falso |
| Leer la documentación | cuando una función no hace lo que se supone |

Funciones del depurador:

| Función | Qué hace |
|---|---|
| Punto de interrupción (*breakpoint*) | pausa en una línea |
| Paso a paso (*step over*) | ejecuta la línea actual |
| Entrar (*step into*) | entra a la función llamada |
| Salir (*step out*) | termina la función actual |
| Inspección | muestra variables y la pila |
| Punto condicional | pausa solo si se cumple una condición |

### Registro

```python
import logging

logging.basicConfig(level=logging.INFO)
logging.info("Procesando %d registros", len(datos))
logging.error("No se pudo abrir %s", ruta)
```

| Nivel | Uso |
|---|---|
| DEBUG | detalle para diagnóstico |
| INFO | funcionamiento normal |
| WARNING | algo inesperado, pero se continúa |
| ERROR | una operación falló |
| CRITICAL | el programa no puede seguir |

## Pruebas

La mejor depuración es la que no hace falta. Una **prueba unitaria** verifica una función de forma automática.

```python
def test_promedio():
    assert promedio([2, 4, 6]) == 4
    assert promedio([5]) == 5
```

Casos que conviene probar siempre:

- el caso normal;
- los **bordes**: vacío, un solo elemento, el máximo;
- valores inválidos: negativos, nulos, tipos incorrectos.

Cuando se encuentra un error, primero se escribe una prueba que lo reproduzca y recién después se corrige.

## Herramientas que previenen errores

| Herramienta | Qué detecta |
|---|---|
| Compilador y sistema de tipos | errores de tipos antes de ejecutar |
| *Linter* | código sospechoso y malas prácticas |
| Formateador | estilo inconsistente |
| Analizador estático | fugas, valores nulos, código inalcanzable |
| Revisión de código | lo que las herramientas no ven |

> [!TIP]
> Cambiar **una sola cosa por vez** y volver a probar. Si se cambian cinco cosas y el error desaparece, no se sabe cuál lo arregló.
