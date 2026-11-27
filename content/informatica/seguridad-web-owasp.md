+++
title = "Seguridad web: OWASP Top 10"
summary = "Las fallas más comunes en aplicaciones web y cómo prevenirlas: control de acceso, inyección, XSS, CSRF, secretos y dependencias."
branch = "Buenas prácticas"
order = 4
level = "Intermedio"
tags = ["seguridad", "OWASP", "XSS", "inyección SQL", "autenticación"]
+++

## Pensar como atacante

Toda aplicación conectada a Internet recibe ataques automáticos a los pocos minutos de publicarse. La regla base de la seguridad: **nunca confiar en los datos que llegan de afuera**. El navegador, la aplicación móvil y los formularios están en manos del usuario, y un atacante puede enviar cualquier cosa.

**OWASP** es una fundación que publica el *Top 10*: la lista de los riesgos más graves en aplicaciones web.

## OWASP Top 10 (2021)

| N.º | Riesgo | Ejemplo |
|---|---|---|
| A01 | Control de acceso roto | ver datos de otro usuario cambiando un id |
| A02 | Fallas criptográficas | contraseñas en texto plano, sin HTTPS |
| A03 | Inyección | SQL, comandos, XSS |
| A04 | Diseño inseguro | no limitar los intentos de inicio de sesión |
| A05 | Configuración insegura | claves por defecto, modo depuración en producción |
| A06 | Componentes vulnerables | dependencias desactualizadas |
| A07 | Fallas de autenticación | contraseñas débiles, sesiones que no vencen |
| A08 | Fallas de integridad | actualizaciones o paquetes sin verificar |
| A09 | Registro y monitoreo insuficientes | nadie se entera del ataque |
| A10 | SSRF | el servidor hace pedidos a donde el atacante indica |

## Control de acceso roto

Es el riesgo número uno. Ocurre cuando el servidor comprueba que el usuario inició sesión, pero no que **ese recurso sea suyo**.

```http
GET /api/facturas/1001      la mía
GET /api/facturas/1002      la de otro: debe responder 403 o 404
```

```ts
const factura = await db.factura.findFirst({
  where: { id, usuarioId: sesion.usuarioId },   // filtra por dueño
});
if (!factura) return res.status(404).end();
```

> [!WARNING]
> Ocultar un botón en la interfaz **no es seguridad**. Cada ruta del servidor debe comprobar los permisos por su cuenta.

## Inyección SQL

Aparece al armar una consulta concatenando texto del usuario.

```ts
// VULNERABLE
db.query(`SELECT * FROM usuarios WHERE email = '${email}'`);
```

Si `email` vale `' OR '1'='1`, la condición es siempre verdadera y devuelve todos los usuarios.

La solución son las **consultas parametrizadas**: los datos viajan separados del SQL.

```ts
db.query('SELECT * FROM usuarios WHERE email = $1', [email]);
```

Los ORM y constructores de consultas lo hacen solos, mientras no se use texto crudo concatenado.

## XSS

En el *cross-site scripting*, el atacante logra que la página ejecute su JavaScript en el navegador de otra persona, por ejemplo con un comentario que contiene `<script>`. Con eso puede robar la sesión o actuar en nombre de la víctima.

| Defensa | Cómo |
|---|---|
| Escapar la salida | los frameworks (React, Svelte, Vue, Angular) lo hacen por defecto |
| Evitar HTML crudo | cuidado con `{@html}`, `v-html`, `dangerouslySetInnerHTML`, `innerHTML` |
| Sanear | si hay que mostrar HTML del usuario, pasarlo por DOMPurify |
| Política de contenido | encabezado `Content-Security-Policy` |
| Cookies | marcarlas `HttpOnly` para que JavaScript no las lea |

## CSRF

En la falsificación de pedidos entre sitios, una página maliciosa hace que el navegador de la víctima envíe un pedido a otro sitio donde tiene sesión abierta; el navegador adjunta la cookie por su cuenta.

Defensas: cookies con `SameSite=Lax` o `Strict`, **tokens anti-CSRF** en los formularios, y no usar `GET` para acciones que modifican datos.

## Autenticación

| Tema | Práctica |
|---|---|
| Contraseñas | guardarlas con Argon2 o BCrypt, nunca en texto plano |
| Longitud | preferir frases largas a reglas rebuscadas |
| Segundo factor | ofrecer aplicación de códigos o llave física |
| Fuerza bruta | limitar intentos y demorar las respuestas |
| Sesiones | identificador aleatorio, cookie `HttpOnly`, `Secure` y `SameSite`; vencimiento |
| Mensajes | «usuario o contraseña incorrectos», sin decir cuál |

## Secretos

Las claves de API, contraseñas de base y claves de firma **no van en el repositorio**.

- Variables de entorno o un gestor de secretos.
- `.env` listado en `.gitignore`.
- Si un secreto se filtró, **rotarlo**: borrarlo del historial no alcanza, porque ya pudo copiarse.
- Nada secreto en el código del frontend: todo lo que llega al navegador es público.

## Dependencias

La mayor parte del código de una aplicación es de terceros.

| Ecosistema | Herramienta |
|---|---|
| Rust | `cargo audit`, `cargo deny` |
| Java | OWASP Dependency-Check |
| JavaScript | `npm audit`, `bun audit` |
| Todos | Dependabot o Renovate para actualizar |

Conviene usar el archivo de bloqueo, pocas dependencias y bien mantenidas.

## Encabezados de seguridad

| Encabezado | Efecto |
|---|---|
| `Strict-Transport-Security` | obliga a usar HTTPS |
| `Content-Security-Policy` | limita de dónde se cargan scripts y recursos |
| `X-Content-Type-Options: nosniff` | impide adivinar el tipo de contenido |
| `X-Frame-Options: DENY` | evita que el sitio se incruste en otro (*clickjacking*) |
| `Referrer-Policy` | controla qué se envía como origen |

## Otros riesgos

- **Subida de archivos**: validar tipo y tamaño, renombrar y guardar fuera de la carpeta pública.
- **Recorrido de rutas**: un nombre como `../../etc/passwd`. Hay que normalizar la ruta y comprobar que quede dentro de la carpeta permitida.
- **SSRF**: si el servidor descarga una URL dada por el usuario, restringir a destinos permitidos y bloquear las direcciones internas.
- **Asignación masiva**: aceptar el JSON entero permite que alguien envíe `"esAdmin": true`. Usar DTO con los campos permitidos.

## Principios

| Principio | Significa |
|---|---|
| Mínimo privilegio | cada parte tiene solo los permisos que necesita |
| Defensa en profundidad | varias capas: si una falla, queda otra |
| Denegar por defecto | lo que no se permite expresamente, se rechaza |
| Fallar de forma segura | ante un error, cerrar, no abrir |
| No inventar criptografía | usar bibliotecas probadas |

## Buenas prácticas

- Validar en el servidor todo lo que entra.
- Consultas parametrizadas siempre.
- Comprobar la propiedad de cada recurso.
- HTTPS en todo el sitio.
- Registrar los eventos de seguridad (inicios de sesión fallidos, accesos denegados), sin guardar contraseñas ni tokens en los registros.
- Actualizar dependencias y revisarlas en integración continua.
- Tener copias de respaldo y probar su restauración.
