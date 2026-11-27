use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path};

#[derive(Deserialize)]
struct AreasFile {
    area: Vec<AreaDef>,
}

#[derive(Deserialize)]
struct AreaDef {
    slug: String,
    title: String,
    description: String,
    symbol: String,
    color: String,
    #[serde(default)]
    branches: Vec<String>,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    summary: String,
    branch: String,
    #[serde(default)]
    order: u32,
    #[serde(default = "default_level")]
    level: String,
    #[serde(default)]
    tags: Vec<String>,
}

fn default_level() -> String {
    "Básico".into()
}

#[derive(Serialize, Clone)]
pub struct AreaSummary {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub symbol: String,
    pub color: String,
    pub branches: Vec<String>,
    pub topic_count: usize,
}

#[derive(Serialize, Clone)]
pub struct TopicSummary {
    pub area: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub branch: String,
    pub level: String,
    pub tags: Vec<String>,
    pub minutes: usize,
}

#[derive(Serialize, Clone)]
pub struct Heading {
    pub id: String,
    pub text: String,
}

pub struct Topic {
    pub summary: TopicSummary,
    pub html: String,
    pub headings: Vec<Heading>,
    /// Texto plano normalizado (sin tildes, en minúsculas) para la búsqueda.
    pub search_text: String,
    pub search_title: String,
    pub search_meta: String,
}

pub struct Library {
    pub areas: Vec<AreaSummary>,
    /// Temas ordenados por área, rama y orden.
    pub topics: Vec<Topic>,
    index: HashMap<(String, String), usize>,
}

impl Library {
    pub fn load(dir: &Path) -> io::Result<Self> {
        let raw = fs::read_to_string(dir.join("areas.toml"))?;
        let defs: AreasFile = toml::from_str(&raw).map_err(invalid)?;

        let mut areas = Vec::new();
        let mut topics = Vec::new();
        for def in defs.area {
            let mut found: Vec<(usize, u32, Topic)> = Vec::new();
            let area_dir = dir.join(&def.slug);
            if area_dir.is_dir() {
                for entry in fs::read_dir(&area_dir)? {
                    let path = entry?.path();
                    if path.extension().and_then(|e| e.to_str()) != Some("md") {
                        continue;
                    }
                    let slug = path.file_stem().unwrap().to_string_lossy().to_string();
                    let text = fs::read_to_string(&path)?;
                    let (meta, body) = split_front_matter(&text).ok_or_else(|| {
                        invalid(format!("{}: falta el encabezado +++", path.display()))
                    })?;
                    let meta: FrontMatter = toml::from_str(meta)
                        .map_err(|e| invalid(format!("{}: {e}", path.display())))?;
                    let branch_pos = def
                        .branches
                        .iter()
                        .position(|b| *b == meta.branch)
                        .unwrap_or(usize::MAX);
                    found.push((
                        branch_pos,
                        meta.order,
                        build_topic(&def.slug, slug, meta, body),
                    ));
                }
            }
            found.sort_by(|a, b| (a.0, a.1, &a.2.summary.slug).cmp(&(b.0, b.1, &b.2.summary.slug)));
            areas.push(AreaSummary {
                slug: def.slug,
                title: def.title,
                description: def.description,
                symbol: def.symbol,
                color: def.color,
                branches: def.branches,
                topic_count: found.len(),
            });
            topics.extend(found.into_iter().map(|t| t.2));
        }

        let index = topics
            .iter()
            .enumerate()
            .map(|(i, t)| ((t.summary.area.clone(), t.summary.slug.clone()), i))
            .collect();
        Ok(Self {
            areas,
            topics,
            index,
        })
    }

    pub fn area(&self, slug: &str) -> Option<&AreaSummary> {
        self.areas.iter().find(|a| a.slug == slug)
    }

    pub fn topics_of<'a>(&'a self, area: &'a str) -> impl Iterator<Item = &'a Topic> {
        self.topics.iter().filter(move |t| t.summary.area == area)
    }

    /// Devuelve el tema junto con el anterior y el siguiente dentro de su área.
    pub fn topic(
        &self,
        area: &str,
        slug: &str,
    ) -> Option<(&Topic, Option<&Topic>, Option<&Topic>)> {
        let i = *self.index.get(&(area.to_string(), slug.to_string()))?;
        let same_area = |t: &&Topic| t.summary.area == area;
        let prev = i
            .checked_sub(1)
            .and_then(|p| self.topics.get(p))
            .filter(same_area);
        let next = self.topics.get(i + 1).filter(same_area);
        Some((&self.topics[i], prev, next))
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<&Topic> {
        let terms: Vec<String> = normalize(query)
            .split_whitespace()
            .map(String::from)
            .collect();
        if terms.is_empty() {
            return Vec::new();
        }
        let mut hits: Vec<(usize, &Topic)> = self
            .topics
            .iter()
            .filter_map(|t| {
                let mut score = 0;
                for term in &terms {
                    let s = 8 * t.search_title.matches(term.as_str()).count()
                        + 4 * t.search_meta.matches(term.as_str()).count()
                        + t.search_text.matches(term.as_str()).count().min(5);
                    if s == 0 {
                        return None;
                    }
                    score += s;
                }
                Some((score, t))
            })
            .collect();
        hits.sort_by_key(|a| std::cmp::Reverse(a.0));
        hits.into_iter().take(limit).map(|h| h.1).collect()
    }
}

fn invalid<E: ToString>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
}

