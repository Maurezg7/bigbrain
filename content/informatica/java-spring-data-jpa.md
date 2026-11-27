+++
title = "Spring Data JPA e Hibernate"
summary = "Entidades, relaciones, repositorios, consultas derivadas y JPQL, transacciones, carga perezosa y el problema N+1."
branch = "Frameworks de Java"
order = 2
level = "Intermedio"
tags = ["Java", "JPA", "Hibernate", "Spring Data", "ORM"]
+++

## Qué es cada cosa

| Nombre | Qué es |
|---|---|
| **ORM** | técnica que relaciona objetos con tablas |
| **JPA** | la especificación estándar de Java para ORM (hoy Jakarta Persistence) |
| **Hibernate** | la implementación más usada de JPA |
| **Spring Data JPA** | capa de Spring que genera los repositorios |

## Entidades

Una **entidad** es una clase cuyos objetos se guardan como filas.

```java
@Entity
@Table(name = "temas")
public class Tema {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(nullable = false, length = 120)
    private String titulo;

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "area_id")
    private Area area;

    protected Tema() {}            // JPA exige un constructor sin argumentos

    public Tema(String titulo, Area area) {
        this.titulo = titulo;
        this.area = area;
    }
    // getters
}
```

## Relaciones

| Anotación | Relación | Ejemplo |
|---|---|---|
| `@ManyToOne` | muchos a uno | muchos temas pertenecen a un área |
| `@OneToMany` | uno a muchos | un área tiene muchos temas |
| `@OneToOne` | uno a uno | usuario y perfil |
| `@ManyToMany` | muchos a muchos | temas y etiquetas |

```java
@Entity
public class Area {
    @Id @GeneratedValue private Long id;

    @OneToMany(mappedBy = "area", cascade = CascadeType.ALL, orphanRemoval = true)
    private List<Tema> temas = new ArrayList<>();
}
```

`mappedBy` indica que la clave foránea está del otro lado. La columna existe **una sola vez**, en la tabla `temas`.

## Repositorios

Alcanza con declarar una interfaz; Spring Data crea la implementación.

```java
public interface TemaRepository extends JpaRepository<Tema, Long> {

    List<Tema> findByAreaNombreOrderByTituloAsc(String nombre);

    Optional<Tema> findByTituloIgnoreCase(String titulo);

    boolean existsByTitulo(String titulo);

    @Query("select t from Tema t join fetch t.area where t.titulo like %:texto%")
    List<Tema> buscar(@Param("texto") String texto);

    Page<Tema> findByAreaId(Long areaId, Pageable pagina);
}
```

`JpaRepository` ya trae `save`, `findById`, `findAll`, `deleteById` y `count`. Las **consultas derivadas** se arman con el nombre del método. Para algo más complejo se usa `@Query` con **JPQL**, un lenguaje parecido a SQL pero sobre entidades y atributos, no sobre tablas y columnas.

Paginación:

```java
Page<Tema> pagina = repositorio.findByAreaId(3L, PageRequest.of(0, 20, Sort.by("titulo")));
```

## El contexto de persistencia

Dentro de una transacción, Hibernate recuerda las entidades que cargó. Si se modifica una, **detecta el cambio y emite el `UPDATE` solo** al confirmar, sin llamar a `save`.

```java
@Transactional
public void renombrar(Long id, String nuevoTitulo) {
    Tema tema = repositorio.findById(id).orElseThrow();
    tema.setTitulo(nuevoTitulo);     // se guarda al terminar el método
}
```

## Transacciones

`@Transactional` abre una transacción al entrar al método y la confirma al salir; si se lanza una excepción no verificada, la deshace.

| Detalle | Regla |
|---|---|
| Dónde | en la capa de servicio |
| Lecturas | `@Transactional(readOnly = true)` |
| Vuelta atrás | por defecto solo con `RuntimeException` |
| Llamada interna | un método que llama a otro **de la misma clase** no pasa por el proxy y la anotación no se aplica |

## Carga perezosa y N+1

Con `FetchType.LAZY` la relación se trae de la base recién cuando se la usa. Es lo correcto por defecto, pero tiene una trampa:

```java
List<Tema> temas = repositorio.findAll();          // 1 consulta
for (Tema t : temas) {
    System.out.println(t.getArea().getNombre());   // 1 consulta por cada tema
}
```

Con 100 temas son 101 consultas. Es el problema **N+1**, la causa más común de lentitud con un ORM.

| Solución | Cómo |
|---|---|
| `join fetch` | `select t from Tema t join fetch t.area` |
| `@EntityGraph` | `@EntityGraph(attributePaths = "area")` sobre el método |
| Proyección | consultar directamente un DTO con los campos necesarios |
| Lotes | `hibernate.default_batch_fetch_size` |

> [!WARNING]
> `@ManyToOne` y `@OneToOne` son **ansiosas** (`EAGER`) por defecto. Conviene declararlas `LAZY` siempre y pedir la relación de forma explícita cuando haga falta.

Activá `spring.jpa.show-sql=true` en desarrollo para ver qué consultas se emiten.

## Migraciones

`spring.jpa.hibernate.ddl-auto=update` sirve para experimentar, no para producción. El esquema se versiona con **Flyway** o **Liquibase**: archivos numerados (`V1__crear_temas.sql`) que se aplican en orden al arrancar. En producción se deja `ddl-auto=validate`.

## Buenas prácticas

- Relaciones `LAZY` y consultas que traigan justo lo necesario.
- No devolver entidades desde los controladores: convertirlas a DTO dentro de la transacción.
- `equals` y `hashCode` basados en una clave de negocio o en el id, con cuidado; evitar `@Data` de Lombok en entidades.
- Paginar siempre los listados.
- Índices en las columnas de búsqueda y de claves foráneas.
- Para consultas muy complejas o informes, SQL nativo, **jOOQ** o `JdbcClient`: un ORM no es obligatorio para todo.
