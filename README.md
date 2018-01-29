# rust-telebasc

> The name is short for **rust-tele**gram-**b**ot-**a**pi-**sc**raper.  
> A webscraper to gather information from the telegram documentation,
> to generate a raw and low-level model of the api, for the rust-language. Written in rust.

I write it for my own telegram bot api implementation, as a basic foundation.

I used another api model in the past, but I had some problems with it. Unfortunately it was scraped using python, and before that I read a nice [article about webscraping in rust][scrapingarticle] on ["this week in rust"][twir]. As I thought this would be a nice exercise, I decided to try it in rust. I hope writing it in rust will enable everyone (that writes a bot in rust anyway ;) ) to extend and adjust it, if necessary. Please send me pull requests in this case :) .

[twir]: https://this-week-in-rust.org/
[scrapingarticle]: https://codeburst.io/web-scraping-in-rust-881b534a60f7


# Status of the project: proof-of-concept

> proof-of-concept. Not ready at all!
It will be available on cargo when it's ready to test.

It currently contains only some basic evaluations on how and if I can do it. So, see it as a proof-of-concept for now. I am currently rewriting it.


# Goals

- I want the model to be easily updateable. Using this scraper and `git diff` or so, you can see what changed and update your software according to the newest features.
- I want to include all relevant information, including comments to objects and fields.
	- I'll also try to model the available methods and their parameters, but we'll see.
- I want to include the update section of the webpage, to document the changes within the generated code and the corresponding api, the date things changed and so on.

> Note: While I hope to get everything right, I will not test everything, but only the parts I use myself, which currently the main message stuff and keyboards. If you find any errors or want to write some tests to check if the telegram api responds as expected, feel free to open an issue or send a pull request.

## How exactly will the generated model look like in the end?

Very basic. It will contain structs as data-holders, which will be serializeable using serde-json. They will be annotated with all necessary stuff to compile in rust and to be accepted as serialized data by the telegram bot api (I hope, this is hard to test). Do not expect something high-level: It's just the foundation for higher-level-libraries, nothing more!

## Then what is the difference to other rust-telegram-bot-api-model-scrapers?

It's written in rust. It's really the main reason. This way, someone who wants to write something in rust, can adjust it, if needed, without touching another language. The code shall be maintainable and readable, so adjusting it will be easy. That's not much of a reason? True, but I thought it's a good exercise for a beginner. Also, rust isn't the type of language that would be considered perfect for this task, so I just needed to do it! ;)

Also, since this project is managed using `cargo`, you can add it as a subproject (i.e. using cargo workspaces) to have the scraper/generator bound to your project.
