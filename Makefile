.PHONY: gen

gen:
	buf generate --include-imports --include-wkt buf.build/envoyproxy/envoy:80530fd0a32e242327c684cfe262d88e0f5eacbb
	patch -p0 < patches/001-fix-conflict.patch
