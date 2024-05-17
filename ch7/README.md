# Rust Web Development - Chapter 7

Adding a Question
$body = @{
    id = 1001  
    title = "Where?"
    content = "Over Here!
    tags = @("rust", "location")
} | ConvertTo-Json
Invoke-RestMethod -Uri "http://localhost:3000/questions" -Method Post -ContentType "application/json" -Body $body