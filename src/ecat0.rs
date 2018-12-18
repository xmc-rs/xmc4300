#[doc = r" Register block"]
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
    _reserved0: [u8; 6usize],
    #[doc = "0x10 - Configured Station Address"]
    pub station_adr: STATION_ADR,
    #[doc = "0x12 - Configured Station Alias"]
    pub station_alias: STATION_ALIAS,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Write Register Enable"]
    pub wr_reg_enable: WR_REG_ENABLE,
    #[doc = "0x21 - Write Register Protection"]
    pub wr_reg_protect: WR_REG_PROTECT,
    _reserved2: [u8; 14usize],
    #[doc = "0x30 - ESC Write Enable"]
    pub esc_wr_enable: ESC_WR_ENABLE,
    #[doc = "0x31 - ESC Write Protection"]
    pub esc_wr_protect: ESC_WR_PROTECT,
    _reserved3: [u8; 14usize],
    #[doc = "0x40 - ESC Reset ECAT \\[WRITE Mode\\]"]
    pub esc_reset_ecat: ESC_RESET_ECAT,
    #[doc = "0x41 - ESC Reset PDI \\[WRITE Mode\\]"]
    pub esc_reset_pdi: ESC_RESET_PDI,
    _reserved4: [u8; 190usize],
    #[doc = "0x100 - ESC DL Control"]
    pub esc_dl_control: ESC_DL_CONTROL,
    _reserved5: [u8; 4usize],
    #[doc = "0x108 - Physical Read/Write Offset"]
    pub physical_rw_offset: PHYSICAL_RW_OFFSET,
    _reserved6: [u8; 6usize],
    #[doc = "0x110 - ESC DL Status"]
    pub esc_dl_status: ESC_DL_STATUS,
    _reserved7: [u8; 14usize],
    #[doc = "0x120 - AL Control"]
    pub al_control: AL_CONTROL,
    _reserved8: [u8; 14usize],
    #[doc = "0x130 - AL Status"]
    pub al_status: AL_STATUS,
    _reserved9: [u8; 2usize],
    #[doc = "0x134 - AL Status Code"]
    pub al_status_code: AL_STATUS_CODE,
    _reserved10: [u8; 2usize],
    #[doc = "0x138 - RUN LED Override"]
    pub run_led: RUN_LED,
    #[doc = "0x139 - RUN ERR Override"]
    pub err_led: ERR_LED,
    _reserved11: [u8; 6usize],
    #[doc = "0x140 - PDI Control"]
    pub pdi_control: PDI_CONTROL,
    #[doc = "0x141 - ESC Configuration"]
    pub esc_config: ESC_CONFIG,
    _reserved12: [u8; 14usize],
    #[doc = "0x150 - PDI Control"]
    pub pdi_config: PDI_CONFIG,
    #[doc = "0x151 - Sync/Latch\\[1:0\\] PDI Configuration"]
    pub sync_latch_config: SYNC_LATCH_CONFIG,
    #[doc = "0x152 - PDI Synchronous Microcontroller extended Configuration"]
    pub pdi_ext_config: PDI_EXT_CONFIG,
    _reserved13: [u8; 172usize],
    #[doc = "0x200 - ECAT Event Mask"]
    pub event_mask: EVENT_MASK,
    _reserved14: [u8; 2usize],
    #[doc = "0x204 - PDI AL Event Mask"]
    pub al_event_mask: AL_EVENT_MASK,
    _reserved15: [u8; 8usize],
    #[doc = "0x210 - ECAT Event Request"]
    pub event_req: EVENT_REQ,
    _reserved16: [u8; 14usize],
    #[doc = "0x220 - AL Event Request"]
    pub al_event_req: AL_EVENT_REQ,
    _reserved17: [u8; 220usize],
    #[doc = "0x300 - RX Error Counter Port 0"]
    pub rx_err_count0: RX_ERR_COUNT0,
    #[doc = "0x302 - RX Error Counter Port 1"]
    pub rx_err_count1: RX_ERR_COUNT1,
    _reserved18: [u8; 4usize],
    #[doc = "0x308 - Forwarded RX Error Counter Port 0"]
    pub fwd_rx_err_count0: FWD_RX_ERR_COUNT0,
    #[doc = "0x309 - Forwarded RX Error Counter Port 1"]
    pub fwd_rx_err_count1: FWD_RX_ERR_COUNT1,
    _reserved19: [u8; 2usize],
    #[doc = "0x30c - ECAT Processing Unit Error Counter"]
    pub proc_err_count: PROC_ERR_COUNT,
    #[doc = "0x30d - PDI Error Counter"]
    pub pdi_err_count: PDI_ERR_COUNT,
    _reserved20: [u8; 2usize],
    #[doc = "0x310 - Lost Link Counter Port 0"]
    pub lost_link_count0: LOST_LINK_COUNT0,
    #[doc = "0x311 - Lost Link Counter Port 1"]
    pub lost_link_count1: LOST_LINK_COUNT1,
    _reserved21: [u8; 238usize],
    #[doc = "0x400 - Watchdog Divider"]
    pub wd_divide: WD_DIVIDE,
    _reserved22: [u8; 14usize],
    #[doc = "0x410 - Watchdog Time PDI"]
    pub wd_time_pdi: WD_TIME_PDI,
    _reserved23: [u8; 14usize],
    #[doc = "0x420 - Watchdog Time Process Data"]
    pub wd_time_pdata: WD_TIME_PDATA,
    _reserved24: [u8; 30usize],
    #[doc = "0x440 - Watchdog Status Process Data"]
    pub wd_stat_pdata: WD_STAT_PDATA,
    #[doc = "0x442 - Watchdog Counter Process Data"]
    pub wd_count_pdata: WD_COUNT_PDATA,
    #[doc = "0x443 - Watchdog Counter PDI"]
    pub wd_count_pdi: WD_COUNT_PDI,
    _reserved25: [u8; 188usize],
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
    _reserved26: [u8; 1000usize],
    #[doc = "0x900 - Receive Time Port 0"]
    pub dc_rcv_time_port0: DC_RCV_TIME_PORT0,
    #[doc = "0x904 - Receive Time Port 1"]
    pub dc_rcv_time_port1: DC_RCV_TIME_PORT1,
    _reserved27: [u8; 8usize],
    #[doc = "0x910 - System Time read access"]
    pub dc_sys_time: [DC_SYS_TIME; 2],
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
    _reserved28: [u8; 74usize],
    #[doc = "0x980 - Cyclic Unit Control"]
    pub dc_cyc_cont: DC_CYC_CONT,
    #[doc = "0x981 - Activation register"]
    pub dc_act: DC_ACT,
    #[doc = "0x982 - Pulse Length of SyncSignals"]
    pub dc_pulse_len: DC_PULSE_LEN,
    #[doc = "0x984 - Activation Status"]
    pub dc_act_stat: DC_ACT_STAT,
    _reserved29: [u8; 9usize],
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
    _reserved30: [u8; 4usize],
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
    _reserved31: [u8; 32usize],
    #[doc = "0x9f0 - EtherCAT Buffer Change Event Time"]
    pub dc_ecat_cng_ev_time: DC_ECAT_CNG_EV_TIME,
    _reserved32: [u8; 4usize],
    #[doc = "0x9f8 - PDI Buffer Start Event Time"]
    pub dc_pdi_start_ev_time: DC_PDI_START_EV_TIME,
    #[doc = "0x9fc - PDI Buffer Change Event Time"]
    pub dc_pdi_cng_ev_time: DC_PDI_CNG_EV_TIME,
    _reserved33: [u8; 1024usize],
    #[doc = "0xe00 - ECAT0 Module ID"]
    pub id: ID,
    _reserved34: [u8; 4usize],
    #[doc = "0xe08 - ECAT0 Status"]
    pub status: STATUS,
}
#[doc = "Type of EtherCAT Controller"]
pub struct TYPE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Type of EtherCAT Controller"]
pub mod type_;
#[doc = "Revision of EtherCAT Controller"]
pub struct REVISION {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Revision of EtherCAT Controller"]
pub mod revision;
#[doc = "Build Version"]
pub struct BUILD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Build Version"]
pub mod build;
#[doc = "FMMUs Supported"]
pub struct FMMU_NUM {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "FMMUs Supported"]
pub mod fmmu_num;
#[doc = "SyncManagers Supported"]
pub struct SYNC_MANAGER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SyncManagers Supported"]
pub mod sync_manager;
#[doc = "RAM Size"]
pub struct RAM_SIZE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RAM Size"]
pub mod ram_size;
#[doc = "Port Descriptor"]
pub struct PORT_DESC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port Descriptor"]
pub mod port_desc;
#[doc = "ESC Features Supported"]
pub struct FEATURE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ESC Features Supported"]
pub mod feature;
#[doc = "Configured Station Address"]
pub struct STATION_ADR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Configured Station Address"]
pub mod station_adr;
#[doc = "Configured Station Alias"]
pub struct STATION_ALIAS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Configured Station Alias"]
pub mod station_alias;
#[doc = "Write Register Enable"]
pub struct WR_REG_ENABLE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Write Register Enable"]
pub mod wr_reg_enable;
#[doc = "Write Register Protection"]
pub struct WR_REG_PROTECT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Write Register Protection"]
pub mod wr_reg_protect;
#[doc = "ESC Write Enable"]
pub struct ESC_WR_ENABLE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Write Enable"]
pub mod esc_wr_enable;
#[doc = "ESC Write Protection"]
pub struct ESC_WR_PROTECT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Write Protection"]
pub mod esc_wr_protect;
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]"]
pub struct ESC_RESET_ECAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]"]
pub mod esc_reset_ecat;
#[doc = "ESC Reset ECAT \\[READ Mode\\]"]
pub struct ESC_RESET_ECAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Reset ECAT \\[READ Mode\\]"]
pub mod esc_reset_ecat;
#[doc = "ESC Reset PDI \\[WRITE Mode\\]"]
pub struct ESC_RESET_PDI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Reset PDI \\[WRITE Mode\\]"]
pub mod esc_reset_pdi;
#[doc = "ESC Reset PDI \\[READ Mode\\]"]
pub struct ESC_RESET_PDI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Reset PDI \\[READ Mode\\]"]
pub mod esc_reset_pdi;
#[doc = "ESC DL Control"]
pub struct ESC_DL_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ESC DL Control"]
pub mod esc_dl_control;
#[doc = "Physical Read/Write Offset"]
pub struct PHYSICAL_RW_OFFSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Physical Read/Write Offset"]
pub mod physical_rw_offset;
#[doc = "ESC DL Status"]
pub struct ESC_DL_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ESC DL Status"]
pub mod esc_dl_status;
#[doc = "AL Control"]
pub struct AL_CONTROL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "AL Control"]
pub mod al_control;
#[doc = "AL Status"]
pub struct AL_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "AL Status"]
pub mod al_status;
#[doc = "AL Status Code"]
pub struct AL_STATUS_CODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "AL Status Code"]
pub mod al_status_code;
#[doc = "RUN LED Override"]
pub struct RUN_LED {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RUN LED Override"]
pub mod run_led;
#[doc = "RUN ERR Override"]
pub struct ERR_LED {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RUN ERR Override"]
pub mod err_led;
#[doc = "PDI Control"]
pub struct PDI_CONTROL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "PDI Control"]
pub mod pdi_control;
#[doc = "ESC Configuration"]
pub struct ESC_CONFIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ESC Configuration"]
pub mod esc_config;
#[doc = "PDI Control"]
pub struct PDI_CONFIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "PDI Control"]
pub mod pdi_config;
#[doc = "Sync/Latch\\[1:0\\] PDI Configuration"]
pub struct SYNC_LATCH_CONFIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sync/Latch\\[1:0\\] PDI Configuration"]
pub mod sync_latch_config;
#[doc = "PDI Synchronous Microcontroller extended Configuration"]
pub struct PDI_EXT_CONFIG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PDI Synchronous Microcontroller extended Configuration"]
pub mod pdi_ext_config;
#[doc = "ECAT Event Mask"]
pub struct EVENT_MASK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ECAT Event Mask"]
pub mod event_mask;
#[doc = "PDI AL Event Mask"]
pub struct AL_EVENT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDI AL Event Mask"]
pub mod al_event_mask;
#[doc = "ECAT Event Request"]
pub struct EVENT_REQ {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ECAT Event Request"]
pub mod event_req;
#[doc = "AL Event Request"]
pub struct AL_EVENT_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AL Event Request"]
pub mod al_event_req;
#[doc = "RX Error Counter Port 0"]
pub struct RX_ERR_COUNT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RX Error Counter Port 0"]
pub mod rx_err_count0;
#[doc = "RX Error Counter Port 1"]
pub struct RX_ERR_COUNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RX Error Counter Port 1"]
pub mod rx_err_count1;
#[doc = "Forwarded RX Error Counter Port 0"]
pub struct FWD_RX_ERR_COUNT0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Forwarded RX Error Counter Port 0"]
pub mod fwd_rx_err_count0;
#[doc = "Forwarded RX Error Counter Port 1"]
pub struct FWD_RX_ERR_COUNT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Forwarded RX Error Counter Port 1"]
pub mod fwd_rx_err_count1;
#[doc = "ECAT Processing Unit Error Counter"]
pub struct PROC_ERR_COUNT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "ECAT Processing Unit Error Counter"]
pub mod proc_err_count;
#[doc = "PDI Error Counter"]
pub struct PDI_ERR_COUNT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "PDI Error Counter"]
pub mod pdi_err_count;
#[doc = "Lost Link Counter Port 0"]
pub struct LOST_LINK_COUNT0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Lost Link Counter Port 0"]
pub mod lost_link_count0;
#[doc = "Lost Link Counter Port 1"]
pub struct LOST_LINK_COUNT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Lost Link Counter Port 1"]
pub mod lost_link_count1;
#[doc = "Watchdog Divider"]
pub struct WD_DIVIDE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Divider"]
pub mod wd_divide;
#[doc = "Watchdog Time PDI"]
pub struct WD_TIME_PDI {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Time PDI"]
pub mod wd_time_pdi;
#[doc = "Watchdog Time Process Data"]
pub struct WD_TIME_PDATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Time Process Data"]
pub mod wd_time_pdata;
#[doc = "Watchdog Status Process Data"]
pub struct WD_STAT_PDATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Status Process Data"]
pub mod wd_stat_pdata;
#[doc = "Watchdog Counter Process Data"]
pub struct WD_COUNT_PDATA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Watchdog Counter Process Data"]
pub mod wd_count_pdata;
#[doc = "Watchdog Counter PDI"]
pub struct WD_COUNT_PDI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Watchdog Counter PDI"]
pub mod wd_count_pdi;
#[doc = "EEPROM Configuration"]
pub struct EEP_CONF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "EEPROM Configuration"]
pub mod eep_conf;
#[doc = "EEPROM PDI Access State"]
pub struct EEP_STATE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "EEPROM PDI Access State"]
pub mod eep_state;
#[doc = "EEPROM Control/Status"]
pub struct EEP_CONT_STAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "EEPROM Control/Status"]
pub mod eep_cont_stat;
#[doc = "EEPROM Address"]
pub struct EEP_ADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Address"]
pub mod eep_adr;
#[doc = "EEPROM Read/Write data"]
pub struct EEP_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Read/Write data"]
pub mod eep_data;
#[doc = "MII Management Control/Status"]
pub struct MII_CONT_STAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MII Management Control/Status"]
pub mod mii_cont_stat;
#[doc = "PHY Address"]
pub struct MII_PHY_ADR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "PHY Address"]
pub mod mii_phy_adr;
#[doc = "PHY Register Address"]
pub struct MII_PHY_REG_ADR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "PHY Register Address"]
pub mod mii_phy_reg_adr;
#[doc = "PHY Data"]
pub struct MII_PHY_DATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PHY Data"]
pub mod mii_phy_data;
#[doc = "MII ECAT ACS STATE"]
pub struct MII_ECAT_ACS_STATE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MII ECAT ACS STATE"]
pub mod mii_ecat_acs_state;
#[doc = "MII PDI ACS STATE"]
pub struct MII_PDI_ACS_STATE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MII PDI ACS STATE"]
pub mod mii_pdi_acs_state;
#[doc = "Receive Time Port 0"]
pub struct DC_RCV_TIME_PORT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Time Port 0"]
pub mod dc_rcv_time_port0;
#[doc = "Receive Time Port 1"]
pub struct DC_RCV_TIME_PORT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Time Port 1"]
pub mod dc_rcv_time_port1;
#[doc = "Local time of the beginning of a frame"]
pub struct RECEIVE_TIME_PU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local time of the beginning of a frame"]
pub mod receive_time_pu;
#[doc = "System Time read access"]
pub struct DC_SYS_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time read access"]
pub mod dc_sys_time;
#[doc = "System Time \\[WRITE Mode\\]"]
pub struct DC_SYS_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time \\[WRITE Mode\\]"]
pub mod dc_sys_time;
#[doc = "Difference between local time and System Time"]
pub struct DC_SYS_TIME_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Difference between local time and System Time"]
pub mod dc_sys_time_offset;
#[doc = "System Time Delay"]
pub struct DC_SYS_TIME_DELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time Delay"]
pub mod dc_sys_time_delay;
#[doc = "System Time Difference"]
pub struct DC_SYS_TIME_DIFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time Difference"]
pub mod dc_sys_time_diff;
#[doc = "Speed Counter Start"]
pub struct DC_SPEED_COUNT_START {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Speed Counter Start"]
pub mod dc_speed_count_start;
#[doc = "Speed Counter Diff"]
pub struct DC_SPEED_COUNT_DIFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Speed Counter Diff"]
pub mod dc_speed_count_diff;
#[doc = "System Time Difference Filter Depth"]
pub struct DC_SYS_TIME_FIL_DEPTH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "System Time Difference Filter Depth"]
pub mod dc_sys_time_fil_depth;
#[doc = "Speed Counter Filter Depth"]
pub struct DC_SPEED_COUNT_FIL_DEPTH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Speed Counter Filter Depth"]
pub mod dc_speed_count_fil_depth;
#[doc = "Cyclic Unit Control"]
pub struct DC_CYC_CONT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Cyclic Unit Control"]
pub mod dc_cyc_cont;
#[doc = "Activation register"]
pub struct DC_ACT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Activation register"]
pub mod dc_act;
#[doc = "Pulse Length of SyncSignals"]
pub struct DC_PULSE_LEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Pulse Length of SyncSignals"]
pub mod dc_pulse_len;
#[doc = "Activation Status"]
pub struct DC_ACT_STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Activation Status"]
pub mod dc_act_stat;
#[doc = "SYNC0 Status"]
pub struct DC_SYNC0_STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SYNC0 Status"]
pub mod dc_sync0_stat;
#[doc = "SYNC1 Status"]
pub struct DC_SYNC1_STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SYNC1 Status"]
pub mod dc_sync1_stat;
#[doc = "Start Time Cyclic Operation"]
pub struct DC_CYC_START_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Time Cyclic Operation"]
pub mod dc_cyc_start_time;
#[doc = "System time of next SYNC1 pulse in ns"]
pub struct DC_NEXT_SYNC1_PULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System time of next SYNC1 pulse in ns"]
pub mod dc_next_sync1_pulse;
#[doc = "SYNC0 Cycle Time"]
pub struct DC_SYNC0_CYC_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYNC0 Cycle Time"]
pub mod dc_sync0_cyc_time;
#[doc = "SYNC1 Cycle Time"]
pub struct DC_SYNC1_CYC_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYNC1 Cycle Time"]
pub mod dc_sync1_cyc_time;
#[doc = "Latch0 Control"]
pub struct DC_LATCH0_CONT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Latch0 Control"]
pub mod dc_latch0_cont;
#[doc = "Latch1 Control"]
pub struct DC_LATCH1_CONT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Latch1 Control"]
pub mod dc_latch1_cont;
#[doc = "Latch0 Status"]
pub struct DC_LATCH0_STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Latch0 Status"]
pub mod dc_latch0_stat;
#[doc = "Latch1 Status"]
pub struct DC_LATCH1_STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Latch1 Status"]
pub mod dc_latch1_stat;
#[doc = "Register captures System time at the positive edge of the Latch0 signal"]
pub struct DC_LATCH0_TIME_POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register captures System time at the positive edge of the Latch0 signal"]
pub mod dc_latch0_time_pos;
#[doc = "Register captures System time at the negative edge of the Latch0 signal"]
pub struct DC_LATCH0_TIME_NEG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register captures System time at the negative edge of the Latch0 signal"]
pub mod dc_latch0_time_neg;
#[doc = "Register captures System time at the positive edge of the Latch1 signal"]
pub struct DC_LATCH1_TIME_POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register captures System time at the positive edge of the Latch1 signal"]
pub mod dc_latch1_time_pos;
#[doc = "Register captures System time at the negative edge of the Latch1 signal"]
pub struct DC_LATCH1_TIME_NEG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register captures System time at the negative edge of the Latch1 signal"]
pub mod dc_latch1_time_neg;
#[doc = "EtherCAT Buffer Change Event Time"]
pub struct DC_ECAT_CNG_EV_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EtherCAT Buffer Change Event Time"]
pub mod dc_ecat_cng_ev_time;
#[doc = "PDI Buffer Start Event Time"]
pub struct DC_PDI_START_EV_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDI Buffer Start Event Time"]
pub mod dc_pdi_start_ev_time;
#[doc = "PDI Buffer Change Event Time"]
pub struct DC_PDI_CNG_EV_TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDI Buffer Change Event Time"]
pub mod dc_pdi_cng_ev_time;
#[doc = "ECAT0 Module ID"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECAT0 Module ID"]
pub mod id;
#[doc = "ECAT0 Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECAT0 Status"]
pub mod status;
