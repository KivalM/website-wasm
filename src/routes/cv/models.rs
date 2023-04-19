use yew::{html, Html};

pub struct XPEntry<'a> {
    pub title: &'static str,
    pub time: &'static str,
    pub place: &'static str,
    pub place_link: Option<&'static str>,
    pub description: &'a [&'static str],
}

impl XPEntry<'_> {
    pub fn to_html(&self) -> Html {
        html! {
        <div class="p-2 roboto">
            <div class="flex justify-between">
                <h2 class="font-bold text-cyan-900">{self.title}</h2>
                <h3 class="font-bold ml-5"> {self.place}

                    {if self.place_link.is_some() {
                        html! {
                            <a href={self.place_link.unwrap()} target="_blank" class="text-cyan-900">
                                {" (link)"}
                            </a>
                        }
                    } else {
                        html! {}
                    }
                }



                </h3>
            </div>
            <h4 class="text-xs text-gray-500 font-bold">{self.time}</h4>
            <ul class="list-disc text-sm mt-2 ml-5">
                {for self.description.iter().map(|f| html! {
                  <li> {f} </li>
                })}
            </ul>
        </div>
        }
    }
}

pub const EDUCATION: [XPEntry; 1] = [
        XPEntry {
        title: "BSc Computer Science",
        time: "2021 - 2023(current)",
        place: "UKZN",
        description: &[
        "Working towards my BSc Computer Science degree at the University of KwaZulu-Natal with a focus on Machine
        Learning. I am currently in my final year."
        ],
            place_link: None,
        },
        ];
pub const WORK: [XPEntry; 3] = [
        XPEntry {
        title: "Freelancer",
        time: "Jan 2021 - Present",
        place: "Upwork",
        description: &[
        "Designed and built a system for real-time monitoring of industrial machinery",
        "Work on a cryptocurrency focused no-code platform",
        "Various consulting and tutoring jobs.",
        ], place_link: None,
        
        },
    XPEntry {
        title: "Software Engineer",
        time: "May 2022 - April 2023",
        place: "QuickAI",
        description: &[
        "Building serverless Business Intelligence tools with WebAssembly",
        "Data Visualisation with Plotly.js",
        "Building GraphQL servers for handling high-frequency click analytics for integration with Apache SuperSet.",
        "Open-source project contributions",
        "Front-end development with SvelteKit and TypeScript",
        "Integration with AWS services like Cognito and RDS",
        "PostgreSQL Database management.",
        "Unit and integration testing",
        ], place_link:  Some("https://quickai.app/"),
        },     XPEntry {
            title: "Competitive Programming Tutor",
            time: "April 2021 - Feb 2022",
            place: "Star College",
            description: &[
                "I tutored students in competitive programming with C++ in preparation for the South African Programming Olympiad."
            ], place_link: None,
            }
        ];
