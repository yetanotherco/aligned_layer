#!/bin/bash

# The script fetches the latest release tag from the repository and checks out that tag.
latesttag=$(git describe --tags)
echo checking out ${latesttag}
git checkout ${latesttag}
