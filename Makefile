.PHONY: gen buf_gen clean

gen: buf_gen patch

buf_gen: clean
	buf generate --include-imports buf.build/envoyproxy/envoy:80530fd0a32e242327c684cfe262d88e0f5eacbb

clean:
	rm -rf gen

PATCHES = patches/*.patch

.PHONY: patch $(PATCHES)

$(PATCHES):
	patch -p1 < $@

patch: $(PATCHES)
