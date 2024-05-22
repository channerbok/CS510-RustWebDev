# Rust Web Development - Chapter 9

Invoke-RestMethod -Method POST -Uri "http://localhost:3000/registration" -ContentType "application/json" -Body '{
    "email": "test@email.com",
    "password": "cleartext"
}'
