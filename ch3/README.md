# Rust Web Development - Chapter 3
This is the intial REST Crud API implementation.
It utilizes a Hash Map of the Store type to store the Question data.

There is just the GET action, wherein the entire database of questions can be displayed at
http://localhost:3000/questions

It also uses pagination to display set amounts of questions as well
Questions using Pagination
http://localhost:3000/questions?start=0&end=1

The questions are read in from questions.json

I implemented a custom error type for errors at runtime if there are URL mistakes
-MyError-