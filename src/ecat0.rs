#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Type of EtherCAT Controller"]
    pub type_: TYPE,
    #[doc = "0x01 - Revision of EtherCAT Controller"]
    pub revision: REVISION,
    #[doc = "0x02 - Build Version"]
    pub build: BUILD,
    #[doc = "0x04 - FMMUs Supported"]
    pub fmmu_num: FMMU_NUM,
    #[doc = "0x05 - SyncManagers Supported"]
    pub sync_manager: SYNC_MANAGER,
    #[doc = "0x06 - RAM Size"]
    pub ram_size: RAM_SIZE,
    #[doc = "0x07 - Port Descriptor"]
    pub port_desc: PORT_DESC,
    #[doc = "0x08 - ESC Features Supported"]
    pub feature: FEATURE,
    _reserved8: [u8; 0x06],
    #[doc = "0x10 - Configured Station Address"]
    pub station_adr: STATION_ADR,
    #[doc = "0x12 - Configured Station Alias"]
    pub station_alias: STATION_ALIAS,
    _reserved10: [u8; 0x0c],
    #[doc = "0x20 - Write Register Enable"]
    pub wr_reg_enable: WR_REG_ENABLE,
    #[doc = "0x21 - Write Register Protection"]
    pub wr_reg_protect: WR_REG_PROTECT,
    _reserved12: [u8; 0x0e],
    #[doc = "0x30 - ESC Write Enable"]
    pub esc_wr_enable: ESC_WR_ENABLE,
    #[doc = "0x31 - ESC Write Protection"]
    pub esc_wr_protect: ESC_WR_PROTECT,
    _reserved14: [u8; 0x0e],
    _reserved_14_readmode_esc_reset_ecat: [u8; 0x01],
    _reserved_15_readmode_esc_reset_pdi: [u8; 0x01],
    _reserved16: [u8; 0xbe],
    #[doc = "0x100 - ESC DL Control"]
    pub esc_dl_control: ESC_DL_CONTROL,
    _reserved17: [u8; 0x04],
    #[doc = "0x108 - Physical Read/Write Offset"]
    pub physical_rw_offset: PHYSICAL_RW_OFFSET,
    _reserved18: [u8; 0x06],
    #[doc = "0x110 - ESC DL Status"]
    pub esc_dl_status: ESC_DL_STATUS,
    _reserved19: [u8; 0x0e],
    #[doc = "0x120 - AL Control"]
    pub al_control: AL_CONTROL,
    _reserved20: [u8; 0x0e],
    #[doc = "0x130 - AL Status"]
    pub al_status: AL_STATUS,
    _reserved21: [u8; 0x02],
    #[doc = "0x134 - AL Status Code"]
    pub al_status_code: AL_STATUS_CODE,
    _reserved22: [u8; 0x02],
    #[doc = "0x138 - RUN LED Override"]
    pub run_led: RUN_LED,
    #[doc = "0x139 - RUN ERR Override"]
    pub err_led: ERR_LED,
    _reserved24: [u8; 0x06],
    #[doc = "0x140 - PDI Control"]
    pub pdi_control: PDI_CONTROL,
    #[doc = "0x141 - ESC Configuration"]
    pub esc_config: ESC_CONFIG,
    _reserved26: [u8; 0x0e],
    #[doc = "0x150 - PDI Control"]
    pub pdi_config: PDI_CONFIG,
    #[doc = "0x151 - Sync/Latch\\[1:0\\]
