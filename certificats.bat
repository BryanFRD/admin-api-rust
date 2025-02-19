@echo off

set CERTS_DIR=certs
set CERTIFICATE_FILE=%CERTS_DIR%\localhost.crt
set PRIVATE_KEY_FILE=%CERTS_DIR%\localhost.key

if not exist "%CERTS_DIR%" (
  mkdir "%CERTS_DIR%"
)

where openssl >nul 2>&1
if %ERRORLEVEL% neq 0 (
  echo OpenSSL is not installed or not in the PATH.
  echo Please install OpenSSL and try again.
  exit /b 1
)

openssl genrsa -out "%PRIVATE_KEY_FILE%" 2048
if %ERRORLEVEL% neq 0 (
  echo Error generating private key.
  exit /b 1
)

openssl req -new -x509 -key "%PRIVATE_KEY_FILE%" -out "%CERTIFICATE_FILE%" -days 365 -subj "/CN=localhost"
if %ERRORLEVEL% neq 0 (
  echo Error generating certificate.
  exit /b 1
)

echo.
echo Certificate and private key successfully generated in the following files:
echo   - %CERTIFICATE_FILE%
echo   - %PRIVATE_KEY_FILE%
echo.