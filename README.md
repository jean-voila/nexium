# Nexium

_Nexium_ is a decentralized, blockchain-based cryptocurrency for Epita.

**Main language:** Rust

**Target OS:** Linux

**Team and contacts :**
	- william.valenduc@epita.fr
	- jean.herail@epita.fr *(project manager)*
	- antonin.bessieres@epita.fr
	- milo.delbos@epita.fr


## Please, install the following VSCode extensions:
- [Rust](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle). It's a bundle with some useful extensions for Rust development

- [Live Share](https://marketplace.visualstudio.com/items?itemName=MS-vsliveshare.vsliveshare). It's a tool to share your code with other developers in real-time. We'll use it for live coding sessions.

## IMPORTANT: 
Before writing any code, **change your Word Wrapp setting to "wordWrapColumn": 80** in your VSCode settings. This is essential, as it's said on our CdC that we have to give a project that respects the 80 characters per line rule. If you have any questions about this procedure ask on the group's Discord server.

## Developping with Tauri
To test the `nexium_client` package, you need to use `cargo tauri dev` instead of `cargo run`. This will start the Tauri application and allow you to test the client.

## License
See the `LICENSE` file.