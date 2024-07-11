#!/bin/bash

# ++++| 1. Download CRX files |++++

# ++++| 2. Start Chrome with remote debugging |++++

# ++++| 3. Establish connection to Chrome |++++

# ++++| 4. Install CRX files |++++

# ++++| 5. Send the Command to Install CRX files |++++

# ++++| 6. Send the CRX Content |++++

# ++++| 7. Wait for the Response and Clean Up |++++

# # Define paths and variables
# $chromePath = "C:\Program Files (x86)\Google\Chrome\Application\chrome.exe" # Path to Chrome executable
# $extensionNetworkPath = "\\Server\Share\Extension.crx" # Network path to the extension file
# $tempExtensionPath = "$env:TEMP\Extension.crx" # Temporary local path to download the extension file

# # Download the extension file from the network path
# Invoke-WebRequest -Uri $extensionNetworkPath -OutFile $tempExtensionPath

# # Start Chrome with remote debugging enabled
# $chromeProcess = Start-Process -FilePath $chromePath -ArgumentList "--remote-debugging-port=9222" -PassThru

# # Wait for Chrome to start and establish a connection
# Start-Sleep -Seconds 3

# # Connect to the remote Chrome instance
# $chromeSocket = New-Object Net.Sockets.TcpClient("localhost", 9222)
# $stream = $chromeSocket.GetStream()
# $writer = New-Object System.IO.StreamWriter($stream)
# $reader = New-Object System.IO.StreamReader($stream)

# # Send the command to install the extension
# $writer.WriteLine("POST /json/extensions/install HTTP/1.1")
# $writer.WriteLine("Host: localhost:9222")
# $writer.WriteLine("Content-Length: " + (Get-Item $tempExtensionPath).Length)
# $writer.WriteLine("Content-Type: application/x-www-form-urlencoded")
# $writer.WriteLine()
# $writer.Flush()

# # Read the content of the extension file and send it
# $extensionContent = Get-Content $tempExtensionPath -Raw
# $writer.Write($extensionContent)
# $writer.Flush()

# # Wait for the response
# $response = $reader.ReadToEnd()

# # Close the connections
# $writer.Close()
# $reader.Close()
# $chromeSocket.Close()

# # Terminate the Chrome process
# $chromeProcess.Kill()

# # Remove the temporary extension file
# Remove-Item -Path $tempExtensionPath

