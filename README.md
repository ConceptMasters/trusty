# TRusty

## Pre-commit hooks
[Docs](https://pre-commit.com/)

`pre-commit` enables us to automate running various linters/formatters/checkers and verify their success. These checks will run against the whole repo
as part of GitHub actions and will block the PR if any check fails. It is also possible to run the hooks from the command line manually or "install"
them to automatically run as a pre-commit git hook.

First, make sure you're inside a python virtual environment (see "Setting up your python virtual environment" section below) and that poetry and docker are installed. The docker daemon must also be running.

### Setting up your python virtual environment
```shell
# Install pyenv and pyenv-virtualenv (example for Mac OS using brew)
brew install pyenv
brew install pyenv-virtualenv

# At this point I'd recommend updating your .bash_profile or .bashrc or whatever you use to include the following lines then re-sourcing your environment.
# If you don't do that, just run the lines in your active shell instead. That will also work.
export PYENV_ROOT="$HOME/.pyenv"
export PATH="$PYENV_ROOT/bin:$PATH"
export PATH="$PYENV_ROOT/shims:${PATH}"
eval "$(pyenv init -)"
eval "$(pyenv virtualenv-init -)"

# Install needed python version with pyenv
pyenv install 3.10.11

# Create a name for our virtual environment
# This will be of the format "trusty_<branch name with slashes replaced by dashes>"
NEW_VENV_NAME="trusty_$(git rev-parse --abbrev-ref HEAD | sed 's/\//-/g')"

# Create virtual environment with 3.10.11
# Note that the last argument is the name of our new virtual environment
pyenv virtualenv 3.10.11 "${NEW_VENV_NAME}"

# Active our new virtual environment
pyenv activate "${NEW_VENV_NAME}"

# Now python should be set to 3.10.11. Let's check...
if [[ "$(python --version)" == 'Python 3.10.11' ]]; then printf '\033[1;32mPYTHON VERSION SET TO 3.10.11\033[0m\n'; else printf '\033[1;31mFAILED TO SET PYTHON VERSION TO  3.10.11\033[0m\n'; fi

# Install static pre-commit version for consistency
pip install 'pre-commit==3.2.2'
```

### Tearing down your python virtual environment
```shell
# To get out of the virtual environment
pyenv deactivate

# To list created virtual environments
pyenv virtualenvs

# From the output of that last command we can do
pyenv virtualenv-delete <venv you want to delete>
```

### Running pre-commit hooks
```shell
# To install pre-commit hooks to run on git commit and git push
poetry run pre-commit install

# To manually run hooks against modified files
poetry run pre-commit

# To manuall run hooks against all files in the repo
poetry run pre-commit run -a
```
