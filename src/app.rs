use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="lker.dev"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
            <Navbar/>
            <BusinessCard/>
            <Footer/>
    }
}

/// Footer component for a website.
#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="footer">
            <div class="footer-content">
                <div class="footer-section">
                    <h5>
                        "Jonathan is a software engineer and certified practicing accountant (CPA). Having started his professional career in accounting he soon sought ways to enhance the work he could do with software engineering practices. After seeing the effectiveness of these he transitioned to a focused software engineering effort; working as a software engineer and undertaking formal studies in computing. Having developed in both fields he is seeking ways to draw out the strengths from each discipline to the enrichment of both."
                    </h5>
                </div>
            </div>
        </footer>
    }
}

/// Navbar component for a website.
#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="navbar">
            <ul>
                <li><a href="#"><img src="walkerdev.png" alt="Logo made of W and @" /></a></li>
                <li><h1>"Jonathan Walker"</h1></li>
            </ul>
        </nav>
    }
}

#[component]
fn BusinessCard(cx: Scope) -> impl IntoView {
    view! { cx, 
        <div class="center-container">
            <div class="card">
                <div class="card-left">
                    <a href="mailto:w@lker.dev">
                        <h2>"w@lker.dev"</h2>
                    <p>"Software Engineer | CPA"</p>
                    </a>
                </div>
                    <div class="socials">
                        <a target="_blank" rel="noopener" href="https://github.com/lker-dev">
                            <svg height="50" width="50" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg"><rect fill="#1b1817" height="512" rx="15%" width="512"/><path d="m335 499c14 0 12 17 12 17h-182s-2-17 12-17c13 0 16-6 16-12l-1-50c-71 16-86-28-86-28-12-30-28-37-28-37-24-16 1-16 1-16 26 2 40 26 40 26 22 39 59 28 74 22 2-17 9-28 16-35-57-6-116-28-116-126 0-28 10-51 26-69-3-6-11-32 3-67 0 0 21-7 70 26 42-12 86-12 128 0 49-33 70-26 70-26 14 35 6 61 3 67 16 18 26 41 26 69 0 98-60 120-117 126 10 8 18 24 18 48l-1 70c0 6 3 12 16 12z" fill="#fff"/></svg>
                        </a>
                        <a target="_blank" rel="noopener" href="https://www.linkedin.com/in/lkerdev/">
                            <svg height="50" width="50" fill="#fff" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg"><rect fill="#0077b5" height="512" rx="15%" width="512"/><circle cx="142" cy="138" r="37"/><path d="m244 194v198m-102-198v198" stroke="#fff" stroke-width="66"/><path d="m276 282c0-20 13-40 36-40 24 0 33 18 33 45v105h66v-113c0-61-32-89-76-89-34 0-51 19-59 32"/></svg>
                        </a>
                </div>
            </div>
        </div>
    }
}


/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
