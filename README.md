# Warpgate

<a href="https://github.com/warp-tech/warpgate/releases/latest"><img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/warp-tech/warpgate/total.svg?label=DOWNLOADS&logo=github&style=for-the-badge"></a> &nbsp; <a href="https://nightly.link/warp-tech/warpgate/workflows/build/main"><img src="https://shields.io/badge/-Nightly%20Builds-orange?logo=hackthebox&logoColor=fff&style=for-the-badge"/></a> &nbsp;

Warpgate is a smart SSH bastion host for Linux that can be used with _any_ SSH client.

* Set it up in your DMZ, add user accounts and easily assign them to specific hosts within the network.
* Warpgate will record every session for you to view (live) and replay later through a built-in admin web UI.
* Not a jump host - forwards your connections straight to the target instead.
* 2FA support
* Single binary with no dependencies.
* Written in 100% safe Rust.

## Getting started & downloads

* See the [Getting started](https://github.com/warp-tech/warpgate/wiki/Getting-started) wiki page.
* [Release / beta binaries](https://github.com/warp-tech/warpgate/releases)
* [Nightly builds](https://nightly.link/warp-tech/warpgate/workflows/build/main)

<center>
      <img width="783" alt="image" src="https://user-images.githubusercontent.com/161476/162640762-a91a2816-48c0-44d9-8b03-5b1e2cb42d51.png">
</center>

<table>
  <tr>
  <td>
    <img width="1016" alt="image" src="https://user-images.githubusercontent.com/161476/171013863-f087ab75-1b29-4489-b08d-0eacf62fd98c.png">

  </td>
  <td>
    <img width="1016" alt="image" src="https://user-images.githubusercontent.com/161476/171013410-f2b7374c-073e-4a66-b9c6-fe0a6b2b0dd0.png">
  </td>
  </tr>
</table>

## Project Status

The project is currently in **alpha** stage and is gathering community feedback. See the [official roadmap](https://github.com/orgs/warp-tech/projects/1/views/2) for the upcoming features.

In particular, we're working on:

* Support for exposing HTTP(S) endpoints through the bastion,
* Support for tunneling database connections,
* Requesting admin approval for sessions
* and much more.

## How it works

Warpgate is a service that you deploy on the bastion/DMZ host, which will accept SSH connections and provide an (optional) web admin UI.

Run `warpgate setup` to interactively generate a config file, including port bindings. See [Getting started](https://github.com/warp-tech/warpgate/wiki/Getting-started) for details.

It receives SSH connections with specifically formatted credentials, authenticates the user locally, connects to the target itself, and then connects both parties together while (optionally) recording the session.

You manage the target and user lists and assign them to each other through a config file (default: `/etc/warpgate.yaml`), and the session history is stored in an SQLite database (default: in `/var/lib/warpgate`).

You can use the web interface to view the live session list, review session recordings and more.

## Contributing / building from source

* You'll need Rust, NodeJS and Yarn
* Clone the repo
* [Just](https://github.com/casey/just) is used to run tasks - install it: `cargo install just`
* Install the admin UI deps: `just yarn`
* Build the API SDK: `just openapi`
* Build the frontend: `just yarn build`
* Build Warpgate: `cargo build` (optionally `--release`)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/Eugeny"><img src="https://avatars.githubusercontent.com/u/161476?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Eugeny</b></sub></a><br /><a href="https://github.com/Eugeny/warpgate/commits?author=Eugeny" title="Code">💻</a></td>
    <td align="center"><a href="https://the-empire.systems/"><img src="https://avatars.githubusercontent.com/u/18178614?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Spencer Heywood</b></sub></a><br /><a href="https://github.com/Eugeny/warpgate/commits?author=heywoodlh" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/apiening"><img src="https://avatars.githubusercontent.com/u/2064875?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Andreas Piening</b></sub></a><br /><a href="https://github.com/Eugeny/warpgate/commits?author=apiening" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
