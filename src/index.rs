use maud::{html, PreEscaped, DOCTYPE};

pub fn get_index() -> PreEscaped<String> {
    html! {
           (DOCTYPE)
               html data-theme="dark" {
                   head {
                       link rel="stylesheet" href="./css/pico.min.css";
                       style {
                           ":root { --primary: #d81b60; }"
                       }
                   }
                   body {
                       nav class="container-fluid" style="position: fixed" {
                           h3 { "Torrent Downloader" }
                       }

                       main class="container" {
                           aside {
                               nav class="closed-on-mobile" {
                                ul {
                                    li { "option 1" }
                                    li { "option 2" }
                                    li { "option 3" }
                                    li { "option 4" }
                                    li { "option 5" }
                                    li { "option 8" }

                                }
                                                                      }
                           }
                           section id="containers" {
                               div {
                                @for i in 1..3 {
                                    p {
    " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce quis tellus ante. Nunc facilisis eros quam, sit amet ornare eros venenatis et. Curabitur pulvinar congue est eget eleifend. Maecenas vel mollis diam, non fringilla nisi. Cras vulputate gravida rutrum. Pellentesque elementum arcu ac diam egestas, eget molestie metus venenatis. Aliquam erat volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Aenean sit amet justo massa. Etiam in tincidunt urna.

Curabitur vitae lorem augue. Mauris sit amet erat ex. Pellentesque egestas mi sit amet leo porta, mattis iaculis augue pretium. Donec ornare pellentesque ex eget pellentesque. Vestibulum in ipsum ultricies, euismod purus eget, maximus metus. Suspendisse luctus, arcu sed sagittis volutpat, felis velit tempor dolor, tincidunt molestie augue nisi eget dolor. Integer a dapibus elit. Cras at laoreet orci. Donec dolor lacus, pulvinar non convallis nec, maximus vel ante. Phasellus vehicula pharetra egestas. " 
                                    }
                                }

                                button { "click" }
                               code {
                                   "test {
                                    hehe().add(?)
                                }"
                               }
                           }
                           }

                       }
                   }
               }
           }
}
