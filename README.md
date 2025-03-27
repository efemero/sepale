# Sépale

This is a toy project of an application that generate an [EPC QR code](https://en.wikipedia.org/wiki/EPC_QR_code).

Eventually it will be available on multiple platforms: Linux, Windows, MacOs, Android and iOS.

## Why?

In Belgium, we have a system called [payconiq](https://www.payconiq.be) that can be used to transfer money from person to person, or from person to business.
This is a private company that sets some limits, and you can raise those limits if you pay them.

But in Europe we have the SEPA system, and there is a standard to generate QR codes that can be read by some banking apps: [EPC QR code](https://en.wikipedia.org/wiki/EPC_QR_code).
There are no limitations to the generated QR codes, and I wanted to be able to generate my own QR codes.

And this is also a good opportunity to learn some new techs: [tauri](https://tauri.app/), [htmx](https://htmx.org/) and [GitHub actions](https://docs.github.com/en/actions).
