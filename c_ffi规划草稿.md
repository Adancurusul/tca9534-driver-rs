# TCA9534 C语言 FFI 接口规划文档

## 1. 概述

本文档详细规划了 TCA9534 驱动库的 C 语言 FFI (Foreign Function Interface) 实现方案。该接口专为 `no_std` 环境设计，通过函数指针注册机制实现 I2C 传输层抽象，让 C 语言程序能够直接调用 TCA9534 芯片的功能。

### 1.1 设计目标
- 提供简洁、安全的 C 语言接口
- 支持 `no_std` 环境，适用于嵌入式系统
- 通过函数指针实现 I2C 传输层抽象
- 统一的错误处理机制
- 最小化内存占用和运行时开销

### 1.2 架构特点
- **零分配设计**：所有操作都在栈上完成，无需动态内存分配
- **传输层抽象**：通过函数指针支持不同的 I2C 实现
- **错误码映射**：将 Rust 错误类型映射为 C 语言错误码
- **线程安全**：通过明确的所有权模型确保线程安全

### 1.3 系统架构图
```
┌─────────────────────────────────────┐
│           C 用户代码                 │
├─────────────────────────────────────┤
│         C FFI API 层                │  <- 用户接口
├─────────────────────────────────────┤
│    C 函数指针 Transport 适配器       │  <- 适配 SyncTransport
├─────────────────────────────────────┤
│       TCA9534 Rust 核心库           │  <- 现有的 Rust 实现
├─────────────────────────────────────┤
│       SyncTransport Trait          │  <- 现有的 trait 接口
└─────────────────────────────────────┘
```

**层次说明**：
- **C 用户代码**：应用程序层，直接调用 C FFI 接口
- **C FFI API 层**：提供 C 兼容的函数接口，处理类型转换和错误映射
- **C 函数指针 Transport 适配器**：将 C 函数指针包装为 Rust 的 SyncTransport 实现
- **TCA9534 Rust 核心库**：现有的 Rust 驱动实现，无需修改
- **SyncTransport Trait**：现有的 I2C 传输层抽象接口

### 1.4 数据流说明
```
调用流程：
C 用户代码 → C FFI API → Transport 适配器 → TCA9534 核心库 → I2C 硬件

返回路径：
I2C 硬件 ← TCA9534 核心库 ← Transport 适配器 ← C FFI API ← C 用户代码
```

**关键特性**：
- **向下兼容**：现有的 Rust 库无需任何修改
- **灵活适配**：通过函数指针支持任意 I2C 实现
- **错误透明**：错误信息完整传递到 C 层
- **零成本抽象**：编译时优化，运行时无额外开销

> **注意**：上方显示的是文本架构图，实际开发中可参考更详细的组件关系图来理解各层之间的交互。

## 2. 核心数据结构

### 2.1 设备句柄结构
```c
typedef struct {
    uint8_t address;           // I2C 设备地址
    void* transport_ctx;       // 传输层上下文指针
    i2c_operations_t* ops;     // I2C 操作函数指针表
} tca9534_handle_t;
```

### 2.2 I2C 操作函数指针表
```c
typedef struct {
    int (*write)(void* ctx, uint8_t addr, const uint8_t* data, size_t len);
    int (*read)(void* ctx, uint8_t addr, uint8_t* data, size_t len);
    int (*write_read)(void* ctx, uint8_t addr, const uint8_t* wr_data, 
                     size_t wr_len, uint8_t* rd_data, size_t rd_len);
} i2c_operations_t;
```

### 2.3 枚举类型定义
```c
typedef enum {
    TCA9534_PIN_INPUT  = 1,
    TCA9534_PIN_OUTPUT = 0
} tca9534_pin_config_t;

typedef enum {
    TCA9534_LEVEL_LOW  = 0,
    TCA9534_LEVEL_HIGH = 1
} tca9534_pin_level_t;

typedef enum {
    TCA9534_POLARITY_NORMAL   = 0,
    TCA9534_POLARITY_INVERTED = 1
} tca9534_pin_polarity_t;
```

## 3. 错误处理机制

### 3.1 错误码定义
```c
typedef enum {
    TCA9534_OK                = 0,
    TCA9534_ERROR_INVALID_PIN = -1,
    TCA9534_ERROR_I2C_WRITE   = -2,
    TCA9534_ERROR_I2C_READ    = -3,
    TCA9534_ERROR_NULL_PTR    = -4,
    TCA9534_ERROR_INIT_FAILED = -5
} tca9534_error_t;
```

