# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.2.1 - 2025-11-01
#### Bug Fixes
- use ssh for cloning - (caceca6) - Billie Thompson
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- push the tags - (a627abe) - Billie Thompson
- use ssh to push - (dd6d545) - Billie Thompson
- specify remote more explicitly - (de978b6) - Billie Thompson
- create ssh dir - (6e340e4) - Billie Thompson
- add missing deps - (c73f690) - Billie Thompson
- just push from the rust image - (f5c6038) - Billie Thompson
- correct rustup version - (079fd33) - Billie Thompson
- remove ssh key from the cog run - (3f66be4) - Billie Thompson
- try another release method - (a939db7) - Billie Thompson
- re-add user setting - (5a898cf) - Billie Thompson
- try a different approach to pushing changes - (6a163f1) - Billie Thompson
- fix indentation - (bb370ff) - Billie Thompson
- try using ssh^ - (a86e420) - Billie Thompson
- push from cog rather than elsewhere - (4cfe093) - Billie Thompson
- add autofix - (10ab57a) - Billie Thompson
- add release to ci triggers and add changelog to prevent looping - (53fb659) - Billie Thompson
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (1ad3a68) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (c23bbfa) - Billie Thompson
- (**version**) v0.2.1 - (69bbf07) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (6314b9d) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (9a25c1c) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (624c0b7) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-11-01
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-11-01
#### Bug Fixes
- use ssh for cloning - (caceca6) - Billie Thompson
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- use ssh to push - (dd6d545) - Billie Thompson
- specify remote more explicitly - (de978b6) - Billie Thompson
- create ssh dir - (6e340e4) - Billie Thompson
- add missing deps - (c73f690) - Billie Thompson
- just push from the rust image - (f5c6038) - Billie Thompson
- correct rustup version - (079fd33) - Billie Thompson
- remove ssh key from the cog run - (3f66be4) - Billie Thompson
- try another release method - (a939db7) - Billie Thompson
- re-add user setting - (5a898cf) - Billie Thompson
- try a different approach to pushing changes - (6a163f1) - Billie Thompson
- fix indentation - (bb370ff) - Billie Thompson
- try using ssh^ - (a86e420) - Billie Thompson
- push from cog rather than elsewhere - (4cfe093) - Billie Thompson
- add autofix - (10ab57a) - Billie Thompson
- add release to ci triggers and add changelog to prevent looping - (53fb659) - Billie Thompson
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (c23bbfa) - Billie Thompson
- (**version**) v0.2.1 - (69bbf07) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (6314b9d) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (9a25c1c) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (624c0b7) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-11-01
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-11-01
#### Bug Fixes
- use ssh for cloning - (caceca6) - Billie Thompson
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- correct rustup version - (079fd33) - Billie Thompson
- remove ssh key from the cog run - (3f66be4) - Billie Thompson
- try another release method - (a939db7) - Billie Thompson
- re-add user setting - (5a898cf) - Billie Thompson
- try a different approach to pushing changes - (6a163f1) - Billie Thompson
- fix indentation - (bb370ff) - Billie Thompson
- try using ssh^ - (a86e420) - Billie Thompson
- push from cog rather than elsewhere - (4cfe093) - Billie Thompson
- add autofix - (10ab57a) - Billie Thompson
- add release to ci triggers and add changelog to prevent looping - (53fb659) - Billie Thompson
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (69bbf07) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (6314b9d) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (9a25c1c) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (624c0b7) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-11-01
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-10-31
#### Bug Fixes
- use ssh for cloning - (caceca6) - Billie Thompson
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- correct rustup version - (079fd33) - Billie Thompson
- remove ssh key from the cog run - (3f66be4) - Billie Thompson
- try another release method - (a939db7) - Billie Thompson
- re-add user setting - (5a898cf) - Billie Thompson
- try a different approach to pushing changes - (6a163f1) - Billie Thompson
- fix indentation - (bb370ff) - Billie Thompson
- try using ssh^ - (a86e420) - Billie Thompson
- push from cog rather than elsewhere - (4cfe093) - Billie Thompson
- add autofix - (10ab57a) - Billie Thompson
- add release to ci triggers and add changelog to prevent looping - (53fb659) - Billie Thompson
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (6314b9d) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (9a25c1c) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (624c0b7) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-10-31
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-10-31
#### Bug Fixes
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- add autofix - (10ab57a) - Billie Thompson
- add release to ci triggers and add changelog to prevent looping - (53fb659) - Billie Thompson
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (9a25c1c) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (624c0b7) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-10-31
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-10-31
#### Bug Fixes
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- add release to ci triggers and add changelog to prevent looping - (53fb659) - Billie Thompson
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (624c0b7) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-10-31
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-10-31
#### Bug Fixes
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (3a96251) - Solace System Renovate Fox [bot]
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-10-31
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-10-31
#### Bug Fixes
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth
#### Miscellaneous Chores
- (**version**) v0.2.1 - (d438f84) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-10-31
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.1 - 2025-10-31
#### Bug Fixes
- only release on stable - (fee10ee) - Billie Thompson
- add more cargo components - (ca1c14b) - Billie Thompson
- add rustfmt - (062ac01) - Billie Thompson
#### Continuous Integration
- install binstall with the curl script - (deb90c1) - Billie Thompson
- remove old pipeline - (c7e7a25) - Billie Thompson
- add a remote for the push - (35d487c) - Billie Thompson
- remove restriction on where test flow runs - (92bde4a) - Billie Thompson
- back out sccache - didn't work - (7b212c5) - Billie Thompson
- try different plugin source - (bbb3c92) - Billie Thompson
- remove unused variable - (1b23076) - Billie Thompson
- try enabling ssl - (0f31011) - Billie Thompson
- remove uneeded entrypoint - (80fdae5) - Billie Thompson
- give prs an unprivilaged route - (a5cd6e7) - Billie Thompson
- disable caching from prs - (561d597) - Billie Thompson
- correct command to match docs - (8670359) - Billie Thompson
- specify bash - (30a899a) - Billie Thompson
- make dependencies explicit - (78b7f48) - Billie Thompson
- correct source - (d9a720b) - Billie Thompson
- save cache changes - (3edc7f1) - Billie Thompson
- try other image - (ee10aff) - Billie Thompson
- add caching - (d926949) - Billie Thompson
- remove ssh - (ab0333a) - Billie Thompson
- add private key for pushing - (92537c6) - Billie Thompson
- clone with auth - (875c026) - Billie Thompson
- pull tags for release - (d3260a4) - Billie Thompson
- set th euser manually - (6a5d8b5) - Billie Thompson
- add deploy workflow - (0a1e279) - Billie Thompson
- set a git-user-name for the repo - (351b9d0) - Billie Thompson
- set cargo registry token - (5f50339) - Billie Thompson
- push tags - (62f5721) - Billie Thompson
- depth should be 0 - (5c4dc20) - Billie Thompson
- correct the depth query - (4ea5895) - Billie Thompson
- push changelog changes - (74efa2d) - Billie Thompson
- fan in after tests - (ab657ca) - Billie Thompson
- try adding release step - (38e3fc3) - Billie Thompson
- add audit step - (4173dc0) - Billie Thompson
- test woodpacker - (457ce94) - PurpleBooth

