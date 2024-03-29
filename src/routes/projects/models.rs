use yew::{html, Html};

pub struct Project<'a> {
    pub name: &'static str,
    pub description: &'static str,
    pub source_code: Option<&'static str>,
    pub link: Option<&'static str>,
    pub image_url: Option<&'static str>,
    pub tags: &'a [&'static str],
}

impl<'a> Project<'a> {
    pub fn to_html(&self) -> Html {
        html! {
        <div class="border-[#3a86ff] border-l-4 p-4 bg-[#3a86ff]/20  hover:bg-[#3a86ff]/30 transition-all" >
            <div class="flex gap-3 flex-col">
                <div class="text-[#ff006e] font-black text-2xl comfortaa">
                    {self.name}
                </div>
                <div class="font-black text-lg comfortaa">
                    {self.description}
                </div>

                <div class="flex flex-row gap-2">
                    {for self.tags.iter().map(|f| html! {
                    <div class="text-gray-500 font-black text-sm comfortaa">
                        {f}
                    </div>
                    })}
                </div>

                <div class="flex flex-row gap-4">
                    {if let Some(f) = self.source_code {
                    html! {
                    <a href={f} class="text-black font-black text-2xl comfortaa">
                        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32"
                            fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
                            <path
                                d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z" />
                        </svg>
                    </a>
                    }
                    } else {
                    html! {}
                    }}
                    {if let Some(f) = self.link {
                    html! {
                    <a href={f} class="text-black font-black text-2xl comfortaa">
                        {"Link"}
                    </a>
                    }
                    } else {
                    html! {}
                    }}
                </div>
            </div>

        </div>

        }
    }
}

pub const PROJECTS: [Project; 3] = [
    Project {
        name: "kivalm.com",
        description: "This website!",
        source_code: Some("https://github.com/KivalM/website"),
        link: Some("https://kivalm.com"),
        image_url: None,
        tags: &["Rust", "Yew", "WebAssembly", "HTML", "TailwindCSS"],
    },
    Project {
        name: "Spotblock-rs",
        description: "(unmaintained) A spotify adblocker for linux written in rust",
        source_code: Some("https://github.com/KivalM/spotblock-rs"),
        link: None,
        image_url: None,
        tags: &["Rust", "*nix"],
    },
    Project {
        name: "SML",
        description: "(wip) A simple markup language written in rust, focused on simplicity",
        source_code: Some("https://github.com/KivalM/kml"),
        link: None,
        image_url: None,
        tags: &["Rust", "Markdown"],
    },
];
