+++
title = "Spring Boot"
summary = "Inversión de control, inyección de dependencias, controladores REST, configuración, validación, manejo de errores y arquitectura en capas."
branch = "Frameworks de Java"
order = 1
level = "Intermedio"
tags = ["Java", "Spring Boot", "API REST", "inyección de dependencias", "backend"]
+++

## Spring y Spring Boot

**Spring** es el framework más usado del ecosistema Java. Su núcleo es un **contenedor** que crea los objetos de la aplicación y los conecta entre sí. **Spring Boot** agrega sobre eso configuración automática, un servidor embebido y dependencias ya combinadas, para que una aplicación arranque con muy poco código.

```java
@SpringBootApplication
public class BigBrainApplication {
    public static void main(String[] args) {
        SpringApplication.run(BigBrainApplication.class, args);
    }
}
```

El proyecto se genera en *start.spring.io* eligiendo los **starters**: paquetes que traen todo lo necesario para una función.

| Starter | Trae |
|---|---|
| `spring-boot-starter-web` | Spring MVC, Tomcat embebido, JSON con Jackson |
| `spring-boot-starter-data-jpa` | JPA con Hibernate |
| `spring-boot-starter-security` | autenticación y autorización |
| `spring-boot-starter-validation` | validación de datos |
| `spring-boot-starter-test` | JUnit, Mockito, AssertJ |
| `spring-boot-starter-actuator` | salud y métricas |

## Inversión de control e inyección de dependencias

En lugar de que cada clase cree lo que necesita con `new`, **declara** qué necesita y el contenedor se lo entrega. A eso se lo llama **inyección de dependencias**. Los objetos que administra el contenedor son los **beans**.

```java
@Service
public class TemaService {
    private final TemaRepository repositorio;

    public TemaService(TemaRepository repositorio) {   // inyección por constructor
        this.repositorio = repositorio;
    }

    public Tema buscar(Long id) {
        return repositorio.findById(id)
                .orElseThrow(() -> new TemaNoEncontradoException(id));
    }
}
```

| Anotación | Marca |
|---|---|
| `@Component` | un bean genérico |
| `@Service` | lógica de negocio |
| `@Repository` | acceso a datos |
| `@RestController` | controlador que responde JSON |
| `@Configuration` y `@Bean` | beans creados a mano |

> [!TIP]
> Preferí siempre la **inyección por constructor** con campos `final`. Deja las dependencias a la vista, impide objetos a medio armar y permite probar la clase con un simple `new`, sin levantar Spring.

Por defecto cada bean es un **singleton**: una sola instancia compartida por todos los pedidos. Por eso no debe guardar en sus campos datos de un pedido en particular.

## Controladores REST

```java
@RestController
@RequestMapping("/api/temas")
public class TemaController {
    private final TemaService servicio;

    public TemaController(TemaService servicio) { this.servicio = servicio; }

    @GetMapping
    public List<TemaDto> listar(@RequestParam(defaultValue = "0") int pagina) {
        return servicio.listar(pagina);
    }

    @GetMapping("/{id}")
    public TemaDto ver(@PathVariable Long id) {
        return servicio.ver(id);
    }

    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public TemaDto crear(@Valid @RequestBody NuevoTema nuevo) {
        return servicio.crear(nuevo);
    }
}
```

| Anotación | Obtiene |
|---|---|
| `@PathVariable` | parte de la ruta |
| `@RequestParam` | parámetro de la URL |
| `@RequestBody` | cuerpo JSON convertido en objeto |
| `@RequestHeader` | un encabezado |

## DTO y validación

Un **DTO** (objeto de transferencia de datos) define qué entra y qué sale de la API, separado de las entidades de la base. Los `record` de Java son ideales:

```java
public record NuevoTema(
        @NotBlank @Size(max = 120) String titulo,
        @NotBlank String area,
        @Min(1) @Max(5) int nivel) {}

public record TemaDto(Long id, String titulo, String area) {}
```

Con `@Valid`, si los datos no cumplen las reglas Spring responde 400 sin llegar al método.

## Manejo de errores

Un único lugar traduce las excepciones en respuestas:

```java
@RestControllerAdvice
public class ManejadorDeErrores {

    @ExceptionHandler(TemaNoEncontradoException.class)
    public ProblemDetail noEncontrado(TemaNoEncontradoException e) {
        return ProblemDetail.forStatusAndDetail(HttpStatus.NOT_FOUND, e.getMessage());
    }
}
```

`ProblemDetail` sigue el formato estándar de errores HTTP (RFC 9457).

## Configuración

Los valores van en `application.yml` y pueden cambiar según el **perfil** activo (`dev`, `test`, `prod`):

```yaml
server:
  port: 8080
spring:
  datasource:
    url: ${DATABASE_URL}
bigbrain:
  temas-por-pagina: 20
```

```java
@ConfigurationProperties(prefix = "bigbrain")
public record BigBrainProperties(int temasPorPagina) {}
```

> [!WARNING]
> Las contraseñas y claves no se escriben en el repositorio. Se leen de variables de entorno o de un gestor de secretos.

## Arquitectura en capas

| Capa | Responsabilidad | No debe |
|---|---|---|
| Controlador | HTTP: recibir, validar, responder | tener lógica de negocio |
| Servicio | reglas de negocio y transacciones | conocer HTTP |
| Repositorio | acceso a datos | decidir reglas |

## Actuator

Agrega rutas de operación: `/actuator/health` para saber si la aplicación está viva y `/actuator/metrics` para las métricas. Hay que exponer solo las necesarias y protegerlas.

## Buenas prácticas

- Inyección por constructor y clases pequeñas con una responsabilidad.
- DTO en la frontera de la API; nunca exponer las entidades JPA.
- Validar toda entrada con `@Valid`.
- Un `@RestControllerAdvice` para los errores.
- Configuración por perfil y secretos fuera del código.
- Registrar con SLF4J (`log.info("tema creado id={}", id)`), no con `System.out`.
- Desde Java 21, los **hilos virtuales** (`spring.threads.virtual.enabled=true`) permiten atender muchos pedidos bloqueantes con poca memoria.
