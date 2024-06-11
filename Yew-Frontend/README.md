# Question and Answer Yew Frontend

This implementation operates as the frontend of my Rust Axum build.
It pulls Question data from my ch10-docker build at http://localhost:8000/api/v1/question
**The ch10-docker container needs to be running in order for this front end to properly function**
This program will pull a random question from my Docker database and display it on http://localhost:3000
There are two buttons, one to grab a random question and another to add an answer to a displayed question.


Steps to Execute:
1. Open Docker Desktop
2. CD into ch10-docker folder
3. Run: docker compose up --build
4. CD into Frontend folder
5. Run: trunk serve --address 127.0.0.1 --port 3000 --open
6. A browser window should open automatically, if not open brower and go to this URL: http://localhost:3000


References:
https://github.com/pdx-cs-rust-web/knock-knock-yew/tree/main
https://yew.rs/docs/tutorial
https://pudding-entertainment.medium.com/rust-building-web-frontend-with-yew-ca7421fe01d4
https://dev.to/davidedelpapa/series/5838



