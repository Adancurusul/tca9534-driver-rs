/**
 * @file c_example.c
 * @brief Example C code demonstrating TCA9534 FFI interface usage
 * 
 * This example shows how to use the TCA9534 C FFI interface to control
 * the TCA9534 I2C IO expander from C code.
 */

#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "../../include/tca9534.h"

// =============================================================================
// Mock I2C implementation for demonstration
// =============================================================================

typedef struct {
    uint8_t device_address;
    uint8_t registers[4];  // Mock registers: Input, Output, Polarity, Config
} mock_i2c_context_t;

/**
 * Mock I2C write function
 */
int mock_i2c_write(void* ctx, uint8_t addr, const uint8_t* data, size_t len) {
    mock_i2c_context_t* i2c_ctx = (mock_i2c_context_t*)ctx;
    
    printf("I2C Write: addr=0x%02X, data=[", addr);
    for (size_t i = 0; i < len; i++) {
        printf("0x%02X", data[i]);
        if (i < len - 1) printf(", ");
    }
    printf("]\n");
    
    // Mock register write
    if (len == 2 && data[0] <= 0x03) {
        i2c_ctx->registers[data[0]] = data[1];
        printf("  -> Register 0x%02X set to 0x%02X\n", data[0], data[1]);
    }
    
    return 0;  // Success
}

/**
 * Mock I2C read function
 */
int mock_i2c_read(void* ctx, uint8_t addr, uint8_t* data, size_t len) {
    mock_i2c_context_t* i2c_ctx = (mock_i2c_context_t*)ctx;
    
    printf("I2C Read: addr=0x%02X, len=%zu\n", addr, len);
    
    // Mock register read (assuming single byte read)
    if (len == 1) {
        data[0] = 0x00;  // Default value
        printf("  -> Read: 0x%02X\n", data[0]);
    }
    
    return 0;  // Success
}

/**
 * Mock I2C write-read function
 */
int mock_i2c_write_read(void* ctx, uint8_t addr, const uint8_t* wr_data, 
                       size_t wr_len, uint8_t* rd_data, size_t rd_len) {
    mock_i2c_context_t* i2c_ctx = (mock_i2c_context_t*)ctx;
    
    printf("I2C Write-Read: addr=0x%02X, wr_len=%zu, rd_len=%zu\n", 
           addr, wr_len, rd_len);
    
    // Mock register read
    if (wr_len == 1 && rd_len == 1 && wr_data[0] <= 0x03) {
        rd_data[0] = i2c_ctx->registers[wr_data[0]];
        printf("  -> Read register 0x%02X: 0x%02X\n", wr_data[0], rd_data[0]);
    }
    
    return 0;  // Success
}

// =============================================================================
// Example functions
// =============================================================================

/**
 * Example 1: Basic pin configuration and control
 */
void example_basic_pin_control(void) {
    printf("\n=== Example 1: Basic Pin Control ===\n");
    
    // Initialize mock I2C context
    mock_i2c_context_t i2c_ctx = {
        .device_address = TCA9534_ADDR_000,
        .registers = {0xFF, 0x00, 0x00, 0xFF}  // Default state
    };
    
    // Setup I2C operations
    tca9534_i2c_ops_t i2c_ops = {
        .write = mock_i2c_write,
        .read = mock_i2c_read,
        .write_read = mock_i2c_write_read
    };
    
    // Initialize device handle
    tca9534_handle_t device;
    tca9534_error_t result = tca9534_init(&device, TCA9534_ADDR_000, &i2c_ctx, &i2c_ops);
    
    if (result != TCA9534_OK) {
        printf("Error: Failed to initialize device (error: %d)\n", result);
        return;
    }
    
    printf("Device initialized successfully!\n");
    
    // Configure pin 0 as output
    result = tca9534_set_pin_config(&device, 0, TCA9534_PIN_OUTPUT);
    if (result != TCA9534_OK) {
        printf("Error: Failed to configure pin 0 as output (error: %d)\n", result);
        return;
    }
    printf("Pin 0 configured as output\n");
    
    // Set pin 0 to high
    result = tca9534_set_pin_output(&device, 0, TCA9534_LEVEL_HIGH);
    if (result != TCA9534_OK) {
        printf("Error: Failed to set pin 0 high (error: %d)\n", result);
        return;
    }
    printf("Pin 0 set to HIGH\n");
    
    // Toggle pin 0
    result = tca9534_toggle_pin_output(&device, 0);
    if (result != TCA9534_OK) {
        printf("Error: Failed to toggle pin 0 (error: %d)\n", result);
        return;
    }
    printf("Pin 0 toggled\n");
}

/**
 * Example 2: Port-wide operations
 */