### 3.2 错误处理策略
- **非零返回值**：所有函数返回 `tca9534_error_t` 类型，成功返回 0
- **参数验证**：严格验证输入参数，包括空指针检查和引脚号范围检查
- **错误传播**：I2C 层错误会被映射为相应的 TCA9534 错误码
- **状态一致性**：错误发生时保证设备状态的一致性

## 4. 初始化和配置接口

### 4.1 设备初始化
```c
tca9534_error_t tca9534_init(tca9534_handle_t* handle, 
                             uint8_t address,
                             void* transport_ctx,
                             i2c_operations_t* ops);
```

**功能描述**：
- 初始化 TCA9534 设备句柄
- 配置 I2C 地址和传输层函数指针
- 执行设备初始化序列（所有引脚设为输入，输出设为低电平，极性设为正常）

### 4.2 默认地址初始化
```c
tca9534_error_t tca9534_init_default(tca9534_handle_t* handle,
                                     void* transport_ctx,
                                     i2c_operations_t* ops);
```

**功能描述**：
- 使用默认 I2C 地址 (0x20) 初始化设备
- 简化常见使用场景的初始化过程

## 5. 寄存器操作接口

### 5.1 寄存器读写
```c
tca9534_error_t tca9534_read_register(tca9534_handle_t* handle,
                                      uint8_t reg_addr,
                                      uint8_t* value);

tca9534_error_t tca9534_write_register(tca9534_handle_t* handle,
                                       uint8_t reg_addr,
                                       uint8_t value);
```

**功能描述**：
- 提供底层寄存器访问接口
- 支持直接读写 TCA9534 的四个寄存器
- 为高级操作提供基础支持

## 6. 输入输出操作接口

### 6.1 端口级操作
```c
// 读取输入端口（8位一次性读取）
tca9534_error_t tca9534_read_input_port(tca9534_handle_t* handle,
                                        uint8_t* port_value);

// 写入输出端口（8位一次性写入）
tca9534_error_t tca9534_write_output_port(tca9534_handle_t* handle,
                                          uint8_t port_value);

// 读取输出端口当前值
tca9534_error_t tca9534_read_output_port(tca9534_handle_t* handle,
                                         uint8_t* port_value);
```

### 6.2 单引脚操作
```c
// 读取单个引脚输入
tca9534_error_t tca9534_read_pin_input(tca9534_handle_t* handle,
                                       uint8_t pin,
                                       tca9534_pin_level_t* level);

// 设置单个引脚输出
tca9534_error_t tca9534_set_pin_output(tca9534_handle_t* handle,
                                       uint8_t pin,
                                       tca9534_pin_level_t level);

// 切换单个引脚输出状态
tca9534_error_t tca9534_toggle_pin_output(tca9534_handle_t* handle,
                                          uint8_t pin);
```

## 7. 配置管理接口

### 7.1 引脚方向配置
```c
// 配置单个引脚方向
tca9534_error_t tca9534_set_pin_config(tca9534_handle_t* handle,
                                       uint8_t pin,
                                       tca9534_pin_config_t config);

// 配置整个端口方向
tca9534_error_t tca9534_set_port_config(tca9534_handle_t* handle,
                                        uint8_t config);

// 读取端口配置
tca9534_error_t tca9534_read_port_config(tca9534_handle_t* handle,
                                         uint8_t* config);
```

### 7.2 引脚极性配置
```c
// 配置单个引脚极性
tca9534_error_t tca9534_set_pin_polarity(tca9534_handle_t* handle,
                                         uint8_t pin,
                                         tca9534_pin_polarity_t polarity);

// 配置整个端口极性
tca9534_error_t tca9534_set_port_polarity(tca9534_handle_t* handle,
                                          uint8_t polarity);

// 读取端口极性配置
tca9534_error_t tca9534_read_port_polarity(tca9534_handle_t* handle,
                                           uint8_t* polarity);
```

## 8. 地址管理接口

### 8.1 地址操作
```c
// 设置I2C地址（支持多设备）
void tca9534_set_address(tca9534_handle_t* handle, uint8_t address);

// 获取当前I2C地址
uint8_t tca9534_get_address(const tca9534_handle_t* handle);
```

