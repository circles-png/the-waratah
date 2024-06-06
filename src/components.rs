use crate::crossword::Vec2;
use itertools::Itertools;
use std::iter::once;
use std::ops::Not;

use crate::ad::ADS;
use crate::article::{Article, ARTICLES};
use crate::article::{Fragment, Image};
use crate::crossword::CROSSWORDS;
use chrono::Local;

use leptos::{
    component, create_effect, create_signal, view, Children, CollectView, IntoView, Params,
    SignalGet, SignalWith,
};
use leptos_router::Params;
use leptos_router::A;
use leptos_router::{use_params, Route, Router, Routes};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base=option_env!("BASE_URL").unwrap_or_default()>
            <div class="flex flex-col h-full">
                <Header/>
                <PageContainer>
                    <Routes base=option_env!("BASE_URL").unwrap_or_default().to_string()>
                        <Route path="/" view=ArticlePreviews/>
                        <Route path="/articles/:id" view=Article/>
                        <Route path="/crosswords/:id" view=Crossword/>
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
            <div class="inset-0 items-center justify-between hidden pointer-events-none sm:p-4 sm:absolute sm:flex">
                <div>{Local::now().format("%B %-d, %Y").to_string()}</div>
                <A
                    href="https://angusmason.github.io/theaccountgame"
                    target="_blank"
                    class="pointer-events-auto"
                >
                    "Sign Up"
                </A>
            </div>
            <A class="w-full text-center" href="/">
                <Heading>
                    <div class="text-5xl capitalize font-blackletter">"The Yesterday"</div>
                    <div class="block font-serif text-base">"Trusted by dozens."</div>
                </Heading>
            </A>
        </header>
    }
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    let ad = ADS.choose(&mut thread_rng()).unwrap();
    let (ad_open, set_ad_open) = create_signal(true);
    view! {
        <main class="flex justify-center gap-4 p-4 grow">
            <div class="w-full max-w-2xl shrink-0">{children()}</div>
            <div class=move || {
                format!(
                    "fixed p-2 bg-gray-100 border rounded-t-lg inset-x-1/3 transition duration-1000 ease-in bottom-0 {}",
                    ad_open.get().not().then_some("translate-y-[150%]").unwrap_or_default(),
                )
            }>
                <img src=format!("/images/ads/{}", *ad) class="cursor-pointer size-full"/>
                <button
                    class="absolute right-0 text-center bg-gray-100 border rounded-t-lg size-8 bottom-full"
                    on:click=move |_| set_ad_open(false)
                >
                    "X"
                </button>
            </div>
        </main>
    }
}