- - -

## v0.2.0 - 2025-10-31
#### Continuous Integration
- add parallel execution for validation tasks in concourse pipeline - (42c8310) - Billie Thompson
- update concourse pipeline to use uppercase 'CRITICAL' for fail-on parameter - (7dffe28) - Billie Thompson
- remove redundant image specifications in concourse pipeline - (78c8c5a) - Billie Thompson
- reduce resource check interval from 24h to 1h - (9d4a89d) - Billie Thompson
- update concourse pipeline job dependencies - (ac753c2) - Billie Thompson
- add verify-rust-env job and replace docker-rust with ci-rust-env - (017dc0c) - Billie Thompson
- enable tag fetching for Concourse pipeline - (357de8a) - Billie Thompson
- add git author and committer details for renovate bot - (44b60c1) - Billie Thompson
- update GAR resource credentials for docker images - (8b209d5) - Billie Thompson
- update CI runtime image to custom artifact registry repository - (67d24da) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (bd83f44) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (e159ff5) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (0fb92c7) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (38a8287) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (aa5957d) - Solace System Renovate Fox
- replace grype with trivy in Concourse pipeline tasks - (12e0e7a) - Billie Thompson
- remove post-bump git push hooks from cog.toml - (810dd36) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (f095b8d) - Billie Thompson
- remove empty test-and-tag workflow file - (d7a0596) - Billie Thompson
- migrate from Forgejo workflows to Concourse CI configuration - (a545b71) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (85cb09d) - Solace System Renovate Fox [bot]

- - -

## v0.2.0 - 2025-07-10
#### Bug Fixes
- update random range to include 9 in suffix generation - (2c72dce) - Billie Thompson
#### Continuous Integration
- run less often - (f9ed88d) - PurpleBooth
- set a lower retension - (dba3c0d) - PurpleBooth
- use library preset - (b6f1743) - PurpleBooth
#### Features
- add random numeric suffix to readable name generator - (eb18e60) - Billie Thompson

- - -

## v0.1.5 - 2025-05-11
#### Bug Fixes
- mark extern block as unsafe in doc test macro - (206066d) - Billie Thompson (aider)
#### Build system
- Update repository URL in Cargo.toml - (1c3541e) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update https://code.forgejo.org/actions/cache digest to 5a3ec84 - (1a2f336) - Solace System Renovate Fox
- **(deps)** update https://code.forgejo.org/actions/cache digest to 0c907a7 - (128f8ec) - Solace System Renovate Fox
- update Rust edition, dependencies, and CI workflows - (b9b17d4) - Billie Thompson
#### Style
- Reorganize and update lint attributes in lib.rs - (97fab7e) - Billie Thompson

- - -

## v0.1.4 - 2025-02-10
#### Bug Fixes
- Test release - (f4da23f) - Billie Thompson
#### Continuous Integration
- create forgejo release - (62c93f6) - Billie Thompson
- Configure Renovate to widen dependency ranges - (a35da06) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin dependencies - (545429c) - Solace System Renovate Fox

- - -

## v0.1.3 - 2025-02-09
#### Bug Fixes
- Swap test RNG to ChaCha and bump rand to v0.9.0 - (0de9a15) - Billie Thompson
#### Continuous Integration
- move to forgejo - (ff090c6) - Billie Thompson
#### Miscellaneous Chores
- unused file - (449504b) - PurpleBooth

- - -

## v0.1.2 - 2024-08-14
#### Bug Fixes
- Test release as it's been some time - (e2a09de) - Billie Thompson
#### Continuous Integration
- **(deps)** bump PurpleBooth/versio-release-action from 0.1.8 to 0.1.13 - (bc73352) - dependabot[bot]
- **(deps)** bump PurpleBooth/versio-release-action from 0.1.7 to 0.1.8 - (fac0c4d) - dependabot[bot]
- Bump versions in cargo toml - (a8f98f5) - Billie Thompson
- Switch to cog for releases - (ad3effe) - Billie Thompson
- Add cog.toml - (45225c2) - Billie Thompson
#### Documentation
- Document what looks like a panic - (a4c049d) - Billie Thompson
#### Miscellaneous Chores
- Add renovate.json - (ad2d54f) - renovate[bot]

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).