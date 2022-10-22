<h1 align="center"> lonk </h1> <br>
<p align="center">
  <a href="https://github.com/postrequest/link">
    <img alt="🔗" title="link" src="https://i.giphy.com/6d2rN7dGii0eiKB2HO.gif" width="450">
  </a>
</p>

<p align="center">
  lonk is a fork of link, a command and control framework written in rust. Currently in <strong>beta</strong>.
</p>


</p>

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Feedback](#feedback)
- [Build Process](#build-process)
- [Acknowledgments](#acknowledgments)

## Introduction

[![Repo Size](https://img.shields.io/github/repo-size/postrequest/link)](https://img.shields.io/github/repo-size/postrequest/link)
[![Lines of code](https://img.shields.io/tokei/lines/github/postrequest/link)](https://img.shields.io/tokei/lines/github/postrequest/link)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0.en.html)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/postrequest/link)](https://rust-reportcard.xuri.me/report/github.com/postrequest/link)

Lonk provides MacOS, Linux and Windows implants which may lack the necessary evasive tradecraft provided by other more mature command and control frameworks.

A fork of postrequest's link, Lonk is r3wst3r's first pass at developing an adversarial C2 framework in the Rust programming language. A project goal is to be able to deploy from a kubernetes mesh using kris-nova's Aurae, currently in development, for no reason other than because it's neat.

The Link link (link^2?): https://github.com/postrequest/link<br>
Aurae: https://github.com/aurae-runtime/community


## Features

Hopefully this list expands for humans to actually want to use this:

* HTTPS communication
* Process injection
* In-memory .NET assembly and Windows PE execution
* SharpCollection tools
* sRDI implementation for shellcode generation
* Windows link reloads DLLs from disk into current process

## Feedback

Feel free to [file an issue](https://github.com/postrequest/link/issues/new).

## Build Process

- Clone or download the repo
- `cargo run` if you are eager to run it **right now**
- `cargo build --release` to build the link server executable

For more information check out [Installation and Usage](https://github.com/postrequest/link/wiki/Installation-and-Usage).

## Acknowledgments

A non-exhaustive list of those who have in some way inspired this project by means of writing code, snippets, ideas or inspiration:

[@rust](https://github.com/rust-lang)  
[@moloch--](https://github.com/moloch--)  
[@djhohnstein](https://github.com/djhohnstein)  
[@lesnuages](https://github.com/lesnuages)  
[@Flangvik](https://github.com/Flangvik)  
[@monoxgas](https://github.com/monoxgas)  
[@b4rtik](https://github.com/b4rtik)  
