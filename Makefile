CC = rustc
CFLAGS = --edition 2021 -C panic=abort -C linker=gcc -C opt-level=3 -C lto 
LDFLAGS = -lc
TARGET = brainsuck
SOURCE = interpreter.rs

# Default target
all: $(TARGET)

# Build the main target
$(TARGET): $(SOURCE)
	$(CC) $(CFLAGS) $(SOURCE) -o $(TARGET) $(LDFLAGS) --verbose

# Clean build artifacts
clean:
	rm -f $(TARGET) liblib.rlib

# Run the program
run: $(TARGET)
	./$(TARGET)

# Debug build (with debug symbols)
debug: CFLAGS += -g
debug: $(TARGET)

# Release build (optimized)
release: CFLAGS += -C opt-level=3
release: $(TARGET)

# Install to system (optional)
install: $(TARGET)
	cp $(TARGET) /usr/local/bin/

# Uninstall from system
uninstall:
	rm -f /usr/local/bin/$(TARGET)

# Force rebuild
rebuild: clean all

# Check if source file exists
check:
	@if [ ! -f $(SOURCE) ]; then \
		echo "Error: $(SOURCE) not found!"; \
		exit 1; \
	fi

# Help
help:
	@echo "Available targets:"
	@echo "  all      - Build the program (default)"
	@echo "  clean    - Remove build artifacts"
	@echo "  run      - Build and run the program"
	@echo "  debug    - Build with debug symbols"
	@echo "  release  - Build optimized version"
	@echo "  install  - Install to /usr/local/bin"
	@echo "  uninstall- Remove from /usr/local/bin"
	@echo "  rebuild  - Clean and build"
	@echo "  check    - Check if source file exists"
	@echo "  help     - Show this help"

.PHONY: all clean run debug release install uninstall rebuild check help
