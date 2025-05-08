use rocal::{
    rocal_core::{
        router::link_to,
        traits::{SharedRouter, Template},
    },
    view,
};

use crate::view_models::root_view_model::RootViewModel;

pub struct RootTemplate {
    router: SharedRouter,
}

impl Template for RootTemplate {
    type Data = RootViewModel;

    fn new(router: SharedRouter) -> Self {
        RootTemplate { router }
    }

    fn body(&self, data: Self::Data) -> String {
        view! {
            <div class="w-screen h-screen">
                <header class="flex justify-start drop-shadow-md">
                  <h1 class="m-4 font-semibold text-xl">{"Simple Note"}</h1>
                  <h2 class="mt-5">{"powered by"}<a href="https://github.com/rocal-dev/rocal" target="_blank" class="text-sky-600">{"Rocal"}</a></h2>
                </header>
                <div class="m-6 grid grid-cols-6 gap-4">
                  <div class="col-span-1">
                    <form action="/notes">
                      <button type="submit" class="text-xl">{"+ New"}</button>
                    </form>
                    <ul>
                      for note in data.get_notes() {
                        <li class="m-3">
                          <a href={{ link_to(&format!("/?note_id={}", &note.id), false) }}>
                            if let Some(title) = &note.get_title() {
                              {{ title }}
                            } else {
                              {"Untitled"}
                            }
                          </a>
                        </li>
                      }
                    </ul>
                  </div>
                  <div class="col-span-5">
                    if let Some(note) = data.get_note() {
                      <form action={{ &format!("/notes/{}", &note.id) }} method="patch">
                        if let Some(title) = note.get_title() {
                          <input type="text" name="title" placeholder="Title" class="border-none text-5xl appearance-none w-full py-4 px-3 text-gray-700 leading-tight outline-none" value={{ title }}/>
                        } else {
                          <input type="text" name="title" placeholder="Title" class="border-none text-5xl appearance-none w-full py-4 px-3 text-gray-700 leading-tight outline-none" />
                        }
                        <textarea name="body" class="border-none text-4xl appearance-none w-full h-[60vh] py-4 px-3 text-gray-700 leading-tight outline-none" placeholder="Type something...">
                          if let Some(body) = note.get_body() {
                              {{ body }}
                          }
                        </textarea>
                        <button type="submit" class="underline p-3 mt-3 text-xl text-gray-800">{"Save changes"}</button>
                      </form>
                      <form action={{ &format!("/notes/{}", &note.id) }} method="delete">
                        <button type="submit" class="underline p-3 mt-1 text-xl text-red-700">{"Delete"}</button>
                      </form>
                    } else {
                      <form action="/notes" method="post">
                        <input type="text" name="title" placeholder="Title" class="border-none text-5xl appearance-none w-full py-4 px-3 text-gray-700 leading-tight outline-none" />
                        <textarea name="body" class="border-none text-4xl h-[60vh] appearance-none w-full py-4 px-3 text-gray-700 leading-tight outline-none" placeholder="Type something..."></textarea>
                        <button type="submit" class="underline p-3 mt-3 text-xl text-gray-800">{"Save changes"}</button>
                      </form>
                    }
                  </div>
                </div>
            </div>
        }
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
