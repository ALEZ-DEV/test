use std::fs::File;

use maud::{html, DOCTYPE};
use rouille::{Request, Response};

mod index;

fn main() {
    rouille::start_server("127.0.0.1:80", |r| request_handler(r));
}

fn request_handler(request: &Request) -> Response {
    if request.url() == "/test" {
        let content = format!("parameter are {}", request.get_param("param").unwrap());
        let theme = request.get_param("t");
        return Response::html(index::get_index());
    } else if request.url() == "/" {
        return Response::redirect_302("/test?param=1&t=light");
    } else if request.url() == "/css/pico.min.css" {
        let file = File::open("./css/pico.min.css").unwrap();
        return Response::from_file("text/css", file);
    }
    Response::empty_404()
}