#[component]
pub fn ArticlePreviews() -> impl IntoView {
    const ALL: &str = "All";
    let (filter, set_filter) = create_signal(None::<&str>);
    view! {
        <div class="flex flex-col gap-2">
            <div class="flex *:px-2 divide-x font-serif justify-center">

                {move || {
                    ARTICLES
                        .iter()
                        .map(|article| article.topic)
                        .chain(once(ALL))
                        .unique()
                        .map(|topic| {
                            view! {
                                <button
                                    class=filter
                                        .get()
                                        .as_ref()
                                        .map_or(false, |filter| topic == *filter)
                                        .then_some("text-blue-800")

                                    on:click=move |_| set_filter(Some(topic))
                                >

                                    {topic}
                                </button>
                            }
                        })
                        .collect_view()
                }}

            </div>
            <div class="flex flex-col gap-2">
                {move || {
                    const LATEST: &str = "Latest";
                    once(LATEST)
                        .chain(ARTICLES.iter().map(|article| article.topic).unique())
                        .chain(once(ALL))
                        .filter(|topic| {
                            filter.get().as_ref().map_or(*topic != ALL, |filter| topic == filter)
                        })
                        .map(|topic| {
                            view! {
                                <Heading>{topic}</Heading>
                                <Divider/>
                                <div class="flex flex-col gap-8 sm:grid sm:grid-cols-2">

                                    {move || match topic {
                                        LATEST => {
                                            ARTICLES
                                                .iter()
                                                .take(6)
                                                .map(|article| {
                                                    view! { <ArticlePreview article=article.clone()/> }
                                                })
                                                .collect_view()
                                        }
                                        ALL => {
                                            ARTICLES
                                                .iter()
                                                .map(|article| {
                                                    view! { <ArticlePreview article=article.clone()/> }
                                                })
                                                .collect_view()
                                        }
                                        topic => {
                                            ARTICLES
                                                .iter()
                                                .filter(|article| { article.topic == topic })
                                                .map(|article| {
                                                    view! { <ArticlePreview article=article.clone()/> }
                                                })
                                                .collect_view()
                                        }
                                    }}

                                </div>
                            }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ArticlePreview(
    article: Article,
    #[prop(optional)] no_blurb: bool,
    #[prop(optional)] horizontal: bool,
) -> impl IntoView {
    view! {
        <A
            class=format!(
                "flex gap-2 {}",
                horizontal.not().then_some("flex-col").unwrap_or_default(),
            )

            href=format!("/articles/{}", article.id)
        >
            <img src=article.image.url alt=article.title/>
            <div>
                <small class="text-sm font-light text-blue-800">
                    {article.topic.to_uppercase()}
                </small>
                <Heading>
                    <article class="text-xl">{article.title}</article>
                </Heading>
                {no_blurb
                    .not()
                    .then_some(
                        view! {
                            <Caption>
                                <div class="text-sm text-left">{article.blurb}</div>
                            </Caption>
                        },
                    )}

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
                    <div class="text-blue-800">{move || article().topic.to_uppercase()}</div>
                    "\u{b7} "
                    {move || article().reading_time()}
                    " min read"
                </div>
            </div>
            <div class="sm:px-16">
                <img
                    src=move || article().image.url
                    alt=move || article().title
                    class="object-cover w-full"
                />
                <Caption>{move || article().image.caption}</Caption>
            </div>
            <Divider/>
            <div class="flex flex-col gap-5 sm:text-lg
            [&>div:first-child>p]:first-letter:text-[2.8rem]
            sm:[&>div:first-child>p]:first-letter:text-[3.5rem]
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
            <div class="flex flex-col w-full gap-2 [&_img]:w-1/4">
                {move || {
                    let mut articles = ARTICLES.to_vec();
                    articles.shuffle(&mut thread_rng());
                    let same_topic = articles
                        .iter()
                        .filter(|article| {
                            **article != *this_article() && article.topic == this_article().topic
                        })
                        .collect_vec();
                    let selected = if same_topic.is_empty() {
                        articles.iter().filter(|article| **article != *this_article()).collect_vec()
                    } else {
                        same_topic
                    };
                    selected
                        .into_iter()
                        .take(5)
                        .map(|article| {
                            view! {
                                <ArticlePreview
                                    article=article.clone()
                                    horizontal=true
                                    no_blurb=true
                                />
                            }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! { <h1 class="font-serif text-2xl font-medium uppercase sm:text-4xl">{children()}</h1> }
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
                <div class="hidden sm:block">
                    "Brought to you by incredible (and a few credible) reporters."
                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn Caption(children: Children) -> impl IntoView {
    view! { <caption class="block w-full py-2 text-sm text-right opacity-50">{children()}</caption> }
}

#[component]
#[allow(clippy::needless_lifetimes)]
pub fn CrosswordGrid<'a>(grid: &'a [Option<(char, Option<usize>)>], size: Vec2) -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div
                class="grid text-xs"
                style=format!("grid-template-columns: repeat({}, minmax(0, 1fr));", size.x)
            >

                {grid
                    .iter()
                    .map(|cell| {
                        cell.as_ref()
                            .map_or_else(
                                || {
                                    view! {
                                        <div class="grid h-full bg-black place-content-center"></div>
                                    }
                                },
                                |(letter, word_start)| {
                                    view! {
                                        <div class="relative grid h-full p-1 border border-black place-content-center">
                                            {*letter}
                                            <div class="absolute text-xs leading-none opacity-50 inset-0.5">
                                                {*word_start}
                                            </div>
                                        </div>
                                    }
                                },
                            )
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn Crossword() -> impl IntoView {
    #[derive(Params, PartialEq)]
    struct CrosswordParams {
        id: usize,
    }
    let crossword =
        || &CROSSWORDS[use_params::<CrosswordParams>().with(|params| params.as_ref().unwrap().id)];
    view! {
        <div>
            {move || {
                let crossword = crossword();
                let letters = crossword.to_letters();
                let bounds = crossword.bounds();
                let letters = letters
                    .iter()
                    .map(|&(mut letter)| {
                        letter.position -= bounds.0;
                        letter
                    })
                    .collect_vec();
                let size = crossword.size();
                let grid: Vec<Option<(char, Option<usize>)>> = (0..size.y)
                    .flat_map(|y| {
                        (0..size.x)
                            .map(|x| {
                                letters
                                    .clone()
                                    .iter()
                                    .find(|letter| letter.position == Vec2 { x, y })
                                    .map(|letter| (
                                        letter.character,
                                        crossword
                                            .words
                                            .iter()
                                            .position(|word| {
                                                word.position - bounds.0 == letter.position
                                            })
                                            .map(|index| index + 1),
                                    ))
                            })
                            .collect_vec()
                    })
                    .collect();
                view! { <CrosswordGrid grid=&grid size=size/> }
            }}

        </div>
    }
}
