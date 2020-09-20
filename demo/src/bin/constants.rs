// RCC (Reset and Clock Control), page 65
pub const RCC_REGISTER: u32 = 0x4002_3800;
pub const RCC_AHB1ENR_REGISTER: u32 = RCC_REGISTER + 0x30; // page 242, 243
pub const RCC_AHB1LPENR_REGISTER: u32 = RCC_REGISTER + 0x50; // Low power (sleep) mode, page 250, 252,

// RCC clock control register (RCC_CR), page 224
pub const RCC_CR_HSI_IS_ON: u32 = 1u32;
pub const RCC_CR_HSI_IS_STABLE: u32 = 1 << 1;
pub const RCC_CR_HSE_IS_ON: u32 = 1 << 16;
pub const RCC_CR_HSE_IS_STABLE: u32 = 1 << 17;
pub const RCC_CR_HSE_BYPASS: u32 = 1 << 18;
pub const RCC_CR_CLOCK_SECURITY_IS_ON: u32 = 1 << 19;
pub const RCC_CR_MAIN_PLL_IS_ON: u32 = 1 << 24;
pub const RCC_CR_MAIN_PLL_IS_READY: u32 = 1 << 25;
pub const RCC_CR_PLLI2S_IS_ON: u32 = 1 << 26;
pub const RCC_CR_PLLI2S_IS_READY: u32 = 1 << 27;
