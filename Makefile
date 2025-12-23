TEST_NAME=test_create_thread

test:
	cargo test tests::$(TEST_NAME) -- --nocapture