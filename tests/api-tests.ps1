$Tests = @(
    # Valid Paths (Expected to pass).
    @{ Uri = "http://localhost:3000/"; Method="GET"; ExpectedStatusCode=200; ExpectedBody="Welcome to Midnight"},
    @{ Uri = "http://localhost:3000/health"; Method="GET"; ExpectedStatusCode=200; ExpectedBody="ok"}

    # Invalid Paths (Expected to fail).
    @{ Uri ="http://localhost:3000/invalid"; Method="GET"; ExpectedStatusCode=404}

    # Invalid Methods (Expected to fail).
    @{ Uri ="http://localhost:3000/"; Method="POST"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="PUT"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="DELETE"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="PATCH"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="HEAD"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="OPTIONS"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="TRACE"; ExpectedStatusCode=405}
    @{ Uri ="http://localhost:3000/"; Method="CONNECT"; ExpectedStatusCode=405}
)

foreach ($Test in $Tests) {
    try {
        _$response = Invoke-RestMethod -Uri $Test.Uri -Method $Test.Method -StatusCodeVariable "actualCode"
    } catch {
        $actualCode = $_.Exception.Response.StatusCode.value__
    }

    $statusText = "[$($Test.Method)] $($Test.Uri)"
    
    if ($actualCode -eq $Test.Expected) {
        Write-Host "PASS: $statusText - Received expected $actualCode" -ForegroundColor Green
    } else {
        Write-Host "FAIL: $statusText - Expected $($Test.Expected), but got $actualCode" -ForegroundColor Red
    }
}