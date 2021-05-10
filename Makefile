NAPI_VERSION := 6
NPM := $(CURDIR)/node_modules
NODEMON := $(CURDIR)/node_modules/.bin/nodemon
JEST := $(CURDIR)/node_modules/.bin/jest
LIBDIR := $(CURDIR)/lib/v$(NAPI_VERSION)
LIB := $(LIBDIR)/index.node
GIT_TAG = $(shell git describe)
PACKAGE_VERSION = $(shell npm run env | grep npm_package_version | cut -d '=' -f 2)
NPM_VERSION = $(shell npm view skia-canvas version)
.PHONY: build run test check clean visual package publish

$(NPM):
	npm install

$(LIB): $(NPM)
	npm run build

build: $(LIB)
	@npm run build
	@echo build complete

test: $(LIB)
	@$(JEST)

visual: $(LIB)
	@$(NODEMON) test/visual -w native/index.node -w test/visual -e js,html

check:
	cargo check

clean:
	@rm $(LIB)
	@rmdir $(LIBDIR)

distclean:
	cargo clean

run: $(LIB)
	@npm run build
	@node check.js

preview: run
	@open -a Preview.app out.png
	@open -a "Visual Studio Code"

# stats:
# 	@GIT_TAG = $(shell git describe)
# 	@PACKAGE_VERSION = $(shell npm run env | grep npm_package_version | cut -d '=' -f 2)
# 	@NPM_VERSION = $(shell npm view skia-canvas version)
# 	@echo "NPM Version: $(NPM_VERSION)"
# 	@echo "Skia Canvas: $(PACKAGE_VERSION)"

# release:
# 	@if [[ `git status -s package.json` != "" ]]; then echo "Commit changes to package.json first:"; git --no-pager diff package.json; exit 1; fi
# 	@if [[ `git cherry -v` != "" ]]; then echo "Unpushed commits:"; git --no-pager log --branches --not --remotes; exit 1; fi
# 	@if [[ $(GIT_TAG) =~ ^v$(PACKAGE_VERSION) ]]; then echo "Already published $(GIT_TAG)"; exit 1; fi
# 	@echo "NPM Version: $(NPM_VERSION)"
# 	@echo "Skia Canvas: $(PACKAGE_VERSION)"
# 	@echo "   Bindings: $(RUST_SKIA_VERSION)"
# 	@echo
# 	@/bin/echo -n "Update release -> v$(PACKAGE_VERSION)? [y/N] "
# 	@read line; if [[ $$line = "y" ]]; then echo "Pushing tag to github..."; else exit 1; fi
# 	git tag -a v$(PACKAGE_VERSION) -m v$(PACKAGE_VERSION)
# 	git push origin --tags
# 	@echo "Next: run 'make publish' when travis’s build completes..."


GIT_TAG = $(shell git describe)
PACKAGE_VERSION = $(shell npm run env | grep npm_package_version | cut -d '=' -f 2)
NPM_VERSION = $(shell npm view skia-canvas version)

release:
	@if [[ `git status -s package.json` != "" ]]; then printf "Commit changes to package.json first:\n\n"; git --no-pager diff package.json; exit 1; fi
	@if [[ `git cherry -v` != "" ]]; then printf "Unpushed commits:\n\n"; git --no-pager log --branches --not --remotes; exit 1; fi
	@if [[ $(GIT_TAG) =~ ^v$(PACKAGE_VERSION) ]]; then printf "Already published $(GIT_TAG)"; exit 1; fi
	@echo
	@echo "Git Tag: $(GIT_TAG)"
	@echo "NPM Version: $(NPM_VERSION)"
	@echo "Skia Canvas: $(PACKAGE_VERSION)"
	@echo
	@/bin/echo -n "Update release -> v$(PACKAGE_VERSION)? [y/N] "
	@read line; if [[ $$line = "y" ]]; then echo "Pushing tag to github..."; else exit 1; fi
	git tag -a v$(PACKAGE_VERSION) -m v$(PACKAGE_VERSION)
	git push origin --tags
	@echo "Next: publish the release on github to submit to npm"
