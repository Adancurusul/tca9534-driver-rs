#ifndef TCA9534_H
#define TCA9534_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file tca9534.h
 * @brief TCA9534 I2C IO Expander C API
 * 
 * This header provides a C interface for the TCA9534 I2C IO expander driver.
 * The interface is designed for no_std environments and uses function pointers
 * for I2C transport abstraction.
 */

// =============================================================================
// Type Definitions
// =============================================================================

/**
 * @brief I2C operations function pointers structure
 * 
 * This structure contains function pointers for I2C operations that must be
 * implemented by the user to provide I2C transport functionality.
 */
typedef struct {
    /**
     * @brief Write data to I2C device
     * @param ctx Transport context pointer
     * @param addr I2C device address (7-bit, without R/W bit)
     * @param data Pointer to data to write
     * @param len Number of bytes to write
     * @return 0 on success, non-zero on error
     */
    int (*write)(void* ctx, uint8_t addr, const uint8_t* data, size_t len);
    
    /**
     * @brief Read data from I2C device
     * @param ctx Transport context pointer
     * @param addr I2C device address (7-bit, without R/W bit)
     * @param data Pointer to buffer for received data
     * @param len Number of bytes to read
     * @return 0 on success, non-zero on error
     */
    int (*read)(void* ctx, uint8_t addr, uint8_t* data, size_t len);
    
    /**
     * @brief Write then read data from I2C device
     * @param ctx Transport context pointer
     * @param addr I2C device address (7-bit, without R/W bit)
     * @param wr_data Pointer to data to write
     * @param wr_len Number of bytes to write
     * @param rd_data Pointer to buffer for received data
     * @param rd_len Number of bytes to read
     * @return 0 on success, non-zero on error
     */
    int (*write_read)(void* ctx, uint8_t addr, const uint8_t* wr_data, 
                     size_t wr_len, uint8_t* rd_data, size_t rd_len);
} tca9534_i2c_ops_t;

/**
 * @brief TCA9534 device handle structure
 * 
 * This structure represents a TCA9534 device instance and contains
 * all necessary information for device communication.
 */
typedef struct {
    uint8_t address;                    /**< I2C device address */
    void* transport_ctx;                /**< Transport context pointer */
    tca9534_i2c_ops_t* ops;            /**< I2C operations function pointers */
} tca9534_handle_t;

/**
 * @brief Pin configuration enumeration
 */
typedef enum {
    TCA9534_PIN_INPUT  = 1,    /**< Pin configured as input (high impedance) */
    TCA9534_PIN_OUTPUT = 0     /**< Pin configured as output */
} tca9534_pin_config_t;

/**
 * @brief Pin logic level enumeration
 */
typedef enum {
    TCA9534_LEVEL_LOW  = 0,    /**< Logic low (0V) */
    TCA9534_LEVEL_HIGH = 1     /**< Logic high (VCC) */
} tca9534_pin_level_t;

/**
 * @brief Pin polarity enumeration
 */
typedef enum {
    TCA9534_POLARITY_NORMAL   = 0,    /**< Normal polarity (default) */
    TCA9534_POLARITY_INVERTED = 1     /**< Inverted polarity */
} tca9534_pin_polarity_t;

/**
 * @brief Error codes enumeration
 */
typedef enum {
    TCA9534_OK                = 0,     /**< Success */
    TCA9534_ERROR_INVALID_PIN = -1,    /**< Invalid pin number (must be 0-7) */
    TCA9534_ERROR_I2C_WRITE   = -2,    /**< I2C write error */
    TCA9534_ERROR_I2C_READ    = -3,    /**< I2C read error */
    TCA9534_ERROR_NULL_PTR    = -4,    /**< Null pointer error */
    TCA9534_ERROR_INIT_FAILED = -5     /**< Device initialization failed */
} tca9534_error_t;

// =============================================================================
// Address Constants
// =============================================================================

/** I2C address constants based on A2, A1, A0 pins */
#define TCA9534_ADDR_000  0x20  /**< A2=0, A1=0, A0=0 (default) */
#define TCA9534_ADDR_001  0x21  /**< A2=0, A1=0, A0=1 */
#define TCA9534_ADDR_010  0x22  /**< A2=0, A1=1, A0=0 */
#define TCA9534_ADDR_011  0x23  /**< A2=0, A1=1, A0=1 */
#define TCA9534_ADDR_100  0x24  /**< A2=1, A1=0, A0=0 */
#define TCA9534_ADDR_101  0x25  /**< A2=1, A1=0, A0=1 */
#define TCA9534_ADDR_110  0x26  /**< A2=1, A1=1, A0=0 */
#define TCA9534_ADDR_111  0x27  /**< A2=1, A1=1, A0=1 */

/** Register addresses */
#define TCA9534_REG_INPUT_PORT  0x00  /**< Input port register */
#define TCA9534_REG_OUTPUT_PORT 0x01  /**< Output port register */
#define TCA9534_REG_POLARITY    0x02  /**< Polarity inversion register */
#define TCA9534_REG_CONFIG      0x03  /**< Configuration register */

/** Configuration constants */
#define TCA9534_ALL_INPUTS              0xFF  /**< All pins as inputs */
#define TCA9534_ALL_OUTPUTS             0x00  /**< All pins as outputs */
#define TCA9534_ALL_NORMAL_POLARITY     0x00  /**< All pins normal polarity */
#define TCA9534_ALL_INVERTED_POLARITY   0xFF  /**< All pins inverted polarity */
#define TCA9534_ALL_OUTPUTS_LOW         0x00  /**< All outputs low */
#define TCA9534_ALL_OUTPUTS_HIGH        0xFF  /**< All outputs high */

