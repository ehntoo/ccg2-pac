#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub pwr_control: crate::Reg<pwr_control::PWR_CONTROL_SPEC>,
    #[doc = "0x04 - "]
    pub pwr_key_delay: crate::Reg<pwr_key_delay::PWR_KEY_DELAY_SPEC>,
    #[doc = "0x08 - "]
    pub pwr_adft_select: crate::Reg<pwr_adft_select::PWR_ADFT_SELECT_SPEC>,
    #[doc = "0x0c - "]
    pub pwr_ddft_select: crate::Reg<pwr_ddft_select::PWR_DDFT_SELECT_SPEC>,
    #[doc = "0x10 - "]
    pub pwr_ddft_xres: crate::Reg<pwr_ddft_xres::PWR_DDFT_XRES_SPEC>,
    #[doc = "0x14 - "]
    pub tst_mode: crate::Reg<tst_mode::TST_MODE_SPEC>,
    #[doc = "0x18 - "]
    pub tst_ddft_ctrl: crate::Reg<tst_ddft_ctrl::TST_DDFT_CTRL_SPEC>,
    #[doc = "0x1c - "]
    pub tst_trim_cntr1: crate::Reg<tst_trim_cntr1::TST_TRIM_CNTR1_SPEC>,
    #[doc = "0x20 - "]
    pub tst_trim_cntr2: crate::Reg<tst_trim_cntr2::TST_TRIM_CNTR2_SPEC>,
    #[doc = "0x24 - "]
    pub tst_adft_ctrl: crate::Reg<tst_adft_ctrl::TST_ADFT_CTRL_SPEC>,
    #[doc = "0x28 - "]
    pub clk_select: crate::Reg<clk_select::CLK_SELECT_SPEC>,
    #[doc = "0x2c - "]
    pub clk_ilo_config: crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>,
    #[doc = "0x30 - "]
    pub clk_imo_config: crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>,
    #[doc = "0x34 - "]
    pub clk_dft_select: crate::Reg<clk_dft_select::CLK_DFT_SELECT_SPEC>,
    #[doc = "0x38 - "]
    pub wdt_disable_key: crate::Reg<wdt_disable_key::WDT_DISABLE_KEY_SPEC>,
    #[doc = "0x3c - "]
    pub wdt_counter: crate::Reg<wdt_counter::WDT_COUNTER_SPEC>,
    #[doc = "0x40 - "]
    pub wdt_match: crate::Reg<wdt_match::WDT_MATCH_SPEC>,
    #[doc = "0x44 - "]
    pub srss_intr: crate::Reg<srss_intr::SRSS_INTR_SPEC>,
    #[doc = "0x48 - "]
    pub srss_intr_set: crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>,
    #[doc = "0x4c - "]
    pub srss_intr_mask: crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>,
    #[doc = "0x50 - "]
    pub srss_adft_control: crate::Reg<srss_adft_control::SRSS_ADFT_CONTROL_SPEC>,
    #[doc = "0x54 - "]
    pub res_cause: crate::Reg<res_cause::RES_CAUSE_SPEC>,
    #[doc = "0x58 - "]
    pub res_dft: crate::Reg<res_dft::RES_DFT_SPEC>,
    _reserved23: [u8; 0x0ea4],
    #[doc = "0xf00 - "]
    pub pwr_bg_trim1: crate::Reg<pwr_bg_trim1::PWR_BG_TRIM1_SPEC>,
    #[doc = "0xf04 - "]
    pub pwr_bg_trim2: crate::Reg<pwr_bg_trim2::PWR_BG_TRIM2_SPEC>,
    #[doc = "0xf08 - "]
    pub clk_imo_select: crate::Reg<clk_imo_select::CLK_IMO_SELECT_SPEC>,
    #[doc = "0xf0c - "]
    pub clk_imo_trim1: crate::Reg<clk_imo_trim1::CLK_IMO_TRIM1_SPEC>,
    #[doc = "0xf10 - "]
    pub clk_imo_trim2: crate::Reg<clk_imo_trim2::CLK_IMO_TRIM2_SPEC>,
    #[doc = "0xf14 - "]
    pub pwr_pwrsys_trim1: crate::Reg<pwr_pwrsys_trim1::PWR_PWRSYS_TRIM1_SPEC>,
    #[doc = "0xf18 - "]
    pub clk_imo_trim3: crate::Reg<clk_imo_trim3::CLK_IMO_TRIM3_SPEC>,
}
#[doc = "PWR_CONTROL register accessor: an alias for `Reg<PWR_CONTROL_SPEC>`"]
pub type PWR_CONTROL = crate::Reg<pwr_control::PWR_CONTROL_SPEC>;
#[doc = ""]
pub mod pwr_control;
#[doc = "PWR_KEY_DELAY register accessor: an alias for `Reg<PWR_KEY_DELAY_SPEC>`"]
pub type PWR_KEY_DELAY = crate::Reg<pwr_key_delay::PWR_KEY_DELAY_SPEC>;
#[doc = ""]
pub mod pwr_key_delay;
#[doc = "PWR_ADFT_SELECT register accessor: an alias for `Reg<PWR_ADFT_SELECT_SPEC>`"]
pub type PWR_ADFT_SELECT = crate::Reg<pwr_adft_select::PWR_ADFT_SELECT_SPEC>;
#[doc = ""]
pub mod pwr_adft_select;
#[doc = "PWR_DDFT_SELECT register accessor: an alias for `Reg<PWR_DDFT_SELECT_SPEC>`"]
pub type PWR_DDFT_SELECT = crate::Reg<pwr_ddft_select::PWR_DDFT_SELECT_SPEC>;
#[doc = ""]
pub mod pwr_ddft_select;
#[doc = "PWR_DDFT_XRES register accessor: an alias for `Reg<PWR_DDFT_XRES_SPEC>`"]
pub type PWR_DDFT_XRES = crate::Reg<pwr_ddft_xres::PWR_DDFT_XRES_SPEC>;
#[doc = ""]
pub mod pwr_ddft_xres;
#[doc = "TST_MODE register accessor: an alias for `Reg<TST_MODE_SPEC>`"]
pub type TST_MODE = crate::Reg<tst_mode::TST_MODE_SPEC>;
#[doc = ""]
pub mod tst_mode;
#[doc = "TST_DDFT_CTRL register accessor: an alias for `Reg<TST_DDFT_CTRL_SPEC>`"]
pub type TST_DDFT_CTRL = crate::Reg<tst_ddft_ctrl::TST_DDFT_CTRL_SPEC>;
#[doc = ""]
pub mod tst_ddft_ctrl;
#[doc = "TST_TRIM_CNTR1 register accessor: an alias for `Reg<TST_TRIM_CNTR1_SPEC>`"]
pub type TST_TRIM_CNTR1 = crate::Reg<tst_trim_cntr1::TST_TRIM_CNTR1_SPEC>;
#[doc = ""]
pub mod tst_trim_cntr1;
#[doc = "TST_TRIM_CNTR2 register accessor: an alias for `Reg<TST_TRIM_CNTR2_SPEC>`"]
pub type TST_TRIM_CNTR2 = crate::Reg<tst_trim_cntr2::TST_TRIM_CNTR2_SPEC>;
#[doc = ""]
pub mod tst_trim_cntr2;
#[doc = "TST_ADFT_CTRL register accessor: an alias for `Reg<TST_ADFT_CTRL_SPEC>`"]
pub type TST_ADFT_CTRL = crate::Reg<tst_adft_ctrl::TST_ADFT_CTRL_SPEC>;
#[doc = ""]
pub mod tst_adft_ctrl;
#[doc = "CLK_SELECT register accessor: an alias for `Reg<CLK_SELECT_SPEC>`"]
pub type CLK_SELECT = crate::Reg<clk_select::CLK_SELECT_SPEC>;
#[doc = ""]
pub mod clk_select;
#[doc = "CLK_ILO_CONFIG register accessor: an alias for `Reg<CLK_ILO_CONFIG_SPEC>`"]
pub type CLK_ILO_CONFIG = crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>;
#[doc = ""]
pub mod clk_ilo_config;
#[doc = "CLK_IMO_CONFIG register accessor: an alias for `Reg<CLK_IMO_CONFIG_SPEC>`"]
pub type CLK_IMO_CONFIG = crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>;
#[doc = ""]
pub mod clk_imo_config;
#[doc = "CLK_DFT_SELECT register accessor: an alias for `Reg<CLK_DFT_SELECT_SPEC>`"]
pub type CLK_DFT_SELECT = crate::Reg<clk_dft_select::CLK_DFT_SELECT_SPEC>;
#[doc = ""]
pub mod clk_dft_select;
#[doc = "WDT_DISABLE_KEY register accessor: an alias for `Reg<WDT_DISABLE_KEY_SPEC>`"]
pub type WDT_DISABLE_KEY = crate::Reg<wdt_disable_key::WDT_DISABLE_KEY_SPEC>;
#[doc = ""]
pub mod wdt_disable_key;
#[doc = "WDT_COUNTER register accessor: an alias for `Reg<WDT_COUNTER_SPEC>`"]
pub type WDT_COUNTER = crate::Reg<wdt_counter::WDT_COUNTER_SPEC>;
#[doc = ""]
pub mod wdt_counter;
#[doc = "WDT_MATCH register accessor: an alias for `Reg<WDT_MATCH_SPEC>`"]
pub type WDT_MATCH = crate::Reg<wdt_match::WDT_MATCH_SPEC>;
#[doc = ""]
pub mod wdt_match;
#[doc = "SRSS_INTR register accessor: an alias for `Reg<SRSS_INTR_SPEC>`"]
pub type SRSS_INTR = crate::Reg<srss_intr::SRSS_INTR_SPEC>;
#[doc = ""]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET register accessor: an alias for `Reg<SRSS_INTR_SET_SPEC>`"]
pub type SRSS_INTR_SET = crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>;
#[doc = ""]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK register accessor: an alias for `Reg<SRSS_INTR_MASK_SPEC>`"]
pub type SRSS_INTR_MASK = crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>;
#[doc = ""]
pub mod srss_intr_mask;
#[doc = "SRSS_ADFT_CONTROL register accessor: an alias for `Reg<SRSS_ADFT_CONTROL_SPEC>`"]
pub type SRSS_ADFT_CONTROL = crate::Reg<srss_adft_control::SRSS_ADFT_CONTROL_SPEC>;
#[doc = ""]
pub mod srss_adft_control;
#[doc = "RES_CAUSE register accessor: an alias for `Reg<RES_CAUSE_SPEC>`"]
pub type RES_CAUSE = crate::Reg<res_cause::RES_CAUSE_SPEC>;
#[doc = ""]
pub mod res_cause;
#[doc = "RES_DFT register accessor: an alias for `Reg<RES_DFT_SPEC>`"]
pub type RES_DFT = crate::Reg<res_dft::RES_DFT_SPEC>;
#[doc = ""]
pub mod res_dft;
#[doc = "PWR_BG_TRIM1 register accessor: an alias for `Reg<PWR_BG_TRIM1_SPEC>`"]
pub type PWR_BG_TRIM1 = crate::Reg<pwr_bg_trim1::PWR_BG_TRIM1_SPEC>;
#[doc = ""]
pub mod pwr_bg_trim1;
#[doc = "PWR_BG_TRIM2 register accessor: an alias for `Reg<PWR_BG_TRIM2_SPEC>`"]
pub type PWR_BG_TRIM2 = crate::Reg<pwr_bg_trim2::PWR_BG_TRIM2_SPEC>;
#[doc = ""]
pub mod pwr_bg_trim2;
#[doc = "CLK_IMO_SELECT register accessor: an alias for `Reg<CLK_IMO_SELECT_SPEC>`"]
pub type CLK_IMO_SELECT = crate::Reg<clk_imo_select::CLK_IMO_SELECT_SPEC>;
#[doc = ""]
pub mod clk_imo_select;
#[doc = "CLK_IMO_TRIM1 register accessor: an alias for `Reg<CLK_IMO_TRIM1_SPEC>`"]
pub type CLK_IMO_TRIM1 = crate::Reg<clk_imo_trim1::CLK_IMO_TRIM1_SPEC>;
#[doc = ""]
pub mod clk_imo_trim1;
#[doc = "CLK_IMO_TRIM2 register accessor: an alias for `Reg<CLK_IMO_TRIM2_SPEC>`"]
pub type CLK_IMO_TRIM2 = crate::Reg<clk_imo_trim2::CLK_IMO_TRIM2_SPEC>;
#[doc = ""]
pub mod clk_imo_trim2;
#[doc = "PWR_PWRSYS_TRIM1 register accessor: an alias for `Reg<PWR_PWRSYS_TRIM1_SPEC>`"]
pub type PWR_PWRSYS_TRIM1 = crate::Reg<pwr_pwrsys_trim1::PWR_PWRSYS_TRIM1_SPEC>;
#[doc = ""]
pub mod pwr_pwrsys_trim1;
#[doc = "CLK_IMO_TRIM3 register accessor: an alias for `Reg<CLK_IMO_TRIM3_SPEC>`"]
pub type CLK_IMO_TRIM3 = crate::Reg<clk_imo_trim3::CLK_IMO_TRIM3_SPEC>;
#[doc = ""]
pub mod clk_imo_trim3;
