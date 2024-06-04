use crate::article::{Article, ARTICLES};
use crate::article::{Fragment, Image};
use chrono::Local;

use leptos::logging::log;
use leptos::{component, view, Children, CollectView, IntoView, Params, SignalWith};
use leptos_router::Params;
use leptos_router::A;
use leptos_router::{use_params, Route, Router, Routes};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[component]
pub fn App() -> impl IntoView {
    log!("{}", option_env!("BASE_URL").unwrap_or_default().to_string());
    view! {
        <Router base=option_env!("BASE_URL").unwrap_or_default()>
            <div class="flex flex-col h-full">
                <Header/>
                <PageContainer>
                    <Routes base=option_env!("BASE_URL").unwrap_or_default().to_string()>
                        <Route path="/" view=ArticlePreviews/>
                        <Route path="/articles/:id" view=Article/>
                        <Route path="/*" view=|| "404"/>
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
            <div class="inset-0 flex items-center pointer-events-none md:p-4 md:absolute">
                <div>{Local::now().format("%B %-d, %Y").to_string()}</div>
            </div>
            <A class="w-full text-center" href="/">
                <Heading>
                    <div class="text-5xl capitalize font-blackletter">"The Yesterday"</div>
                    <div class="font-serif text-base">"Trusted by dozens."</div>
                </Heading>
            </A>
        </header>
    }
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <main class="flex justify-center p-4 grow">
            <div class="max-w-2xl">{children()}</div>
        </main>
    }
}

#[component]
pub fn ArticlePreviews() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <div class="flex flex-col gap-2">
                <Heading>"Articles"</Heading>
                <Divider/>
                <div class="flex flex-col gap-8 sm:grid sm:grid-cols-2">
                    {ARTICLES
                        .iter()
                        .map(|article| view! { <ArticlePreview article=article.clone()/> })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ArticlePreview(article: Article) -> impl IntoView {
    view! {
        <A class="flex flex-col gap-2 size-full" href=format!("/articles/{}", article.id)>
            <img src=article.image.url alt=article.title/>
            <div>
                <small class="text-sm font-light text-blue-800">"ARTICLE"</small>
                <Heading>
                    <div class="text-xl text-balance">{article.title}</div>
                </Heading>
                <Caption>
                    <div class="text-sm text-left text-pretty">{article.blurb}</div>
                </Caption>
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
    let article = || {
        use_params::<ArticleParams>().with(|params| {
            ARTICLES
                .iter()
                .find(|article| article.id == params.as_ref().unwrap().id.clone())
                .unwrap()
        })
    };
    view! {
        <div class="flex flex-col gap-4">
            <div>
                <Heading>{move || article().title.to_uppercase()}</Heading>
                <div class="flex gap-1 text-sm font-light">
                    <div class="text-blue-800">"ARTICLE"</div>
                    "\u{b7} "
                    {move || article().reading_time()}
                    " min read"
                </div>
            </div>
            <div class="px-16">
                <img
                    src=move || article().image.url
                    alt=move || article().title
                    class="object-cover w-full"
                />
                <Caption>{move || article().image.caption}</Caption>
            </div>
            <Divider/>
            <div class="flex flex-col gap-5 text-lg
            [&>div:first-child>p]:first-letter:text-[3.5rem]
            [&>div:first-child>p]:first-letter:leading-none
            [&>div:first-child>p]:first-letter:font-bold
            [&>div:first-child>p]:first-letter:font-serif
            [&>div:first-child>p]:first-letter:float-left
            [&>div:first-child>p]:first-letter:pr-2">
                {move || {
                    article()
                        .fragments
                        .iter()
                        .map(|fragment| {
                            match fragment {
                                Fragment::Image(Image { url, caption }) => {
                                    view! {
                                        <div class="px-16">
                                            <img src=*url class="object-cover w-full"/>
                                            <Caption>{*caption}</Caption>
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
                        .collect_view()
                }}

            </div>
            <Divider/>
            <ReadMore this_article=article/>
        </div>
    }
}

#[component]
pub fn Divider(#[prop(optional)] light: bool) -> impl IntoView {
    view! {
        <div class=format!(
            "w-full h-px {}",
            if light { "bg-gray-200" } else { "bg-gray-800" },
        )></div>
    }
}

#[component]
pub fn ReadMore(this_article: impl Fn() -> &'static Article + 'static) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <Heading>"Read More"</Heading>
            <div class="flex gap-2 [&>*]:w-1/3">

                {move || {
                    let mut articles = ARTICLES.to_vec();
                    articles.shuffle(&mut thread_rng());
                    articles
                        .into_iter()
                        .filter(|article| *article != *this_article())
                        .take(5)
                        .map(|article| {
                            view! { <ArticlePreview article=article/> }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! { <h1 class="font-serif text-4xl font-medium uppercase">{children()}</h1> }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col p-4 text-white bg-black">
            <A href="/">
                <Heading>
                    <div class="capitalize font-blackletter">"The Yesterday"</div>
                </Heading>
            </A>
            <div class="flex justify-between">
                <div>"Copyright \u{a9} 2024"</div>
                <div>"Brought to you by incredible (and few credible) reporters."</div>
            </div>
        </footer>
    }
}

#[component]
pub fn Caption(children: Children) -> impl IntoView {
    view! { <caption class="block w-full py-2 text-sm text-right opacity-50">{children()}</caption> }
}
