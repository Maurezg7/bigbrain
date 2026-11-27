+++
title = "Quarkus, Micronaut y Jakarta EE"
summary = "Frameworks pensados para la nube: arranque rápido, compilación nativa con GraalVM y comparación con Spring Boot."
branch = "Frameworks de Java"
order = 4
level = "Avanzado"
tags = ["Java", "Quarkus", "Micronaut", "Jakarta EE", "GraalVM"]
+++

## El problema que resuelven

Spring descubre los beans y arma la aplicación **al arrancar**, usando reflexión. Eso es flexible, pero cuesta tiempo de arranque y memoria. En contenedores y funciones sin servidor, donde las instancias se crean y destruyen todo el tiempo, ese costo pesa.

**Quarkus** y **Micronaut** hacen ese trabajo **al compilar**: la aplicación llega ya armada, arranca en una fracción del tiempo y usa menos memoria.

## Jakarta EE y MicroProfile

**Jakarta EE** (antes Java EE) es el conjunto de especificaciones estándar de Java para aplicaciones empresariales. No es un framework: es un contrato que varias implementaciones cumplen.

| Especificación | Para qué | Equivalente en Spring |
|---|---|---|
| CDI | inyección de dependencias | contenedor de Spring |
| Jakarta REST (JAX-RS) | API REST | Spring MVC |
| Jakarta Persistence (JPA) | ORM | igual: Spring también usa JPA |
| Bean Validation | validación | igual |

**MicroProfile** agrega especificaciones para microservicios: configuración, salud, métricas, tolerancia a fallos.

## Quarkus

Usa las anotaciones estándar de Jakarta EE, así que quien las conoce se adapta rápido.

```java
@Path("/api/temas")
@Produces(MediaType.APPLICATION_JSON)
public class TemaResource {

    @Inject
    TemaService servicio;

    @GET
    public List<TemaDto> listar() {
        return servicio.listar();
    }

    @GET
    @Path("/{id}")
    public TemaDto ver(@PathParam("id") Long id) {
        return servicio.ver(id);
    }

    @POST
    @Transactional
    public Response crear(@Valid NuevoTema nuevo) {
        TemaDto creado = servicio.crear(nuevo);
        return Response.status(Response.Status.CREATED).entity(creado).build();
    }
}
```

### Panache

Simplifica JPA con el patrón de registro activo:

```java
@Entity
public class Tema extends PanacheEntity {
    public String titulo;
    public String area;

    public static List<Tema> deArea(String area) {
        return list("area", Sort.by("titulo"), area);
    }
}
```

### Modo desarrollo

`./mvnw quarkus:dev` recompila al guardar, sin reiniciar. Los **Dev Services** levantan solos, en contenedores, la base de datos o el broker que la aplicación necesita.

## Micronaut

Se parece más a Spring en el estilo:

```java
@Controller("/api/temas")
public class TemaController {
    private final TemaService servicio;

    public TemaController(TemaService servicio) { this.servicio = servicio; }

    @Get("/{id}")
    public TemaDto ver(Long id) {
        return servicio.ver(id);
    }
}
```

Toda la inyección se resuelve con procesadores de anotaciones al compilar, sin reflexión.

## Imágenes nativas con GraalVM

**GraalVM** puede compilar una aplicación Java a un **ejecutable nativo**, sin máquina virtual.

| | JVM | Nativo |
|---|---|---|
| Arranque | segundos | decenas de milisegundos |
| Memoria en reposo | cientos de MB | decenas de MB |
| Rendimiento sostenido | muy alto, gracias al compilador JIT | algo menor |
| Tiempo de compilación | segundos | minutos |
| Reflexión | libre | hay que declararla |

> [!NOTE]
> Nativo no significa «más rápido en todo». Gana en arranque y memoria; en un servicio que corre semanas con carga alta, la JVM suele rendir igual o mejor.

Spring Boot 3 también admite compilación nativa, así que la diferencia entre frameworks se achicó.

## Comparación

| | Spring Boot | Quarkus | Micronaut |
|---|---|---|---|
| Resolución de dependencias | en ejecución | en compilación | en compilación |
| API | propia de Spring | estándares Jakarta y MicroProfile | propia, parecida a Spring |
| Ecosistema y documentación | el más grande | grande, en crecimiento | mediano |
| Oferta laboral | la mayor | creciente | menor |
| Punto fuerte | madurez y comunidad | experiencia de desarrollo y nube | bajo consumo |

## Cómo elegir

- Para **aprender y conseguir trabajo**, Spring Boot primero: es el estándar de hecho.
- Para microservicios en Kubernetes o funciones sin servidor, donde importan el arranque y la memoria, Quarkus o Micronaut.
- Si el equipo viene de Jakarta EE, Quarkus resulta natural.

Los conceptos (inyección de dependencias, capas, JPA, validación, pruebas) son los mismos en los tres. Quien domina uno aprende otro en días.

## Buenas prácticas

- Elegir por las necesidades del proyecto y del equipo, no por las pruebas de rendimiento de un «hola mundo».
- Probar la compilación nativa en integración continua si se la va a usar: algunos fallos solo aparecen ahí.
- Exponer salud y métricas desde el principio.
- Configurar todo por variables de entorno (principio de las aplicaciones de doce factores).
