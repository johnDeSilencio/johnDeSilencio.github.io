use leptos::prelude::*;

#[component]
pub fn Biography() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4 text-base text-white border-solid border-2 border-white rounded m-1 p-2 shadow-lg shadow-white">
            <p>"Hello! 👋 My name is Nicholas R. Smith."</p>
            <p>
                "I am currently a software engineer at Schweitzer Engineering Laboratories working on the "
                <a class="inline-flex" href="https://selinc.com/deployBlueframe">
                    <nobr>
                        <u>
                            <i>"Blueframe Platform."</i>
                        </u>
                    </nobr>
                </a> " During the daytime, I am a fullstack software engineer
                working primarily in Golang, React, TypeScript, and dabbling in DevOps when the
                Jenkins pipeline inevitably breaks."
            </p>
            <p>
                "During the evenings, I primarily work with Rust, Nix, and NixOS. I have been using "
                <a class="inline-flex" href="https://github.com/johnDeSilencio/NicksOS">
                    <nobr>
                        <u>"NixOS"</u>
                    </nobr>
                </a> " as my daily driver on my Framework 16 laptop for almost a year now."
            </p>
            <p>
                "I am a practicing, confirmed Roman Catholic who is in love with Jesus and His Church."
            </p>
            <p>
                "When I'm not ricing my desktop, I love to jog, lift weights, and read in my free time."
            </p>
            <p>
                "I grew up in the Inland Northwest in the rolling hills of the "
                <a class="inline-flex" href="https://www.google.com/search?q=palouse&udm=2">
                    <nobr>
                        <u>"Palouse"</u>
                    </nobr>
                </a> " and hope to live in the PNW for the rest of my life."
            </p>
        </div>
    }
}
