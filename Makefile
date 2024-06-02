build-js:
	@echo "Building JS"
	@cd node-binding && yarn build

run-js:
	@cd node-binding && node
