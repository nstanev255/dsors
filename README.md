![fb77b5338833875acedf272cdba437a1.png](https://i.postimg.cc/J7p8jjJj/fb77b5338833875acedf272cdba437a1.png)
# dsors 
![GitHub License](https://img.shields.io/github/license/nstanev255/dsors) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nstanev255/dsors/rust.yml)

Dsors is a small, unofficial, discord gateway sdk.

## how to use
Currently `dsors` is not usable as a library, for now its a normal rust program. But with ambitions to become a fully fledged sdk, which supports all the basic features that are expected.

## contribution
Feel free to contribute to the `dsors` library by opening an issue, or a pull request.

## tungstenite
At the heart of DSORS is the `tungstenite` package, which is used as the websocket library, which powers the client connection. For now only `blocking` is used as a primary read/write method to the sockets, but in the near future the library will move to `tokio` and `async`. 

*NOTE: If you are running on ubuntu you will need to install the `libssl-dev` and `pkg-config`, in order for the connection to happen.*

## license
Distributed under the MIT License. See LICENSE.txt for more information.