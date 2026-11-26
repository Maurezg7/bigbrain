+++
title = "Diseño de API REST"
summary = "Recursos y verbos HTTP, códigos de estado, paginación, errores, versionado, idempotencia y documentación con OpenAPI."
branch = "Buenas prácticas"
order = 3
level = "Intermedio"
tags = ["API", "REST", "HTTP", "OpenAPI", "backend"]
+++

## Qué es una API REST

Una **API** es el contrato por el que dos programas se comunican. **REST** es un estilo para diseñarla sobre HTTP: la API expone **recursos** (temas, usuarios, pedidos) identificados por una URL, y se opera sobre ellos con los métodos estándar de HTTP.

## Recursos y URL

Las URL nombran **cosas**, en plural y con sustantivos. La acción la indica el método.

| Bien | Mal |
|---|---|
| `GET /temas` | `GET /obtenerTemas` |
| `GET /temas/42` | `GET /tema?id=42` |
| `POST /temas` | `POST /crearTema` |
| `DELETE /temas/42` | `GET /temas/42/borrar` |
| `GET /areas/3/temas` | `GET /temasDelArea3` |

## Métodos

| Método | Uso | Seguro | Idempotente |
|---|---|---|---|
| `GET` | leer | sí | sí |
| `POST` | crear, o ejecutar una acción | no | no |
| `PUT` | reemplazar entero | no | sí |
| `PATCH` | modificar en parte | no | no necesariamente |
| `DELETE` | borrar | no | sí |

**Seguro**: no modifica nada. **Idempotente**: repetirlo deja el mismo resultado que hacerlo una vez. Importa porque las redes fallan y los clientes reintentan.

## Códigos de estado

| Código | Significado | Cuándo |
|---|---|---|
| 200 | OK | lectura o modificación correcta |
| 201 | Creado | después de un `POST` que crea; con el encabezado `Location` |
| 204 | Sin contenido | borrado correcto |
| 400 | Pedido incorrecto | JSON mal formado |
| 401 | No autenticado | falta el token o no es válido |
| 403 | Prohibido | autenticado pero sin permiso |
| 404 | No encontrado | el recurso no existe |
| 409 | Conflicto | duplicado, o choque de versiones |
| 422 | No procesable | los datos no pasan la validación |
| 429 | Demasiados pedidos | se superó el límite |
| 500 | Error interno | falla del servidor |
| 503 | No disponible | sobrecarga o mantenimiento |

Regla: **4xx** es culpa del cliente y **5xx** es culpa del servidor.

> [!WARNING]
> No respondas `200 OK` con `{ "error": "..." }` en el cuerpo. Los clientes, las cachés y el monitoreo confían en el código de estado.

## Ejemplo

```http
POST /api/temas HTTP/1.1
Content-Type: application/json
Authorization: Bearer eyJhbGciOi...

{ "titulo": "Ley de Ohm", "area": "electromecanica" }
```

```http
HTTP/1.1 201 Created
Location: /api/temas/42
Content-Type: application/json

{ "id": 42, "titulo": "Ley de Ohm", "area": "electromecanica", "creado": "2026-03-01T14:30:00Z" }
```

## Formato

- JSON con un estilo de nombres **consistente** en toda la API.
- Fechas en ISO 8601 y UTC: `2026-03-01T14:30:00Z`.
- Dinero en enteros (centavos) o en texto decimal, nunca en coma flotante.
- Ids opacos: el cliente no debe deducir nada de ellos.

## Errores

Un formato único para todos. El estándar es **Problem Details** (RFC 9457):

```json
{
  "type": "https://bigbrain.example/errores/validacion",
  "title": "Datos inválidos",
  "status": 422,
  "detail": "El título no puede estar vacío.",
  "errors": [{ "campo": "titulo", "mensaje": "es obligatorio" }]
}
```

El mensaje debe servirle al cliente para corregir el pedido, sin revelar detalles internos.

## Listas

Nunca devolver una colección sin límite.

```http
GET /api/temas?area=fisica&orden=-creado&limite=20&cursor=eyJpZCI6NDJ9
```

| Paginación | Cómo | Ventaja | Problema |
|---|---|---|---|
| Por desplazamiento | `?pagina=3&limite=20` | simple; permite saltar de página | lenta en páginas lejanas; se desordena si cambian los datos |
| Por cursor | `?cursor=...&limite=20` | rápida y estable | no permite saltar |

```json
{ "items": [], "siguiente": "eyJpZCI6NjJ9" }
```

## Idempotencia en la creación

Si un `POST` se corta, el cliente no sabe si se creó el recurso. Reintentar podría duplicarlo (un pago, por ejemplo). Solución: el cliente envía una clave única y el servidor recuerda el resultado.

```http
POST /api/pagos
Idempotency-Key: 7f3c2a9e-1b4d-4e8a-9c6f-2d5b8e1a3c47
```

## Versionado

Los clientes no se actualizan todos a la vez. Hay que evitar los cambios que rompen.

| Cambio | ¿Rompe? |
|---|---|
| agregar un campo opcional a la respuesta | no |
| agregar una ruta nueva | no |
| quitar o renombrar un campo | sí |
| cambiar el tipo de un campo | sí |
| volver obligatorio un parámetro | sí |

Cuando el cambio incompatible es inevitable, se publica una versión nueva (`/api/v2/temas`) y la anterior se mantiene un tiempo anunciado.

## Caché y concurrencia

- `Cache-Control: max-age=300` deja que el cliente reutilice la respuesta.
- `ETag` identifica la versión de un recurso. Con `If-None-Match` el servidor responde `304 Not Modified` sin cuerpo; con `If-Match` evita que dos personas se pisen los cambios.

## Seguridad

- HTTPS siempre.
- Autenticación por token en el encabezado, no en la URL.
- Comprobar en cada pedido que el recurso pertenece a quien lo pide.
- Límite de pedidos por cliente y de tamaño del cuerpo.
- Validar toda entrada.

## Documentación

**OpenAPI** describe la API en un archivo que las herramientas convierten en documentación interactiva, clientes generados y pruebas.

```yaml
paths:
  /temas/{id}:
    get:
      summary: Obtiene un tema
      parameters:
        - { name: id, in: path, required: true, schema: { type: integer } }
      responses:
        '200': { description: El tema }
        '404': { description: No existe }
```

## Otras formas de API

| Estilo | Cuándo |
|---|---|
| **REST** | API públicas y casos generales |
| **GraphQL** | el cliente elige qué campos pedir; pantallas con datos muy variados |
| **gRPC** | comunicación interna entre servicios, binaria y rápida |
| **WebSocket** y SSE | datos en tiempo real |

## Buenas prácticas

- Consistencia ante todo: los mismos nombres, formatos y errores en toda la API.
- Diseñar desde el punto de vista de quien la consume.
- Paginar, filtrar y ordenar con parámetros de consulta.
- No exponer la estructura de la base de datos tal cual.
- Documentar con ejemplos reales y mantener el documento junto al código.
