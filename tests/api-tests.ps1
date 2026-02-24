# URLs paths that should pass.
Invoke-RestMethod -Uri http://localhost:3000/
Invoke-RestMethod -Uri http://localhost:3000/health

# Invalid URLs paths that should fail.
Invoke-RestMethod -Uri http://localhost:3000/invalid

Invoke-RestMethod -Uri http://localhost:3000/ -Method POST
Invoke-RestMethod -Uri http://localhost:3000/ -Method PUT
Invoke-RestMethod -Uri http://localhost:3000/ -Method DELETE
Invoke-RestMethod -Uri http://localhost:3000/ -Method PATCH

Invoke-RestMethod -Uri http://localhost:3000/health -Method POST
Invoke-RestMethod -Uri http://localhost:3000/health -Method PUT
Invoke-RestMethod -Uri http://localhost:3000/health -Method DELETE
Invoke-RestMethod -Uri http://localhost:3000/health -Method PATCH
