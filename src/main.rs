mod article;

use crate::components::App;
use lazy_static::lazy_static;

use anyhow::{anyhow, Result};
use article::Article;
use console_error_panic_hook::set_once;
use leptos::{mount_to_body, view};

fn get_articles() -> Result<&'static [Article]> {
    let mut data = include_str!(concat!(env!("OUT_DIR"), "/articles"));
    let mut articles = Vec::new();
    while !data.is_empty() {
        let (length, rest) = data
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid data"))?;
        let length: usize = length.parse()?;
        let article = Article::from_str(&rest[..length])?;
        articles.push(article);
        data = rest
            .get(length + 1..)
            .or_else(|| rest.get(length..))
            .unwrap();
    }
    Ok(articles.leak())
}

lazy_static! {
    static ref ARTICLES: &'static [Article] = get_articles().unwrap();
}

fn main() {
    set_once();
    mount_to_body(move || view! { <App/> });
}

#[allow(non_snake_case)]
mod components {
    use crate::article::Fragment;
    use crate::{article::Article, ARTICLES};
    use chrono::Local;
    use leptos::{component, view, Children, CollectView, IntoView, Params, SignalWithUntracked};
    use leptos_router::Params;
    use leptos_router::A;
    use leptos_router::{use_params, Route, Router, Routes};

    #[component]
    pub fn App() -> impl IntoView {
        view! {
            <Router>
                <div class="flex flex-col h-full">
                    <Header/>
                    <PageContainer>
                        <Routes>
                            <Route
                                path="/"
                                view=|| {
                                    view! { <ArticlePreviews/> }
                                }
                            />

                            <Route path="/articles/:id" view=Article/>
                            <Route path="*" view=|| "404"/>
                        </Routes>
                    </PageContainer>
                    <Footer/>
                </div>
            </Router>
        }
    }

    #[component]
    pub fn Header() -> impl IntoView {
        view! {
            <header class="relative p-4 text-white bg-black">
                <div class="md:absolute">{Local::now().format("%B %-d, %Y").to_string()}</div>
                <A class="w-full text-center" href="/">
                    <Heading>"The Yesterday"</Heading>
                </A>
            </header>
        }
    }

    #[component]
    pub fn PageContainer(children: Children) -> impl IntoView {
        view! {
            <main class="flex justify-center p-4 grow">
                <div class="max-w-3xl">{children()}</div>
            </main>
        }
    }

    #[component]
    pub fn ArticlePreviews() -> impl IntoView {
        view! {
            <div class="flex flex-col gap-2">
                <div class="flex flex-col gap-2">
                    <Heading>"Articles"</Heading>
                    <div class="flex flex-col gap-8 sm:grid sm:grid-cols-2">
                        {ARTICLES
                            .iter()
                            .map(|article| view! { <ArticlePreview article=article/> })
                            .collect_view()}
                    </div>
                </div>
            </div>
        }
    }

    #[component]
    pub fn ArticlePreview(article: &'static Article) -> impl IntoView {
        view! {
            <A class="flex flex-col gap-2 size-full" href=format!("/articles/{}", article.id)>
                <img src=article.image_url alt=article.title/>
                <div>
                    <small class="text-xs font-light text-blue-800">"ARTICLE"</small>
                    <Heading>
                        <div class="text-xl">{article.title}</div>
                    </Heading>
                </div>
            </A>
        }
    }

    #[component]
    pub fn Article() -> impl IntoView {
        #[derive(Params, PartialEq)]
        struct ArticleParams {
            id: String,
        }
        let article = use_params::<ArticleParams>().with_untracked(|params| {
            ARTICLES
                .iter()
                .find(|article| article.id == params.as_ref().unwrap().id.clone())
                .unwrap()
        });
        view! {
            <div class="flex flex-col gap-4">
                <div>
                    <Heading>{article.title.to_uppercase()}</Heading>
                    <div class="flex gap-1 text-xs font-light">
                        <div class="text-blue-800">"ARTICLE \u{b7}"</div>
                        {article.reading_time()}
                        " min read"
                    </div>
                </div>
                <img src=article.image_url alt=article.title class="object-cover w-full px-16"/>
                <Divider/>
                <div class="flex flex-col gap-4
                [&>div:first-child>p]:first-letter:text-5xl
                [&>div:first-child>p]:first-letter:leading-none
                [&>div:first-child>p]:first-letter:font-serif
                [&>div:first-child>p]:first-letter:font-bold
                [&>div:first-child>p]:first-letter:float-left
                [&>div:first-child>p]:first-letter:pr-1">
                    {article
                        .fragments
                        .iter()
                        .map(|fragment| {
                            match fragment {
                                Fragment::Image(source) => {
                                    view! {
                                        <div class="px-16">
                                            <img src=*source class="object-cover w-full"/>
                                        </div>
                                    }
                                }
                                Fragment::Text(text) => {
                                    view! {
                                        <div>
                                            <p>{*text}</p>
                                        </div>
                                    }
                                }
                            }
                        })
                        .collect_view()}
                </div>
                <Divider/>
                <ReadMore this_article=article/>
            </div>
        }
    }

    #[component]
    pub fn Divider(#[prop(optional)] light: bool) -> impl IntoView {
        view! { <div class="w-full h-px bg-gray-800" class:bg-gray-200=light ></div> }
    }

    #[component]
    pub fn ReadMore(this_article: &'static Article) -> impl IntoView {
        view! {
            <div class="flex flex-col gap-2">
                <Heading>"Read More"</Heading>
                <div class="flex w-1/3 gap-2">
                    {ARTICLES
                        .iter()
                        .filter(|article| *article != this_article)
                        .take(5)
                        .map(|article| {
                            view! { <ArticlePreview article=article/> }
                        })
                        .collect_view()}
                </div>
            </div>
        }
    }

    #[component]
    pub fn Heading(children: Children) -> impl IntoView {
        view! { <h1 class="font-serif text-3xl font-medium uppercase">{children()}</h1> }
    }

    pub fn Footer() -> impl IntoView {
        view! {
            <footer class="flex flex-col p-4 text-white bg-black">
                <Heading>"The Yesterday"</Heading>
                <Divider light=true/>
                "Copyright \u{a9} 2024"
            </footer>
        }
    }
}
