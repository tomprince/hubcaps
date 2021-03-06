# 0.3.12

* fixed issue with persistence of repo term permission

# 0.3.11

* fixed PUT vs PATCH issue with repo team adds

# 0.3.10

* add ability to add team to repository

# 0.3.9

* add support for fetching a single repo by name

# 0.3.8

* add support for org repo creation

# 0.3.7

* add support for updating a repository branch's protection

# 0.3.6

* added `per_page` to various repo list builder interfaces
* fixed org list builder's type filter to use org repo type

# 0.3.5

* hubcaps::git::GitFile's now have an optional url because commits types don't
  have urls.

# 0.3.4

* added git tree and blob fetching interfaces

# 0.3.3

* added org repos interface

# 0.3.2

* use error_chain to generate error types
* add support for posting issue comments [#71](https://github.com/softprops/hubcaps/pull/71)
* add support for repo teams
* add team permissions
* add iter support to branches, repos, pulls, and teams

# 0.3.1

* fix order of Iter traversal

# 0.3.0

* added support for repo hooks
* `Github::new` now takes an owned reference to a hyper::Client. this makes it possible
  to pass a github instance into a threaded context.
* upgrade to serde 0.9 (and now uneeded build.rs machinery)
* sizable code restructure to support scalability in future growth. move foo.rs modules to foo/mod.rs files. moved respective rep.rs reps into mods
* the effect of the above is that everything may no longer be accessible via the top level `hubcaps` module. For instance, in the past you would be able to to access `hubcaps::Pull` directly, now you would access it via is api category `hubcaps::pulls::Pull`.
* update hyper to 0.10. the implications are that you now need to bring your own tls-configured hyper client

# 0.2.8

* expose more pub fields on pull commits

# 0.2.7

* added support for listing pull commits
* added support for returning an iterator over all pull commits

# 0.2.6

* added support for listing issue/pull comments
* added support for listing review comments

# 0.2.5

* added support for search issues api
* add partial support for new Iter type which serves as an transparent iterator over pages of results

# 0.2.4

Improved coverage of pull request api

* Pull.body is now represented as an `Option<String>`
* Pull.assignees is now deserialized
* added `pull.files()` which returns a `Vec<FileDiff>`

# 0.2.3

* added support for repo creation [#38](https://github.com/softprops/hubcaps/pull/38)
* upgrade syntex build dependency to 0.35

# 0.2.2

* upgrade to [hyper 0.8](https://github.com/hyperium/hyper/blob/master/CHANGELOG.md#v080-2016-03-14)
* upgrade syntex build dependency to 0.33

# 0.2.1 (2016-04-09)

* Added support for listing organization repositories [via @carols10cents](https://github.com/softprops/hubcaps/pull/29)
* Fixed deserialization issue related to error response in release api calls [issue #31](https://github.com/softprops/hubcaps/issues/31)

# 0.2.0

Many changes were made to transition into using serde as a serialization backend and to focus on making interfaces more consistent across the board. A more flexible interface for authenticating requests was added as well as a new interface for requesting organization repository listings. Relevant itemized changes are listed below.

* port serialization from `rustc-serialize` to `serde`!
* as a result of the serde port, `Error::{Decoding, Encoding}` which were wrappers around rustc-serialize error types, were removed and replaced with a unified `Error::Codec` which wraps serde's error type
* renamed `hubcaps::statuses::State` to `hubcaps::StatusState`
* added `payload` field to `hubcaps::Deployment` represented as a `serde_json::Value`
* added `content_type` field to `hubcaps::GistFile` represented as `String`
* added `truncated` field to `hubcaps::Gist` represented as an `bool` and updated `truncated` field of `hubcaps::GistFile` to be `Option<bool>` (this field is omitted in gist listing responses)
* introduces `hubcaps::Credentials` as the means of authenticating with the Github api. A `Credentials` value is needed to instantiate a `Github` instance. This is a breaking change from the previous `Option<String>` token api, with a more flexible set options. `hubcaps::Credentials::{None, Token, Client}`. `hubcaps::Credentials` implements `Default` returning `hubcaps::Credentials::None`
* `hubcaps::Error` enum now implements `std::error::Error`
* pull request and issue listing fn's now both take options structs. This is a breaking change.
* repo listing fn's now take option structs. This is a breaking change.
* gist listing fn's now take option structs. This is a breaking change.
* added support for fetching organization repoistory listings [via @carols10cents](https://github.com/softprops/hubcaps/pull/28)

# 0.1.1

* DeploymentStatusOptions now have an optional field for `target_url` and `description`

# 0.1.0

* initial release
