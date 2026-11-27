+++
title = "Spring Security"
summary = "Autenticación y autorización, cadena de filtros, contraseñas con BCrypt, JWT y OAuth 2, CSRF y CORS."
branch = "Frameworks de Java"
order = 3
level = "Avanzado"
tags = ["Java", "Spring Security", "JWT", "OAuth", "seguridad"]
+++

## Dos preguntas

| Concepto | Pregunta | Respuesta HTTP si falla |
|---|---|---|
| **Autenticación** | ¿quién sos? | 401 |
| **Autorización** | ¿tenés permiso para esto? | 403 |

Spring Security responde las dos antes de que el pedido llegue al controlador.

## La cadena de filtros

Cada pedido atraviesa una serie de **filtros**. Cada uno hace una tarea: leer el token, cargar el usuario, comprobar permisos, traducir errores. Si alguno rechaza el pedido, el controlador nunca se ejecuta.

Al agregar el starter, **todo queda protegido** por defecto. La configuración indica qué se abre:

```java
@Configuration
@EnableMethodSecurity
public class SeguridadConfig {

    @Bean
    SecurityFilterChain filtros(HttpSecurity http) throws Exception {
        return http
            .authorizeHttpRequests(auth -> auth
                .requestMatchers(HttpMethod.GET, "/api/temas/**").permitAll()
                .requestMatchers("/api/admin/**").hasRole("ADMIN")
                .anyRequest().authenticated())
            .sessionManagement(s -> s.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
            .csrf(csrf -> csrf.disable())            // API sin cookies de sesión
            .oauth2ResourceServer(o -> o.jwt(Customizer.withDefaults()))
            .build();
    }

    @Bean
    PasswordEncoder codificador() {
        return new BCryptPasswordEncoder();
    }
}
```

Las reglas se evalúan **en orden**: las más específicas van primero y `anyRequest()` al final.

## Contraseñas

Nunca se guarda la contraseña, sino un **hash** lento y con sal, hecho para resistir ataques por fuerza bruta.

```java
String hash = codificador.encode("clave-del-usuario");       // al registrarse
boolean ok = codificador.matches("clave-ingresada", hash);   // al iniciar sesión
```

| Algoritmo | Apto para contraseñas |
|---|---|
| BCrypt, Argon2, scrypt | sí |
| SHA-256, MD5 | no: son rápidos, justo lo contrario de lo que hace falta |

## Sesión o token

| | Sesión con cookie | Token JWT |
|---|---|---|
| Estado | en el servidor | en el token |
| Típico de | aplicaciones web renderizadas en servidor | API para SPA y móviles |
| Revocar | fácil: se borra la sesión | difícil: vale hasta que vence |
| Riesgo principal | CSRF | robo del token (XSS) |

## JWT

Un **JSON Web Token** tiene tres partes en Base64 separadas por puntos: encabezado, datos (*claims*) y **firma**.

```text
eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhbmEiLCJyb2xlcyI6WyJVU0VSIl0sImV4cCI6MTc2NTAwMH0.firma
```

El servidor verifica la firma con su clave: si alguien altera los datos, la firma deja de coincidir.

> [!WARNING]
> El contenido de un JWT está **codificado, no cifrado**: cualquiera puede leerlo. No se ponen ahí datos sensibles. Deben vencer pronto (minutos) y renovarse con un token de refresco.

## OAuth 2 y OpenID Connect

En lugar de manejar contraseñas propias, se delega el inicio de sesión en un **proveedor de identidad** (Google, Keycloak, Auth0). La aplicación recibe un token emitido por él y solo lo valida: a eso se lo llama ser un **servidor de recursos**.

```yaml
spring:
  security:
    oauth2:
      resourceserver:
        jwt:
          issuer-uri: https://identidad.ejemplo.com/realms/bigbrain
```

## Autorización por método

```java
@PreAuthorize("hasRole('ADMIN')")
public void borrar(Long id) { ... }

@PreAuthorize("#usuarioId == authentication.principal.id")
public Perfil verPerfil(Long usuarioId) { ... }
```

El segundo ejemplo comprueba que cada usuario vea **solo lo suyo**. Olvidar ese control es la falla más frecuente en aplicaciones web: el control de acceso roto.

## CSRF y CORS

| | Qué es | Qué hacer |
|---|---|---|
| **CSRF** | otro sitio hace que el navegador envíe un pedido con la cookie de la víctima | dejar la protección activa si se usan cookies de sesión |
| **CORS** | regla del navegador que impide leer respuestas de otro origen | permitir solo los orígenes propios, no `*` |

Con tokens en el encabezado `Authorization` y sin cookies, CSRF no aplica y puede desactivarse.

## Pruebas

```java
@Test
@WithMockUser(roles = "ADMIN")
void unAdministradorPuedeBorrar() throws Exception {
    mockMvc.perform(delete("/api/admin/temas/1")).andExpect(status().isNoContent());
}

@Test
void sinSesionRecibe401() throws Exception {
    mockMvc.perform(delete("/api/admin/temas/1")).andExpect(status().isUnauthorized());
}
```

## Buenas prácticas

- **Denegar por defecto** y abrir lo mínimo.
- HTTPS siempre.
- BCrypt o Argon2 para contraseñas; nunca texto plano ni hashes rápidos.
- Comprobar permisos en el servidor, aunque la interfaz oculte el botón.
- Tokens de vida corta y claves fuera del repositorio.
- Limitar los intentos de inicio de sesión.
- Probar los casos negativos: sin sesión, con rol insuficiente, con datos de otro usuario.
