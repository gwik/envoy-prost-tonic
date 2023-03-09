.PHONY: gen buf_gen

gen: buf_gen patch

buf_gen:
	buf generate --include-imports --include-wkt buf.build/envoyproxy/envoy:80530fd0a32e242327c684cfe262d88e0f5eacbb

PATCHES = patches/*.patch

.PHONY: patch $(PATCHES)

$(PATCHES):
	patch -p1 < $@

patch: $(PATCHES)