PDI Configuration"]
    pub sync_latch_config: SYNC_LATCH_CONFIG,
    #[doc = "0x152 - PDI Synchronous Microcontroller extended Configuration"]
    pub pdi_ext_config: PDI_EXT_CONFIG,
    _reserved29: [u8; 0xac],
    #[doc = "0x200 - ECAT Event Mask"]
    pub event_mask: EVENT_MASK,
    _reserved30: [u8; 0x02],
    #[doc = "0x204 - PDI AL Event Mask"]
    pub al_event_mask: AL_EVENT_MASK,
    _reserved31: [u8; 0x08],
    #[doc = "0x210 - ECAT Event Request"]
    pub event_req: EVENT_REQ,
    _reserved32: [u8; 0x0e],
    #[doc = "0x220 - AL Event Request"]
    pub al_event_req: AL_EVENT_REQ,
    _reserved33: [u8; 0xdc],
    #[doc = "0x300 - RX Error Counter Port 0"]
    pub rx_err_count0: RX_ERR_COUNT0,
    #[doc = "0x302 - RX Error Counter Port 1"]
    pub rx_err_count1: RX_ERR_COUNT1,
    _reserved35: [u8; 0x04],
    #[doc = "0x308 - Forwarded RX Error Counter Port 0"]
    pub fwd_rx_err_count0: FWD_RX_ERR_COUNT0,
    #[doc = "0x309 - Forwarded RX Error Counter Port 1"]
    pub fwd_rx_err_count1: FWD_RX_ERR_COUNT1,
    _reserved37: [u8; 0x02],
    #[doc = "0x30c - ECAT Processing Unit Error Counter"]
    pub proc_err_count: PROC_ERR_COUNT,
    #[doc = "0x30d - PDI Error Counter"]
    pub pdi_err_count: PDI_ERR_COUNT,
    _reserved39: [u8; 0x02],
    #[doc = "0x310 - Lost Link Counter Port 0"]
    pub lost_link_count0: LOST_LINK_COUNT0,
    #[doc = "0x311 - Lost Link Counter Port 1"]
    pub lost_link_count1: LOST_LINK_COUNT1,
    _reserved41: [u8; 0xee],
    #[doc = "0x400 - Watchdog Divider"]
    pub wd_divide: WD_DIVIDE,
    _reserved42: [u8; 0x0e],
    #[doc = "0x410 - Watchdog Time PDI"]
    pub wd_time_pdi: WD_TIME_PDI,
    _reserved43: [u8; 0x0e],
    #[doc = "0x420 - Watchdog Time Process Data"]
    pub wd_time_pdata: WD_TIME_PDATA,
    _reserved44: [u8; 0x1e],
    #[doc = "0x440 - Watchdog Status Process Data"]
    pub wd_stat_pdata: WD_STAT_PDATA,
    #[doc = "0x442 - Watchdog Counter Process Data"]
    pub wd_count_pdata: WD_COUNT_PDATA,
    #[doc = "0x443 - Watchdog Counter PDI"]
    pub wd_count_pdi: WD_COUNT_PDI,
    _reserved47: [u8; 0xbc],
    #[doc = "0x500 - EEPROM Configuration"]
    pub eep_conf: EEP_CONF,
    #[doc = "0x501 - EEPROM PDI Access State"]
    pub eep_state: EEP_STATE,
    #[doc = "0x502 - EEPROM Control/Status"]
    pub eep_cont_stat: EEP_CONT_STAT,
    #[doc = "0x504 - EEPROM Address"]
    pub eep_adr: EEP_ADR,
    #[doc = "0x508..0x510 - EEPROM Read/Write data"]
    pub eep_data: [EEP_DATA; 2],
    #[doc = "0x510 - MII Management Control/Status"]
    pub mii_cont_stat: MII_CONT_STAT,
    #[doc = "0x512 - PHY Address"]
    pub mii_phy_adr: MII_PHY_ADR,
    #[doc = "0x513 - PHY Register Address"]
    pub mii_phy_reg_adr: MII_PHY_REG_ADR,
    #[doc = "0x514 - PHY Data"]
    pub mii_phy_data: MII_PHY_DATA,
    #[doc = "0x516 - MII ECAT ACS STATE"]
    pub mii_ecat_acs_state: MII_ECAT_ACS_STATE,
    #[doc = "0x517 - MII PDI ACS STATE"]
    pub mii_pdi_acs_state: MII_PDI_ACS_STATE,
    _reserved58: [u8; 0x03e8],
    #[doc = "0x900 - Receive Time Port 0"]
    pub dc_rcv_time_port0: DC_RCV_TIME_PORT0,
    #[doc = "0x904 - Receive Time Port 1"]
    pub dc_rcv_time_port1: DC_RCV_TIME_PORT1,
    _reserved60: [u8; 0x08],
    _reserved_60_readmode_dc_sys_time: [u8; 0x08],
    #[doc = "0x918..0x920 - Local time of the beginning of a frame"]
    pub receive_time_pu: [RECEIVE_TIME_PU; 2],
    #[doc = "0x920..0x928 - Difference between local time and System Time"]
    pub dc_sys_time_offset: [DC_SYS_TIME_OFFSET; 2],
    #[doc = "0x928 - System Time Delay"]
    pub dc_sys_time_delay: DC_SYS_TIME_DELAY,
    #[doc = "0x92c - System Time Difference"]
    pub dc_sys_time_diff: DC_SYS_TIME_DIFF,
    #[doc = "0x930 - Speed Counter Start"]
    pub dc_speed_count_start: DC_SPEED_COUNT_START,
    #[doc = "0x932 - Speed Counter Diff"]
    pub dc_speed_count_diff: DC_SPEED_COUNT_DIFF,
    #[doc = "0x934 - System Time Difference Filter Depth"]
    pub dc_sys_time_fil_depth: DC_SYS_TIME_FIL_DEPTH,
    #[doc = "0x935 - Speed Counter Filter Depth"]
    pub dc_speed_count_fil_depth: DC_SPEED_COUNT_FIL_DEPTH,
    _reserved69: [u8; 0x4a],
    #[doc = "0x980 - Cyclic Unit Control"]
    pub dc_cyc_cont: DC_CYC_CONT,
    #[doc = "0x981 - Activation register"]
    pub dc_act: DC_ACT,
    #[doc = "0x982 - Pulse Length of SyncSignals"]
    pub dc_pulse_len: DC_PULSE_LEN,
    #[doc = "0x984 - Activation Status"]
    pub dc_act_stat: DC_ACT_STAT,
    _reserved73: [u8; 0x09],
    #[doc = "0x98e - SYNC0 Status"]
    pub dc_sync0_stat: DC_SYNC0_STAT,
    #[doc = "0x98f - SYNC1 Status"]
    pub dc_sync1_stat: DC_SYNC1_STAT,
    #[doc = "0x990..0x998 - Start Time Cyclic Operation"]
    pub dc_cyc_start_time: [DC_CYC_START_TIME; 2],
    #[doc = "0x998..0x9a0 - System time of next SYNC1 pulse in ns"]
    pub dc_next_sync1_pulse: [DC_NEXT_SYNC1_PULSE; 2],
    #[doc = "0x9a0 - SYNC0 Cycle Time"]
    pub dc_sync0_cyc_time: DC_SYNC0_CYC_TIME,
    #[doc = "0x9a4 - SYNC1 Cycle Time"]
    pub dc_sync1_cyc_time: DC_SYNC1_CYC_TIME,
    #[doc = "0x9a8 - Latch0 Control"]
    pub dc_latch0_cont: DC_LATCH0_CONT,
    #[doc = "0x9a9 - Latch1 Control"]
    pub dc_latch1_cont: DC_LATCH1_CONT,
    _reserved81: [u8; 0x04],
    #[doc = "0x9ae - Latch0 Status"]
    pub dc_latch0_stat: DC_LATCH0_STAT,
    #[doc = "0x9af - Latch1 Status"]
    pub dc_latch1_stat: DC_LATCH1_STAT,
    #[doc = "0x9b0..0x9b8 - Register captures System time at the positive edge of the Latch0 signal"]
    pub dc_latch0_time_pos: [DC_LATCH0_TIME_POS; 2],
    #[doc = "0x9b8..0x9c0 - Register captures System time at the negative edge of the Latch0 signal"]
    pub dc_latch0_time_neg: [DC_LATCH0_TIME_NEG; 2],
    #[doc = "0x9c0..0x9c8 - Register captures System time at the positive edge of the Latch1 signal"]
    pub dc_latch1_time_pos: [DC_LATCH1_TIME_POS; 2],
    #[doc = "0x9c8..0x9d0 - Register captures System time at the negative edge of the Latch1 signal"]
    pub dc_latch1_time_neg: [DC_LATCH1_TIME_NEG; 2],
    _reserved87: [u8; 0x20],
    #[doc = "0x9f0 - EtherCAT Buffer Change Event Time"]
    pub dc_ecat_cng_ev_time: DC_ECAT_CNG_EV_TIME,
    _reserved88: [u8; 0x04],
    #[doc = "0x9f8 - PDI Buffer Start Event Time"]
    pub dc_pdi_start_ev_time: DC_PDI_START_EV_TIME,
    #[doc = "0x9fc - PDI Buffer Change Event Time"]
    pub dc_pdi_cng_ev_time: DC_PDI_CNG_EV_TIME,
    _reserved90: [u8; 0x0400],
    #[doc = "0xe00 - ECAT0 Module ID"]
    pub id: ID,
    _reserved91: [u8; 0x04],
    #[doc = "0xe08 - ECAT0 Status"]
    pub status: STATUS,
}
impl RegisterBlock {
    #[doc = "0x40 - ESC Reset ECAT \\[READ Mode\\]"]
    #[inline(always)]
    pub const fn readmode_esc_reset_ecat(&self) -> &READMODE_ESC_RESET_ECAT {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x40 - ESC Reset ECAT \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_esc_reset_ecat(&self) -> &WRITEMODE_ESC_RESET_ECAT {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    #[doc = "0x41 - ESC Reset PDI \\[READ Mode\\]"]
    #[inline(always)]
    pub const fn readmode_esc_reset_pdi(&self) -> &READMODE_ESC_RESET_PDI {
        unsafe { &*(self as *const Self).cast::<u8>().add(65usize).cast() }
    }
    #[doc = "0x41 - ESC Reset PDI \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_esc_reset_pdi(&self) -> &WRITEMODE_ESC_RESET_PDI {
        unsafe { &*(self as *const Self).cast::<u8>().add(65usize).cast() }
    }
    #[doc = "0x910 - System Time \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_dc_sys_time(&self) -> &WRITEMODE_DC_SYS_TIME {
        unsafe { &*(self as *const Self).cast::<u8>().add(2320usize).cast() }
    }
    #[doc = "0x910..0x918 - System Time read access"]
    #[inline(always)]
    pub const fn readmode_dc_sys_time(&self) -> &[READMODE_DC_SYS_TIME; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(2320usize).cast() }
    }
}
#[doc = "TYPE (r) register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Type of EtherCAT Controller"]
pub mod type_;
#[doc = "REVISION (r) register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "Revision of EtherCAT Controller"]
pub mod revision;
#[doc = "BUILD (r) register accessor: an alias for `Reg<BUILD_SPEC>`"]
pub type BUILD = crate::Reg<build::BUILD_SPEC>;
#[doc = "Build Version"]
pub mod build;
#[doc = "FMMU_NUM (r) register accessor: an alias for `Reg<FMMU_NUM_SPEC>`"]
pub type FMMU_NUM = crate::Reg<fmmu_num::FMMU_NUM_SPEC>;
#[doc = "FMMUs Supported"]
pub mod fmmu_num;
#[doc = "SYNC_MANAGER (r) register accessor: an alias for `Reg<SYNC_MANAGER_SPEC>`"]
pub type SYNC_MANAGER = crate::Reg<sync_manager::SYNC_MANAGER_SPEC>;
#[doc = "SyncManagers Supported"]
pub mod sync_manager;
#[doc = "RAM_SIZE (r) register accessor: an alias for `Reg<RAM_SIZE_SPEC>`"]
pub type RAM_SIZE = crate::Reg<ram_size::RAM_SIZE_SPEC>;
#[doc = "RAM Size"]
pub mod ram_size;
#[doc = "PORT_DESC (r) register accessor: an alias for `Reg<PORT_DESC_SPEC>`"]
pub type PORT_DESC = crate::Reg<port_desc::PORT_DESC_SPEC>;
#[doc = "Port Descriptor"]
pub mod port_desc;
#[doc = "FEATURE (r) register accessor: an alias for `Reg<FEATURE_SPEC>`"]
pub type FEATURE = crate::Reg<feature::FEATURE_SPEC>;
#[doc = "ESC Features Supported"]
pub mod feature;
#[doc = "STATION_ADR (r) register accessor: an alias for `Reg<STATION_ADR_SPEC>`"]
pub type STATION_ADR = crate::Reg<station_adr::STATION_ADR_SPEC>;
#[doc = "Configured Station Address"]
pub mod station_adr;
#[doc = "STATION_ALIAS (rw) register accessor: an alias for `Reg<STATION_ALIAS_SPEC>`"]
pub type STATION_ALIAS = crate::Reg<station_alias::STATION_ALIAS_SPEC>;
#[doc = "Configured Station Alias"]
pub mod station_alias;
#[doc = "WR_REG_ENABLE (r) register accessor: an alias for `Reg<WR_REG_ENABLE_SPEC>`"]
pub type WR_REG_ENABLE = crate::Reg<wr_reg_enable::WR_REG_ENABLE_SPEC>;
#[doc = "Write Register Enable"]
pub mod wr_reg_enable;
#[doc = "WR_REG_PROTECT (r) register accessor: an alias for `Reg<WR_REG_PROTECT_SPEC>`"]
pub type WR_REG_PROTECT = crate::Reg<wr_reg_protect::WR_REG_PROTECT_SPEC>;
#[doc = "Write Register Protection"]
pub mod wr_reg_protect;
#[doc = "ESC_WR_ENABLE (r) register accessor: an alias for `Reg<ESC_WR_ENABLE_SPEC>`"]
pub type ESC_WR_ENABLE = crate::Reg<esc_wr_enable::ESC_WR_ENABLE_SPEC>;
#[doc = "ESC Write Enable"]
pub mod esc_wr_enable;
#[doc = "ESC_WR_PROTECT (r) register accessor: an alias for `Reg<ESC_WR_PROTECT_SPEC>`"]
pub type ESC_WR_PROTECT = crate::Reg<esc_wr_protect::ESC_WR_PROTECT_SPEC>;
#[doc = "ESC Write Protection"]
pub mod esc_wr_protect;
#[doc = "WRITEMode_ESC_RESET_ECAT (r) register accessor: an alias for `Reg<WRITEMODE_ESC_RESET_ECAT_SPEC>`"]
pub type WRITEMODE_ESC_RESET_ECAT = crate::Reg<writemode_esc_reset_ecat::WRITEMODE_ESC_RESET_ECAT_SPEC>;
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]"]
pub mod writemode_esc_reset_ecat;
#[doc = "READMode_ESC_RESET_ECAT (r) register accessor: an alias for `Reg<READMODE_ESC_RESET_ECAT_SPEC>`"]
pub type READMODE_ESC_RESET_ECAT = crate::Reg<readmode_esc_reset_ecat::READMODE_ESC_RESET_ECAT_SPEC>;
#[doc = "ESC Reset ECAT \\[READ Mode\\]"]
pub mod readmode_esc_reset_ecat;
#[doc = "WRITEMode_ESC_RESET_PDI (r) register accessor: an alias for `Reg<WRITEMODE_ESC_RESET_PDI_SPEC>`"]
pub type WRITEMODE_ESC_RESET_PDI = crate::Reg<writemode_esc_reset_pdi::WRITEMODE_ESC_RESET_PDI_SPEC>;
#[doc = "ESC Reset PDI \\[WRITE Mode\\]"]
pub mod writemode_esc_reset_pdi;
#[doc = "READMode_ESC_RESET_PDI (r) register accessor: an alias for `Reg<READMODE_ESC_RESET_PDI_SPEC>`"]
pub type READMODE_ESC_RESET_PDI = crate::Reg<readmode_esc_reset_pdi::READMODE_ESC_RESET_PDI_SPEC>;
#[doc = "ESC Reset PDI \\[READ Mode\\]"]
pub mod readmode_esc_reset_pdi;
#[doc = "ESC_DL_CONTROL (r) register accessor: an alias for `Reg<ESC_DL_CONTROL_SPEC>`"]
pub type ESC_DL_CONTROL = crate::Reg<esc_dl_control::ESC_DL_CONTROL_SPEC>;
#[doc = "ESC DL Control"]
pub mod esc_dl_control;
#[doc = "PHYSICAL_RW_OFFSET (r) register accessor: an alias for `Reg<PHYSICAL_RW_OFFSET_SPEC>`"]
pub type PHYSICAL_RW_OFFSET = crate::Reg<physical_rw_offset::PHYSICAL_RW_OFFSET_SPEC>;
#[doc = "Physical Read/Write Offset"]
pub mod physical_rw_offset;
#[doc = "ESC_DL_STATUS (r) register accessor: an alias for `Reg<ESC_DL_STATUS_SPEC>`"]
pub type ESC_DL_STATUS = crate::Reg<esc_dl_status::ESC_DL_STATUS_SPEC>;
#[doc = "ESC DL Status"]
pub mod esc_dl_status;
#[doc = "AL_CONTROL (r) register accessor: an alias for `Reg<AL_CONTROL_SPEC>`"]
pub type AL_CONTROL = crate::Reg<al_control::AL_CONTROL_SPEC>;
#[doc = "AL Control"]
pub mod al_control;
#[doc = "AL_STATUS (rw) register accessor: an alias for `Reg<AL_STATUS_SPEC>`"]
pub type AL_STATUS = crate::Reg<al_status::AL_STATUS_SPEC>;
#[doc = "AL Status"]
pub mod al_status;
#[doc = "AL_STATUS_CODE (rw) register accessor: an alias for `Reg<AL_STATUS_CODE_SPEC>`"]
pub type AL_STATUS_CODE = crate::Reg<al_status_code::AL_STATUS_CODE_SPEC>;
#[doc = "AL Status Code"]
pub mod al_status_code;
#[doc = "RUN_LED (rw) register accessor: an alias for `Reg<RUN_LED_SPEC>`"]
pub type RUN_LED = crate::Reg<run_led::RUN_LED_SPEC>;
#[doc = "RUN LED Override"]
pub mod run_led;
#[doc = "ERR_LED (rw) register accessor: an alias for `Reg<ERR_LED_SPEC>`"]
pub type ERR_LED = crate::Reg<err_led::ERR_LED_SPEC>;
#[doc = "RUN ERR Override"]
pub mod err_led;
#[doc = "PDI_CONTROL (r) register accessor: an alias for `Reg<PDI_CONTROL_SPEC>`"]
pub type PDI_CONTROL = crate::Reg<pdi_control::PDI_CONTROL_SPEC>;
#[doc = "PDI Control"]
pub mod pdi_control;
#[doc = "ESC_CONFIG (r) register accessor: an alias for `Reg<ESC_CONFIG_SPEC>`"]
pub type ESC_CONFIG = crate::Reg<esc_config::ESC_CONFIG_SPEC>;
#[doc = "ESC Configuration"]
pub mod esc_config;
#[doc = "PDI_CONFIG (r) register accessor: an alias for `Reg<PDI_CONFIG_SPEC>`"]
pub type PDI_CONFIG = crate::Reg<pdi_config::PDI_CONFIG_SPEC>;
#[doc = "PDI Control"]
pub mod pdi_config;
#[doc = "SYNC_LATCH_CONFIG (r) register accessor: an alias for `Reg<SYNC_LATCH_CONFIG_SPEC>`"]
pub type SYNC_LATCH_CONFIG = crate::Reg<sync_latch_config::SYNC_LATCH_CONFIG_SPEC>;
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration"]
pub mod sync_latch_config;
#[doc = "PDI_EXT_CONFIG (r) register accessor: an alias for `Reg<PDI_EXT_CONFIG_SPEC>`"]
pub type PDI_EXT_CONFIG = crate::Reg<pdi_ext_config::PDI_EXT_CONFIG_SPEC>;
#[doc = "PDI Synchronous Microcontroller extended Configuration"]
pub mod pdi_ext_config;
#[doc = "EVENT_MASK (r) register accessor: an alias for `Reg<EVENT_MASK_SPEC>`"]
pub type EVENT_MASK = crate::Reg<event_mask::EVENT_MASK_SPEC>;
#[doc = "ECAT Event Mask"]
pub mod event_mask;
#[doc = "AL_EVENT_MASK (rw) register accessor: an alias for `Reg<AL_EVENT_MASK_SPEC>`"]
pub type AL_EVENT_MASK = crate::Reg<al_event_mask::AL_EVENT_MASK_SPEC>;
#[doc = "PDI AL Event Mask"]
pub mod al_event_mask;
#[doc = "EVENT_REQ (r) register accessor: an alias for `Reg<EVENT_REQ_SPEC>`"]
pub type EVENT_REQ = crate::Reg<event_req::EVENT_REQ_SPEC>;
#[doc = "ECAT Event Request"]
pub mod event_req;
#[doc = "AL_EVENT_REQ (rw) register accessor: an alias for `Reg<AL_EVENT_REQ_SPEC>`"]
pub type AL_EVENT_REQ = crate::Reg<al_event_req::AL_EVENT_REQ_SPEC>;
#[doc = "AL Event Request"]
pub mod al_event_req;
#[doc = "RX_ERR_COUNT0 (r) register accessor: an alias for `Reg<RX_ERR_COUNT0_SPEC>`"]
pub type RX_ERR_COUNT0 = crate::Reg<rx_err_count0::RX_ERR_COUNT0_SPEC>;
#[doc = "RX Error Counter Port 0"]
pub mod rx_err_count0;
#[doc = "RX_ERR_COUNT1 (r) register accessor: an alias for `Reg<RX_ERR_COUNT1_SPEC>`"]
pub type RX_ERR_COUNT1 = crate::Reg<rx_err_count1::RX_ERR_COUNT1_SPEC>;
#[doc = "RX Error Counter Port 1"]
pub mod rx_err_count1;
#[doc = "FWD_RX_ERR_COUNT0 (r) register accessor: an alias for `Reg<FWD_RX_ERR_COUNT0_SPEC>`"]
pub type FWD_RX_ERR_COUNT0 = crate::Reg<fwd_rx_err_count0::FWD_RX_ERR_COUNT0_SPEC>;
#[doc = "Forwarded RX Error Counter Port 0"]
pub mod fwd_rx_err_count0;
#[doc = "FWD_RX_ERR_COUNT1 (r) register accessor: an alias for `Reg<FWD_RX_ERR_COUNT1_SPEC>`"]
pub type FWD_RX_ERR_COUNT1 = crate::Reg<fwd_rx_err_count1::FWD_RX_ERR_COUNT1_SPEC>;
#[doc = "Forwarded RX Error Counter Port 1"]
pub mod fwd_rx_err_count1;
#[doc = "PROC_ERR_COUNT (r) register accessor: an alias for `Reg<PROC_ERR_COUNT_SPEC>`"]
pub type PROC_ERR_COUNT = crate::Reg<proc_err_count::PROC_ERR_COUNT_SPEC>;
#[doc = "ECAT Processing Unit Error Counter"]
pub mod proc_err_count;
#[doc = "PDI_ERR_COUNT (r) register accessor: an alias for `Reg<PDI_ERR_COUNT_SPEC>`"]
pub type PDI_ERR_COUNT = crate::Reg<pdi_err_count::PDI_ERR_COUNT_SPEC>;
#[doc = "PDI Error Counter"]
pub mod pdi_err_count;
#[doc = "LOST_LINK_COUNT0 (r) register accessor: an alias for `Reg<LOST_LINK_COUNT0_SPEC>`"]
pub type LOST_LINK_COUNT0 = crate::Reg<lost_link_count0::LOST_LINK_COUNT0_SPEC>;
#[doc = "Lost Link Counter Port 0"]
pub mod lost_link_count0;
#[doc = "LOST_LINK_COUNT1 (r) register accessor: an alias for `Reg<LOST_LINK_COUNT1_SPEC>`"]
pub type LOST_LINK_COUNT1 = crate::Reg<lost_link_count1::LOST_LINK_COUNT1_SPEC>;
#[doc = "Lost Link Counter Port 1"]
pub mod lost_link_count1;
#[doc = "WD_DIVIDE (rw) register accessor: an alias for `Reg<WD_DIVIDE_SPEC>`"]
pub type WD_DIVIDE = crate::Reg<wd_divide::WD_DIVIDE_SPEC>;
#[doc = "Watchdog Divider"]
pub mod wd_divide;
#[doc = "WD_TIME_PDI (rw) register accessor: an alias for `Reg<WD_TIME_PDI_SPEC>`"]
pub type WD_TIME_PDI = crate::Reg<wd_time_pdi::WD_TIME_PDI_SPEC>;
#[doc = "Watchdog Time PDI"]
pub mod wd_time_pdi;
#[doc = "WD_TIME_PDATA (rw) register accessor: an alias for `Reg<WD_TIME_PDATA_SPEC>`"]
pub type WD_TIME_PDATA = crate::Reg<wd_time_pdata::WD_TIME_PDATA_SPEC>;
#[doc = "Watchdog Time Process Data"]
pub mod wd_time_pdata;
#[doc = "WD_STAT_PDATA (r) register accessor: an alias for `Reg<WD_STAT_PDATA_SPEC>`"]
pub type WD_STAT_PDATA = crate::Reg<wd_stat_pdata::WD_STAT_PDATA_SPEC>;
#[doc = "Watchdog Status Process Data"]
pub mod wd_stat_pdata;
#[doc = "WD_COUNT_PDATA (r) register accessor: an alias for `Reg<WD_COUNT_PDATA_SPEC>`"]
pub type WD_COUNT_PDATA = crate::Reg<wd_count_pdata::WD_COUNT_PDATA_SPEC>;
#[doc = "Watchdog Counter Process Data"]
pub mod wd_count_pdata;
#[doc = "WD_COUNT_PDI (r) register accessor: an alias for `Reg<WD_COUNT_PDI_SPEC>`"]
pub type WD_COUNT_PDI = crate::Reg<wd_count_pdi::WD_COUNT_PDI_SPEC>;
#[doc = "Watchdog Counter PDI"]
pub mod wd_count_pdi;
#[doc = "EEP_CONF (r) register accessor: an alias for `Reg<EEP_CONF_SPEC>`"]
pub type EEP_CONF = crate::Reg<eep_conf::EEP_CONF_SPEC>;
#[doc = "EEPROM Configuration"]
pub mod eep_conf;
#[doc = "EEP_STATE (rw) register accessor: an alias for `Reg<EEP_STATE_SPEC>`"]
pub type EEP_STATE = crate::Reg<eep_state::EEP_STATE_SPEC>;
#[doc = "EEPROM PDI Access State"]
pub mod eep_state;
#[doc = "EEP_CONT_STAT (rw) register accessor: an alias for `Reg<EEP_CONT_STAT_SPEC>`"]
pub type EEP_CONT_STAT = crate::Reg<eep_cont_stat::EEP_CONT_STAT_SPEC>;
#[doc = "EEPROM Control/Status"]
pub mod eep_cont_stat;
#[doc = "EEP_ADR (rw) register accessor: an alias for `Reg<EEP_ADR_SPEC>`"]
pub type EEP_ADR = crate::Reg<eep_adr::EEP_ADR_SPEC>;
#[doc = "EEPROM Address"]
pub mod eep_adr;
#[doc = "EEP_DATA (rw) register accessor: an alias for `Reg<EEP_DATA_SPEC>`"]
pub type EEP_DATA = crate::Reg<eep_data::EEP_DATA_SPEC>;
#[doc = "EEPROM Read/Write data"]
pub mod eep_data;
#[doc = "MII_CONT_STAT (rw) register accessor: an alias for `Reg<MII_CONT_STAT_SPEC>`"]
pub type MII_CONT_STAT = crate::Reg<mii_cont_stat::MII_CONT_STAT_SPEC>;
#[doc = "MII Management Control/Status"]
pub mod mii_cont_stat;
#[doc = "MII_PHY_ADR (rw) register accessor: an alias for `Reg<MII_PHY_ADR_SPEC>`"]
pub type MII_PHY_ADR = crate::Reg<mii_phy_adr::MII_PHY_ADR_SPEC>;
#[doc = "PHY Address"]
pub mod mii_phy_adr;
#[doc = "MII_PHY_REG_ADR (rw) register accessor: an alias for `Reg<MII_PHY_REG_ADR_SPEC>`"]
pub type MII_PHY_REG_ADR = crate::Reg<mii_phy_reg_adr::MII_PHY_REG_ADR_SPEC>;
#[doc = "PHY Register Address"]
pub mod mii_phy_reg_adr;
#[doc = "MII_PHY_DATA (rw) register accessor: an alias for `Reg<MII_PHY_DATA_SPEC>`"]
pub type MII_PHY_DATA = crate::Reg<mii_phy_data::MII_PHY_DATA_SPEC>;
#[doc = "PHY Data"]
pub mod mii_phy_data;
#[doc = "MII_ECAT_ACS_STATE (r) register accessor: an alias for `Reg<MII_ECAT_ACS_STATE_SPEC>`"]
pub type MII_ECAT_ACS_STATE = crate::Reg<mii_ecat_acs_state::MII_ECAT_ACS_STATE_SPEC>;
#[doc = "MII ECAT ACS STATE"]
pub mod mii_ecat_acs_state;
#[doc = "MII_PDI_ACS_STATE (rw) register accessor: an alias for `Reg<MII_PDI_ACS_STATE_SPEC>`"]
pub type MII_PDI_ACS_STATE = crate::Reg<mii_pdi_acs_state::MII_PDI_ACS_STATE_SPEC>;
#[doc = "MII PDI ACS STATE"]
pub mod mii_pdi_acs_state;
#[doc = "DC_RCV_TIME_PORT0 (r) register accessor: an alias for `Reg<DC_RCV_TIME_PORT0_SPEC>`"]
pub type DC_RCV_TIME_PORT0 = crate::Reg<dc_rcv_time_port0::DC_RCV_TIME_PORT0_SPEC>;
#[doc = "Receive Time Port 0"]
pub mod dc_rcv_time_port0;
#[doc = "DC_RCV_TIME_PORT1 (r) register accessor: an alias for `Reg<DC_RCV_TIME_PORT1_SPEC>`"]
pub type DC_RCV_TIME_PORT1 = crate::Reg<dc_rcv_time_port1::DC_RCV_TIME_PORT1_SPEC>;
#[doc = "Receive Time Port 1"]
pub mod dc_rcv_time_port1;
#[doc = "RECEIVE_TIME_PU (r) register accessor: an alias for `Reg<RECEIVE_TIME_PU_SPEC>`"]
pub type RECEIVE_TIME_PU = crate::Reg<receive_time_pu::RECEIVE_TIME_PU_SPEC>;
#[doc = "Local time of the beginning of a frame"]
pub mod receive_time_pu;
#[doc = "READMode_DC_SYS_TIME (r) register accessor: an alias for `Reg<READMODE_DC_SYS_TIME_SPEC>`"]
pub type READMODE_DC_SYS_TIME = crate::Reg<readmode_dc_sys_time::READMODE_DC_SYS_TIME_SPEC>;
#[doc = "System Time read access"]
pub mod readmode_dc_sys_time;
#[doc = "WRITEMode_DC_SYS_TIME (w) register accessor: an alias for `Reg<WRITEMODE_DC_SYS_TIME_SPEC>`"]
pub type WRITEMODE_DC_SYS_TIME = crate::Reg<writemode_dc_sys_time::WRITEMODE_DC_SYS_TIME_SPEC>;
#[doc = "System Time \\[WRITE Mode\\]"]
pub mod writemode_dc_sys_time;
#[doc = "DC_SYS_TIME_OFFSET (rw) register accessor: an alias for `Reg<DC_SYS_TIME_OFFSET_SPEC>`"]
pub type DC_SYS_TIME_OFFSET = crate::Reg<dc_sys_time_offset::DC_SYS_TIME_OFFSET_SPEC>;
#[doc = "Difference between local time and System Time"]
pub mod dc_sys_time_offset;
#[doc = "DC_SYS_TIME_DELAY (rw) register accessor: an alias for `Reg<DC_SYS_TIME_DELAY_SPEC>`"]
pub type DC_SYS_TIME_DELAY = crate::Reg<dc_sys_time_delay::DC_SYS_TIME_DELAY_SPEC>;
#[doc = "System Time Delay"]
pub mod dc_sys_time_delay;
#[doc = "DC_SYS_TIME_DIFF (r) register accessor: an alias for `Reg<DC_SYS_TIME_DIFF_SPEC>`"]
pub type DC_SYS_TIME_DIFF = crate::Reg<dc_sys_time_diff::DC_SYS_TIME_DIFF_SPEC>;
#[doc = "System Time Difference"]
pub mod dc_sys_time_diff;
#[doc = "DC_SPEED_COUNT_START (rw) register accessor: an alias for `Reg<DC_SPEED_COUNT_START_SPEC>`"]
pub type DC_SPEED_COUNT_START = crate::Reg<dc_speed_count_start::DC_SPEED_COUNT_START_SPEC>;
#[doc = "Speed Counter Start"]
pub mod dc_speed_count_start;
#[doc = "DC_SPEED_COUNT_DIFF (r) register accessor: an alias for `Reg<DC_SPEED_COUNT_DIFF_SPEC>`"]
pub type DC_SPEED_COUNT_DIFF = crate::Reg<dc_speed_count_diff::DC_SPEED_COUNT_DIFF_SPEC>;
#[doc = "Speed Counter Diff"]
pub mod dc_speed_count_diff;
#[doc = "DC_SYS_TIME_FIL_DEPTH (rw) register accessor: an alias for `Reg<DC_SYS_TIME_FIL_DEPTH_SPEC>`"]
pub type DC_SYS_TIME_FIL_DEPTH = crate::Reg<dc_sys_time_fil_depth::DC_SYS_TIME_FIL_DEPTH_SPEC>;
#[doc = "System Time Difference Filter Depth"]
pub mod dc_sys_time_fil_depth;
#[doc = "DC_SPEED_COUNT_FIL_DEPTH (rw) register accessor: an alias for `Reg<DC_SPEED_COUNT_FIL_DEPTH_SPEC>`"]
pub type DC_SPEED_COUNT_FIL_DEPTH = crate::Reg<dc_speed_count_fil_depth::DC_SPEED_COUNT_FIL_DEPTH_SPEC>;
#[doc = "Speed Counter Filter Depth"]
pub mod dc_speed_count_fil_depth;
#[doc = "DC_CYC_CONT (r) register accessor: an alias for `Reg<DC_CYC_CONT_SPEC>`"]
pub type DC_CYC_CONT = crate::Reg<dc_cyc_cont::DC_CYC_CONT_SPEC>;
#[doc = "Cyclic Unit Control"]
pub mod dc_cyc_cont;
#[doc = "DC_ACT (rw) register accessor: an alias for `Reg<DC_ACT_SPEC>`"]
pub type DC_ACT = crate::Reg<dc_act::DC_ACT_SPEC>;
#[doc = "Activation register"]
pub mod dc_act;
#[doc = "DC_PULSE_LEN (r) register accessor: an alias for `Reg<DC_PULSE_LEN_SPEC>`"]
pub type DC_PULSE_LEN = crate::Reg<dc_pulse_len::DC_PULSE_LEN_SPEC>;
#[doc = "Pulse Length of SyncSignals"]
pub mod dc_pulse_len;
#[doc = "DC_ACT_STAT (r) register accessor: an alias for `Reg<DC_ACT_STAT_SPEC>`"]
pub type DC_ACT_STAT = crate::Reg<dc_act_stat::DC_ACT_STAT_SPEC>;
#[doc = "Activation Status"]
pub mod dc_act_stat;
#[doc = "DC_SYNC0_STAT (r) register accessor: an alias for `Reg<DC_SYNC0_STAT_SPEC>`"]
pub type DC_SYNC0_STAT = crate::Reg<dc_sync0_stat::DC_SYNC0_STAT_SPEC>;
#[doc = "SYNC0 Status"]
pub mod dc_sync0_stat;
#[doc = "DC_SYNC1_STAT (r) register accessor: an alias for `Reg<DC_SYNC1_STAT_SPEC>`"]
pub type DC_SYNC1_STAT = crate::Reg<dc_sync1_stat::DC_SYNC1_STAT_SPEC>;
#[doc = "SYNC1 Status"]
pub mod dc_sync1_stat;
#[doc = "DC_CYC_START_TIME (rw) register accessor: an alias for `Reg<DC_CYC_START_TIME_SPEC>`"]
pub type DC_CYC_START_TIME = crate::Reg<dc_cyc_start_time::DC_CYC_START_TIME_SPEC>;
#[doc = "Start Time Cyclic Operation"]
pub mod dc_cyc_start_time;
#[doc = "DC_NEXT_SYNC1_PULSE (r) register accessor: an alias for `Reg<DC_NEXT_SYNC1_PULSE_SPEC>`"]
pub type DC_NEXT_SYNC1_PULSE = crate::Reg<dc_next_sync1_pulse::DC_NEXT_SYNC1_PULSE_SPEC>;
#[doc = "System time of next SYNC1 pulse in ns"]
pub mod dc_next_sync1_pulse;
#[doc = "DC_SYNC0_CYC_TIME (rw) register accessor: an alias for `Reg<DC_SYNC0_CYC_TIME_SPEC>`"]
pub type DC_SYNC0_CYC_TIME = crate::Reg<dc_sync0_cyc_time::DC_SYNC0_CYC_TIME_SPEC>;
#[doc = "SYNC0 Cycle Time"]
pub mod dc_sync0_cyc_time;
#[doc = "DC_SYNC1_CYC_TIME (rw) register accessor: an alias for `Reg<DC_SYNC1_CYC_TIME_SPEC>`"]
pub type DC_SYNC1_CYC_TIME = crate::Reg<dc_sync1_cyc_time::DC_SYNC1_CYC_TIME_SPEC>;
#[doc = "SYNC1 Cycle Time"]
pub mod dc_sync1_cyc_time;
#[doc = "DC_LATCH0_CONT (rw) register accessor: an alias for `Reg<DC_LATCH0_CONT_SPEC>`"]
pub type DC_LATCH0_CONT = crate::Reg<dc_latch0_cont::DC_LATCH0_CONT_SPEC>;
#[doc = "Latch0 Control"]
pub mod dc_latch0_cont;
#[doc = "DC_LATCH1_CONT (rw) register accessor: an alias for `Reg<DC_LATCH1_CONT_SPEC>`"]
pub type DC_LATCH1_CONT = crate::Reg<dc_latch1_cont::DC_LATCH1_CONT_SPEC>;
#[doc = "Latch1 Control"]
pub mod dc_latch1_cont;
#[doc = "DC_LATCH0_STAT (r) register accessor: an alias for `Reg<DC_LATCH0_STAT_SPEC>`"]
pub type DC_LATCH0_STAT = crate::Reg<dc_latch0_stat::DC_LATCH0_STAT_SPEC>;
#[doc = "Latch0 Status"]
pub mod dc_latch0_stat;
#[doc = "DC_LATCH1_STAT (r) register accessor: an alias for `Reg<DC_LATCH1_STAT_SPEC>`"]
pub type DC_LATCH1_STAT = crate::Reg<dc_latch1_stat::DC_LATCH1_STAT_SPEC>;
#[doc = "Latch1 Status"]
pub mod dc_latch1_stat;
#[doc = "DC_LATCH0_TIME_POS (r) register accessor: an alias for `Reg<DC_LATCH0_TIME_POS_SPEC>`"]
pub type DC_LATCH0_TIME_POS = crate::Reg<dc_latch0_time_pos::DC_LATCH0_TIME_POS_SPEC>;
#[doc = "Register captures System time at the positive edge of the Latch0 signal"]
pub mod dc_latch0_time_pos;
#[doc = "DC_LATCH0_TIME_NEG (r) register accessor: an alias for `Reg<DC_LATCH0_TIME_NEG_SPEC>`"]
pub type DC_LATCH0_TIME_NEG = crate::Reg<dc_latch0_time_neg::DC_LATCH0_TIME_NEG_SPEC>;
#[doc = "Register captures System time at the negative edge of the Latch0 signal"]
pub mod dc_latch0_time_neg;
#[doc = "DC_LATCH1_TIME_POS (r) register accessor: an alias for `Reg<DC_LATCH1_TIME_POS_SPEC>`"]
pub type DC_LATCH1_TIME_POS = crate::Reg<dc_latch1_time_pos::DC_LATCH1_TIME_POS_SPEC>;
#[doc = "Register captures System time at the positive edge of the Latch1 signal"]
pub mod dc_latch1_time_pos;
#[doc = "DC_LATCH1_TIME_NEG (r) register accessor: an alias for `Reg<DC_LATCH1_TIME_NEG_SPEC>`"]
pub type DC_LATCH1_TIME_NEG = crate::Reg<dc_latch1_time_neg::DC_LATCH1_TIME_NEG_SPEC>;
#[doc = "Register captures System time at the negative edge of the Latch1 signal"]
pub mod dc_latch1_time_neg;
#[doc = "DC_ECAT_CNG_EV_TIME (r) register accessor: an alias for `Reg<DC_ECAT_CNG_EV_TIME_SPEC>`"]
pub type DC_ECAT_CNG_EV_TIME = crate::Reg<dc_ecat_cng_ev_time::DC_ECAT_CNG_EV_TIME_SPEC>;
#[doc = "EtherCAT Buffer Change Event Time"]
pub mod dc_ecat_cng_ev_time;
#[doc = "DC_PDI_START_EV_TIME (r) register accessor: an alias for `Reg<DC_PDI_START_EV_TIME_SPEC>`"]
pub type DC_PDI_START_EV_TIME = crate::Reg<dc_pdi_start_ev_time::DC_PDI_START_EV_TIME_SPEC>;
#[doc = "PDI Buffer Start Event Time"]
pub mod dc_pdi_start_ev_time;
#[doc = "DC_PDI_CNG_EV_TIME (r) register accessor: an alias for `Reg<DC_PDI_CNG_EV_TIME_SPEC>`"]
pub type DC_PDI_CNG_EV_TIME = crate::Reg<dc_pdi_cng_ev_time::DC_PDI_CNG_EV_TIME_SPEC>;
#[doc = "PDI Buffer Change Event Time"]
pub mod dc_pdi_cng_ev_time;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ECAT0 Module ID"]
pub mod id;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "ECAT0 Status"]
pub mod status;
