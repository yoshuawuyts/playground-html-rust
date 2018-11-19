#![feature(async_await, futures_api)]
#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]

use http::status::StatusCode;
use tide::{body, Response};
use typed_html::types::LinkType;
use typed_html::{dom::DOMTree, html, text};

mod cli;
mod error;

pub use crate::cli::Cli;

pub async fn index() -> Result<Response, StatusCode> {
  let doc: DOMTree<String> = html!{
    <html>
      <head>
        <title>"Hello Kitty!"</title>
        <link rel=LinkType::StyleSheet href="lol.css"/>
      </head>
      <body>
        <h1 data-lol="omg">"Hello Kitty!"</h1>
        <p class="official-position-of-sanrio-ltd emphasis">
          "She is not a "<em><a
            href="https://en.wikipedia.org/wiki/Cat">"cat"</a></em>". She is a
          "<em>"human girl"</em>"."
        </p>
        <p class=["urgent", "question"]>"But how does she eat?"</p>
        {
          (1..4).map(|i| {
            html!(<p>{ text!("{}. Ceci n'est pas une chatte.", i) }</p>)
          })
        }
        <p>"<img src=\"javascript:alert('pwned lol')\">"</p>
        <button onclick="alert('She is not a cat.')">"Click me!"</button>
      </body>
    </html>
  };

  http::Response::builder()
    .header("Content-Type", "text/html")
    .body(body::Body::from(doc.to_string().into_bytes()))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
