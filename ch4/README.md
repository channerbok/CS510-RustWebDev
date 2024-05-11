# Rust Web Development - Chapter 4
This is the complete implementation of the REST Crud API.
It has 

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