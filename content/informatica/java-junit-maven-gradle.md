+++
title = "Herramientas de Java: Maven, Gradle, JUnit y Mockito"
summary = "Construcción del proyecto, gestión de dependencias, pruebas unitarias y de integración, dobles de prueba y Testcontainers."
branch = "Frameworks de Java"
order = 5
level = "Intermedio"
tags = ["Java", "Maven", "Gradle", "JUnit", "Mockito"]
+++

## Herramientas de construcción

Compilan, descargan dependencias, ejecutan pruebas y empaquetan.

| | Maven | Gradle |
|---|---|---|
| Archivo | `pom.xml` (XML) | `build.gradle.kts` (Kotlin) o `build.gradle` (Groovy) |
| Estilo | declarativo y con convenciones fijas | programable |
| Velocidad | correcta | mayor, con caché y compilación incremental |
| Uso típico | empresas, proyectos estándar | Android, proyectos grandes |

### Maven

Estructura estándar: código en `src/main/java`, recursos en `src/main/resources`, pruebas en `src/test/java`.

```xml
<dependencies>
    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-web</artifactId>
    </dependency>
    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-test</artifactId>
        <scope>test</scope>
    </dependency>
</dependencies>
```

Cada dependencia se identifica con sus **coordenadas**: grupo, artefacto y versión.

| Comando | Hace |
|---|---|
| `mvn compile` | compila |
| `mvn test` | ejecuta las pruebas unitarias |
| `mvn package` | genera el `.jar` |
| `mvn verify` | suma las pruebas de integración |
| `mvn dependency:tree` | muestra el árbol de dependencias |

Las fases son acumulativas: `package` ejecuta antes `compile` y `test`.

### Gradle

```kotlin
plugins {
    java
    id("org.springframework.boot") version "3.4.0"
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-web")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}

tasks.test { useJUnitPlatform() }
```

> [!TIP]
> Usá el **wrapper** (`./mvnw`, `./gradlew`) que viene en el repositorio: descarga la versión exacta de la herramienta y todos compilan igual.

## JUnit 5

```java
class CalculadoraDeNotasTest {

    private final CalculadoraDeNotas calculadora = new CalculadoraDeNotas();

    @Test
    void promediaLasNotas() {
        double promedio = calculadora.promedio(List.of(8, 6, 10));

        assertThat(promedio).isEqualTo(8.0);
    }

    @Test
    void rechazaUnaListaVacia() {
        assertThatThrownBy(() -> calculadora.promedio(List.of()))
                .isInstanceOf(IllegalArgumentException.class);
    }

    @ParameterizedTest
    @CsvSource({ "4, true", "3, false", "10, true" })
    void apruebaConCuatroOMas(int nota, boolean esperado) {
        assertThat(calculadora.aprueba(nota)).isEqualTo(esperado);
    }
}
```

Cada prueba sigue tres pasos: **preparar**, **actuar**, **verificar**. Las afirmaciones del ejemplo son de **AssertJ**, que se leen casi como una frase.

| Anotación | Uso |
|---|---|
| `@Test` | una prueba |
| `@BeforeEach`, `@AfterEach` | antes y después de cada prueba |
| `@ParameterizedTest` | la misma prueba con varios datos |
| `@Nested` | agrupar pruebas relacionadas |
| `@DisplayName` | nombre legible |

## Mockito

Un **doble de prueba** reemplaza una dependencia real (la base, un servicio externo) para probar una clase aislada.

```java
@ExtendWith(MockitoExtension.class)
class TemaServiceTest {

    @Mock TemaRepository repositorio;
    @InjectMocks TemaService servicio;

    @Test
    void lanzaErrorSiElTemaNoExiste() {
        when(repositorio.findById(99L)).thenReturn(Optional.empty());

        assertThatThrownBy(() -> servicio.buscar(99L))
                .isInstanceOf(TemaNoEncontradoException.class);

        verify(repositorio).findById(99L);
    }
}
```

> [!WARNING]
> Una prueba llena de dobles termina verificando los dobles y no el comportamiento. Conviene simular solo lo que está en la frontera del sistema (red, reloj, servicios de terceros).

## Pruebas con Spring

| Anotación | Levanta | Velocidad |
|---|---|---|
| ninguna (JUnit puro) | nada | la más rápida |
| `@WebMvcTest` | solo la capa web | rápida |
| `@DataJpaTest` | solo JPA y la base | rápida |
| `@SpringBootTest` | toda la aplicación | lenta |

```java
@WebMvcTest(TemaController.class)
class TemaControllerTest {

    @Autowired MockMvc mockMvc;
    @MockitoBean TemaService servicio;

    @Test
    void devuelve404SiNoExiste() throws Exception {
        when(servicio.ver(99L)).thenThrow(new TemaNoEncontradoException(99L));

        mockMvc.perform(get("/api/temas/99")).andExpect(status().isNotFound());
    }
}
```

## Testcontainers

Levanta en un contenedor la **misma base de datos** que se usa en producción, solo durante la prueba:

```java
@SpringBootTest
@Testcontainers
class TemaRepositoryIT {

    @Container
    @ServiceConnection
    static PostgreSQLContainer<?> postgres = new PostgreSQLContainer<>("postgres:17");
}
```

Así se evita la sorpresa de que algo funcione en una base en memoria y falle en la real.

## Calidad

| Herramienta | Qué controla |
|---|---|
| JaCoCo | cobertura de pruebas |
| Checkstyle, Spotless | formato y estilo |
| SpotBugs, Error Prone | errores probables |
| SonarQube | calidad global y deuda técnica |
| OWASP Dependency-Check | dependencias con fallas conocidas |

## Buenas prácticas

- Muchas pruebas unitarias rápidas, menos de integración y pocas de punta a punta.
- Nombres que describan el comportamiento: `rechazaUnaListaVacia`, no `test2`.
- Una razón de fallo por prueba.
- Pruebas independientes entre sí y del orden.
- Ejecutarlas en integración continua en cada cambio.
- Fijar las versiones de las dependencias y actualizarlas con regularidad.
