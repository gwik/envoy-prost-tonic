.PHONY: gen buf_gen patch

gen: buf_get patch

buf_gen:
	buf generate --include-imports --include-wkt buf.build/envoyproxy/envoy:80530fd0a32e242327c684cfe262d88e0f5eacbb

patch:
	patch -p1 < patches/001-fix-conflict.patch
