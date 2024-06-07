use crate::crossword::Vec2;
use itertools::Itertools;
use leptos::leptos_dom::helpers::location;
use std::iter::once;
use std::ops::Not;

use crate::ad::ADS;
use crate::article::{Article, ARTICLES};
use crate::article::{Fragment, Image};
use crate::crossword::CROSSWORDS;
use chrono::Local;

use leptos::{
    component, create_signal, view, Children, CollectView, IntoView, Params,
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
            <a
                class="w-full text-center"
                href="/"
                on:click=|_| {
                    location().reload().unwrap();
                }
            >

                <Heading>
                    <div class="text-5xl capitalize font-blackletter">"The Yesterday"</div>
                    <div class="block font-serif text-base">"Trusted by dozens."</div>
                </Heading>
            </a>
        </header>
    }
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <main class="flex justify-center gap-4 p-4 grow">
            <div class="w-full max-w-2xl shrink-0">{children()}</div>
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
    let ad = ADS.choose(&mut thread_rng()).unwrap();
    let (show_overlay, set_show_overlay) = create_signal(false);
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
        <div class="sticky bottom-0 flex justify-center w-full p-2 bg-gray-100 border">
            <div class="relative">
                <div class="relative min-h-36">
                    <img
                        src=format!("/images/horizontal-ads/{}", *ad)
                        class="w-full max-w-2xl cursor-pointer"
                    />
                    <div class=move || {
                        format!(
                            "absolute inset-0 z-10 flex flex-col items-center gap-1 p-2 bg-gray-100 border text-neutral-500 {}",
                            if show_overlay.get().not() {
                                "opacity-0 pointer-events-none"
                            } else {
                                "opacity-100 transition-opacity duration-1000"
                            },
                        )
                    }>
                        <button
                            class="absolute top-0 left-0 p-2 text-2xl leading-none"
                            on:click=move |_| set_show_overlay(false)
                        >
                            "\u{2190}"
                        </button>
                        <h1 class="text-2xl">
                            "Ads not by " <span class="font-bold">"Google"</span>
                        </h1>
                        <div class="flex flex-col w-full gap-1 px-16">
                            <button
                                class="w-full py-2 text-white bg-blue-500 rounded-sm shadow"
                                on:click=move |_| set_show_overlay(false)
                            >
                                "Keep seeing this ad"
                            </button>
                            <button
                                class="w-full py-2 bg-white rounded-sm shadow"
                                on:click=move |_| set_show_overlay(false)
                            >
                                "Why not this ad? \u{25B7}"
                            </button>
                        </div>
                    </div>
                </div>
                <div class="text-sm text-center opacity-50">"Advertisement"</div>
                <button
                    class="absolute top-0 right-0 flex text-xs leading-none text-blue-500"
                    on:click=move |_| set_show_overlay(true)
                >
                    <div class="grid border bg-gray-100/50 size-4 place-content-center">
                        <div class="cursor-pointer border rounded-full text-[8px] aspect-square size-3 grid place-content-center border-blue-500 font-medium">
                            i
                        </div>
                    </div>
                    <div class="grid border place-content-center bg-gray-100/50 size-4">"X"</div>
                </button>
            </div>
        </div>
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
