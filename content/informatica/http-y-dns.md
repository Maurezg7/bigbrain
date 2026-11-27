+++
title = "HTTP, DNS y la web"
summary = "Resolución de nombres con DNS, peticiones y respuestas HTTP, métodos, códigos de estado, cabeceras, cookies, caché, HTTPS y las versiones HTTP/2 y HTTP/3."
branch = "Redes"
order = 4
level = "Intermedio"
tags = ["HTTP", "DNS", "HTTPS", "cookies", "caché", "web"]
+++

## DNS

Las personas usan nombres (`ejemplo.com`); la red, direcciones IP. El **sistema de nombres de dominio** traduce unos en otras. Es una base de datos **distribuida y jerárquica**.

```text
                    . (raíz)
          ┌─────────┼─────────┐
         com       org        ar
          │                    │
       ejemplo              edu.ar
          │                    │
         www                 unsa
```

Un nombre se lee de derecha a izquierda: en `www.unsa.edu.ar`, `ar` es el dominio de nivel superior.

### Resolución

1. La aplicación consulta al **resolvedor** configurado (el del proveedor, o uno público como `1.1.1.1` u `8.8.8.8`).
2. Si no tiene la respuesta en caché, el resolvedor pregunta a un servidor **raíz**, que lo deriva a los servidores del dominio de nivel superior.
3. Estos lo derivan al servidor **autoritativo** del dominio.
4. El autoritativo da la respuesta, que el resolvedor guarda en caché.

Cada registro tiene un **TTL**: el tiempo durante el cual puede conservarse en caché. Por eso un cambio de DNS tarda en propagarse.

### Tipos de registros

| Registro | Contenido |
|---|---|
| A | dirección IPv4 |
| AAAA | dirección IPv6 |
| CNAME | alias de otro nombre |
| MX | servidor de correo del dominio |
| NS | servidores autoritativos del dominio |
| TXT | texto libre (verificaciones, SPF, DKIM) |
| SOA | datos administrativos de la zona |
| PTR | resolución inversa (de IP a nombre) |

```bash
dig +short ejemplo.com A
dig ejemplo.com MX
```

DNS usa UDP en el puerto 53 (y TCP para respuestas grandes). Las consultas tradicionales viajan sin cifrar; **DNS sobre HTTPS** y **DNS sobre TLS** las protegen.

## HTTP

**HTTP** es el protocolo de la web: un cliente envía una **petición** y el servidor devuelve una **respuesta**. Es **sin estado**: cada petición es independiente de las anteriores.

### URL

```text
https://ejemplo.com:443/cursos/fisica?nivel=basico&pagina=2#ondas
└─┬─┘   └────┬────┘└┬┘└─────┬──────┘└─────────┬──────────┘└─┬─┘
esquema   dominio puerto   ruta             consulta      fragmento
```

El fragmento no se envía al servidor: lo usa el navegador.

### Petición

```http
GET /cursos/fisica?nivel=basico HTTP/1.1
Host: ejemplo.com
Accept: text/html
Accept-Language: es
User-Agent: Mozilla/5.0
```

### Respuesta

```http
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 5120
Cache-Control: max-age=3600

<!doctype html>
<html>...
```

Ambas tienen la misma forma: una línea inicial, cabeceras, una línea en blanco y un cuerpo opcional.

## Métodos

| Método | Uso | Seguro | Idempotente |
|---|---|---|---|
| GET | obtener un recurso | sí | sí |
| HEAD | como GET, sin cuerpo | sí | sí |
| POST | crear o procesar | no | no |
| PUT | reemplazar por completo | no | sí |
| PATCH | modificar parcialmente | no | no |
| DELETE | eliminar | no | sí |
| OPTIONS | consultar las opciones admitidas | sí | sí |

- **Seguro**: no modifica nada en el servidor.
- **Idempotente**: repetirlo deja el mismo resultado que hacerlo una vez. Importa para reintentar con tranquilidad después de un fallo de red.

## Códigos de estado

| Clase | Significado | Ejemplos |
|---|---|---|
| 1xx | informativo | 101 Switching Protocols |
| 2xx | éxito | 200 OK, 201 Created, 204 No Content |
| 3xx | redirección | 301 Moved Permanently, 302 Found, 304 Not Modified |
| 4xx | error del cliente | 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 409 Conflict, 422 Unprocessable Content, 429 Too Many Requests |
| 5xx | error del servidor | 500 Internal Server Error, 502 Bad Gateway, 503 Service Unavailable, 504 Gateway Timeout |

> [!NOTE]
> **401** significa «no sé quién sos» (falta autenticarse) y **403**, «sé quién sos y no tenés permiso».

## Cabeceras

| Cabecera | Sentido | Función |
|---|---|---|
| `Host` | petición | el dominio pedido |
| `Content-Type` | ambos | formato del cuerpo (`application/json`, `text/html`) |
| `Content-Length` | ambos | tamaño del cuerpo |
| `Accept` | petición | formatos que acepta el cliente |
| `Authorization` | petición | credenciales |
| `Cookie` / `Set-Cookie` | petición / respuesta | cookies |
| `Cache-Control` | ambos | reglas de caché |
| `Location` | respuesta | destino de una redirección |
| `Content-Encoding` | respuesta | compresión (`gzip`, `br`) |

