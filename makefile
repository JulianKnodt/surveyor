create_page:
	cargo run --bin generate_page -- --output docs/test_info.md
	cd docs; python3 gen.py
	rm temp.rs