// =============================================================================
// Function Declarations
// =============================================================================

/**
 * @brief Initialize TCA9534 device
 * 
 * Initializes the TCA9534 device handle with the specified I2C address and
 * transport operations. Performs device initialization sequence.
 * 
 * @param handle Pointer to device handle structure
 * @param address I2C device address (7-bit)
 * @param transport_ctx Transport context pointer
 * @param ops Pointer to I2C operations structure
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_init(tca9534_handle_t* handle, 
                             uint8_t address,
                             void* transport_ctx,
                             tca9534_i2c_ops_t* ops);

/**
 * @brief Initialize TCA9534 device with default address
 * 
 * Initializes the TCA9534 device handle with the default I2C address (0x20).
 * 
 * @param handle Pointer to device handle structure
 * @param transport_ctx Transport context pointer
 * @param ops Pointer to I2C operations structure
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_init_default(tca9534_handle_t* handle,
                                     void* transport_ctx,
                                     tca9534_i2c_ops_t* ops);

// =============================================================================
// Register Operations
// =============================================================================

/**
 * @brief Read a register value
 * 
 * @param handle Pointer to device handle
 * @param reg_addr Register address
 * @param value Pointer to store the read value
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_read_register(tca9534_handle_t* handle,
                                      uint8_t reg_addr,
                                      uint8_t* value);

/**
 * @brief Write a register value
 * 
 * @param handle Pointer to device handle
 * @param reg_addr Register address
 * @param value Value to write
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_write_register(tca9534_handle_t* handle,
                                       uint8_t reg_addr,
                                       uint8_t value);

// =============================================================================
// Port Operations
// =============================================================================

/**
 * @brief Read input port (all 8 pins at once)
 * 
 * @param handle Pointer to device handle
 * @param port_value Pointer to store the port value
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_read_input_port(tca9534_handle_t* handle,
                                        uint8_t* port_value);

/**
 * @brief Write output port (all 8 pins at once)
 * 
 * @param handle Pointer to device handle
 * @param port_value Port value to write
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_write_output_port(tca9534_handle_t* handle,
                                          uint8_t port_value);

/**
 * @brief Read current output port register value
 * 
 * @param handle Pointer to device handle
 * @param port_value Pointer to store the port value
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_read_output_port(tca9534_handle_t* handle,
                                         uint8_t* port_value);

// =============================================================================
// Pin Operations
// =============================================================================

/**
 * @brief Read a single pin input
 * 
 * @param handle Pointer to device handle
 * @param pin Pin number (0-7)
 * @param level Pointer to store the pin level
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_read_pin_input(tca9534_handle_t* handle,
                                       uint8_t pin,
                                       tca9534_pin_level_t* level);

/**
 * @brief Set a single pin output
 * 
 * @param handle Pointer to device handle
 * @param pin Pin number (0-7)
 * @param level Pin level to set
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_set_pin_output(tca9534_handle_t* handle,
                                       uint8_t pin,
                                       tca9534_pin_level_t level);

/**
 * @brief Toggle a single pin output
 * 
 * @param handle Pointer to device handle
 * @param pin Pin number (0-7)
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_toggle_pin_output(tca9534_handle_t* handle,
                                          uint8_t pin);

// =============================================================================
// Configuration Operations
// =============================================================================

/**
 * @brief Configure a single pin direction
 * 
 * @param handle Pointer to device handle
 * @param pin Pin number (0-7)
 * @param config Pin configuration (input/output)
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_set_pin_config(tca9534_handle_t* handle,
                                       uint8_t pin,
                                       tca9534_pin_config_t config);

/**
 * @brief Configure all pins direction at once
 * 
 * @param handle Pointer to device handle
 * @param config Configuration value (bit 0 = pin 0, bit 1 = pin 1, etc.)
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_set_port_config(tca9534_handle_t* handle,
                                        uint8_t config);

/**
 * @brief Read port configuration
 * 
 * @param handle Pointer to device handle
 * @param config Pointer to store the configuration value
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_read_port_config(tca9534_handle_t* handle,
                                         uint8_t* config);

/**
 * @brief Configure a single pin polarity
 * 
 * @param handle Pointer to device handle
 * @param pin Pin number (0-7)
 * @param polarity Pin polarity (normal/inverted)
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_set_pin_polarity(tca9534_handle_t* handle,
                                         uint8_t pin,
                                         tca9534_pin_polarity_t polarity);

/**
 * @brief Configure all pins polarity at once
 * 
 * @param handle Pointer to device handle
 * @param polarity Polarity value (bit 0 = pin 0, bit 1 = pin 1, etc.)
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_set_port_polarity(tca9534_handle_t* handle,
                                          uint8_t polarity);

/**
 * @brief Read port polarity configuration
 * 
 * @param handle Pointer to device handle
 * @param polarity Pointer to store the polarity value
 * @return TCA9534_OK on success, error code on failure
 */
tca9534_error_t tca9534_read_port_polarity(tca9534_handle_t* handle,
                                           uint8_t* polarity);

// =============================================================================
// Address Management
// =============================================================================

/**
 * @brief Set I2C address (useful for multiple devices)
 * 
 * @param handle Pointer to device handle
 * @param address New I2C address (7-bit)
 */
void tca9534_set_address(tca9534_handle_t* handle, uint8_t address);

/**
 * @brief Get current I2C address
 * 
 * @param handle Pointer to device handle
 * @return Current I2C address
 */
uint8_t tca9534_get_address(const tca9534_handle_t* handle);

#ifdef __cplusplus
}
#endif

#endif /* TCA9534_H */