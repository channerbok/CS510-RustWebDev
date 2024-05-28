# Rust Web Development - Chapter 7
This build changes towards using a PostgreSQL external database.
The REST CRUD API now uses SQL commands to function.

**THIS DOES NOT CONTAIN A PERSISTENT DATABASE**
**YOU MUST PROVIDE YOUR OWN DATABASE AND LINK TO IT IN MAIN**
let store = store::Store::new("postgres://postgres:4411@localhost:3030/rustwebdev").await;


Execute the code using cargo run

You can utilize the REST API via the URL
All Questions 
http://localhost:3000/questions

Questions using Pagination
http://localhost:3000/questions?start=0&end=1

Grab specific  question based on QuestionID
http://localhost:3000/question/1

Delete a specific question
Invoke-WebRequest -Uri "http://localhost:3000/questions/1" -Method Delete

Update a question
Invoke-RestMethod -Uri "http://localhost:3000/questions/2" -Method Put -Body '{"id": 2, "title": "Updated title", "content": "Updated content", "tags": ["updated", "general"]}' -ContentType "application/json"

Add a question
$body = @{
    id = "1"
    title = "TITLE"
    content = "CONTENTT"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/questions" -Method Post -ContentType "application/json" -Body $body
