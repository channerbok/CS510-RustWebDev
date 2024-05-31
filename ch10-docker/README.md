# Rust Web Development - Chapter 10
This build utilizes Docker to create two containers, one for the database and one for the application.

It is still using a PostgreSQL database, so the code remains the same 
except the addition of a Dockerfile and compose.yaml. Currently the 
password is just stored as a text file in the db folder.

The database is persistent and both questions and answers will be saved upon stopping the container
It contains some of the authentication code from ch9. Unsure if it will be required for this project,
so it has not been fully implemented. Will be removed completely if unrequired.


Execute the code using:
docker compose up --build


You can utilize the REST API via the URL, these commands only work for Windows.

All Questions 
http://localhost:3000/questions


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


Adding an answer

$body= @{
    content = "Mars"
     question_id = 5
 } | ConvertTo-Json

 Invoke-RestMethod -Uri "http://localhost:3000/answer" -Method Post -ContentType "application/json" -Body $body



