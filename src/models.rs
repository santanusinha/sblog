use chrono::NaiveDate;
use serde::Serialize;

/// The site-wide configuration read from `config.toml`.
#[derive(Serialize, Clone)]
pub struct SiteConfig {
    pub title: String,
    pub tagline: String,
    pub header_title: String,
    pub header_tagline: String,
    pub base_url: String,
    pub og_image: String,
    pub twitter_handle: String,
    /// Directory holding the markdown posts.
    pub posts_dir: String,
    /// Directory where generated pages are written.
    pub output_dir: String,
    /// Directory holding the Tera templates.
    pub templates_dir: String,
    /// Directory holding static assets copied verbatim to the output.
    pub static_dir: String,
    /// Path (relative to the static dir) of the stylesheet.
    pub css_file: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "sblog".to_string(),
            tagline: "A tiny static blog".to_string(),
            header_title: "sblog".to_string(),
            header_tagline: "A tiny static blog".to_string(),
            base_url: String::new(),
            og_image: String::new(),
            twitter_handle: String::new(),
            posts_dir: "posts".to_string(),
            output_dir: "public".to_string(),
            templates_dir: "templates".to_string(),
            static_dir: "static".to_string(),
            css_file: "static/style.css".to_string(),
        }
    }
}

/// A single blog post with its frontmatter and rendered body.
#[derive(Clone)]
pub struct Post {
    pub title: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub summary: String,
    pub slug: String,
    pub body_html: String,
    /// The headings extracted from the post body, for the outline.
    pub headings: Vec<HeadingView>,
}

/// An abstract document: the parsed content that templates render.
///
/// This is the intermediate representation produced from the raw markdown
/// posts. It carries both the raw frontmatter fields and pre-computed
/// presentation values (formatted dates, links, tag chips) so the templates
/// stay simple and free of logic.
#[derive(Serialize)]
pub struct Document {
    /// The page title shown in the browser tab.
    pub title: String,
    /// The site title, used in the `<title>` and footer.
    pub site_title: String,
    /// The page description for the `<meta name="description">`.
    pub description: String,
    /// The social (Open Graph / Twitter) meta tags, pre-rendered.
    pub social_meta: String,
    /// The header title shown in the masthead.
    pub header_title: String,
    /// The header tagline shown in the masthead.
    pub header_tagline: String,
    /// The year for the footer copyright.
    pub year: String,
    /// The path to the stylesheet, relative to the current page.
    pub css_href: String,
    /// The path back to the home page, relative to the current page.
    pub home_href: String,
    /// The path to the archive page (article list), relative to the current page.
    pub archive_href: String,
    /// The path to the tags index page, relative to the current page.
    pub tags_href: String,
    /// The path to the about page, relative to the current page.
    pub about_href: String,
    /// The path to the RSS feed, relative to the current page.
    pub feed_href: String,
    pub active_nav: String,
    /// The list of posts (for the index and tag pages).
    pub posts: Vec<PostView>,
    /// The current post (for the post page).
    pub post: Option<PostView>,
    /// The previous (newer) post for prev/next navigation.
    pub prev_post: Option<PostView>,
    /// The next (older) post for prev/next navigation.
    pub next_post: Option<PostView>,
    /// The sidebar data.
    pub sidebar: SidebarView,
    /// The tag list (for the tag index page).
    pub tags: Vec<TagView>,
    /// The current tag (for the tag page).
    pub tag: Option<TagView>,
}

/// A post shaped for the templates.
#[derive(Serialize)]
pub struct PostView {
    pub title: String,
    pub summary: String,
    pub href: String,
    pub date_long: String,
    pub date_short: String,
    pub read_time: String,
    pub tags: Vec<TagView>,
    pub body_html: String,
    /// The headings extracted from the post body, for the outline.
    pub headings: Vec<HeadingView>,
}

/// A heading extracted from a post body, shaped for the templates.
#[derive(Serialize, Clone)]
pub struct HeadingView {
    /// The heading text.
    pub text: String,
    /// The heading level (1-6).
    pub level: u8,
    /// The anchor ID for linking to the heading in the article body.
    pub id: String,
}

/// A tag shaped for the templates.
#[derive(Serialize)]
pub struct TagView {
    pub name: String,
    pub href: String,
    pub count: usize,
}

/// The sidebar data shared by the index and post pages.
#[derive(Serialize)]
pub struct SidebarView {
    pub older_count: usize,
    pub essays: Vec<PostView>,
    pub tags: Vec<TagView>,
    /// The path to the archive page, relative to the current page.
    pub archive_href: String,
}

/// A flattened site view for the templates.
#[derive(Serialize)]
pub struct SiteView {
    pub title: String,
    pub tagline: String,
    pub header_title: String,
    pub header_tagline: String,
}

impl From<&Document> for SiteView {
    fn from(doc: &Document) -> Self {
        Self {
            title: doc.site_title.clone(),
            tagline: doc.description.clone(),
            header_title: doc.header_title.clone(),
            header_tagline: doc.header_tagline.clone(),
        }
    }
}
