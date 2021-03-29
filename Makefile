PREFIX?=/usr/local
BINDIR?=$(PREFIX)/bin
MANDIR?=$(PREFIX)/share/man

all: rhack rhack.1

rhack:
	cargo build --release

test:
	cargo test

rhack.1: rhack.1.scd
	scdoc < $< > $@

clean:
	cargo clean
	rm -f rhack.1

install: all
	mkdir -p $(DESTDIR)/$(BINDIR) $(DESTDIR)/$(MANDIR) $(DESTDIR)/$(MANDIR)/man1
	install -m755 target/release/rhack $(DESTDIR)/$(BINDIR)/rhack
	install -m644 rhack.1 $(DESTDIR)/$(MANDIR)/man1/rhack.1

.PHONY: all clean install
