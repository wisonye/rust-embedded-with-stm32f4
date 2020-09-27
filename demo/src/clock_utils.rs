use crate::clock_frequency::MegaHertz;
use crate::rcc_clock_settings::clock_source_selecting;
use crate::rcc_clock_config_register::RccClockConfigurationRegister;
use crate::rcc_clock_control_register::RccClockControlRegister;
use crate::rcc_pll_config_register::RccPllConfigurationRegister;
use core::fmt::Write;
use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

#[cfg(feature = "enable-debug")]
use heapless::{consts::*, String};

///
#[derive(Debug)]
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
fn debug_clock(speed: &Option<MegaHertz>) -> String<U6> {
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
            .field("hsi", &debug_clock(&self.hsi))
            .field("hse", &debug_clock(&self.hse))
            .field("clock_source", &self.clock_source)
            .field("system_clock", &debug_clock(&self.system_clock))
            .field("hardware_cpu_clock", &debug_clock(&self.hardware_cpu_clock))
            .field("pll_m", &self.pll_m)
            .field("pll_n", &self.pll_n)
            .field("pll_p", &self.pll_p)
            .field("pll_q", &self.pll_q)
            .field(
                "apb1_peripheral_clock",
                &debug_clock(&self.apb1_peripheral_clock),
            )
            .field("apb1_timer_clock", &debug_clock(&self.apb1_timer_clock))
            .field(
                "apb2_peripheral_clock",
                &debug_clock(&self.apb2_peripheral_clock),
            )
            .field("apb2_timer_clock", &debug_clock(&self.apb2_timer_clock))
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
        unsafe {
            
        }
    }

    /// Setup system clock
    pub fn setup_system_clock(clock_source: ClockSource) -> RccClocks {
        Self::init_rcc_clock();

        let rcc_clock = match clock_source {
            ClockSource::Hsi => RccClocks {
                hsi: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                hse: Some(clock_source_selecting::HSE_FREQUENCY.into()),
                clock_source,
                system_clock: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                hardware_cpu_clock: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                ahb_prescaler: None,
                pll_m: None,
                pll_n: None,
                pll_p: None,
                pll_q: None,
                apb1_peripheral_clock: None,
                apb1_timer_clock: None,
                apb2_peripheral_clock: None,
                apb2_timer_clock: None,
            },
            ClockSource::HsiThroughPll => {
                let system_clock_speed = clock_source_selecting::HSI_FREQUENCY
                    / clock_source_selecting::PLL_M_PRESCALER_FOR_HSI
                    * clock_source_selecting::PLL_N_PRESCALER_FOR_HSI
                    / clock_source_selecting::PLL_P_PRESCALER_FOR_HSI;

                let cpu_clock_speed =
                    system_clock_speed / clock_source_selecting::AHB_PRESCALER_FOR_HSI;

                RccClocks {
                    hsi: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                    hse: Some(clock_source_selecting::HSE_FREQUENCY.into()),
                    clock_source,
                    system_clock: Some(system_clock_speed.into()),
                    hardware_cpu_clock: Some(cpu_clock_speed.into()),
                    ahb_prescaler: Some(clock_source_selecting::AHB_PRESCALER_FOR_HSI),
                    pll_m: Some(clock_source_selecting::PLL_M_PRESCALER_FOR_HSI),
                    pll_n: Some(clock_source_selecting::PLL_N_PRESCALER_FOR_HSI),
                    pll_p: Some(clock_source_selecting::PLL_P_PRESCALER_FOR_HSI),
                    pll_q: Some(clock_source_selecting::PLL_Q_PRESCALER_FOR_HSI),
                    apb1_peripheral_clock: None,
                    apb1_timer_clock: None,
                    apb2_peripheral_clock: None,
                    apb2_timer_clock: None,
                }
            }
            ClockSource::HseThroughPll => {
                let system_clock_speed = clock_source_selecting::HSE_FREQUENCY
                    / clock_source_selecting::PLL_M_PRESCALER_FOR_HSE
                    * clock_source_selecting::PLL_N_PRESCALER_FOR_HSE
                    / clock_source_selecting::PLL_P_PRESCALER_FOR_HSE;

                let cpu_clock_speed =
                    system_clock_speed / clock_source_selecting::AHB_PRESCALER_FOR_HSE;

                RccClocks {
                    hsi: Some(clock_source_selecting::HSI_FREQUENCY.into()),
                    hse: Some(clock_source_selecting::HSE_FREQUENCY.into()),
                    clock_source,
                    system_clock: Some(system_clock_speed.into()),
                    hardware_cpu_clock: Some(cpu_clock_speed.into()),
                    ahb_prescaler: Some(clock_source_selecting::AHB_PRESCALER_FOR_HSE),
                    pll_m: Some(clock_source_selecting::PLL_M_PRESCALER_FOR_HSE),
                    pll_n: Some(clock_source_selecting::PLL_N_PRESCALER_FOR_HSE),
                    pll_p: Some(clock_source_selecting::PLL_P_PRESCALER_FOR_HSE),
                    pll_q: Some(clock_source_selecting::PLL_Q_PRESCALER_FOR_HSE),
                    apb1_peripheral_clock: None,
                    apb1_timer_clock: None,
                    apb2_peripheral_clock: None,
                    apb2_timer_clock: None,
                }
            }
        };

        #[cfg(feature = "enable-debug")]
        hprintln!("\n{:#?}", &rcc_clock);

        rcc_clock
    }
}
