PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)
SITE_PKG=/opt/homebrew/lib/python3.9/site-packages

# MODULAR CORE STUFF

build:
	$(PYTHON) -m build --sdist

FORCE:

clean:
	rm -rf build dist src/gitnu.egg-info .pytest_cache src/gitnu/__pycache__ test/__pycache__

build-full:
	$(PYTHON) -m build

clear-site-packages:
	# if installed by pip, this will be a directory
	# if installed by make dev, this will be a symlink to src/gitnu
	rm -rf $(SITE_PKG)/gitnu

# PIP STUFF

pip-uninstall:
	make clear-site-packages
	pip uninstall --yes gitnu

pip-install:
	pip install gitnu

fresh-pip-install:
	make pip-uninstall
	make pip-install

# derivative stuff

upload:
	make clean
	make build-full
	twine upload --username "brew4k" dist/*

dev: # use with caution
	@pip show gitnu || pip install .
	make clear-site-packages
	ln -sf $$PWD/src/gitnu $(SITE_PKG)

# aliases

i:
	make pip-install

fi:
	make fresh-pip-install

u:
	make pip-uninstall

test: FORCE
	@echo "Running all tests..."
	$(PYTHON) -m unittest discover

pytest:
	pytest

pt:
	pytest

test-shell:
	pytest test/test_shell.py

ts:
	make test-shell
