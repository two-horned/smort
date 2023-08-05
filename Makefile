DEST = ~/.local/bin
NAME = smort

all: build install clean

clean:
	cargo c

build:
	cargo b

install:
	mv target/debug/${NAME} ${DEST}/${NAME}

uninstall:
	rm ${DEST}/${NAME}