void example_port_operations(void) {
    printf("\n=== Example 2: Port Operations ===\n");
    
    // Initialize mock I2C context
    mock_i2c_context_t i2c_ctx = {
        .device_address = TCA9534_ADDR_000,
        .registers = {0xFF, 0x00, 0x00, 0xFF}
    };
    
    // Setup I2C operations
    tca9534_i2c_ops_t i2c_ops = {
        .write = mock_i2c_write,
        .read = mock_i2c_read,
        .write_read = mock_i2c_write_read
    };
    
    // Initialize device handle
    tca9534_handle_t device;
    tca9534_error_t result = tca9534_init_default(&device, &i2c_ctx, &i2c_ops);
    
    if (result != TCA9534_OK) {
        printf("Error: Failed to initialize device (error: %d)\n", result);
        return;
    }
    
    printf("Device initialized with default address\n");
    
    // Configure all pins as outputs
    result = tca9534_set_port_config(&device, TCA9534_ALL_OUTPUTS);
    if (result != TCA9534_OK) {
        printf("Error: Failed to configure all pins as outputs (error: %d)\n", result);
        return;
    }
    printf("All pins configured as outputs\n");
    
    // Set all outputs to high
    result = tca9534_write_output_port(&device, TCA9534_ALL_OUTPUTS_HIGH);
    if (result != TCA9534_OK) {
        printf("Error: Failed to set all outputs high (error: %d)\n", result);
        return;
    }
    printf("All outputs set to HIGH\n");
    
    // Read back the configuration
    uint8_t config;
    result = tca9534_read_port_config(&device, &config);
    if (result == TCA9534_OK) {
        printf("Port configuration: 0x%02X\n", config);
    }
}

/**
 * Example 3: Input reading with polarity
 */
void example_input_reading(void) {
    printf("\n=== Example 3: Input Reading ===\n");
    
    // Initialize mock I2C context
    mock_i2c_context_t i2c_ctx = {
        .device_address = TCA9534_ADDR_001,
        .registers = {0xAA, 0x00, 0x00, 0xFF}  // Mock input pattern
    };
    
    // Setup I2C operations
    tca9534_i2c_ops_t i2c_ops = {
        .write = mock_i2c_write,
        .read = mock_i2c_read,
        .write_read = mock_i2c_write_read
    };
    
    // Initialize device handle with different address
    tca9534_handle_t device;
    tca9534_error_t result = tca9534_init(&device, TCA9534_ADDR_001, &i2c_ctx, &i2c_ops);
    
    if (result != TCA9534_OK) {
        printf("Error: Failed to initialize device (error: %d)\n", result);
        return;
    }
    
    printf("Device initialized with address 0x%02X\n", TCA9534_ADDR_001);
    
    // Configure all pins as inputs (default)
    result = tca9534_set_port_config(&device, TCA9534_ALL_INPUTS);
    if (result != TCA9534_OK) {
        printf("Error: Failed to configure pins as inputs (error: %d)\n", result);
        return;
    }
    printf("All pins configured as inputs\n");
    
    // Read all input pins
    uint8_t input_value;
    result = tca9534_read_input_port(&device, &input_value);
    if (result == TCA9534_OK) {
        printf("Input port value: 0x%02X\n", input_value);
    }
    
    // Read individual pins
    for (int pin = 0; pin < 8; pin++) {
        tca9534_pin_level_t level;
        result = tca9534_read_pin_input(&device, pin, &level);
        if (result == TCA9534_OK) {
            printf("Pin %d: %s\n", pin, (level == TCA9534_LEVEL_HIGH) ? "HIGH" : "LOW");
        }
    }
    
    // Set inverted polarity for pin 0
    result = tca9534_set_pin_polarity(&device, 0, TCA9534_POLARITY_INVERTED);
    if (result == TCA9534_OK) {
        printf("Pin 0 polarity set to inverted\n");
    }
}

/**
 * Example 4: Address management
 */
void example_address_management(void) {
    printf("\n=== Example 4: Address Management ===\n");
    
    // Initialize mock I2C context
    mock_i2c_context_t i2c_ctx = {
        .device_address = TCA9534_ADDR_000,
        .registers = {0x00, 0x00, 0x00, 0xFF}
    };
    
    // Setup I2C operations
    tca9534_i2c_ops_t i2c_ops = {
        .write = mock_i2c_write,
        .read = mock_i2c_read,
        .write_read = mock_i2c_write_read
    };
    
    // Initialize device handle
    tca9534_handle_t device;
    tca9534_error_t result = tca9534_init(&device, TCA9534_ADDR_000, &i2c_ctx, &i2c_ops);
    
    if (result != TCA9534_OK) {
        printf("Error: Failed to initialize device (error: %d)\n", result);
        return;
    }
    
    printf("Device initialized with address 0x%02X\n", tca9534_get_address(&device));
    
    // Change device address
    tca9534_set_address(&device, TCA9534_ADDR_111);
    printf("Device address changed to 0x%02X\n", tca9534_get_address(&device));
    
    // List all possible addresses
    printf("Available TCA9534 addresses:\n");
    printf("  TCA9534_ADDR_000: 0x%02X\n", TCA9534_ADDR_000);
    printf("  TCA9534_ADDR_001: 0x%02X\n", TCA9534_ADDR_001);
    printf("  TCA9534_ADDR_010: 0x%02X\n", TCA9534_ADDR_010);
    printf("  TCA9534_ADDR_011: 0x%02X\n", TCA9534_ADDR_011);
    printf("  TCA9534_ADDR_100: 0x%02X\n", TCA9534_ADDR_100);
    printf("  TCA9534_ADDR_101: 0x%02X\n", TCA9534_ADDR_101);
    printf("  TCA9534_ADDR_110: 0x%02X\n", TCA9534_ADDR_110);
    printf("  TCA9534_ADDR_111: 0x%02X\n", TCA9534_ADDR_111);
}

// =============================================================================
// Main function
// =============================================================================

int main(void) {
    printf("TCA9534 C FFI Interface Example\n");
    printf("================================\n");
    
    // Run all examples
    example_basic_pin_control();
    example_port_operations();
    example_input_reading();
    example_address_management();
    
    printf("\n=== All Examples Completed ===\n");
    
    return 0;
} 