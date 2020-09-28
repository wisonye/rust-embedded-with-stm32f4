use crate::clock_frequency::MegaHertz;
use crate::rcc_clock_config_register::{RccClockConfigurationRegister, RccSystemClockSwtich};
use crate::rcc_clock_control_register::RccClockControlRegister;
use crate::rcc_clock_settings::clock_source_selecting;
use crate::rcc_pll_config_register::RccPllConfigurationRegister;
use core::fmt::Write;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

#[cfg(feature = "enable-debug")]
use heapless::{consts::*, String};

///
#[derive(Debug, Clone, PartialEq)]
pub enum ClockSource {
    Hsi,
    HsiThroughPll,
    HseThroughPll,
}

///
pub struct RccClocks {
    hsi: Option<MegaHertz>,
    hse: Option<MegaHertz>,
    clock_source: ClockSource,
    system_clock: Option<MegaHertz>,
    hardware_cpu_clock: Option<MegaHertz>,
    ahb_prescaler: Option<u32>,
    pll_m: Option<u32>,
    pll_n: Option<u32>,
    pll_p: Option<u32>,
    pll_q: Option<u32>,
    apb1_peripheral_clock: Option<MegaHertz>,
    apb1_timer_clock: Option<MegaHertz>,
    apb2_peripheral_clock: Option<MegaHertz>,
    apb2_timer_clock: Option<MegaHertz>,
}

/// As `heapless::String<N>` allocates fixed memory on the stack,
/// and the maximum speed is up to "XXXMhz", that's why take 6bytes.
#[cfg(feature = "enable-debug")]
fn debug_frequency(speed: &Option<MegaHertz>) -> String<U6> {
    if speed.is_some() {
        let mut temp_str: String<U6> = String::new();
        let _ = write!(temp_str, "{}MHz", speed.as_ref().unwrap().0);
        temp_str
    } else {
        String::<U6>::from("None")
    }
}

///
#[cfg(feature = "enable-debug")]
impl core::fmt::Debug for RccClocks {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("[ RccClocks ]:")
            .field("hsi", &debug_frequency(&self.hsi))
            .field("hse", &debug_frequency(&self.hse))
            .field("clock_source", &self.clock_source)
            .field("system_clock", &debug_frequency(&self.system_clock))
            .field(
                "hardware_cpu_clock",
                &debug_frequency(&self.hardware_cpu_clock),
            )
            .field("pll_m", &self.pll_m)
            .field("pll_n", &self.pll_n)
            .field("pll_p", &self.pll_p)
            .field("pll_q", &self.pll_q)
            .field(
                "apb1_peripheral_clock",
                &debug_frequency(&self.apb1_peripheral_clock),
            )
            .field("apb1_timer_clock", &debug_frequency(&self.apb1_timer_clock))
            .field(
                "apb2_peripheral_clock",
                &debug_frequency(&self.apb2_peripheral_clock),
            )
            .field("apb2_timer_clock", &debug_frequency(&self.apb2_timer_clock))
            .finish()
    }
}

///
impl RccClocks {
    #[cfg(feature = "enable-debug")]
    pub fn print_system_clock_info() {
        RccClockControlRegister::print_config();
        RccClockConfigurationRegister::print_config();
        RccPllConfigurationRegister::print_config();
    }

    /// Reset all rcc registers
    fn init_rcc_clock() {
        RccClockControlRegister::reset();
    }

