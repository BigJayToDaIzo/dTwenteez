# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### MVP 0.0.1

   - [x] Interactive mode ask user for roll input until 'q' is passed
        - ex: "d20" "6d6+1 a" "2d10-1 d" "h" "q"
   - [x] Accept cli args
   - [x] Give user interactive quit option
   - [x] Give user interactive help option
   - [x] Handle input errors gracefully
   - [x] Display roll total, and individual rolls in brackets
   - [x] 85+% code coverage in test framework
   - [ ] Remove ambiguity of advantage / disadvantage (this means vastly different things in many dice based game systems)
    - drop highest and drop lowest refactor
   - [ ] Deploy to crates.io
### MVP 0.0.2

   - [ ] logging to text file
   - [ ] read input scripts for predetermined roll sets
   - [ ] a TUI built with [ratatui](https://ratatui.rs)
### MVP 0.0.3

   - [ ] real logging / database system 
   - [ ] public api for client apps
### Task Checklist (moves to changelog when complete)

   - [x] Task 1: Implement README.md with roadmap
   - [x] Task 2: Display greeting
   - [x] Task 3: Capture user input
   - [x] Task 4: display results of roll
   - [x] Task 5: Ask for next roll/quit
   - [ ] Task 6: Refactor adv/disadv

### Added

- v1.1 Brazilian Portuguese translation.


## [0.0.1] - 2025-03-19

### Added

- This CHANGELOG file to hopefully serve as an evolving example of a
  standardized open source project CHANGELOG.

### Changed

- Use frontmatter title & description in each language version template
- Replace broken OpenGraph image with an appropriately-sized Keep a Changelog 
  image that will render properly (although in English for all languages)
- Fix OpenGraph title & description for all languages so the title and 
description when links are shared are language-appropriate

### Removed

- Trademark sign previously shown after the project description in version 
0.3.0

<!-- [unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.1.1...HEAD -->
<!-- [1.1.1]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.1.0...v1.1.1 -->
<!-- [1.1.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...v1.1.0 -->
<!-- [1.0.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.3.0...v1.0.0 -->
<!-- [0.2.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.0...v0.2.0 -->
<!-- [0.1.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.8...v0.1.0 -->
<!-- [0.0.2]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.1...v0.0.2 -->
<!-- [0.0.1]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.0.1 -->
