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
    _reserved8: [u8; 6usize],
    #[doc = "0x10 - Configured Station Address"]
    pub station_adr: STATION_ADR,
    #[doc = "0x12 - Configured Station Alias"]
    pub station_alias: STATION_ALIAS,
    _reserved10: [u8; 12usize],
    #[doc = "0x20 - Write Register Enable"]
    pub wr_reg_enable: WR_REG_ENABLE,
    #[doc = "0x21 - Write Register Protection"]
    pub wr_reg_protect: WR_REG_PROTECT,
    _reserved12: [u8; 14usize],
    #[doc = "0x30 - ESC Write Enable"]
    pub esc_wr_enable: ESC_WR_ENABLE,
    #[doc = "0x31 - ESC Write Protection"]
    pub esc_wr_protect: ESC_WR_PROTECT,
    _reserved14: [u8; 14usize],
    _reserved_14_esc_reset_: [u8; 1usize],
    _reserved_15_esc_reset_: [u8; 1usize],
    _reserved16: [u8; 190usize],
    #[doc = "0x100 - ESC DL Control"]
    pub esc_dl_control: ESC_DL_CONTROL,
    _reserved17: [u8; 4usize],
    #[doc = "0x108 - Physical Read/Write Offset"]
    pub physical_rw_offset: PHYSICAL_RW_OFFSET,
    _reserved18: [u8; 6usize],
    #[doc = "0x110 - ESC DL Status"]
    pub esc_dl_status: ESC_DL_STATUS,
    _reserved19: [u8; 14usize],
    #[doc = "0x120 - AL Control"]
    pub al_control: AL_CONTROL,
    _reserved20: [u8; 14usize],
    #[doc = "0x130 - AL Status"]
    pub al_status: AL_STATUS,
    _reserved21: [u8; 2usize],
    #[doc = "0x134 - AL Status Code"]
    pub al_status_code: AL_STATUS_CODE,
    _reserved22: [u8; 2usize],
    #[doc = "0x138 - RUN LED Override"]
    pub run_led: RUN_LED,
    #[doc = "0x139 - RUN ERR Override"]
    pub err_led: ERR_LED,
    _reserved24: [u8; 6usize],
    #[doc = "0x140 - PDI Control"]
    pub pdi_control: PDI_CONTROL,
    #[doc = "0x141 - ESC Configuration"]
    pub esc_config: ESC_CONFIG,
    _reserved26: [u8; 14usize],
    #[doc = "0x150 - PDI Control"]
    pub pdi_config: PDI_CONFIG,
    #[doc = "0x151 - Sync/Latch\\[1:0\\]
