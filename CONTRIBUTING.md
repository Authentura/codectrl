# Contributing to CodeCTRL

First of all, thank you for considering contributing to CodeCTRL, it's very
much appreciated. Here's some tips on how to make a good contribution to
CodeCTRL:

## 1. Please make sure you accurately describe your contribution.

Though we welcome your contribution to CodeCTRL, if you decide to create a pull
request, please be as clear as possible with the intent and the content of your
contribution. The title doesn't have to be insanely wordy, just enough to
describe what your PR adds/changes/fixes. The description of the PR should
contain more detail and link to any relevant issues that this PR addresses.

For example, the PR title "fix some things" is unhelpful, though the PR itself
may very well be worth merging. A title like "fix compilation issues on WASM
with new login form" is more helpful to describe the general intent of the PR.

## 2. Please make sure that you keep track of the automatic checks.

We have set up continuous integration on this repository to check for code
formatting issues and code linting issues. If one of these checks fails, please
make the necessary changes so that the checks pass. Don't worry if they don't
pass initially, we won't badger you to fix them but the PR won't be able to be
merged until they pass.

## Opening an issue

First of all thank you for taking the time to open an issue.
Before opening an issue, please be sure that your issue hasn't already been asked by someone else.

Here are a few things that will help us help resolve your issues:

- A descriptive title that gives an idea of what your issue refers to
- A thorough description of the issue, (one word descriptions are very hard to understand)
- Screenshots (if appropriate)
- Links (if appropriate)

## Submitting a pull request

0. Fork this repository
0. Clone the repository
0. Configure and install the dependencies: (See the [README](README.md) for more details)
0. Make sure the tests pass on your machine: `script/test`
0. Create a new branch: `git checkout -b my-branch-name`
0. Make your change, add tests, and make sure the tests still pass
0. Push to your branch and submit PR
0. Wait for your pull request to be reviewed and merged!

**⚠️spam PRs and strongly discouraged here and will be labelled as spam or invalid, one can also contribute here by checking for spam PRs in this repo.**
