use yew::{function_component, html, Html};
use yew_router::prelude::Link;

use crate::{components::navbar::Navbar, router::Route};

#[function_component(CVPage)]
pub fn cv() -> Html {
    html! {

            <>
                <Navbar />
                <div class="px-10 mt-10 pb-3 poppins text-3xl font-semibold text-slate-900">
                    <h1>
                        <span class="text-blue-700">{"Kival Mahadew"}</span>
                    </h1>
                    <h2>{"Software Engineer & CS Student"}</h2>
                    <h3 class="text-lg flex items-center gap-1">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-envelope" viewBox="0 0 16 16">
      <path d="M0 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V4Zm2-1a1 1 0 0 0-1 1v.217l7 4.2 7-4.2V4a1 1 0 0 0-1-1H2Zm13 2.383-4.708 2.825L15 11.105V5.383Zm-.034 6.876-5.64-3.471L8 9.583l-1.326-.795-5.64 3.47A1 1 0 0 0 2 13h12a1 1 0 0 0 .966-.741ZM1 11.105l4.708-2.897L1 5.383v5.722Z"/>
    </svg>
                    {"kivalm(at)protonmail.com"}</h3>
                    <h3 class="text-lg flex items-center gap-1">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-geo-alt" viewBox="0 0 16 16">
          <path d="M12.166 8.94c-.524 1.062-1.234 2.12-1.96 3.07A31.493 31.493 0 0 1 8 14.58a31.481 31.481 0 0 1-2.206-2.57c-.726-.95-1.436-2.008-1.96-3.07C3.304 7.867 3 6.862 3 6a5 5 0 0 1 10 0c0 .862-.305 1.867-.834 2.94zM8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10z"/>
          <path d="M8 8a2 2 0 1 1 0-4 2 2 0 0 1 0 4zm0 1a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
        </svg>
                    {"Durban, South Africa"}</h3>


                </div>
                <hr class="border-2 border-black mx-10 rounded-lg" />

                <div class="grid px-10 py-5 grid-cols-1 md:grid-cols-2 poppins gap-5">

                    <div>
                        <h1 class="text-2xl font-bold text-slate-900">{"Education"}</h1>

                        // start job
                        <div class="p-2">
                            <div class="flex justify-between">
                                <h2 class="font-bold text-cyan-900">{"BSc Computer Science"}</h2>
                                <h3 class="font-bold ml-5">{"UKZN"}</h3>
                            </div>
                            <h4 class="text-xs text-gray-500 font-bold">{"Jan 2021 - Present(Dec 2023)"}</h4>
                            <p class="text-sm mt-2 ml-1">
                                {"Working towards my BSc Computer Science degree at the University of KwaZulu-Natal with a focus on
                                Machine Learning. I am currently in my final year."}
                            </p>
                        </div>

                        <h1 class="text-2xl font-bold text-slate-900">{"Links"}</h1>
                        <div class="flex flex-row p-2">
                        <a href="https://github.com/kivalm" target="_blank" class="mx-2 hover:scale-110 transition duration-75">
                            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-github"
                                viewBox="0 0 16 16">
                                <path
                                    d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z" />
                            </svg>
                        </a>

                        <a href="https://www.linkedin.com/in/kivalm/" target="_blank"
                            class="mx-2 hover:scale-110 transition duration-75">
                            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor"
                                class="bi bi-linkedin" viewBox="0 0 16 16">
                                <path
                                    d="M0 1.146C0 .513.526 0 1.175 0h13.65C15.474 0 16 .513 16 1.146v13.708c0 .633-.526 1.146-1.175 1.146H1.175C.526 16 0 15.487 0 14.854V1.146zm4.943 12.248V6.169H2.542v7.225h2.401zm-1.2-8.212c.837 0 1.358-.554 1.358-1.248-.015-.709-.52-1.248-1.342-1.248-.822 0-1.359.54-1.359 1.248 0 .694.521 1.248 1.327 1.248h.016zm4.908 8.212V9.359c0-.216.016-.432.08-.586.173-.431.568-.878 1.232-.878.869 0 1.216.662 1.216 1.634v3.865h2.401V9.25c0-2.22-1.184-3.252-2.764-3.252-1.274 0-1.845.7-2.165 1.193v.025h-.016a5.54 5.54 0 0 1 .016-.025V6.169h-2.4c.03.678 0 7.225 0 7.225h2.4z" />
                            </svg>
                        </a>

                    </div>
                    </div>
                    <div>
                        <h1 class="text-2xl font-bold text-slate-900">{"Work Experience"}</h1>

                        // start job
                        <div class="p-2">
                            <div class="flex justify-between">
                                <h2 class="font-bold text-cyan-900">{"Freelancer"}</h2>
                                <h3 class="font-bold ml-5">{"Upwork"}</h3>
                            </div>
                            <h4 class="text-xs text-gray-500 font-bold">{"Jan 2021 - Present"}</h4>
                            <p class="text-sm mt-2 ml-1">
                                {"Various freelance projects including: "}
                                <ul class="list-disc ml-5">
                                    <li>{"Designed and built a system for real-time monitoring of industrial machinery"}</li>
                                    <li>{"Work on a cryptocurrency focused no-code platform"}</li>
                                    <li>{"Various consulting and tutoring jobs."}</li>
                                </ul>
                            </p>
                        </div>

                        // start job
                        <div class="mt-5 border p-2">
                            <div class="flex justify-between">
                                <h2 class="font-bold text-cyan-900">{"Software Engineer"}</h2>
                                <h3 class="font-bold ml-5 ">{"QuickAI"}<a class="text-blue-600" href="https://quickai.app">{" (link)"}</a></h3>
                            </div>
                            <h4 class="text-xs text-gray-500 font-bold">{"May 2022 - April 2023"}</h4>
                            <p class="text-sm mt-2 pl-1">
                                {"My primary responsibility was the design and development of software for the QuickAI platform. This includes: "}
                                <ul class="list-disc ml-5">
                                    <li>{"Building serverless Business Intelligence tools with WebAssembly"}</li>
                                    <li>{"Data Visualisation with Plotly.js"}</li>
                                    <li>{"Building GraphQL servers for handling high-frequency click analytics for
                                    integration with Apache SuperSet."}</li>
                                    <li>{"Open-source project contributions"}</li>
                                    <li>{"Front-end development with SvelteKit and TypeScript"}</li>
                                    <li>{"Integration with AWS services like Cognito and RDS"}</li>
                                    <li>{"PostgreSQL Database management."}</li>
                                    <li>{"Unit and integration testing"}</li>
                                </ul>
                            </p>
                        </div>

                        // start job
                        <div class="mt-5 border p-2">
                            <div class="flex justify-between">
                                <h2 class="font-bold text-cyan-900">{"Competitive Programming Tutor"}</h2>
                                <h3 class="font-bold ml-5">{"Star College"}</h3>
                            </div>
                            <h4 class="text-xs text-gray-500 font-bold">{"April 2021 - Feb 2022"}</h4>
                            <p class="text-sm mt-2 ml-1">
                                {"I tutored students in competitive programming with C++ in preparation for the South
                                African Programming Olympiad."}
                            </p>
                        </div>
                    </div>
                </div>
            </>
            }
}
