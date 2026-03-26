# Eigene Implementation von RSA

## Beschreibung
In diesem Repository habe ich eine eigene Implementation des RSA-Algorithmus in Rust geschrieben. 
Hierfür wurden nur die nötigsten Packages genutzt und so viel, wie nur möglich selbst per Hand geschrieben. Vereinzelte Funktionen wurden mithilfe einer KI (ChatGPT) geschrieben, diese wurden entsprechend gekennzeichnet. 
Prinzipiell ist dieses Repository bisher nur eine Ansammlung von verschiedenen Funktionen, welche man selbst noch per Hand zusammenfügen muss und anschließend laufen lassen kann. Sollte ich die Zeit dazu haben, werde ich alle Funktionen in ein einziges Binary Package kompilieren mit einer einfachen TUI. 

## Installation
Da dieses Projekt eine reine Rust-Codebase hat kann der gesamte Code mithilfe von Rust / Cargo kompiliert werden. Hierzu muss lediglich `git clone https://github.com/Erebos132/RSA` in einer Command Prompt, in welcher git installiert ist, ausgeführt werden. Anschließend kann man mit dem Cargo Package: `cargo run` ausführen, wodurch das Projekt automatisch kompiliert und ausgeführt wird. Zurzeit muss hierfür noch eine Ansammlung von Funktionen in der `main.rs` Datei existieren. Dies wird frühstmöglich geändert.

Update:
Inzwischen gibt es eine einfache TUI für die rudimentärsten Funktionen. Hierzu muss lediglich das Programm kompiliert und ausgeführt werden.

Hier einige Demos der implementierten Funktionen: 
<video src="./Videos/key-generation_%05d.webp" width="320" height="240"></video>