    /// Create `RccClocks` instance
    fn create_rcc_clocks(clock_source: &ClockSource) -> RccClocks {
        let rcc_clock = match clock_source {
            ClockSource::Hsi => {
                let system_clock_speed = clock_source_selecting::HSI_FREQUENCY;
                let cpu_clock_speed =
                    system_clock_speed / clock_source_selecting::AHB_PRESCALER_FOR_HSI;
                let apb1_peripheral_clock_speed =
                    cpu_clock_speed / clock_source_selecting::APB1_PRESCALER_FOR_HSI;
                let apb2_peripheral_clock_speed =
                    cpu_clock_speed / clock_source_selecting::APB2_PRESCALER_FOR_HSI;
                let apb1_timer_clock_speed =
                    apb1_peripheral_clock_speed * clock_source_selecting::APB1_TIMER_FACTOR;
                let apb2_timer_clock_speed =
                    apb2_peripheral_clock_speed * clock_source_selecting::APB2_TIMER_FACTOR;

                RccClocks {
                    hsi: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                    hse: Some(clock_source_selecting::HSE_FREQUENCY.into()),
                    clock_source: (*clock_source).clone(),
                    system_clock: Some(system_clock_speed.into()),
                    hardware_cpu_clock: Some(cpu_clock_speed.into()),
                    ahb_prescaler: None,
                    pll_m: None,
                    pll_n: None,
                    pll_p: None,
                    pll_q: None,
                    apb1_peripheral_clock: Some(apb1_peripheral_clock_speed.into()),
                    apb1_timer_clock: Some(apb1_timer_clock_speed.into()),
                    apb2_peripheral_clock: Some(apb2_peripheral_clock_speed.into()),
                    apb2_timer_clock: Some(apb2_timer_clock_speed.into()),
                }
            }
            ClockSource::HsiThroughPll => {
                let system_clock_speed = clock_source_selecting::HSI_FREQUENCY
                    / clock_source_selecting::PLL_M_PRESCALER_FOR_HSI
                    * clock_source_selecting::PLL_N_PRESCALER_FOR_HSI
                    / clock_source_selecting::PLL_P_PRESCALER_FOR_HSI;

                let cpu_clock_speed =
                    system_clock_speed / clock_source_selecting::AHB_PRESCALER_FOR_HSI;

                let apb1_peripheral_clock_speed =
                    cpu_clock_speed / clock_source_selecting::APB1_PRESCALER_FOR_HSI;
                let apb2_peripheral_clock_speed =
                    cpu_clock_speed / clock_source_selecting::APB2_PRESCALER_FOR_HSI;
                let apb1_timer_clock_speed =
                    apb1_peripheral_clock_speed * clock_source_selecting::APB1_TIMER_FACTOR;
                let apb2_timer_clock_speed =
                    apb2_peripheral_clock_speed * clock_source_selecting::APB2_TIMER_FACTOR;

                RccClocks {
                    hsi: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                    hse: Some(clock_source_selecting::HSE_FREQUENCY.into()),
                    clock_source: (*clock_source).clone(),
                    system_clock: Some(system_clock_speed.into()),
                    hardware_cpu_clock: Some(cpu_clock_speed.into()),
                    ahb_prescaler: Some(clock_source_selecting::AHB_PRESCALER_FOR_HSI),
                    pll_m: Some(clock_source_selecting::PLL_M_PRESCALER_FOR_HSI),
                    pll_n: Some(clock_source_selecting::PLL_N_PRESCALER_FOR_HSI),
                    pll_p: Some(clock_source_selecting::PLL_P_PRESCALER_FOR_HSI),
                    pll_q: Some(clock_source_selecting::PLL_Q_PRESCALER_FOR_HSI),
                    apb1_peripheral_clock: Some(apb1_peripheral_clock_speed.into()),
                    apb1_timer_clock: Some(apb1_timer_clock_speed.into()),
                    apb2_peripheral_clock: Some(apb2_peripheral_clock_speed.into()),
                    apb2_timer_clock: Some(apb2_timer_clock_speed.into()),
                }
            }
            ClockSource::HseThroughPll => {
                let system_clock_speed = clock_source_selecting::HSE_FREQUENCY
                    / clock_source_selecting::PLL_M_PRESCALER_FOR_HSE
                    * clock_source_selecting::PLL_N_PRESCALER_FOR_HSE
                    / clock_source_selecting::PLL_P_PRESCALER_FOR_HSE;

                let cpu_clock_speed =
                    system_clock_speed / clock_source_selecting::AHB_PRESCALER_FOR_HSE;

                let apb1_peripheral_clock_speed =
                    cpu_clock_speed / clock_source_selecting::APB1_PRESCALER_FOR_HSE;
                let apb2_peripheral_clock_speed =
                    cpu_clock_speed / clock_source_selecting::APB2_PRESCALER_FOR_HSE;
                let apb1_timer_clock_speed =
                    apb1_peripheral_clock_speed * clock_source_selecting::APB1_TIMER_FACTOR;
                let apb2_timer_clock_speed =
                    apb2_peripheral_clock_speed * clock_source_selecting::APB2_TIMER_FACTOR;

                RccClocks {
                    hsi: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                    hse: Some(clock_source_selecting::HSE_FREQUENCY.into()),
                    clock_source: (*clock_source).clone(),
                    system_clock: Some(system_clock_speed.into()),
                    hardware_cpu_clock: Some(cpu_clock_speed.into()),
                    ahb_prescaler: Some(clock_source_selecting::AHB_PRESCALER_FOR_HSE),
                    pll_m: Some(clock_source_selecting::PLL_M_PRESCALER_FOR_HSE),
                    pll_n: Some(clock_source_selecting::PLL_N_PRESCALER_FOR_HSE),
                    pll_p: Some(clock_source_selecting::PLL_P_PRESCALER_FOR_HSE),
                    pll_q: Some(clock_source_selecting::PLL_Q_PRESCALER_FOR_HSE),
                    apb1_peripheral_clock: Some(apb1_peripheral_clock_speed.into()),
                    apb1_timer_clock: Some(apb1_timer_clock_speed.into()),
                    apb2_peripheral_clock: Some(apb2_peripheral_clock_speed.into()),
                    apb2_timer_clock: Some(apb2_timer_clock_speed.into()),
                }
            }
        };

        #[cfg(feature = "enable-debug")]
        let _ = hprintln!("\n{:#?}", &rcc_clock);

        rcc_clock
    }

    /// Setup system clock
    pub fn setup_system_clock(clock_source: ClockSource) -> RccClocks {
        Self::init_rcc_clock();

        let rcc_clock = Self::create_rcc_clocks(&clock_source);

        // For this default option, we do nothing
        if clock_source == ClockSource::Hsi {
            return rcc_clock;
        }

        match clock_source {
            ClockSource::HsiThroughPll => {
                // 2. Setup flash

                // 3. Set PLL factors MNPQ
                RccPllConfigurationRegister::set_pll_mnpq(false);
                // 4. Enable PLL and wait for it stable
                RccClockControlRegister::enable_pll_and_wait_for_it_stable();
                // 5. Set the AHB prescaler, APB1 prescaler, APB2 prescaler
                RccClockConfigurationRegister::set_bus_prescaler(false);
            }
            ClockSource::HseThroughPll => {
                // 1. Enable HSE and wait for it stable
                RccClockControlRegister::enable_hse_as_clock_source_and_wait_for_it_stable();
                // 2. Setup flash

                // 3. Set PLL factors MNPQ
                RccPllConfigurationRegister::set_pll_mnpq(true);
                // 4. Enable PLL and wait for it stable
                RccClockControlRegister::enable_pll_and_wait_for_it_stable();
                // 5. Set the AHB prescaler, APB1 prescaler, APB2 prescaler
                RccClockConfigurationRegister::set_bus_prescaler(true);
            }
            _ => {}
        }

        // 6. Switch clock source
        RccClockConfigurationRegister::switch_clock_source_and_wait_for_stable(
            RccSystemClockSwtich::PllSelectedAsSytemClock,
        );

        rcc_clock
    }
}
