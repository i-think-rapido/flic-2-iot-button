# Flic 2 Button Client Library written in Rust

This is a library for the Flic 1 and Flic 2 Button.

You can create a client that connects to a server and gets notified when a registered button is activated. This is called receiving an *Event*.

With this client you can also transmit *Command*s to the server to perform distinct actions on the server like scanning for new buttons.

## Where you can get a server

[This github project](https://github.com/50ButtonsEach) has implementations of sdks and clients in different languages and environments.

For instance, there you can get FlicSDK.exe which is a server for Windows.

## Examples

- ping -- just pings the server
- simpleclient -- performs button scans and registers buttons

Hint: IP addresses and BlueTooth addresses should to be replaced to your needs to work properly.

---
Thank you for your review: [@matthiasbeyer](https://github.com/matthiasbeyer)
