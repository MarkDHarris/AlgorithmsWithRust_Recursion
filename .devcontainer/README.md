# DEVCONTAINER

## VSCODE

- Open the repo in Visual Studio Code
- Click to open the repo in the .devcontainer.  If not auto-detected, use `CTRL+SHIFT+P` and select `Dev Containers: Rebuild and Reopen in Container`
- Work with the repo

## MANUAL USAGE

- Navigate to the root of the directory: `cd ./AlgorithmsWithRust_SortingAndSearching`
- Select either the fedora or ubuntu base OS and build the container.
  - use `./.devcontainer/scripts/build-fedora.sh` or `./.devcontainer/scripts/build-ubuntu.sh`
- Manually run the container: `./.devcontainer/scripts/run.sh`