fn split_front_matter(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("+++")?;
    let end = rest.find("\n+++")?;
    Some((
        &rest[..end],
        rest[end + 4..].trim_start_matches(['\r', '\n']),
    ))
}

fn build_topic(area: &str, slug: String, meta: FrontMatter, body: &str) -> Topic {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_MATH
        | Options::ENABLE_GFM
        | Options::ENABLE_STRIKETHROUGH;
    let mut events: Vec<Event> = Parser::new_ext(body, options).collect();

    let mut headings = Vec::new();
    let mut plain = String::new();
    let mut open: Option<(usize, String)> = None;
    for i in 0..events.len() {
        match &events[i] {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => {
                open = Some((i, String::new()));
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                if let Some((start, text)) = open.take() {
                    let base = slugify(&text);
                    let mut id = base.clone();
                    let mut n = 2;
                    while headings.iter().any(|h: &Heading| h.id == id) {
                        id = format!("{base}-{n}");
                        n += 1;
                    }
                    if let Event::Start(Tag::Heading { id: slot, .. }) = &mut events[start] {
                        *slot = Some(CowStr::from(id.clone()));
                    }
                    headings.push(Heading { id, text });
                }
                plain.push(' ');
            }
            Event::Text(t) | Event::Code(t) => {
                if let Some((_, text)) = &mut open {
                    text.push_str(t);
                }
                plain.push_str(t);
            }
            Event::End(
                TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link,
            ) => {}
            Event::End(_)
            | Event::SoftBreak
            | Event::HardBreak
            | Event::InlineMath(_)
            | Event::DisplayMath(_) => plain.push(' '),
            _ => {}
        }
    }

    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());

    let words = plain.split_whitespace().count();
    Topic {
        search_title: normalize(&meta.title),
        search_meta: normalize(&format!(
            "{} {} {}",
            meta.summary,
            meta.branch,
            meta.tags.join(" ")
        )),
        search_text: normalize(&plain),
        summary: TopicSummary {
            area: area.to_string(),
            slug,
            title: meta.title,
            summary: meta.summary,
            branch: meta.branch,
            level: meta.level,
            tags: meta.tags,
            minutes: words.div_ceil(180).max(1),
        },
        html: out,
        headings,
    }
}

/// Minúsculas y sin tildes ni marcas de tono, para comparar texto.
pub fn normalize(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .filter(|c| !('\u{300}'..='\u{36f}').contains(c))
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' | 'ã' | 'ā' | 'ǎ' => 'a',
            'é' | 'è' | 'ë' | 'ê' | 'ē' | 'ě' => 'e',
            'í' | 'ì' | 'ï' | 'î' | 'ī' | 'ǐ' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' | 'ō' | 'ǒ' => 'o',
            'ú' | 'ù' | 'ü' | 'û' | 'ū' | 'ǔ' | 'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            c => c,
        })
        .collect()
}

fn slugify(s: &str) -> String {
    let mut out = String::new();
    for c in normalize(s).chars() {
        if c.is_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_spanish_text() {
        assert_eq!(normalize("Ecuación Ñandú"), "ecuacion nandu");
        assert_eq!(slugify("¿Qué es un límite?"), "que-es-un-limite");
    }

    #[test]
    fn normalizes_romanized_vocabulary() {
        assert_eq!(normalize("zǎoshang kōhī nǚ Ānnà"), "zaoshang kohi nu anna");
    }

    #[test]
    fn normalize_strips_combining_accents() {
        assert_eq!(normalize("ecuacio\u{301}n an\u{303}o"), "ecuacion ano");
    }

    #[test]
    fn partly_bold_words_stay_whole_and_duplicate_headings_get_unique_ids() {
        let meta: FrontMatter = toml::from_str(
            "title = \"T\"\nsummary = \"S\"\nbranch = \"B\"\norder = 1\nlevel = \"L\"\ntags = []",
        )
        .unwrap();
        let topic = build_topic(
            "a",
            "s".into(),
            meta,
            "## Ejemplo\n\n**D**erivativo\nfinal\n\n## Ejemplo\n",
        );
        assert_eq!(topic.search_text.trim(), "ejemplo derivativo final ejemplo");
        let ids: Vec<_> = topic.headings.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(ids, ["ejemplo", "ejemplo-2"]);
    }

    #[test]
    fn splits_front_matter() {
        let (meta, body) = split_front_matter("+++\ntitle = \"x\"\n+++\n\nHola").unwrap();
        assert_eq!(meta.trim(), "title = \"x\"");
        assert_eq!(body, "Hola");
    }

    #[test]
    fn renders_headings_and_math() {
        let meta = FrontMatter {
            title: "T".into(),
            summary: "S".into(),
            branch: "B".into(),
            order: 1,
            level: default_level(),
            tags: vec![],
        };
        let t = build_topic("a", "t".into(), meta, "## Definición\n\nSea $x^2$.");
        assert_eq!(t.headings[0].id, "definicion");
        assert!(t.html.contains("<h2 id=\"definicion\">"));
        assert!(t.html.contains("math math-inline"));
    }
}
