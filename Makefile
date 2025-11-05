SRC_DIR := examples/dot
OUT_DIR := examples/output

DOT_FILES := $(shell find $(SRC_DIR) -name '*.dot')

DIRS := $(shell find $(SRC_DIR) -type d)

PNG_FILES := $(DOT_FILES:$(SRC_DIR)/%.dot=$(OUT_DIR)/%.png)

png: dirs $(PNG_FILES)

dirs:
	@echo "Creating folder structure in $(OUT_DIR)..."
	@for d in $(DIRS); do \
		mkdir -p "$(OUT_DIR)/$${d#$(SRC_DIR)/}"; \
	done
	@rm -rf $(OUT_DIR)/examples

$(OUT_DIR)/%.png: $(SRC_DIR)/%.dot
	@echo "Processing '$<' -> '$@'"
	@mkdir -p "$(dir $@)"
	@dot -Tpng "$<" -o "$@"

clean-png:
	@echo "Cleaning $(OUT_DIR)..."
	rm -rf $(OUT_DIR)