PDI Configuration"]
    pub sync_latch_config: SYNC_LATCH_CONFIG,
    #[doc = "0x152 - PDI Synchronous Microcontroller extended Configuration"]
    pub pdi_ext_config: PDI_EXT_CONFIG,
    _reserved29: [u8; 172usize],
    #[doc = "0x200 - ECAT Event Mask"]
    pub event_mask: EVENT_MASK,
    _reserved30: [u8; 2usize],
    #[doc = "0x204 - PDI AL Event Mask"]
    pub al_event_mask: AL_EVENT_MASK,
    _reserved31: [u8; 8usize],
    #[doc = "0x210 - ECAT Event Request"]
    pub event_req: EVENT_REQ,
    _reserved32: [u8; 14usize],
    #[doc = "0x220 - AL Event Request"]
    pub al_event_req: AL_EVENT_REQ,
    _reserved33: [u8; 220usize],
    #[doc = "0x300 - RX Error Counter Port 0"]
    pub rx_err_count0: RX_ERR_COUNT0,
    #[doc = "0x302 - RX Error Counter Port 1"]
    pub rx_err_count1: RX_ERR_COUNT1,
    _reserved35: [u8; 4usize],
    #[doc = "0x308 - Forwarded RX Error Counter Port 0"]
    pub fwd_rx_err_count0: FWD_RX_ERR_COUNT0,
    #[doc = "0x309 - Forwarded RX Error Counter Port 1"]
    pub fwd_rx_err_count1: FWD_RX_ERR_COUNT1,
    _reserved37: [u8; 2usize],
    #[doc = "0x30c - ECAT Processing Unit Error Counter"]
    pub proc_err_count: PROC_ERR_COUNT,
    #[doc = "0x30d - PDI Error Counter"]
    pub pdi_err_count: PDI_ERR_COUNT,
    _reserved39: [u8; 2usize],
    #[doc = "0x310 - Lost Link Counter Port 0"]
    pub lost_link_count0: LOST_LINK_COUNT0,
    #[doc = "0x311 - Lost Link Counter Port 1"]
    pub lost_link_count1: LOST_LINK_COUNT1,
    _reserved41: [u8; 238usize],
    #[doc = "0x400 - Watchdog Divider"]
    pub wd_divide: WD_DIVIDE,
    _reserved42: [u8; 14usize],
    #[doc = "0x410 - Watchdog Time PDI"]
    pub wd_time_pdi: WD_TIME_PDI,
    _reserved43: [u8; 14usize],
    #[doc = "0x420 - Watchdog Time Process Data"]
    pub wd_time_pdata: WD_TIME_PDATA,
    _reserved44: [u8; 30usize],
    #[doc = "0x440 - Watchdog Status Process Data"]
    pub wd_stat_pdata: WD_STAT_PDATA,
    #[doc = "0x442 - Watchdog Counter Process Data"]
    pub wd_count_pdata: WD_COUNT_PDATA,
    #[doc = "0x443 - Watchdog Counter PDI"]
    pub wd_count_pdi: WD_COUNT_PDI,
    _reserved47: [u8; 188usize],
    #[doc = "0x500 - EEPROM Configuration"]
    pub eep_conf: EEP_CONF,
    #[doc = "0x501 - EEPROM PDI Access State"]
    pub eep_state: EEP_STATE,
    #[doc = "0x502 - EEPROM Control/Status"]
    pub eep_cont_stat: EEP_CONT_STAT,
    #[doc = "0x504 - EEPROM Address"]
    pub eep_adr: EEP_ADR,
    #[doc = "0x508 - EEPROM Read/Write data"]
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
    _reserved58: [u8; 1000usize],
    #[doc = "0x900 - Receive Time Port 0"]
    pub dc_rcv_time_port0: DC_RCV_TIME_PORT0,
    #[doc = "0x904 - Receive Time Port 1"]
    pub dc_rcv_time_port1: DC_RCV_TIME_PORT1,
    _reserved60: [u8; 8usize],
    _reserved_60_dc_sys_: [u8; 8usize],
    #[doc = "0x918 - Local time of the beginning of a frame"]
    pub receive_time_pu: [RECEIVE_TIME_PU; 2],
    #[doc = "0x920 - Difference between local time and System Time"]
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
    _reserved69: [u8; 74usize],
    #[doc = "0x980 - Cyclic Unit Control"]
    pub dc_cyc_cont: DC_CYC_CONT,
    #[doc = "0x981 - Activation register"]
    pub dc_act: DC_ACT,
    #[doc = "0x982 - Pulse Length of SyncSignals"]
    pub dc_pulse_len: DC_PULSE_LEN,
    #[doc = "0x984 - Activation Status"]
    pub dc_act_stat: DC_ACT_STAT,
    _reserved73: [u8; 9usize],
    #[doc = "0x98e - SYNC0 Status"]
    pub dc_sync0_stat: DC_SYNC0_STAT,
    #[doc = "0x98f - SYNC1 Status"]
    pub dc_sync1_stat: DC_SYNC1_STAT,
    #[doc = "0x990 - Start Time Cyclic Operation"]
    pub dc_cyc_start_time: [DC_CYC_START_TIME; 2],
    #[doc = "0x998 - System time of next SYNC1 pulse in ns"]
    pub dc_next_sync1_pulse: [DC_NEXT_SYNC1_PULSE; 2],
    #[doc = "0x9a0 - SYNC0 Cycle Time"]
    pub dc_sync0_cyc_time: DC_SYNC0_CYC_TIME,
    #[doc = "0x9a4 - SYNC1 Cycle Time"]
    pub dc_sync1_cyc_time: DC_SYNC1_CYC_TIME,
    #[doc = "0x9a8 - Latch0 Control"]
    pub dc_latch0_cont: DC_LATCH0_CONT,
    #[doc = "0x9a9 - Latch1 Control"]
    pub dc_latch1_cont: DC_LATCH1_CONT,
    _reserved81: [u8; 4usize],
    #[doc = "0x9ae - Latch0 Status"]
    pub dc_latch0_stat: DC_LATCH0_STAT,
    #[doc = "0x9af - Latch1 Status"]
    pub dc_latch1_stat: DC_LATCH1_STAT,
    #[doc = "0x9b0 - Register captures System time at the positive edge of the Latch0 signal"]
    pub dc_latch0_time_pos: [DC_LATCH0_TIME_POS; 2],
    #[doc = "0x9b8 - Register captures System time at the negative edge of the Latch0 signal"]
    pub dc_latch0_time_neg: [DC_LATCH0_TIME_NEG; 2],
    #[doc = "0x9c0 - Register captures System time at the positive edge of the Latch1 signal"]
    pub dc_latch1_time_pos: [DC_LATCH1_TIME_POS; 2],
    #[doc = "0x9c8 - Register captures System time at the negative edge of the Latch1 signal"]
    pub dc_latch1_time_neg: [DC_LATCH1_TIME_NEG; 2],
    _reserved87: [u8; 32usize],
    #[doc = "0x9f0 - EtherCAT Buffer Change Event Time"]
    pub dc_ecat_cng_ev_time: DC_ECAT_CNG_EV_TIME,
    _reserved88: [u8; 4usize],
    #[doc = "0x9f8 - PDI Buffer Start Event Time"]
    pub dc_pdi_start_ev_time: DC_PDI_START_EV_TIME,
    #[doc = "0x9fc - PDI Buffer Change Event Time"]
    pub dc_pdi_cng_ev_time: DC_PDI_CNG_EV_TIME,
    _reserved90: [u8; 1024usize],
    #[doc = "0xe00 - ECAT0 Module ID"]
    pub id: ID,
    _reserved91: [u8; 4usize],
    #[doc = "0xe08 - ECAT0 Status"]
    pub status: STATUS,
}
impl RegisterBlock {
    #[doc = "0x40 - ESC Reset ECAT \\[READ Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_ecat(&self) -> &ESC_RESET_ECAT {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const ESC_RESET_ECAT) }
    }
    #[doc = "0x40 - ESC Reset ECAT \\[READ Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_ecat_mut(&self) -> &mut ESC_RESET_ECAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut ESC_RESET_ECAT) }
    }
    #[doc = "0x40 - ESC Reset ECAT \\[WRITE Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_ecat(&self) -> &ESC_RESET_ECAT {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const ESC_RESET_ECAT) }
    }
    #[doc = "0x40 - ESC Reset ECAT \\[WRITE Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_ecat_mut(&self) -> &mut ESC_RESET_ECAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut ESC_RESET_ECAT) }
    }
    #[doc = "0x41 - ESC Reset PDI \\[READ Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_pdi(&self) -> &ESC_RESET_PDI {
        unsafe { &*(((self as *const Self) as *const u8).add(65usize) as *const ESC_RESET_PDI) }
    }
    #[doc = "0x41 - ESC Reset PDI \\[READ Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_pdi_mut(&self) -> &mut ESC_RESET_PDI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(65usize) as *mut ESC_RESET_PDI) }
    }
    #[doc = "0x41 - ESC Reset PDI \\[WRITE Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_pdi(&self) -> &ESC_RESET_PDI {
        unsafe { &*(((self as *const Self) as *const u8).add(65usize) as *const ESC_RESET_PDI) }
    }
    #[doc = "0x41 - ESC Reset PDI \\[WRITE Mode\\]"]
    #[inline(always)]
    pub fn esc_reset_pdi_mut(&self) -> &mut ESC_RESET_PDI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(65usize) as *mut ESC_RESET_PDI) }
    }
    #[doc = "0x910 - System Time \\[WRITE Mode\\]"]
    #[inline(always)]
    pub fn dc_sys_time(&self) -> &DC_SYS_TIME {
        unsafe { &*(((self as *const Self) as *const u8).add(2320usize) as *const DC_SYS_TIME) }
    }
    #[doc = "0x910 - System Time \\[WRITE Mode\\]"]
    #[inline(always)]
    pub fn dc_sys_time_mut(&self) -> &mut DC_SYS_TIME {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2320usize) as *mut DC_SYS_TIME) }
    }
    #[doc = "0x910 - System Time read access"]
    #[inline(always)]
    pub fn dc_sys_time(&self) -> &[DC_SYS_TIME; 2] {
        unsafe { &*(((self as *const Self) as *const u8).add(2320usize) as *const [DC_SYS_TIME; 2]) }
    }
    #[doc = "0x910 - System Time read access"]
    #[inline(always)]
    pub fn dc_sys_time_mut(&self) -> &mut [DC_SYS_TIME; 2] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2320usize) as *mut [DC_SYS_TIME; 2]) }
    }
}
#[doc = "Type of EtherCAT Controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](type_) module"]
pub type TYPE = crate::Reg<u8, _TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE;
#[doc = "`read()` method returns [type_::R](type_::R) reader structure"]
impl crate::Readable for TYPE {}
#[doc = "Type of EtherCAT Controller"]
pub mod type_;
#[doc = "Revision of EtherCAT Controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u8, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "Revision of EtherCAT Controller"]
pub mod revision;
#[doc = "Build Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [build](build) module"]
pub type BUILD = crate::Reg<u16, _BUILD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUILD;
#[doc = "`read()` method returns [build::R](build::R) reader structure"]
impl crate::Readable for BUILD {}
#[doc = "Build Version"]
pub mod build;
#[doc = "FMMUs Supported\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_num](fmmu_num) module"]
pub type FMMU_NUM = crate::Reg<u8, _FMMU_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMMU_NUM;
#[doc = "`read()` method returns [fmmu_num::R](fmmu_num::R) reader structure"]
impl crate::Readable for FMMU_NUM {}
#[doc = "FMMUs Supported"]
pub mod fmmu_num;
#[doc = "SyncManagers Supported\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync_manager](sync_manager) module"]
pub type SYNC_MANAGER = crate::Reg<u8, _SYNC_MANAGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC_MANAGER;
#[doc = "`read()` method returns [sync_manager::R](sync_manager::R) reader structure"]
impl crate::Readable for SYNC_MANAGER {}
#[doc = "SyncManagers Supported"]
pub mod sync_manager;
#[doc = "RAM Size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_size](ram_size) module"]
pub type RAM_SIZE = crate::Reg<u8, _RAM_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_SIZE;
#[doc = "`read()` method returns [ram_size::R](ram_size::R) reader structure"]
impl crate::Readable for RAM_SIZE {}
#[doc = "RAM Size"]
pub mod ram_size;
#[doc = "Port Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_desc](port_desc) module"]
pub type PORT_DESC = crate::Reg<u8, _PORT_DESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT_DESC;
#[doc = "`read()` method returns [port_desc::R](port_desc::R) reader structure"]
impl crate::Readable for PORT_DESC {}
#[doc = "Port Descriptor"]
pub mod port_desc;
#[doc = "ESC Features Supported\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feature](feature) module"]
pub type FEATURE = crate::Reg<u16, _FEATURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEATURE;
#[doc = "`read()` method returns [feature::R](feature::R) reader structure"]
impl crate::Readable for FEATURE {}
#[doc = "ESC Features Supported"]
pub mod feature;
#[doc = "Configured Station Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [station_adr](station_adr) module"]
pub type STATION_ADR = crate::Reg<u16, _STATION_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATION_ADR;
#[doc = "`read()` method returns [station_adr::R](station_adr::R) reader structure"]
impl crate::Readable for STATION_ADR {}
#[doc = "Configured Station Address"]
pub mod station_adr;
#[doc = "Configured Station Alias\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [station_alias](station_alias) module"]
pub type STATION_ALIAS = crate::Reg<u16, _STATION_ALIAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATION_ALIAS;
#[doc = "`read()` method returns [station_alias::R](station_alias::R) reader structure"]
impl crate::Readable for STATION_ALIAS {}
#[doc = "`write(|w| ..)` method takes [station_alias::W](station_alias::W) writer structure"]
impl crate::Writable for STATION_ALIAS {}
#[doc = "Configured Station Alias"]
pub mod station_alias;
#[doc = "Write Register Enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_reg_enable](wr_reg_enable) module"]
pub type WR_REG_ENABLE = crate::Reg<u8, _WR_REG_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_REG_ENABLE;
#[doc = "`read()` method returns [wr_reg_enable::R](wr_reg_enable::R) reader structure"]
impl crate::Readable for WR_REG_ENABLE {}
#[doc = "Write Register Enable"]
pub mod wr_reg_enable;
#[doc = "Write Register Protection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_reg_protect](wr_reg_protect) module"]
pub type WR_REG_PROTECT = crate::Reg<u8, _WR_REG_PROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_REG_PROTECT;
#[doc = "`read()` method returns [wr_reg_protect::R](wr_reg_protect::R) reader structure"]
impl crate::Readable for WR_REG_PROTECT {}
#[doc = "Write Register Protection"]
pub mod wr_reg_protect;
#[doc = "ESC Write Enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_wr_enable](esc_wr_enable) module"]
pub type ESC_WR_ENABLE = crate::Reg<u8, _ESC_WR_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_WR_ENABLE;
#[doc = "`read()` method returns [esc_wr_enable::R](esc_wr_enable::R) reader structure"]
impl crate::Readable for ESC_WR_ENABLE {}
#[doc = "ESC Write Enable"]
pub mod esc_wr_enable;
#[doc = "ESC Write Protection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_wr_protect](esc_wr_protect) module"]
pub type ESC_WR_PROTECT = crate::Reg<u8, _ESC_WR_PROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_WR_PROTECT;
#[doc = "`read()` method returns [esc_wr_protect::R](esc_wr_protect::R) reader structure"]
impl crate::Readable for ESC_WR_PROTECT {}
#[doc = "ESC Write Protection"]
pub mod esc_wr_protect;
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_reset_ecat](esc_reset_ecat) module"]
pub type ESC_RESET_ECAT = crate::Reg<u8, _ESC_RESET_ECAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_RESET_ECAT;
#[doc = "`read()` method returns [esc_reset_ecat::R](esc_reset_ecat::R) reader structure"]
impl crate::Readable for ESC_RESET_ECAT {}
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]"]
pub mod esc_reset_ecat;
#[doc = "ESC Reset ECAT \\[READ Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_reset_ecat](esc_reset_ecat) module"]
pub type ESC_RESET_ECAT = crate::Reg<u8, _ESC_RESET_ECAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_RESET_ECAT;
#[doc = "`read()` method returns [esc_reset_ecat::R](esc_reset_ecat::R) reader structure"]
impl crate::Readable for ESC_RESET_ECAT {}
#[doc = "ESC Reset ECAT \\[READ Mode\\]"]
pub mod esc_reset_ecat;
#[doc = "ESC Reset PDI \\[WRITE Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_reset_pdi](esc_reset_pdi) module"]
pub type ESC_RESET_PDI = crate::Reg<u8, _ESC_RESET_PDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_RESET_PDI;
#[doc = "`read()` method returns [esc_reset_pdi::R](esc_reset_pdi::R) reader structure"]
impl crate::Readable for ESC_RESET_PDI {}
#[doc = "ESC Reset PDI \\[WRITE Mode\\]"]
pub mod esc_reset_pdi;
#[doc = "ESC Reset PDI \\[READ Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_reset_pdi](esc_reset_pdi) module"]
pub type ESC_RESET_PDI = crate::Reg<u8, _ESC_RESET_PDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_RESET_PDI;
#[doc = "`read()` method returns [esc_reset_pdi::R](esc_reset_pdi::R) reader structure"]
impl crate::Readable for ESC_RESET_PDI {}
#[doc = "ESC Reset PDI \\[READ Mode\\]"]
pub mod esc_reset_pdi;
#[doc = "ESC DL Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_dl_control](esc_dl_control) module"]
pub type ESC_DL_CONTROL = crate::Reg<u32, _ESC_DL_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_DL_CONTROL;
#[doc = "`read()` method returns [esc_dl_control::R](esc_dl_control::R) reader structure"]
impl crate::Readable for ESC_DL_CONTROL {}
#[doc = "ESC DL Control"]
pub mod esc_dl_control;
#[doc = "Physical Read/Write Offset\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physical_rw_offset](physical_rw_offset) module"]
pub type PHYSICAL_RW_OFFSET = crate::Reg<u16, _PHYSICAL_RW_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PHYSICAL_RW_OFFSET;
#[doc = "`read()` method returns [physical_rw_offset::R](physical_rw_offset::R) reader structure"]
impl crate::Readable for PHYSICAL_RW_OFFSET {}
#[doc = "Physical Read/Write Offset"]
pub mod physical_rw_offset;
#[doc = "ESC DL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_dl_status](esc_dl_status) module"]
pub type ESC_DL_STATUS = crate::Reg<u16, _ESC_DL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_DL_STATUS;
#[doc = "`read()` method returns [esc_dl_status::R](esc_dl_status::R) reader structure"]
impl crate::Readable for ESC_DL_STATUS {}
#[doc = "ESC DL Status"]
pub mod esc_dl_status;
#[doc = "AL Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_control](al_control) module"]
pub type AL_CONTROL = crate::Reg<u16, _AL_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AL_CONTROL;
#[doc = "`read()` method returns [al_control::R](al_control::R) reader structure"]
impl crate::Readable for AL_CONTROL {}
#[doc = "AL Control"]
pub mod al_control;
#[doc = "AL Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_status](al_status) module"]
pub type AL_STATUS = crate::Reg<u16, _AL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AL_STATUS;
#[doc = "`read()` method returns [al_status::R](al_status::R) reader structure"]
impl crate::Readable for AL_STATUS {}
#[doc = "`write(|w| ..)` method takes [al_status::W](al_status::W) writer structure"]
impl crate::Writable for AL_STATUS {}
#[doc = "AL Status"]
pub mod al_status;
#[doc = "AL Status Code\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_status_code](al_status_code) module"]
pub type AL_STATUS_CODE = crate::Reg<u16, _AL_STATUS_CODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AL_STATUS_CODE;
#[doc = "`read()` method returns [al_status_code::R](al_status_code::R) reader structure"]
impl crate::Readable for AL_STATUS_CODE {}
#[doc = "`write(|w| ..)` method takes [al_status_code::W](al_status_code::W) writer structure"]
impl crate::Writable for AL_STATUS_CODE {}
#[doc = "AL Status Code"]
pub mod al_status_code;
#[doc = "RUN LED Override\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [run_led](run_led) module"]
pub type RUN_LED = crate::Reg<u8, _RUN_LED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RUN_LED;
#[doc = "`read()` method returns [run_led::R](run_led::R) reader structure"]
impl crate::Readable for RUN_LED {}
#[doc = "`write(|w| ..)` method takes [run_led::W](run_led::W) writer structure"]
impl crate::Writable for RUN_LED {}
#[doc = "RUN LED Override"]
pub mod run_led;
#[doc = "RUN ERR Override\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_led](err_led) module"]
pub type ERR_LED = crate::Reg<u8, _ERR_LED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR_LED;
#[doc = "`read()` method returns [err_led::R](err_led::R) reader structure"]
impl crate::Readable for ERR_LED {}
#[doc = "`write(|w| ..)` method takes [err_led::W](err_led::W) writer structure"]
impl crate::Writable for ERR_LED {}
#[doc = "RUN ERR Override"]
pub mod err_led;
#[doc = "PDI Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_control](pdi_control) module"]
pub type PDI_CONTROL = crate::Reg<u8, _PDI_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDI_CONTROL;
#[doc = "`read()` method returns [pdi_control::R](pdi_control::R) reader structure"]
impl crate::Readable for PDI_CONTROL {}
#[doc = "PDI Control"]
pub mod pdi_control;
#[doc = "ESC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_config](esc_config) module"]
pub type ESC_CONFIG = crate::Reg<u8, _ESC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESC_CONFIG;
#[doc = "`read()` method returns [esc_config::R](esc_config::R) reader structure"]
impl crate::Readable for ESC_CONFIG {}
#[doc = "ESC Configuration"]
pub mod esc_config;
#[doc = "PDI Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_config](pdi_config) module"]
pub type PDI_CONFIG = crate::Reg<u8, _PDI_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDI_CONFIG;
#[doc = "`read()` method returns [pdi_config::R](pdi_config::R) reader structure"]
impl crate::Readable for PDI_CONFIG {}
#[doc = "PDI Control"]
pub mod pdi_config;
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync_latch_config](sync_latch_config) module"]
pub type SYNC_LATCH_CONFIG = crate::Reg<u8, _SYNC_LATCH_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC_LATCH_CONFIG;
#[doc = "`read()` method returns [sync_latch_config::R](sync_latch_config::R) reader structure"]
impl crate::Readable for SYNC_LATCH_CONFIG {}
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration"]
pub mod sync_latch_config;
#[doc = "PDI Synchronous Microcontroller extended Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_ext_config](pdi_ext_config) module"]
pub type PDI_EXT_CONFIG = crate::Reg<u16, _PDI_EXT_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDI_EXT_CONFIG;
#[doc = "`read()` method returns [pdi_ext_config::R](pdi_ext_config::R) reader structure"]
impl crate::Readable for PDI_EXT_CONFIG {}
#[doc = "PDI Synchronous Microcontroller extended Configuration"]
pub mod pdi_ext_config;
#[doc = "ECAT Event Mask\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_mask](event_mask) module"]
pub type EVENT_MASK = crate::Reg<u16, _EVENT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENT_MASK;
#[doc = "`read()` method returns [event_mask::R](event_mask::R) reader structure"]
impl crate::Readable for EVENT_MASK {}
#[doc = "ECAT Event Mask"]
pub mod event_mask;
#[doc = "PDI AL Event Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_event_mask](al_event_mask) module"]
pub type AL_EVENT_MASK = crate::Reg<u32, _AL_EVENT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AL_EVENT_MASK;
#[doc = "`read()` method returns [al_event_mask::R](al_event_mask::R) reader structure"]
impl crate::Readable for AL_EVENT_MASK {}
#[doc = "`write(|w| ..)` method takes [al_event_mask::W](al_event_mask::W) writer structure"]
impl crate::Writable for AL_EVENT_MASK {}
#[doc = "PDI AL Event Mask"]
pub mod al_event_mask;
#[doc = "ECAT Event Request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_req](event_req) module"]
pub type EVENT_REQ = crate::Reg<u16, _EVENT_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENT_REQ;
#[doc = "`read()` method returns [event_req::R](event_req::R) reader structure"]
impl crate::Readable for EVENT_REQ {}
#[doc = "ECAT Event Request"]
pub mod event_req;
#[doc = "AL Event Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [al_event_req](al_event_req) module"]
pub type AL_EVENT_REQ = crate::Reg<u32, _AL_EVENT_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AL_EVENT_REQ;
#[doc = "`read()` method returns [al_event_req::R](al_event_req::R) reader structure"]
impl crate::Readable for AL_EVENT_REQ {}
#[doc = "`write(|w| ..)` method takes [al_event_req::W](al_event_req::W) writer structure"]
impl crate::Writable for AL_EVENT_REQ {}
#[doc = "AL Event Request"]
pub mod al_event_req;
#[doc = "RX Error Counter Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err_count0](rx_err_count0) module"]
pub type RX_ERR_COUNT0 = crate::Reg<u16, _RX_ERR_COUNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ERR_COUNT0;
#[doc = "`read()` method returns [rx_err_count0::R](rx_err_count0::R) reader structure"]
impl crate::Readable for RX_ERR_COUNT0 {}
#[doc = "RX Error Counter Port 0"]
pub mod rx_err_count0;
#[doc = "RX Error Counter Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err_count1](rx_err_count1) module"]
pub type RX_ERR_COUNT1 = crate::Reg<u16, _RX_ERR_COUNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ERR_COUNT1;
#[doc = "`read()` method returns [rx_err_count1::R](rx_err_count1::R) reader structure"]
impl crate::Readable for RX_ERR_COUNT1 {}
#[doc = "RX Error Counter Port 1"]
pub mod rx_err_count1;
#[doc = "Forwarded RX Error Counter Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwd_rx_err_count0](fwd_rx_err_count0) module"]
pub type FWD_RX_ERR_COUNT0 = crate::Reg<u8, _FWD_RX_ERR_COUNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWD_RX_ERR_COUNT0;
#[doc = "`read()` method returns [fwd_rx_err_count0::R](fwd_rx_err_count0::R) reader structure"]
impl crate::Readable for FWD_RX_ERR_COUNT0 {}
#[doc = "Forwarded RX Error Counter Port 0"]
pub mod fwd_rx_err_count0;
#[doc = "Forwarded RX Error Counter Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwd_rx_err_count1](fwd_rx_err_count1) module"]
pub type FWD_RX_ERR_COUNT1 = crate::Reg<u8, _FWD_RX_ERR_COUNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWD_RX_ERR_COUNT1;
#[doc = "`read()` method returns [fwd_rx_err_count1::R](fwd_rx_err_count1::R) reader structure"]
impl crate::Readable for FWD_RX_ERR_COUNT1 {}
#[doc = "Forwarded RX Error Counter Port 1"]
pub mod fwd_rx_err_count1;
#[doc = "ECAT Processing Unit Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_err_count](proc_err_count) module"]
pub type PROC_ERR_COUNT = crate::Reg<u8, _PROC_ERR_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROC_ERR_COUNT;
#[doc = "`read()` method returns [proc_err_count::R](proc_err_count::R) reader structure"]
impl crate::Readable for PROC_ERR_COUNT {}
#[doc = "ECAT Processing Unit Error Counter"]
pub mod proc_err_count;
#[doc = "PDI Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdi_err_count](pdi_err_count) module"]
pub type PDI_ERR_COUNT = crate::Reg<u8, _PDI_ERR_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDI_ERR_COUNT;
#[doc = "`read()` method returns [pdi_err_count::R](pdi_err_count::R) reader structure"]
impl crate::Readable for PDI_ERR_COUNT {}
#[doc = "PDI Error Counter"]
pub mod pdi_err_count;
#[doc = "Lost Link Counter Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lost_link_count0](lost_link_count0) module"]
pub type LOST_LINK_COUNT0 = crate::Reg<u8, _LOST_LINK_COUNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOST_LINK_COUNT0;
#[doc = "`read()` method returns [lost_link_count0::R](lost_link_count0::R) reader structure"]
impl crate::Readable for LOST_LINK_COUNT0 {}
#[doc = "Lost Link Counter Port 0"]
pub mod lost_link_count0;
#[doc = "Lost Link Counter Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lost_link_count1](lost_link_count1) module"]
pub type LOST_LINK_COUNT1 = crate::Reg<u8, _LOST_LINK_COUNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOST_LINK_COUNT1;
#[doc = "`read()` method returns [lost_link_count1::R](lost_link_count1::R) reader structure"]
impl crate::Readable for LOST_LINK_COUNT1 {}
#[doc = "Lost Link Counter Port 1"]
pub mod lost_link_count1;
#[doc = "Watchdog Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_divide](wd_divide) module"]
pub type WD_DIVIDE = crate::Reg<u16, _WD_DIVIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WD_DIVIDE;
#[doc = "`read()` method returns [wd_divide::R](wd_divide::R) reader structure"]
impl crate::Readable for WD_DIVIDE {}
#[doc = "`write(|w| ..)` method takes [wd_divide::W](wd_divide::W) writer structure"]
impl crate::Writable for WD_DIVIDE {}
#[doc = "Watchdog Divider"]
pub mod wd_divide;
#[doc = "Watchdog Time PDI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_time_pdi](wd_time_pdi) module"]
pub type WD_TIME_PDI = crate::Reg<u16, _WD_TIME_PDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WD_TIME_PDI;
#[doc = "`read()` method returns [wd_time_pdi::R](wd_time_pdi::R) reader structure"]
impl crate::Readable for WD_TIME_PDI {}
#[doc = "`write(|w| ..)` method takes [wd_time_pdi::W](wd_time_pdi::W) writer structure"]
impl crate::Writable for WD_TIME_PDI {}
#[doc = "Watchdog Time PDI"]
pub mod wd_time_pdi;
#[doc = "Watchdog Time Process Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_time_pdata](wd_time_pdata) module"]
pub type WD_TIME_PDATA = crate::Reg<u16, _WD_TIME_PDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WD_TIME_PDATA;
#[doc = "`read()` method returns [wd_time_pdata::R](wd_time_pdata::R) reader structure"]
impl crate::Readable for WD_TIME_PDATA {}
#[doc = "`write(|w| ..)` method takes [wd_time_pdata::W](wd_time_pdata::W) writer structure"]
impl crate::Writable for WD_TIME_PDATA {}
#[doc = "Watchdog Time Process Data"]
pub mod wd_time_pdata;
#[doc = "Watchdog Status Process Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_stat_pdata](wd_stat_pdata) module"]
pub type WD_STAT_PDATA = crate::Reg<u16, _WD_STAT_PDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WD_STAT_PDATA;
#[doc = "`read()` method returns [wd_stat_pdata::R](wd_stat_pdata::R) reader structure"]
impl crate::Readable for WD_STAT_PDATA {}
#[doc = "Watchdog Status Process Data"]
pub mod wd_stat_pdata;
#[doc = "Watchdog Counter Process Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_count_pdata](wd_count_pdata) module"]
pub type WD_COUNT_PDATA = crate::Reg<u8, _WD_COUNT_PDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WD_COUNT_PDATA;
#[doc = "`read()` method returns [wd_count_pdata::R](wd_count_pdata::R) reader structure"]
impl crate::Readable for WD_COUNT_PDATA {}
#[doc = "Watchdog Counter Process Data"]
pub mod wd_count_pdata;
#[doc = "Watchdog Counter PDI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wd_count_pdi](wd_count_pdi) module"]
pub type WD_COUNT_PDI = crate::Reg<u8, _WD_COUNT_PDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WD_COUNT_PDI;
#[doc = "`read()` method returns [wd_count_pdi::R](wd_count_pdi::R) reader structure"]
impl crate::Readable for WD_COUNT_PDI {}
#[doc = "Watchdog Counter PDI"]
pub mod wd_count_pdi;
#[doc = "EEPROM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_conf](eep_conf) module"]
pub type EEP_CONF = crate::Reg<u8, _EEP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEP_CONF;
#[doc = "`read()` method returns [eep_conf::R](eep_conf::R) reader structure"]
impl crate::Readable for EEP_CONF {}
#[doc = "EEPROM Configuration"]
pub mod eep_conf;
#[doc = "EEPROM PDI Access State\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_state](eep_state) module"]
pub type EEP_STATE = crate::Reg<u8, _EEP_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEP_STATE;
#[doc = "`read()` method returns [eep_state::R](eep_state::R) reader structure"]
impl crate::Readable for EEP_STATE {}
#[doc = "`write(|w| ..)` method takes [eep_state::W](eep_state::W) writer structure"]
impl crate::Writable for EEP_STATE {}
#[doc = "EEPROM PDI Access State"]
pub mod eep_state;
#[doc = "EEPROM Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_cont_stat](eep_cont_stat) module"]
pub type EEP_CONT_STAT = crate::Reg<u16, _EEP_CONT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEP_CONT_STAT;
#[doc = "`read()` method returns [eep_cont_stat::R](eep_cont_stat::R) reader structure"]
impl crate::Readable for EEP_CONT_STAT {}
#[doc = "`write(|w| ..)` method takes [eep_cont_stat::W](eep_cont_stat::W) writer structure"]
impl crate::Writable for EEP_CONT_STAT {}
#[doc = "EEPROM Control/Status"]
pub mod eep_cont_stat;
#[doc = "EEPROM Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_adr](eep_adr) module"]
pub type EEP_ADR = crate::Reg<u32, _EEP_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEP_ADR;
#[doc = "`read()` method returns [eep_adr::R](eep_adr::R) reader structure"]
impl crate::Readable for EEP_ADR {}
#[doc = "`write(|w| ..)` method takes [eep_adr::W](eep_adr::W) writer structure"]
impl crate::Writable for EEP_ADR {}
#[doc = "EEPROM Address"]
pub mod eep_adr;
#[doc = "EEPROM Read/Write data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eep_data](eep_data) module"]
pub type EEP_DATA = crate::Reg<u32, _EEP_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEP_DATA;
#[doc = "`read()` method returns [eep_data::R](eep_data::R) reader structure"]
impl crate::Readable for EEP_DATA {}
#[doc = "`write(|w| ..)` method takes [eep_data::W](eep_data::W) writer structure"]
impl crate::Writable for EEP_DATA {}
#[doc = "EEPROM Read/Write data"]
pub mod eep_data;
#[doc = "MII Management Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_cont_stat](mii_cont_stat) module"]
pub type MII_CONT_STAT = crate::Reg<u16, _MII_CONT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MII_CONT_STAT;
#[doc = "`read()` method returns [mii_cont_stat::R](mii_cont_stat::R) reader structure"]
impl crate::Readable for MII_CONT_STAT {}
#[doc = "`write(|w| ..)` method takes [mii_cont_stat::W](mii_cont_stat::W) writer structure"]
impl crate::Writable for MII_CONT_STAT {}
#[doc = "MII Management Control/Status"]
pub mod mii_cont_stat;
#[doc = "PHY Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_phy_adr](mii_phy_adr) module"]
pub type MII_PHY_ADR = crate::Reg<u8, _MII_PHY_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MII_PHY_ADR;
#[doc = "`read()` method returns [mii_phy_adr::R](mii_phy_adr::R) reader structure"]
impl crate::Readable for MII_PHY_ADR {}
#[doc = "`write(|w| ..)` method takes [mii_phy_adr::W](mii_phy_adr::W) writer structure"]
impl crate::Writable for MII_PHY_ADR {}
#[doc = "PHY Address"]
pub mod mii_phy_adr;
#[doc = "PHY Register Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_phy_reg_adr](mii_phy_reg_adr) module"]
pub type MII_PHY_REG_ADR = crate::Reg<u8, _MII_PHY_REG_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MII_PHY_REG_ADR;
#[doc = "`read()` method returns [mii_phy_reg_adr::R](mii_phy_reg_adr::R) reader structure"]
impl crate::Readable for MII_PHY_REG_ADR {}
#[doc = "`write(|w| ..)` method takes [mii_phy_reg_adr::W](mii_phy_reg_adr::W) writer structure"]
impl crate::Writable for MII_PHY_REG_ADR {}
#[doc = "PHY Register Address"]
pub mod mii_phy_reg_adr;
#[doc = "PHY Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_phy_data](mii_phy_data) module"]
pub type MII_PHY_DATA = crate::Reg<u16, _MII_PHY_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MII_PHY_DATA;
#[doc = "`read()` method returns [mii_phy_data::R](mii_phy_data::R) reader structure"]
impl crate::Readable for MII_PHY_DATA {}
#[doc = "`write(|w| ..)` method takes [mii_phy_data::W](mii_phy_data::W) writer structure"]
impl crate::Writable for MII_PHY_DATA {}
#[doc = "PHY Data"]
pub mod mii_phy_data;
#[doc = "MII ECAT ACS STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_ecat_acs_state](mii_ecat_acs_state) module"]
pub type MII_ECAT_ACS_STATE = crate::Reg<u8, _MII_ECAT_ACS_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MII_ECAT_ACS_STATE;
#[doc = "`read()` method returns [mii_ecat_acs_state::R](mii_ecat_acs_state::R) reader structure"]
impl crate::Readable for MII_ECAT_ACS_STATE {}
#[doc = "MII ECAT ACS STATE"]
pub mod mii_ecat_acs_state;
#[doc = "MII PDI ACS STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_pdi_acs_state](mii_pdi_acs_state) module"]
pub type MII_PDI_ACS_STATE = crate::Reg<u8, _MII_PDI_ACS_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MII_PDI_ACS_STATE;
#[doc = "`read()` method returns [mii_pdi_acs_state::R](mii_pdi_acs_state::R) reader structure"]
impl crate::Readable for MII_PDI_ACS_STATE {}
#[doc = "`write(|w| ..)` method takes [mii_pdi_acs_state::W](mii_pdi_acs_state::W) writer structure"]
impl crate::Writable for MII_PDI_ACS_STATE {}
#[doc = "MII PDI ACS STATE"]
pub mod mii_pdi_acs_state;
#[doc = "Receive Time Port 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_rcv_time_port0](dc_rcv_time_port0) module"]
pub type DC_RCV_TIME_PORT0 = crate::Reg<u32, _DC_RCV_TIME_PORT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_RCV_TIME_PORT0;
#[doc = "`read()` method returns [dc_rcv_time_port0::R](dc_rcv_time_port0::R) reader structure"]
impl crate::Readable for DC_RCV_TIME_PORT0 {}
#[doc = "Receive Time Port 0"]
pub mod dc_rcv_time_port0;
#[doc = "Receive Time Port 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_rcv_time_port1](dc_rcv_time_port1) module"]
pub type DC_RCV_TIME_PORT1 = crate::Reg<u32, _DC_RCV_TIME_PORT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_RCV_TIME_PORT1;
#[doc = "`read()` method returns [dc_rcv_time_port1::R](dc_rcv_time_port1::R) reader structure"]
impl crate::Readable for DC_RCV_TIME_PORT1 {}
#[doc = "Receive Time Port 1"]
pub mod dc_rcv_time_port1;
#[doc = "Local time of the beginning of a frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_time_pu](receive_time_pu) module"]
pub type RECEIVE_TIME_PU = crate::Reg<u32, _RECEIVE_TIME_PU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECEIVE_TIME_PU;
#[doc = "`read()` method returns [receive_time_pu::R](receive_time_pu::R) reader structure"]
impl crate::Readable for RECEIVE_TIME_PU {}
#[doc = "Local time of the beginning of a frame"]
pub mod receive_time_pu;
#[doc = "System Time read access\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time](dc_sys_time) module"]
pub type DC_SYS_TIME = crate::Reg<u32, _DC_SYS_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYS_TIME;
#[doc = "`read()` method returns [dc_sys_time::R](dc_sys_time::R) reader structure"]
impl crate::Readable for DC_SYS_TIME {}
#[doc = "System Time read access"]
pub mod dc_sys_time;
#[doc = "System Time \\[WRITE Mode\\]\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time](dc_sys_time) module"]
pub type DC_SYS_TIME = crate::Reg<u32, _DC_SYS_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYS_TIME;
#[doc = "`write(|w| ..)` method takes [dc_sys_time::W](dc_sys_time::W) writer structure"]
impl crate::Writable for DC_SYS_TIME {}
#[doc = "System Time \\[WRITE Mode\\]"]
pub mod dc_sys_time;
#[doc = "Difference between local time and System Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_offset](dc_sys_time_offset) module"]
pub type DC_SYS_TIME_OFFSET = crate::Reg<u32, _DC_SYS_TIME_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYS_TIME_OFFSET;
#[doc = "`read()` method returns [dc_sys_time_offset::R](dc_sys_time_offset::R) reader structure"]
impl crate::Readable for DC_SYS_TIME_OFFSET {}
#[doc = "`write(|w| ..)` method takes [dc_sys_time_offset::W](dc_sys_time_offset::W) writer structure"]
impl crate::Writable for DC_SYS_TIME_OFFSET {}
#[doc = "Difference between local time and System Time"]
pub mod dc_sys_time_offset;
#[doc = "System Time Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_delay](dc_sys_time_delay) module"]
pub type DC_SYS_TIME_DELAY = crate::Reg<u32, _DC_SYS_TIME_DELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYS_TIME_DELAY;
#[doc = "`read()` method returns [dc_sys_time_delay::R](dc_sys_time_delay::R) reader structure"]
impl crate::Readable for DC_SYS_TIME_DELAY {}
#[doc = "`write(|w| ..)` method takes [dc_sys_time_delay::W](dc_sys_time_delay::W) writer structure"]
impl crate::Writable for DC_SYS_TIME_DELAY {}
#[doc = "System Time Delay"]
pub mod dc_sys_time_delay;
#[doc = "System Time Difference\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_diff](dc_sys_time_diff) module"]
pub type DC_SYS_TIME_DIFF = crate::Reg<u32, _DC_SYS_TIME_DIFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYS_TIME_DIFF;
#[doc = "`read()` method returns [dc_sys_time_diff::R](dc_sys_time_diff::R) reader structure"]
impl crate::Readable for DC_SYS_TIME_DIFF {}
#[doc = "System Time Difference"]
pub mod dc_sys_time_diff;
#[doc = "Speed Counter Start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_speed_count_start](dc_speed_count_start) module"]
pub type DC_SPEED_COUNT_START = crate::Reg<u16, _DC_SPEED_COUNT_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SPEED_COUNT_START;
#[doc = "`read()` method returns [dc_speed_count_start::R](dc_speed_count_start::R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_START {}
#[doc = "`write(|w| ..)` method takes [dc_speed_count_start::W](dc_speed_count_start::W) writer structure"]
impl crate::Writable for DC_SPEED_COUNT_START {}
#[doc = "Speed Counter Start"]
pub mod dc_speed_count_start;
#[doc = "Speed Counter Diff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_speed_count_diff](dc_speed_count_diff) module"]
pub type DC_SPEED_COUNT_DIFF = crate::Reg<u16, _DC_SPEED_COUNT_DIFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SPEED_COUNT_DIFF;
#[doc = "`read()` method returns [dc_speed_count_diff::R](dc_speed_count_diff::R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_DIFF {}
#[doc = "Speed Counter Diff"]
pub mod dc_speed_count_diff;
#[doc = "System Time Difference Filter Depth\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_fil_depth](dc_sys_time_fil_depth) module"]
pub type DC_SYS_TIME_FIL_DEPTH = crate::Reg<u8, _DC_SYS_TIME_FIL_DEPTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYS_TIME_FIL_DEPTH;
#[doc = "`read()` method returns [dc_sys_time_fil_depth::R](dc_sys_time_fil_depth::R) reader structure"]
impl crate::Readable for DC_SYS_TIME_FIL_DEPTH {}
#[doc = "`write(|w| ..)` method takes [dc_sys_time_fil_depth::W](dc_sys_time_fil_depth::W) writer structure"]
impl crate::Writable for DC_SYS_TIME_FIL_DEPTH {}
#[doc = "System Time Difference Filter Depth"]
pub mod dc_sys_time_fil_depth;
#[doc = "Speed Counter Filter Depth\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_speed_count_fil_depth](dc_speed_count_fil_depth) module"]
pub type DC_SPEED_COUNT_FIL_DEPTH = crate::Reg<u8, _DC_SPEED_COUNT_FIL_DEPTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SPEED_COUNT_FIL_DEPTH;
#[doc = "`read()` method returns [dc_speed_count_fil_depth::R](dc_speed_count_fil_depth::R) reader structure"]
impl crate::Readable for DC_SPEED_COUNT_FIL_DEPTH {}
#[doc = "`write(|w| ..)` method takes [dc_speed_count_fil_depth::W](dc_speed_count_fil_depth::W) writer structure"]
impl crate::Writable for DC_SPEED_COUNT_FIL_DEPTH {}
#[doc = "Speed Counter Filter Depth"]
pub mod dc_speed_count_fil_depth;
#[doc = "Cyclic Unit Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_cyc_cont](dc_cyc_cont) module"]
pub type DC_CYC_CONT = crate::Reg<u8, _DC_CYC_CONT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_CYC_CONT;
#[doc = "`read()` method returns [dc_cyc_cont::R](dc_cyc_cont::R) reader structure"]
impl crate::Readable for DC_CYC_CONT {}
#[doc = "Cyclic Unit Control"]
pub mod dc_cyc_cont;
#[doc = "Activation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_act](dc_act) module"]
pub type DC_ACT = crate::Reg<u8, _DC_ACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_ACT;
#[doc = "`read()` method returns [dc_act::R](dc_act::R) reader structure"]
impl crate::Readable for DC_ACT {}
#[doc = "`write(|w| ..)` method takes [dc_act::W](dc_act::W) writer structure"]
impl crate::Writable for DC_ACT {}
#[doc = "Activation register"]
pub mod dc_act;
#[doc = "Pulse Length of SyncSignals\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_pulse_len](dc_pulse_len) module"]
pub type DC_PULSE_LEN = crate::Reg<u16, _DC_PULSE_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_PULSE_LEN;
#[doc = "`read()` method returns [dc_pulse_len::R](dc_pulse_len::R) reader structure"]
impl crate::Readable for DC_PULSE_LEN {}
#[doc = "Pulse Length of SyncSignals"]
pub mod dc_pulse_len;
#[doc = "Activation Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_act_stat](dc_act_stat) module"]
pub type DC_ACT_STAT = crate::Reg<u8, _DC_ACT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_ACT_STAT;
#[doc = "`read()` method returns [dc_act_stat::R](dc_act_stat::R) reader structure"]
impl crate::Readable for DC_ACT_STAT {}
#[doc = "Activation Status"]
pub mod dc_act_stat;
#[doc = "SYNC0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync0_stat](dc_sync0_stat) module"]
pub type DC_SYNC0_STAT = crate::Reg<u8, _DC_SYNC0_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYNC0_STAT;
#[doc = "`read()` method returns [dc_sync0_stat::R](dc_sync0_stat::R) reader structure"]
impl crate::Readable for DC_SYNC0_STAT {}
#[doc = "SYNC0 Status"]
pub mod dc_sync0_stat;
#[doc = "SYNC1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync1_stat](dc_sync1_stat) module"]
pub type DC_SYNC1_STAT = crate::Reg<u8, _DC_SYNC1_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYNC1_STAT;
#[doc = "`read()` method returns [dc_sync1_stat::R](dc_sync1_stat::R) reader structure"]
impl crate::Readable for DC_SYNC1_STAT {}
#[doc = "SYNC1 Status"]
pub mod dc_sync1_stat;
#[doc = "Start Time Cyclic Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_cyc_start_time](dc_cyc_start_time) module"]
pub type DC_CYC_START_TIME = crate::Reg<u32, _DC_CYC_START_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_CYC_START_TIME;
#[doc = "`read()` method returns [dc_cyc_start_time::R](dc_cyc_start_time::R) reader structure"]
impl crate::Readable for DC_CYC_START_TIME {}
#[doc = "`write(|w| ..)` method takes [dc_cyc_start_time::W](dc_cyc_start_time::W) writer structure"]
impl crate::Writable for DC_CYC_START_TIME {}
#[doc = "Start Time Cyclic Operation"]
pub mod dc_cyc_start_time;
#[doc = "System time of next SYNC1 pulse in ns\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_next_sync1_pulse](dc_next_sync1_pulse) module"]
pub type DC_NEXT_SYNC1_PULSE = crate::Reg<u32, _DC_NEXT_SYNC1_PULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_NEXT_SYNC1_PULSE;
#[doc = "`read()` method returns [dc_next_sync1_pulse::R](dc_next_sync1_pulse::R) reader structure"]
impl crate::Readable for DC_NEXT_SYNC1_PULSE {}
#[doc = "System time of next SYNC1 pulse in ns"]
pub mod dc_next_sync1_pulse;
#[doc = "SYNC0 Cycle Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync0_cyc_time](dc_sync0_cyc_time) module"]
pub type DC_SYNC0_CYC_TIME = crate::Reg<u32, _DC_SYNC0_CYC_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYNC0_CYC_TIME;
#[doc = "`read()` method returns [dc_sync0_cyc_time::R](dc_sync0_cyc_time::R) reader structure"]
impl crate::Readable for DC_SYNC0_CYC_TIME {}
#[doc = "`write(|w| ..)` method takes [dc_sync0_cyc_time::W](dc_sync0_cyc_time::W) writer structure"]
impl crate::Writable for DC_SYNC0_CYC_TIME {}
#[doc = "SYNC0 Cycle Time"]
pub mod dc_sync0_cyc_time;
#[doc = "SYNC1 Cycle Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sync1_cyc_time](dc_sync1_cyc_time) module"]
pub type DC_SYNC1_CYC_TIME = crate::Reg<u32, _DC_SYNC1_CYC_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_SYNC1_CYC_TIME;
#[doc = "`read()` method returns [dc_sync1_cyc_time::R](dc_sync1_cyc_time::R) reader structure"]
impl crate::Readable for DC_SYNC1_CYC_TIME {}
#[doc = "`write(|w| ..)` method takes [dc_sync1_cyc_time::W](dc_sync1_cyc_time::W) writer structure"]
impl crate::Writable for DC_SYNC1_CYC_TIME {}
#[doc = "SYNC1 Cycle Time"]
pub mod dc_sync1_cyc_time;
#[doc = "Latch0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_cont](dc_latch0_cont) module"]
pub type DC_LATCH0_CONT = crate::Reg<u8, _DC_LATCH0_CONT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH0_CONT;
#[doc = "`read()` method returns [dc_latch0_cont::R](dc_latch0_cont::R) reader structure"]
impl crate::Readable for DC_LATCH0_CONT {}
#[doc = "`write(|w| ..)` method takes [dc_latch0_cont::W](dc_latch0_cont::W) writer structure"]
impl crate::Writable for DC_LATCH0_CONT {}
#[doc = "Latch0 Control"]
pub mod dc_latch0_cont;
#[doc = "Latch1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch1_cont](dc_latch1_cont) module"]
pub type DC_LATCH1_CONT = crate::Reg<u8, _DC_LATCH1_CONT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH1_CONT;
#[doc = "`read()` method returns [dc_latch1_cont::R](dc_latch1_cont::R) reader structure"]
impl crate::Readable for DC_LATCH1_CONT {}
#[doc = "`write(|w| ..)` method takes [dc_latch1_cont::W](dc_latch1_cont::W) writer structure"]
impl crate::Writable for DC_LATCH1_CONT {}
#[doc = "Latch1 Control"]
pub mod dc_latch1_cont;
#[doc = "Latch0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_stat](dc_latch0_stat) module"]
pub type DC_LATCH0_STAT = crate::Reg<u8, _DC_LATCH0_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH0_STAT;
#[doc = "`read()` method returns [dc_latch0_stat::R](dc_latch0_stat::R) reader structure"]
impl crate::Readable for DC_LATCH0_STAT {}
#[doc = "Latch0 Status"]
pub mod dc_latch0_stat;
#[doc = "Latch1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch1_stat](dc_latch1_stat) module"]
pub type DC_LATCH1_STAT = crate::Reg<u8, _DC_LATCH1_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH1_STAT;
#[doc = "`read()` method returns [dc_latch1_stat::R](dc_latch1_stat::R) reader structure"]
impl crate::Readable for DC_LATCH1_STAT {}
#[doc = "Latch1 Status"]
pub mod dc_latch1_stat;
#[doc = "Register captures System time at the positive edge of the Latch0 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_time_pos](dc_latch0_time_pos) module"]
pub type DC_LATCH0_TIME_POS = crate::Reg<u32, _DC_LATCH0_TIME_POS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH0_TIME_POS;
#[doc = "`read()` method returns [dc_latch0_time_pos::R](dc_latch0_time_pos::R) reader structure"]
impl crate::Readable for DC_LATCH0_TIME_POS {}
#[doc = "Register captures System time at the positive edge of the Latch0 signal"]
pub mod dc_latch0_time_pos;
#[doc = "Register captures System time at the negative edge of the Latch0 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch0_time_neg](dc_latch0_time_neg) module"]
pub type DC_LATCH0_TIME_NEG = crate::Reg<u32, _DC_LATCH0_TIME_NEG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH0_TIME_NEG;
#[doc = "`read()` method returns [dc_latch0_time_neg::R](dc_latch0_time_neg::R) reader structure"]
impl crate::Readable for DC_LATCH0_TIME_NEG {}
#[doc = "Register captures System time at the negative edge of the Latch0 signal"]
pub mod dc_latch0_time_neg;
#[doc = "Register captures System time at the positive edge of the Latch1 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch1_time_pos](dc_latch1_time_pos) module"]
pub type DC_LATCH1_TIME_POS = crate::Reg<u32, _DC_LATCH1_TIME_POS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH1_TIME_POS;
#[doc = "`read()` method returns [dc_latch1_time_pos::R](dc_latch1_time_pos::R) reader structure"]
impl crate::Readable for DC_LATCH1_TIME_POS {}
#[doc = "Register captures System time at the positive edge of the Latch1 signal"]
pub mod dc_latch1_time_pos;
#[doc = "Register captures System time at the negative edge of the Latch1 signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_latch1_time_neg](dc_latch1_time_neg) module"]
pub type DC_LATCH1_TIME_NEG = crate::Reg<u32, _DC_LATCH1_TIME_NEG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_LATCH1_TIME_NEG;
#[doc = "`read()` method returns [dc_latch1_time_neg::R](dc_latch1_time_neg::R) reader structure"]
impl crate::Readable for DC_LATCH1_TIME_NEG {}
#[doc = "Register captures System time at the negative edge of the Latch1 signal"]
pub mod dc_latch1_time_neg;
#[doc = "EtherCAT Buffer Change Event Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_ecat_cng_ev_time](dc_ecat_cng_ev_time) module"]
pub type DC_ECAT_CNG_EV_TIME = crate::Reg<u32, _DC_ECAT_CNG_EV_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_ECAT_CNG_EV_TIME;
#[doc = "`read()` method returns [dc_ecat_cng_ev_time::R](dc_ecat_cng_ev_time::R) reader structure"]
impl crate::Readable for DC_ECAT_CNG_EV_TIME {}
#[doc = "EtherCAT Buffer Change Event Time"]
pub mod dc_ecat_cng_ev_time;
#[doc = "PDI Buffer Start Event Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_pdi_start_ev_time](dc_pdi_start_ev_time) module"]
pub type DC_PDI_START_EV_TIME = crate::Reg<u32, _DC_PDI_START_EV_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_PDI_START_EV_TIME;
#[doc = "`read()` method returns [dc_pdi_start_ev_time::R](dc_pdi_start_ev_time::R) reader structure"]
impl crate::Readable for DC_PDI_START_EV_TIME {}
#[doc = "PDI Buffer Start Event Time"]
pub mod dc_pdi_start_ev_time;
#[doc = "PDI Buffer Change Event Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_pdi_cng_ev_time](dc_pdi_cng_ev_time) module"]
pub type DC_PDI_CNG_EV_TIME = crate::Reg<u32, _DC_PDI_CNG_EV_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_PDI_CNG_EV_TIME;
#[doc = "`read()` method returns [dc_pdi_cng_ev_time::R](dc_pdi_cng_ev_time::R) reader structure"]
impl crate::Readable for DC_PDI_CNG_EV_TIME {}
#[doc = "PDI Buffer Change Event Time"]
pub mod dc_pdi_cng_ev_time;
#[doc = "ECAT0 Module ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "ECAT0 Module ID"]
pub mod id;
#[doc = "ECAT0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "ECAT0 Status"]
pub mod status;
