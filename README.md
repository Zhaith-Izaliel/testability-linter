
<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a id="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![GPLv3][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">Testability Linter</h3>
  <p align="center">
    Lint Java classfiles with a set of simple rules to check if your code can accommodate unit tests nicely or not.
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This project is a proof of concept linter on Java class files. This linter checks for specific patterns to follow which increasing testability, the relative ease and expense to revealing software flaws, and in our case, make writing robust unit tests easier.

This linter was developed as part of an internship in 2023 in the Laboratoire D'Informatique de Grenoble (LIG), supervised by Lydie du Bousquet. You can find the internship report [here][report].

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* [![Rust][Rust]][Rust-url]
* [![Nix][Nix]][Nix-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

Testability Linter is made using Rust and can be compiled directly from source. 

### Prerequisites

Testability Linter requires the Rust Compiler if you plan to compile it, you will also need Cargo to build the project.
* `rustc` >= 1.77.2
* `cargo` >= 1.77.2

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/Zhaith-Izaliel/testability-linter.git
   ```
2. Install Cargo and Rustc from your package manager.
3. Build Testability Linter with Cargo in release mode 
   ```sh
   cargo build --release
   ```
4. An executable for Testability Linter will be available in `target/release/testability-linter`

#### With Nix

You can run the project with Nix installed with the following command.

```bash
nix run github:Zhaith-Izaliel/testability-linter -- [arguments]
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

Testability Linter requires at least two arguments:

1. A `rules.toml` file containing the rules to check for
1. At least one `.class` file to lint.

For example, if your rules live in `./rules.toml` and your class file lives in `./Main.class`

```bash
testability-linter rules.toml Main.class
```

Note that you can supply multiple class files past the first, like so:

```bash
testability-linter rules.toml Main.class Test.class ObjectPermanence.class ...
```

And the linter will print a report for each file accordingly.

### Rules File

The `rules.toml` file contains the rules to check for.

```toml
check_no_void = true # Check if a method has a return type of void. A void method is usually harder to test and can potentially have side effects.
no_binary_in_names = true # Check if a method name has `and`/`or` in its name, generally indicating it has multiple purposes and thus, hinders testability
too_many_arguments = 4 # Fails a method if the method has more than the given number of arguments. The more arguments a method has, the harder it is to write cogent unit tests
```
Any rules here can be omitted if need be.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the GPLv3 License. See [`LICENSE`][license-url] for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Best-README-Template](https://github.com/othneildrew/Best-README-Template) for this README
* [Code Smells Catalog](https://luzkan.github.io/smells/) that directly inspired this linter

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/Zhaith-Izaliel/testability-linter.svg?style=for-the-badge
[contributors-url]: https://github.com/Zhaith-Izaliel/testability-linter/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Zhaith-Izaliel/testability-linter.svg?style=for-the-badge
[forks-url]: https://github.com/Zhaith-Izaliel/testability-linter/network/members
[stars-shield]: https://img.shields.io/github/stars/Zhaith-Izaliel/testability-linter.svg?style=for-the-badge
[stars-url]: https://github.com/Zhaith-Izaliel/testability-linter/stargazers
[issues-shield]: https://img.shields.io/github/issues/Zhaith-Izaliel/testability-linter.svg?style=for-the-badge
[issues-url]: https://github.com/Zhaith-Izaliel/testability-linter/issues
[license-shield]: https://img.shields.io/github/license/Zhaith-Izaliel/testability-linter.svg?style=for-the-badge
[license-url]: https://github.com/Zhaith-Izaliel/testability-linter/blob/master/LICENSE
[report]: https://github.com/Zhaith-Izaliel/testability-linter/blob/master/Automating%20the%20Verification%20of%20Software_Testability_of_Software_Artifacts.pdf

[product-screenshot]: images/screenshot.png
[Rust]: https://img.shields.io/badge/Rust-B7400F?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Nix]: https://img.shields.io/badge/nix-0B1120?style=for-the-badge&logo=nixos
[Nix-url]: https://nixos.org/
