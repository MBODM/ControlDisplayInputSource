@echo off
cls
echo Creating .exe file, by running 'go build' command...
echo go build -ldflags="-s -w"
go build -ldflags="-s -w"
echo Finished.
echo Have a nice day.
echo.
pause
