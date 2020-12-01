# win2d-rs

This is a simple example of using [Rust/WinRT](https://github.com/microsoft/winrt-rs) with WinRT dependencies like [Win2D](https://www.nuget.org/packages/Win2D.uwp) where that component provides its own winmd for describing its API surface as well as a runtime DLL that must be deployed with the app. It is further complicated as Win2D requires the [VCRTForwarders](https://www.nuget.org/packages/Microsoft.VCRTForwarders.140/) in order to load. This repo provides an example of how this can be achieved.

Here I have simply unpacked the respective dependencies and placed them in the well-known `.windows` folder that Rust/WinRT expects.
