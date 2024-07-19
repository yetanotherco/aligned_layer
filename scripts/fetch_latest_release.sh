#!/bin/bash

# The script fetches the latest release tag from the repository and checks out that tag.
git fetch --tags
latesttag=$(git describe --tags $(git rev-list --tags --max-count=1))
echo checking out ${latesttag}
git checkout ${latesttag}
