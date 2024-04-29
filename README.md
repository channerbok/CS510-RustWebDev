# Rust Web Example
Channer Bok
Repo for assignments in CS 510 Rust Web Dev course

Rust Axum REST CRUD Implementation.

This project consists of a Question based data display.
It utilizes a json file filled with questions and their IDs and
uses the GET,PUT,POST,DELETE features of Axum to populate the questions on the 
web server. It creates the web server hosted on http://localhost:3000 and uses various 
routes to deliver different results.

You can utilize the REST API via the URL
All Questions 
http://localhost:3000/questions

Questions using Pagination
http://localhost:3000/questions?start=0&end=1

Grab specific  question based on QuestionID
http://localhost:3000/question/1

Delete a specific question
Invoke-WebRequest -Uri "http://localhost:3000/questions/6" -Method Delete

Update a question
Invoke-RestMethod -Uri "http://localhost:3000/questions/5" -Method Put -Body '{"id": "5", "title": "Updated title", "content": "Updated content", "tags": ["updated", "general"]}' -ContentType "application/json"

## Acknowledgements

CODE REFERENCED/ADAPTED FROM:

Credit to Axum Documentation
 https://docs.rs/axum/latest/axum/
 https://docs.rs/axum/latest/axum/extract/struct.State.html
 https://docs.rs/tower-http/0.5.2/tower_http/cors/index.html
 https://docs.rs/axum/latest/axum/routing/struct.Router.html

 Credit to Tokio Documentation
 https://github.com/tokio-rs/axum

 Credit to Course Textbook
 Rust Web Development WITH WARP, TOKIO, AND REQWEST - Bastian Gruber

 Credit to ChatGPT 3.5 for Debugging PUT, POST, GET, DELETE implementations
 https://chat.openai.com/

 Credit to Rust Users page
 https://users.rust-lang.org/
