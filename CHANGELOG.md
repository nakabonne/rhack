# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Other

 - <csr-id-bc2239651306548bc3432e42691e655f54ef6d28/> update workflows

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-rhack v0.1.2 ([`f912275`](https://github.com/simonsan/rhack/commit/f912275f39b9c5262585dc6df544a2a86ab07123))
    - Update Readme ([`25b757e`](https://github.com/simonsan/rhack/commit/25b757e3d4edf0c806668972204489adaee99d5c))
    - Update workflows ([`bc22396`](https://github.com/simonsan/rhack/commit/bc2239651306548bc3432e42691e655f54ef6d28))
    - Update code blocks in Readme ([`26e7ce3`](https://github.com/simonsan/rhack/commit/26e7ce3711310b7dbd1088c3a029ffa0c532b310))
    - Update crates.io badge ([`df08e67`](https://github.com/simonsan/rhack/commit/df08e67955a92272b20a6c44de348a2ad5281693))
    - Update Cargo.lock ([`61d0df2`](https://github.com/simonsan/rhack/commit/61d0df241ee7cdb5dc8e2a1dbb09fe76f952def6))
</details>

## v0.1.1 (2023-05-19)

### Chore

 - <csr-id-6f61c97035b4603cb4a023dd35404589d9477c9c/> enable colored help screen
   Probably personal preference, but I find it easier if commands have 
   colored help as it is easier to distinguish the different sections of 
   the help screen.
   People who do not want colors in CLIs almost always have NO_COLOR
   set anyway, so this won't affect those people, as clap follows the no
   color specification: https://no-color.org/

### New Features

 - <csr-id-2018d48527e2992eef32b856a9755665bc76eafb/> use the workspace manifest root
   When using `cargo locate-project`, specify the `--workspace` option.
   This will provide a different result when within a workspace, providing
   the path to the top-level `Cargo.toml` file instead of the one for the
   workspace member. Patch changes to workspace member TOML files will not
   alter builds and will display a warning instead.
   
   For projects that do not use workspaces, the output of this command is
   identical to the previous version.

### Refactor

 - <csr-id-a5bfd3c8eae40ad5b39be2eaed4f45ee5c716ccb/> fix missing comma

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 783 calendar days.
 - 783 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Rename to `cargo-rhack` ([`8024228`](https://github.com/simonsan/rhack/commit/802422875be35d996abfe193c4eb2ff9ce61961e))
    - Create cargo subcommand ([`8558474`](https://github.com/simonsan/rhack/commit/8558474e421aebacc5aca78c3c0f613fad82f414))
    - Rename to cargo-rhack for cargo subcommand usage ([`e05a892`](https://github.com/simonsan/rhack/commit/e05a8929eb26c2ca748d4b8dd4d208dc51ff7fa4))
    - Remove dbg! ([`c3c76c0`](https://github.com/simonsan/rhack/commit/c3c76c0d5978defb62a72c0bcb5c403fecfa7f68))
    - Edit test comment ([`61212f0`](https://github.com/simonsan/rhack/commit/61212f0001501df2ed240ce60027b2d41b3d2a8e))
    - Remove unused imports ([`53c69b7`](https://github.com/simonsan/rhack/commit/53c69b7ca200bb72095ab8d0ab0a03cec2603d62))
    - Fix undo ([`8725831`](https://github.com/simonsan/rhack/commit/87258313bb45382611fc2842052079870dd9d88e))
    - Update dependencies and Clippy lint fixes ([`50a86a6`](https://github.com/simonsan/rhack/commit/50a86a6e7b761510ac72d23dcfdaec3c46efb4e6))
    - Merge pull request #8 from asaaki/fix/clap3-beta5 ([`e1bc681`](https://github.com/simonsan/rhack/commit/e1bc681dd91e4cb1164632a8bf768e394b465fec))
    - Fix issues and clippy recommendations ([`695090c`](https://github.com/simonsan/rhack/commit/695090ce8079778cf59de2c6f68715992e4a1642))
    - Update dependency to latest versions ([`a211a21`](https://github.com/simonsan/rhack/commit/a211a21e4738c69d056107181cb6d21b1e13c261))
    - Fix breaking changes of clap v3's beta5 ([`c67a4e7`](https://github.com/simonsan/rhack/commit/c67a4e79f56f7e663125341dac80d87f7f0d29a8))
    - Merge pull request #6 from alexander-jackson/feat/use-workspace-root ([`5381a73`](https://github.com/simonsan/rhack/commit/5381a730c58652642a0e8eaabca9abf21cbb542d))
    - Merge pull request #5 from SirWindfield/patch-1 ([`440cda3`](https://github.com/simonsan/rhack/commit/440cda36a3be867e974e289d38ec15f12832f12d))
    - Fix missing comma ([`a5bfd3c`](https://github.com/simonsan/rhack/commit/a5bfd3c8eae40ad5b39be2eaed4f45ee5c716ccb))
    - Use the workspace manifest root ([`2018d48`](https://github.com/simonsan/rhack/commit/2018d48527e2992eef32b856a9755665bc76eafb))
    - Enable colored help screen ([`6f61c97`](https://github.com/simonsan/rhack/commit/6f61c97035b4603cb4a023dd35404589d9477c9c))
    - Update installation instructions ([`30c6a8f`](https://github.com/simonsan/rhack/commit/30c6a8fac53c83db6a594a29d10dc0209e955a9a))
    - Merge pull request #2 from bradfier/document-aur-package ([`478b027`](https://github.com/simonsan/rhack/commit/478b0279beff8a229899d98349888b6a8a0e6acb))
    - Add an scdoc format Manpage and Makefile ([`96dfb5e`](https://github.com/simonsan/rhack/commit/96dfb5ebe048946b13ad272c155ea94bf486e300))
    - Add mention of AUR package to README ([`d3ef905`](https://github.com/simonsan/rhack/commit/d3ef905df996ac347b1c7eac76942a21d28d5046))
    - Update installation instructions ([`34b2d2d`](https://github.com/simonsan/rhack/commit/34b2d2d41d583241c1f4573c8c3ae7d7adb14fdc))
</details>

## v0.1.0 (2021-03-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release over the course of 28 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1](https://github.com/simonsan/rhack/issues/1)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/simonsan/rhack/issues/1)**
    - Modularize shared operations for both ([`68175b2`](https://github.com/simonsan/rhack/commit/68175b2eb6bccf1ac78c17b73ff482c985fcdd76))
 * **Uncategorized**
    - Rename deb package name ([`7008b8b`](https://github.com/simonsan/rhack/commit/7008b8b4c31c54133c475551272c4f053de6d891))
    - Rename release banaries names ([`47c196d`](https://github.com/simonsan/rhack/commit/47c196d6c83f54bf1d0ea1c039371ebe33fc9d93))
    - Add release workflow ([`a7fca58`](https://github.com/simonsan/rhack/commit/a7fca5839f58dbb28e37aed299c930da7abbe622))
    - Add LICENSE ([`baca452`](https://github.com/simonsan/rhack/commit/baca4526b6c8379b11fb5da5edaa78e2bbfd719a))
    - Init workflows ([`54b32fd`](https://github.com/simonsan/rhack/commit/54b32fd7011b1a43ee9eb6c8d6b0d744ee8b9868))
    - Drop temp crates from the table ([`e9b8114`](https://github.com/simonsan/rhack/commit/e9b8114e6d617734f83f2985fb8dc61b67b0c9c6))
    - Handle failure from serde_json ([`affa96e`](https://github.com/simonsan/rhack/commit/affa96ea874056a24b532e2109b7b559ccd3d243))
    - Add TODO ([`04adc21`](https://github.com/simonsan/rhack/commit/04adc2110c51fcbe1ae3f95915f14e03df78977a))
    - Enable to update patch table ([`17629af`](https://github.com/simonsan/rhack/commit/17629af469cf62ff12aebe5ef60dd2b9d0fcbf63))
    - Make it possible to emit verbose debug ([`7c96423`](https://github.com/simonsan/rhack/commit/7c96423f41d52578748855d1a197ff87d6c7270e))
    - Enable to insert new table to manifest ([`1e967fa`](https://github.com/simonsan/rhack/commit/1e967fa391ce0946c1305c49131208d095195aaf))
    - Update README ([`27921c7`](https://github.com/simonsan/rhack/commit/27921c7572479c54cd1e816baaf82eecc4d12db6))
    - Make it possible to copy directory recursively ([`b4e0ca2`](https://github.com/simonsan/rhack/commit/b4e0ca2e8ac2101c939f94d225732dfcea9c4b48))
    - Update README ([`70a0715`](https://github.com/simonsan/rhack/commit/70a071534898dd4896d9e266ac816f8202660ec5))
    - Use cargo locate-project to detemine the path ([`0c5ec24`](https://github.com/simonsan/rhack/commit/0c5ec24e6901940f27b8be6248536c9d09236343))
    - Enable to determine the path to registry path ([`844a3d2`](https://github.com/simonsan/rhack/commit/844a3d2139143bcbd1e045a56181a76181820559))
    - Update README ([`f5f9b0b`](https://github.com/simonsan/rhack/commit/f5f9b0b331f640fee071c0e207b88897ada8cb1d))
    - Infer the path to home directory ([`7d6ffac`](https://github.com/simonsan/rhack/commit/7d6ffac84320a1ac9007dc84eade0425622c2a20))
    - Enable to copy depencencies ([`d4fe778`](https://github.com/simonsan/rhack/commit/d4fe77821206060ade0b3957ea750a82e226965d))
    - Add sub-commands ([`1b4ea4b`](https://github.com/simonsan/rhack/commit/1b4ea4bb84817f44759270e6bb4d17468a3a265c))
    - Add README ([`026600e`](https://github.com/simonsan/rhack/commit/026600eea9c31479e741e8689ad55c4254fafb77))
    - Init ([`bec3404`](https://github.com/simonsan/rhack/commit/bec3404645724bb6324f0feb2818db49db94461a))
</details>

