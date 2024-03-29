use crate::{components::terminal::Terminal, state::terminal::TerminalState};
use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn About(about_me_text: ReadSignal<String>) -> impl IntoView {
    let terminal_state: RwSignal<TerminalState<'_>> = expect_context::<RwSignal<TerminalState>>();

    let (terminal_show_count, set_terminal_show_count) = create_slice(
        terminal_state,
        |termina_state| termina_state.terminal_show_count,
        |terminal_state, count| terminal_state.terminal_show_count = count,
    );

    let (show_term, set_show_term) = create_slice(
        terminal_state,
        |termina_state| termina_state.show_terminal,
        |terminal_state, show_term| terminal_state.show_terminal = show_term,
    );

    create_effect(move |_| {
        logging::log!(
            "show_terminal = {}, terminal_show_count = {}",
            show_term(),
            terminal_show_count()
        );
        //   logging::log!("about_me_text =  {}", about_me_text.get());
    });

    view! {
        <section id="about" class="bg-gradient-to-r from-gray-800 via-gray-900 to-black py-16">
        <div class="container mx-auto">
           <h2 class="w-full text-4xl font-bold mb-8 text-center text-white shadow-text font-montserrat">
            {"ABOUT"}
           </h2>
           <div class="grid grid-cols-2 gap-8 items-center flex-2">
              <div>


               <Show when=move || show_term.get()>
                   <Terminal about_me_text={about_me_text().clone()} />
               </Show>

               <Show when=move || !show_term.get()>
                  <p class="text-md text-white leading-relaxed md:text-xl lg:text-lg p-2 font-mono">
                     {about_me_text().clone()}
                  </p>
              </Show>

                 <ul class="mt-4 text-white flex flex-wrap gap-4">
                    <li class="flex items-center justify-center w-12 rounded-full cursor-pointer" on:click=move |_| {
                     // set_show_terminal.update(|show_terminal: &mut bool| *show_terminal = !*show_terminal)
                     set_show_term(!show_term());
                     if !show_term() {
                        set_terminal_show_count(terminal_show_count() + 1);
                     }
                    }>
                        <Icon
                        class="animate-pulse hover:animate-none text-green-500 hover:text-green-400 hover:scale-110 transition duration-300 ease-in-out" icon=i::BsTerminal width="25" height="25" />
                    </li>
                    <li>
                       <span class="inline-block bg-gradient-to-r from-blue-500 to-indigo-500 text-white text-xs font-semibold py-1 px-2 rounded-full uppercase tracking-wide">
                          {"Cricket"}
                       </span>
                    </li>
                    <li>
                       <span class="inline-block bg-gradient-to-r from-blue-500 to-indigo-500 text-white text-xs font-semibold py-1 px-2 rounded-full uppercase tracking-wide">
                          {"Gym"}
                       </span>
                    </li>
                    <li>
                       <span class="inline-block bg-gradient-to-r from-blue-500 to-indigo-500 text-white text-xs font-semibold py-1 px-2 rounded-full uppercase tracking-wide">
                          {"Tinkering"}
                       </span>
                    </li>
                    <li>
                       <span class="inline-block bg-gradient-to-r from-blue-500 to-indigo-500 text-white text-xs font-semibold py-1 px-2 rounded-full uppercase tracking-wide">
                          {"Travelling"}
                       </span>
                    </li>
                 </ul>
              </div>
              <div class="max-w-md mx-auto">
                 <img
                    priority
                    class="w-3/4 h-auto shadow-md transition rounded-full duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-105 cursor-pointer"
                    src="/assets/images/profile.webp"
                    alt="Avatar"
                    width={500}
                    height={500}
                 />
              </div>
           </div>
        </div>
     </section>
    }
}
