TexOptions = --lualatex --interaction=nonstopmode --halt-on-error --output-directory=build

Target = day7

all: build/$(Target).pdf

build/$(Target).pdf: FORCE | build
	latexmk $(TexOptions) $(Target).tex

FORCE:

build:
	mkdir -p build/

clean:
	rm -rf build