#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Control Controls the device power mode options and allows observation of current state."]
    pub pwr_control: crate::Reg<pwr_control::PWR_CONTROL_SPEC>,
    #[doc = "0x04 - Power System Key&Delay Register"]
    pub pwr_key_delay: crate::Reg<pwr_key_delay::PWR_KEY_DELAY_SPEC>,
    #[doc = "0x08 - Power ADFT Mode Selection Register Controls System Resources ADFT mode settings and observability. Writes to this register are ignored and settings in this register have no effect unless the part is in a XRES key selected DfT mode. Entire register is engineering only. Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
    pub pwr_adft_select: crate::Reg<pwr_adft_select::PWR_ADFT_SELECT_SPEC>,
    #[doc = "0x0c - Power DDFT Mode Selection Register Selects the signal sources output to the DDFT outputs of the power subsystem. Entire register is engineering only."]
    pub pwr_ddft_select: crate::Reg<pwr_ddft_select::PWR_DDFT_SELECT_SPEC>,
    #[doc = "0x10 - XRES DfT Key observer logic test register This register is used to test the XRES TestMode key logic. It allows a test routine (firmware or ATE driven) to stimulate the key listener and observe its functionality. Extreme case must be taken in these tests, since they will result in actual test mode entry. For example, shifting in a scan mode key, will transition the system into scan mode immediately. Note that test_scan_mode is not observable in this register for that reason."]
    pub pwr_ddft_xres: crate::Reg<pwr_ddft_xres::PWR_DDFT_XRES_SPEC>,
    #[doc = "0x14 - Test Mode Control Register Controls primary test mode. This is a single bit that can be written directly from the ATE/Programmer in any protection mode. It's main function is to signal to the Boot ROM that normal firmware execution is not to commence after boot is complete. Instead the Boot ROM will enter a wait loop for system commands."]
    pub tst_mode: crate::Reg<tst_mode::TST_MODE_SPEC>,
    #[doc = "0x18 - Digital DFT Control Register Controls system level DDFT observability muxes and comparators."]
    pub tst_ddft_ctrl: crate::Reg<tst_ddft_ctrl::TST_DDFT_CTRL_SPEC>,
    #[doc = "0x1c - IMO trim down-counter and status (clk_sys)"]
    pub tst_trim_cntr1: crate::Reg<tst_trim_cntr1::TST_TRIM_CNTR1_SPEC>,
    #[doc = "0x20 - IMO trim up-counter (ddft)"]
    pub tst_trim_cntr2: crate::Reg<tst_trim_cntr2::TST_TRIM_CNTR2_SPEC>,
    #[doc = "0x24 - ADFT buffer/comparator control register Controls System Resources ADFT mode settings and observability. Writes to this register are ignored and settings in this register have no effect unless the part is in a XRES key selected DfT mode. Entire register is engineering only. Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
    pub tst_adft_ctrl: crate::Reg<tst_adft_ctrl::TST_ADFT_CTRL_SPEC>,
    #[doc = "0x28 - Clock Select Register Configures direction of all clock multiplexers and selectors. See Section 20.3 in SAS for details on clock network topology. See PAS for DSI signal connectivity list."]
    pub clk_select: crate::Reg<clk_select::CLK_SELECT_SPEC>,
    #[doc = "0x2c - ILO Configuration Internal slow speed R/C oscillator (32kHz) configuration register. Note: writes to this register are ignored when WDT_DISABLE_KEY is not set to the magic value defined for it."]
    pub clk_ilo_config: crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>,
    #[doc = "0x30 - IMO Configuration Internal high speed R/C oscillator configuration register. Note that this oscillator comes up active on power up. The oscillator provides the primary system clock (HFCLK) on power up until firmware configures differently. This oscillator is also used before system start to count out power up delays. This is done in fast IMO (FIMO) mode that does not require any external references and runs at a fixed 12MHz."]
    pub clk_imo_config: crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>,
    #[doc = "0x34 - Clock DFT Mode Selection Register Selects which clock signals to bring out to to DFT pins. Two signals can be selected to enable comparison of clocks. Clocks can be divided down to deal with slower equipment and I/Os. See TST_DFT_SELECT for details on bringing these pins out. Entire register is engineering only."]
    pub clk_dft_select: crate::Reg<clk_dft_select::CLK_DFT_SELECT_SPEC>,
    #[doc = "0x38 - Watchdog Disable Key Register This key can be used to disable the watchdog timer reset generation in applications that do not require absolute brown-out safety and do not want to deal with the hassle of feeding the watchdog regularly. Setting the key will also enable the CLK_ILO_CONFIG.ENABLE bit to be effective. It will not have any other effect, i.e. the WDT timer/interrupt functionality can still be used."]
    pub wdt_disable_key: crate::Reg<wdt_disable_key::WDT_DISABLE_KEY_SPEC>,
    #[doc = "0x3c - Watchdog Counter Register Provides actual counter value for watchdog counter. Watchdog counter always counts up, is free-running and is clocked using clk_lf."]
    pub wdt_counter: crate::Reg<wdt_counter::WDT_COUNTER_SPEC>,
    #[doc = "0x40 - Watchdog Match Register Firmware provided match value that is compared against WDT_COUNTER. The expectation is that firmware modifies this register after each match as part of the WDT interrupt service routine."]
    pub wdt_match: crate::Reg<wdt_match::WDT_MATCH_SPEC>,
    #[doc = "0x44 - SRSS Interrupt Register The intent is that this register is cleared for every WDT interrupt under all circumstances, including when the system is in DeepSleep."]
    pub srss_intr: crate::Reg<srss_intr::SRSS_INTR_SPEC>,
    #[doc = "0x48 - SRSS Interrupt Set Register Can be used to set interrupts for firmware testing. Note that SET functionality is not available for WDT."]
    pub srss_intr_set: crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>,
    #[doc = "0x4c - SRSS Interrupt Mask Register Controls whether interrupt is forwarded to CPU."]
    pub srss_intr_mask: crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>,
    #[doc = "0x50 - SRSS ADFT control register This register can be used only when in a test mode entered through an XRES:DFT:* key. It provides direct control over and visibility of the SRSS power and reference circuits. Note that act_reg_en is controlled through PWR_CONTROL.EXT_VCCD. It is possible to cause behavior that is normally considered illegal, such as disabling a circuit without regard for dependencies. Engineering only. Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
    pub srss_adft_control: crate::Reg<srss_adft_control::SRSS_ADFT_CONTROL_SPEC>,
    #[doc = "0x54 - Reset Cause Observation Register Indicates the cause for the latest reset(s) that occurred in the system. Note that resets due to power up and brown-outs below state retention voltages in regulated and unregulated domains cannot be distinguished from eachother. All bits in this register assert when the corresponding reset cause occurs and must be cleared by firmware. These bits are cleared by hardware only during XRES, POR or after a detected brown-out."]
    pub res_cause: crate::Reg<res_cause::RES_CAUSE_SPEC>,
    #[doc = "0x58 - Reset DFT Register Controls the DFT options for the reset system. Writes to this register are ignored and settings in this register have no effect unless DFT is enabled through a XRES:DFT:* key (see SAS for details). Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
    pub res_dft: crate::Reg<res_dft::RES_DFT_SPEC>,
    _reserved23: [u8; 0x0ea4],
    #[doc = "0xf00 - Bandgap Trim Register Trim bits for Reference System. Entire register is engineering only."]
    pub pwr_bg_trim1: crate::Reg<pwr_bg_trim1::PWR_BG_TRIM1_SPEC>,
    #[doc = "0xf04 - Bandgap Trim Register Trim bits for Reference System. Entire register is engineering only."]
    pub pwr_bg_trim2: crate::Reg<pwr_bg_trim2::PWR_BG_TRIM2_SPEC>,
    #[doc = "0xf08 - IMO Frequency Select Register Selects the operating frequency of the IMO"]
    pub clk_imo_select: crate::Reg<clk_imo_select::CLK_IMO_SELECT_SPEC>,
    #[doc = "0xf0c - IMO Trim Register Trims IMO frequency to within datasheet accuracy. Must be applied"]
    pub clk_imo_trim1: crate::Reg<clk_imo_trim1::CLK_IMO_TRIM1_SPEC>,
    #[doc = "0xf10 - IMO Trim Register IMO Trim Bits. Entire register is engineering only."]
    pub clk_imo_trim2: crate::Reg<clk_imo_trim2::CLK_IMO_TRIM2_SPEC>,
    #[doc = "0xf14 - Power System Trim Register Power System Trim Bits. Entire register is engineering only."]
    pub pwr_pwrsys_trim1: crate::Reg<pwr_pwrsys_trim1::PWR_PWRSYS_TRIM1_SPEC>,
    #[doc = "0xf18 - IMO Trim Register IMO Trim Bits. Entire register is engineering only."]
    pub clk_imo_trim3: crate::Reg<clk_imo_trim3::CLK_IMO_TRIM3_SPEC>,
}
#[doc = "PWR_CONTROL register accessor: an alias for `Reg<PWR_CONTROL_SPEC>`"]
pub type PWR_CONTROL = crate::Reg<pwr_control::PWR_CONTROL_SPEC>;
#[doc = "Power Mode Control Controls the device power mode options and allows observation of current state."]
pub mod pwr_control;
#[doc = "PWR_KEY_DELAY register accessor: an alias for `Reg<PWR_KEY_DELAY_SPEC>`"]
pub type PWR_KEY_DELAY = crate::Reg<pwr_key_delay::PWR_KEY_DELAY_SPEC>;
#[doc = "Power System Key&Delay Register"]
pub mod pwr_key_delay;
#[doc = "PWR_ADFT_SELECT register accessor: an alias for `Reg<PWR_ADFT_SELECT_SPEC>`"]
pub type PWR_ADFT_SELECT = crate::Reg<pwr_adft_select::PWR_ADFT_SELECT_SPEC>;
#[doc = "Power ADFT Mode Selection Register Controls System Resources ADFT mode settings and observability. Writes to this register are ignored and settings in this register have no effect unless the part is in a XRES key selected DfT mode. Entire register is engineering only. Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
pub mod pwr_adft_select;
#[doc = "PWR_DDFT_SELECT register accessor: an alias for `Reg<PWR_DDFT_SELECT_SPEC>`"]
pub type PWR_DDFT_SELECT = crate::Reg<pwr_ddft_select::PWR_DDFT_SELECT_SPEC>;
#[doc = "Power DDFT Mode Selection Register Selects the signal sources output to the DDFT outputs of the power subsystem. Entire register is engineering only."]
pub mod pwr_ddft_select;
#[doc = "PWR_DDFT_XRES register accessor: an alias for `Reg<PWR_DDFT_XRES_SPEC>`"]
pub type PWR_DDFT_XRES = crate::Reg<pwr_ddft_xres::PWR_DDFT_XRES_SPEC>;
#[doc = "XRES DfT Key observer logic test register This register is used to test the XRES TestMode key logic. It allows a test routine (firmware or ATE driven) to stimulate the key listener and observe its functionality. Extreme case must be taken in these tests, since they will result in actual test mode entry. For example, shifting in a scan mode key, will transition the system into scan mode immediately. Note that test_scan_mode is not observable in this register for that reason."]
pub mod pwr_ddft_xres;
#[doc = "TST_MODE register accessor: an alias for `Reg<TST_MODE_SPEC>`"]
pub type TST_MODE = crate::Reg<tst_mode::TST_MODE_SPEC>;
#[doc = "Test Mode Control Register Controls primary test mode. This is a single bit that can be written directly from the ATE/Programmer in any protection mode. It's main function is to signal to the Boot ROM that normal firmware execution is not to commence after boot is complete. Instead the Boot ROM will enter a wait loop for system commands."]
pub mod tst_mode;
#[doc = "TST_DDFT_CTRL register accessor: an alias for `Reg<TST_DDFT_CTRL_SPEC>`"]
pub type TST_DDFT_CTRL = crate::Reg<tst_ddft_ctrl::TST_DDFT_CTRL_SPEC>;
#[doc = "Digital DFT Control Register Controls system level DDFT observability muxes and comparators."]
pub mod tst_ddft_ctrl;
#[doc = "TST_TRIM_CNTR1 register accessor: an alias for `Reg<TST_TRIM_CNTR1_SPEC>`"]
pub type TST_TRIM_CNTR1 = crate::Reg<tst_trim_cntr1::TST_TRIM_CNTR1_SPEC>;
#[doc = "IMO trim down-counter and status (clk_sys)"]
pub mod tst_trim_cntr1;
#[doc = "TST_TRIM_CNTR2 register accessor: an alias for `Reg<TST_TRIM_CNTR2_SPEC>`"]
pub type TST_TRIM_CNTR2 = crate::Reg<tst_trim_cntr2::TST_TRIM_CNTR2_SPEC>;
#[doc = "IMO trim up-counter (ddft)"]
pub mod tst_trim_cntr2;
#[doc = "TST_ADFT_CTRL register accessor: an alias for `Reg<TST_ADFT_CTRL_SPEC>`"]
pub type TST_ADFT_CTRL = crate::Reg<tst_adft_ctrl::TST_ADFT_CTRL_SPEC>;
#[doc = "ADFT buffer/comparator control register Controls System Resources ADFT mode settings and observability. Writes to this register are ignored and settings in this register have no effect unless the part is in a XRES key selected DfT mode. Entire register is engineering only. Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
pub mod tst_adft_ctrl;
#[doc = "CLK_SELECT register accessor: an alias for `Reg<CLK_SELECT_SPEC>`"]
pub type CLK_SELECT = crate::Reg<clk_select::CLK_SELECT_SPEC>;
#[doc = "Clock Select Register Configures direction of all clock multiplexers and selectors. See Section 20.3 in SAS for details on clock network topology. See PAS for DSI signal connectivity list."]
pub mod clk_select;
#[doc = "CLK_ILO_CONFIG register accessor: an alias for `Reg<CLK_ILO_CONFIG_SPEC>`"]
pub type CLK_ILO_CONFIG = crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>;
#[doc = "ILO Configuration Internal slow speed R/C oscillator (32kHz) configuration register. Note: writes to this register are ignored when WDT_DISABLE_KEY is not set to the magic value defined for it."]
pub mod clk_ilo_config;
#[doc = "CLK_IMO_CONFIG register accessor: an alias for `Reg<CLK_IMO_CONFIG_SPEC>`"]
pub type CLK_IMO_CONFIG = crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>;
#[doc = "IMO Configuration Internal high speed R/C oscillator configuration register. Note that this oscillator comes up active on power up. The oscillator provides the primary system clock (HFCLK) on power up until firmware configures differently. This oscillator is also used before system start to count out power up delays. This is done in fast IMO (FIMO) mode that does not require any external references and runs at a fixed 12MHz."]
pub mod clk_imo_config;
#[doc = "CLK_DFT_SELECT register accessor: an alias for `Reg<CLK_DFT_SELECT_SPEC>`"]
pub type CLK_DFT_SELECT = crate::Reg<clk_dft_select::CLK_DFT_SELECT_SPEC>;
#[doc = "Clock DFT Mode Selection Register Selects which clock signals to bring out to to DFT pins. Two signals can be selected to enable comparison of clocks. Clocks can be divided down to deal with slower equipment and I/Os. See TST_DFT_SELECT for details on bringing these pins out. Entire register is engineering only."]
pub mod clk_dft_select;
#[doc = "WDT_DISABLE_KEY register accessor: an alias for `Reg<WDT_DISABLE_KEY_SPEC>`"]
pub type WDT_DISABLE_KEY = crate::Reg<wdt_disable_key::WDT_DISABLE_KEY_SPEC>;
#[doc = "Watchdog Disable Key Register This key can be used to disable the watchdog timer reset generation in applications that do not require absolute brown-out safety and do not want to deal with the hassle of feeding the watchdog regularly. Setting the key will also enable the CLK_ILO_CONFIG.ENABLE bit to be effective. It will not have any other effect, i.e. the WDT timer/interrupt functionality can still be used."]
pub mod wdt_disable_key;
#[doc = "WDT_COUNTER register accessor: an alias for `Reg<WDT_COUNTER_SPEC>`"]
pub type WDT_COUNTER = crate::Reg<wdt_counter::WDT_COUNTER_SPEC>;
#[doc = "Watchdog Counter Register Provides actual counter value for watchdog counter. Watchdog counter always counts up, is free-running and is clocked using clk_lf."]
pub mod wdt_counter;
#[doc = "WDT_MATCH register accessor: an alias for `Reg<WDT_MATCH_SPEC>`"]
pub type WDT_MATCH = crate::Reg<wdt_match::WDT_MATCH_SPEC>;
#[doc = "Watchdog Match Register Firmware provided match value that is compared against WDT_COUNTER. The expectation is that firmware modifies this register after each match as part of the WDT interrupt service routine."]
pub mod wdt_match;
#[doc = "SRSS_INTR register accessor: an alias for `Reg<SRSS_INTR_SPEC>`"]
pub type SRSS_INTR = crate::Reg<srss_intr::SRSS_INTR_SPEC>;
#[doc = "SRSS Interrupt Register The intent is that this register is cleared for every WDT interrupt under all circumstances, including when the system is in DeepSleep."]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET register accessor: an alias for `Reg<SRSS_INTR_SET_SPEC>`"]
pub type SRSS_INTR_SET = crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>;
#[doc = "SRSS Interrupt Set Register Can be used to set interrupts for firmware testing. Note that SET functionality is not available for WDT."]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK register accessor: an alias for `Reg<SRSS_INTR_MASK_SPEC>`"]
pub type SRSS_INTR_MASK = crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>;
#[doc = "SRSS Interrupt Mask Register Controls whether interrupt is forwarded to CPU."]
pub mod srss_intr_mask;
#[doc = "SRSS_ADFT_CONTROL register accessor: an alias for `Reg<SRSS_ADFT_CONTROL_SPEC>`"]
pub type SRSS_ADFT_CONTROL = crate::Reg<srss_adft_control::SRSS_ADFT_CONTROL_SPEC>;
#[doc = "SRSS ADFT control register This register can be used only when in a test mode entered through an XRES:DFT:* key. It provides direct control over and visibility of the SRSS power and reference circuits. Note that act_reg_en is controlled through PWR_CONTROL.EXT_VCCD. It is possible to cause behavior that is normally considered illegal, such as disabling a circuit without regard for dependencies. Engineering only. Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
pub mod srss_adft_control;
#[doc = "RES_CAUSE register accessor: an alias for `Reg<RES_CAUSE_SPEC>`"]
pub type RES_CAUSE = crate::Reg<res_cause::RES_CAUSE_SPEC>;
#[doc = "Reset Cause Observation Register Indicates the cause for the latest reset(s) that occurred in the system. Note that resets due to power up and brown-outs below state retention voltages in regulated and unregulated domains cannot be distinguished from eachother. All bits in this register assert when the corresponding reset cause occurs and must be cleared by firmware. These bits are cleared by hardware only during XRES, POR or after a detected brown-out."]
pub mod res_cause;
#[doc = "RES_DFT register accessor: an alias for `Reg<RES_DFT_SPEC>`"]
pub type RES_DFT = crate::Reg<res_dft::RES_DFT_SPEC>;
#[doc = "Reset DFT Register Controls the DFT options for the reset system. Writes to this register are ignored and settings in this register have no effect unless DFT is enabled through a XRES:DFT:* key (see SAS for details). Note that PWR_DDFT_XRES can be used to enter an XRES key if XRES key sequence is not desired."]
pub mod res_dft;
#[doc = "PWR_BG_TRIM1 register accessor: an alias for `Reg<PWR_BG_TRIM1_SPEC>`"]
pub type PWR_BG_TRIM1 = crate::Reg<pwr_bg_trim1::PWR_BG_TRIM1_SPEC>;
#[doc = "Bandgap Trim Register Trim bits for Reference System. Entire register is engineering only."]
pub mod pwr_bg_trim1;
#[doc = "PWR_BG_TRIM2 register accessor: an alias for `Reg<PWR_BG_TRIM2_SPEC>`"]
pub type PWR_BG_TRIM2 = crate::Reg<pwr_bg_trim2::PWR_BG_TRIM2_SPEC>;
#[doc = "Bandgap Trim Register Trim bits for Reference System. Entire register is engineering only."]
pub mod pwr_bg_trim2;
#[doc = "CLK_IMO_SELECT register accessor: an alias for `Reg<CLK_IMO_SELECT_SPEC>`"]
pub type CLK_IMO_SELECT = crate::Reg<clk_imo_select::CLK_IMO_SELECT_SPEC>;
#[doc = "IMO Frequency Select Register Selects the operating frequency of the IMO"]
pub mod clk_imo_select;
#[doc = "CLK_IMO_TRIM1 register accessor: an alias for `Reg<CLK_IMO_TRIM1_SPEC>`"]
pub type CLK_IMO_TRIM1 = crate::Reg<clk_imo_trim1::CLK_IMO_TRIM1_SPEC>;
#[doc = "IMO Trim Register Trims IMO frequency to within datasheet accuracy. Must be applied"]
pub mod clk_imo_trim1;
#[doc = "CLK_IMO_TRIM2 register accessor: an alias for `Reg<CLK_IMO_TRIM2_SPEC>`"]
pub type CLK_IMO_TRIM2 = crate::Reg<clk_imo_trim2::CLK_IMO_TRIM2_SPEC>;
#[doc = "IMO Trim Register IMO Trim Bits. Entire register is engineering only."]
pub mod clk_imo_trim2;
#[doc = "PWR_PWRSYS_TRIM1 register accessor: an alias for `Reg<PWR_PWRSYS_TRIM1_SPEC>`"]
pub type PWR_PWRSYS_TRIM1 = crate::Reg<pwr_pwrsys_trim1::PWR_PWRSYS_TRIM1_SPEC>;
#[doc = "Power System Trim Register Power System Trim Bits. Entire register is engineering only."]
pub mod pwr_pwrsys_trim1;
#[doc = "CLK_IMO_TRIM3 register accessor: an alias for `Reg<CLK_IMO_TRIM3_SPEC>`"]
pub type CLK_IMO_TRIM3 = crate::Reg<clk_imo_trim3::CLK_IMO_TRIM3_SPEC>;
#[doc = "IMO Trim Register IMO Trim Bits. Entire register is engineering only."]
pub mod clk_imo_trim3;
