#!/bin/bash

error=0

diff_project() {
  local project=$1
  cp "$project/public-api.txt" "$project/old-public-api.txt"
  cargo public-api -p "$project" > "$project/public-api.txt"
  diff -u "$project/old-public-api.txt" "$project/public-api.txt" > /dev/null
  local current_error=$?
  rm "$project/old-public-api.txt"
  if [[ $current_error -ne 0 ]]
  then
    echo "FAIL: Public API changed! To bless, git commit changed public-api files and increase semvers."
  fi
  return $current_error
}

for project in dotfiles_*/
do
    project=${project%/}
    diff_project "$project"
    current_error=$?
    error=$((error + current_error))
    echo "$error errors total, $current_error"
done

exit $((error))