### 8.2 预定义地址常量
```c
#define TCA9534_ADDR_000  0x20  // A2=0, A1=0, A0=0 (默认)
#define TCA9534_ADDR_001  0x21  // A2=0, A1=0, A0=1
#define TCA9534_ADDR_010  0x22  // A2=0, A1=1, A0=0
#define TCA9534_ADDR_011  0x23  // A2=0, A1=1, A0=1
#define TCA9534_ADDR_100  0x24  // A2=1, A1=0, A0=0
#define TCA9534_ADDR_101  0x25  // A2=1, A1=0, A0=1
#define TCA9534_ADDR_110  0x26  // A2=1, A1=1, A0=0
#define TCA9534_ADDR_111  0x27  // A2=1, A1=1, A0=1
```

## 9. 传输层抽象实现

### 9.1 函数指针注册机制
传输层抽象是 C FFI 的核心设计，允许 C 代码提供自己的 I2C 实现：

```c
// 示例：注册 I2C 操作函数
i2c_operations_t my_i2c_ops = {
    .write = my_i2c_write,
    .read = my_i2c_read,
    .write_read = my_i2c_write_read
};

// 初始化时注册
tca9534_init(&handle, TCA9534_ADDR_000, &my_i2c_bus, &my_i2c_ops);
```

### 9.2 I2C 函数签名约定
- **返回值**：成功返回 0，失败返回非零值
- **地址参数**：7位I2C地址，不包含读写位
- **数据指针**：非空指针，调用者负责内存管理
- **长度参数**：数据字节数，必须与实际缓冲区大小匹配

## 10. 内存管理策略

### 10.1 零分配设计
- **栈上分配**：所有临时数据都在栈上分配
- **静态结构**：设备句柄结构体通常作为静态变量或在栈上分配
- **无动态内存**：完全避免使用 malloc/free

### 10.2 生命周期管理
- **句柄初始化**：在使用前必须调用初始化函数
- **状态保持**：句柄在整个使用期间保持有效
- **资源清理**：当前版本无需显式清理资源

## 11. 线程安全考虑

### 11.1 并发访问模型
- **非线程安全**：单个设备句柄不支持并发访问
- **多设备支持**：不同设备句柄可以在不同线程中使用
- **同步责任**：应用程序负责确保访问同步

### 11.2 推荐使用模式
```c
// 单线程模式：直接使用
tca9534_handle_t device;
tca9534_init(&device, ...);
tca9534_set_pin_output(&device, 0, TCA9534_LEVEL_HIGH);

// 多线程模式：使用互斥锁
pthread_mutex_t device_mutex;
// 在访问设备前加锁，访问后解锁
```

## 12. 使用示例和最佳实践

### 12.1 典型使用流程
1. **定义I2C操作函数**：实现平台特定的I2C读写函数
2. **初始化设备句柄**：调用初始化函数并注册I2C操作
3. **配置引脚**：设置引脚方向、极性等参数
4. **执行I/O操作**：读取输入或设置输出
5. **错误处理**：检查返回值并处理错误情况

### 12.2 性能优化建议
- **批量操作**：优先使用端口级操作而非单引脚操作
- **缓存配置**：避免重复读取配置寄存器
- **错误预检**：在调用前验证参数有效性

## 13. 未来扩展考虑

### 13.1 可能的功能扩展
- **中断支持**：添加中断处理相关接口
- **异步操作**：提供异步I/O操作支持
- **批量设备**：支持同时管理多个TCA9534设备
- **调试接口**：添加调试和诊断功能

### 13.2 兼容性保证
- **ABI稳定性**：保证二进制接口的向后兼容性
- **版本管理**：通过版本号管理接口变更
- **弃用策略**：逐步弃用旧接口而非直接删除

## 14. 实现注意事项

### 14.1 Rust侧实现要点
- **#[no_mangle]**：确保函数名不被修改
- **extern "C"**：使用C调用约定
- **异常安全**：避免跨FFI边界的panic
- **内存布局**：确保结构体内存布局与C兼容

### 14.2 C侧集成要点
- **头文件设计**：提供完整的类型定义和函数声明
- **链接配置**：正确配置静态库链接
- **编译选项**：确保C和Rust编译选项兼容
- **测试覆盖**：提供完整的C语言测试用例

这个设计方案在保持简洁性的同时，提供了完整的TCA9534功能访问能力，适合在各种嵌入式系统中使用。