## Cookies y sesiones

Como HTTP no tiene estado, las **cookies** permiten reconocer al mismo cliente entre peticiones. El servidor las envía con `Set-Cookie` y el navegador las devuelve en cada petición al mismo sitio.

```http
Set-Cookie: sesion=a81f3c; HttpOnly; Secure; SameSite=Lax; Max-Age=86400
```

| Atributo | Efecto |
|---|---|
| `HttpOnly` | JavaScript no puede leerla (protege ante XSS) |
| `Secure` | solo se envía por HTTPS |
| `SameSite` | limita su envío desde otros sitios (protege ante CSRF) |
| `Max-Age`, `Expires` | duración |
| `Domain`, `Path` | a qué direcciones se envía |

En una **sesión**, la cookie contiene solo un identificador aleatorio; los datos quedan en el servidor. La alternativa son los **tokens** (JWT), que llevan los datos firmados.

## Caché

Evita repetir transferencias. La controla la cabecera `Cache-Control`:

| Directiva | Significado |
|---|---|
| `max-age=3600` | la respuesta vale durante una hora |
| `no-cache` | se puede guardar, pero hay que **revalidar** antes de usarla |
| `no-store` | no guardar nunca (datos sensibles) |
| `private` | solo en la caché del navegador |
| `public` | también en cachés intermedias |
| `immutable` | no cambiará: no hace falta revalidar |

**Revalidación.** El servidor envía un `ETag` (una huella del contenido). El navegador pregunta luego con `If-None-Match`; si no cambió, el servidor responde **304 Not Modified**, sin cuerpo.

Una **CDN** (red de distribución de contenido) guarda copias en servidores repartidos por el mundo, cerca de los usuarios.

> [!TIP]
> Práctica habitual: los archivos estáticos llevan una huella en el nombre (`app.3f9a1c.js`) y se sirven con `max-age` de un año e `immutable`; el HTML, con `no-cache`.

## HTTPS

**HTTPS** es HTTP sobre **TLS**. Aporta:

| Propiedad | Significado |
|---|---|
| Confidencialidad | nadie en el camino puede leer el contenido |
| Integridad | nadie puede modificarlo sin que se note |
| Autenticación | el servidor demuestra ser quien dice, con un **certificado** |

El certificado vincula un dominio con una clave pública y está firmado por una **autoridad de certificación** en la que el navegador confía. Let's Encrypt los emite gratis y de forma automática.

Un observador sigue viendo la dirección IP de destino y el volumen de tráfico; no ve las rutas, las cabeceras ni el contenido.

## Versiones

| Versión | Transporte | Novedad principal |
|---|---|---|
| HTTP/1.1 | TCP | conexiones persistentes; una petición por vez en cada conexión |
| HTTP/2 | TCP | **multiplexación**: muchas peticiones simultáneas por una conexión; cabeceras comprimidas; formato binario |
| HTTP/3 | QUIC (sobre UDP) | elimina el bloqueo que una pérdida de TCP causa a todos los flujos; conexión más rápida |

La semántica (métodos, códigos, cabeceras) es la misma en las tres.

## Política del mismo origen y CORS

Un **origen** es la combinación de esquema, dominio y puerto. El navegador impide que el código de un origen lea respuestas de otro.

**CORS** permite al servidor autorizar orígenes concretos:

```http
Access-Control-Allow-Origin: https://app.ejemplo.com
Access-Control-Allow-Methods: GET, POST
```

Para las peticiones que no son simples, el navegador envía antes una **petición previa** (*preflight*) con el método OPTIONS.

> [!WARNING]
> CORS lo aplica **el navegador**, no el servidor: no protege a una API de clientes que no sean navegadores. No reemplaza a la autenticación.

## Más allá de petición y respuesta

| Tecnología | Uso |
|---|---|
| WebSocket | canal bidireccional permanente (chat, juegos) |
| *Server-Sent Events* | el servidor envía eventos por una conexión HTTP abierta |
| Sondeo (*polling*) | el cliente pregunta periódicamente |
| gRPC | llamadas a procedimientos remotos sobre HTTP/2 |

## Qué pasa al abrir una página

1. El navegador **resuelve** el nombre por DNS.
2. Abre una conexión **TCP** con el servidor (o QUIC).
3. Negocia **TLS** y verifica el certificado.
4. Envía la petición **HTTP** `GET /`.
5. Recibe el HTML y lo analiza.
6. Pide los recursos referenciados: CSS, JavaScript, imágenes, fuentes.
7. Construye el árbol del documento, calcula los estilos y **dibuja** la página.
8. Ejecuta el JavaScript, que puede hacer más peticiones.

## Herramientas

```bash
curl -i https://ejemplo.com/api/temas             # muestra cabeceras y cuerpo
curl -X POST https://ejemplo.com/api/temas \
     -H "Content-Type: application/json" \
     -d '{"titulo": "Redes"}'
```

> [!TIP]
> La pestaña **Red** de las herramientas de desarrollo del navegador (F12) muestra cada petición con sus cabeceras, tiempos, estado y si vino de la caché. Es la mejor forma de ver HTTP en acción.
