#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    type_: TYPE,
    revision: REVISION,
    build: BUILD,
    fmmu_num: FMMU_NUM,
    sync_manager: SYNC_MANAGER,
    ram_size: RAM_SIZE,
    port_desc: PORT_DESC,
    feature: FEATURE,
    _reserved8: [u8; 0x06],
    station_adr: STATION_ADR,
    station_alias: STATION_ALIAS,
    _reserved10: [u8; 0x0c],
    wr_reg_enable: WR_REG_ENABLE,
    wr_reg_protect: WR_REG_PROTECT,
    _reserved12: [u8; 0x0e],
    esc_wr_enable: ESC_WR_ENABLE,
    esc_wr_protect: ESC_WR_PROTECT,
    _reserved14: [u8; 0x0e],
    _reserved_14_readmode_esc_reset_ecat: [u8; 0x01],
    _reserved_15_readmode_esc_reset_pdi: [u8; 0x01],
    _reserved16: [u8; 0xbe],
    esc_dl_control: ESC_DL_CONTROL,
    _reserved17: [u8; 0x04],
    physical_rw_offset: PHYSICAL_RW_OFFSET,
    _reserved18: [u8; 0x06],
    esc_dl_status: ESC_DL_STATUS,
    _reserved19: [u8; 0x0e],
    al_control: AL_CONTROL,
    _reserved20: [u8; 0x0e],
    al_status: AL_STATUS,
    _reserved21: [u8; 0x02],
    al_status_code: AL_STATUS_CODE,
    _reserved22: [u8; 0x02],
    run_led: RUN_LED,
    err_led: ERR_LED,
    _reserved24: [u8; 0x06],
    pdi_control: PDI_CONTROL,
    esc_config: ESC_CONFIG,
    _reserved26: [u8; 0x0e],
    pdi_config: PDI_CONFIG,
    sync_latch_config: SYNC_LATCH_CONFIG,
    pdi_ext_config: PDI_EXT_CONFIG,
    _reserved29: [u8; 0xac],
    event_mask: EVENT_MASK,
    _reserved30: [u8; 0x02],
    al_event_mask: AL_EVENT_MASK,
    _reserved31: [u8; 0x08],
    event_req: EVENT_REQ,
    _reserved32: [u8; 0x0e],
    al_event_req: AL_EVENT_REQ,
    _reserved33: [u8; 0xdc],
    rx_err_count0: RX_ERR_COUNT0,
    rx_err_count1: RX_ERR_COUNT1,
    _reserved35: [u8; 0x04],
    fwd_rx_err_count0: FWD_RX_ERR_COUNT0,
    fwd_rx_err_count1: FWD_RX_ERR_COUNT1,
    _reserved37: [u8; 0x02],
    proc_err_count: PROC_ERR_COUNT,
    pdi_err_count: PDI_ERR_COUNT,
    _reserved39: [u8; 0x02],
    lost_link_count0: LOST_LINK_COUNT0,
    lost_link_count1: LOST_LINK_COUNT1,
    _reserved41: [u8; 0xee],
    wd_divide: WD_DIVIDE,
    _reserved42: [u8; 0x0e],
    wd_time_pdi: WD_TIME_PDI,
    _reserved43: [u8; 0x0e],
    wd_time_pdata: WD_TIME_PDATA,
    _reserved44: [u8; 0x1e],
    wd_stat_pdata: WD_STAT_PDATA,
    wd_count_pdata: WD_COUNT_PDATA,
    wd_count_pdi: WD_COUNT_PDI,
    _reserved47: [u8; 0xbc],
    eep_conf: EEP_CONF,
    eep_state: EEP_STATE,
    eep_cont_stat: EEP_CONT_STAT,
    eep_adr: EEP_ADR,
    eep_data: [EEP_DATA; 2],
    mii_cont_stat: MII_CONT_STAT,
    mii_phy_adr: MII_PHY_ADR,
    mii_phy_reg_adr: MII_PHY_REG_ADR,
    mii_phy_data: MII_PHY_DATA,
    mii_ecat_acs_state: MII_ECAT_ACS_STATE,
    mii_pdi_acs_state: MII_PDI_ACS_STATE,
    _reserved58: [u8; 0x03e8],
    dc_rcv_time_port0: DC_RCV_TIME_PORT0,
    dc_rcv_time_port1: DC_RCV_TIME_PORT1,
    _reserved60: [u8; 0x08],
    _reserved_60_readmode_dc_sys_time: [u8; 0x08],
    receive_time_pu: [RECEIVE_TIME_PU; 2],
    dc_sys_time_offset: [DC_SYS_TIME_OFFSET; 2],
    dc_sys_time_delay: DC_SYS_TIME_DELAY,
    dc_sys_time_diff: DC_SYS_TIME_DIFF,
    dc_speed_count_start: DC_SPEED_COUNT_START,
    dc_speed_count_diff: DC_SPEED_COUNT_DIFF,
    dc_sys_time_fil_depth: DC_SYS_TIME_FIL_DEPTH,
    dc_speed_count_fil_depth: DC_SPEED_COUNT_FIL_DEPTH,
    _reserved69: [u8; 0x4a],
    dc_cyc_cont: DC_CYC_CONT,
    dc_act: DC_ACT,
    dc_pulse_len: DC_PULSE_LEN,
    dc_act_stat: DC_ACT_STAT,
    _reserved73: [u8; 0x09],
    dc_sync0_stat: DC_SYNC0_STAT,
    dc_sync1_stat: DC_SYNC1_STAT,
    dc_cyc_start_time: [DC_CYC_START_TIME; 2],
    dc_next_sync1_pulse: [DC_NEXT_SYNC1_PULSE; 2],
    dc_sync0_cyc_time: DC_SYNC0_CYC_TIME,
    dc_sync1_cyc_time: DC_SYNC1_CYC_TIME,
    dc_latch0_cont: DC_LATCH0_CONT,
    dc_latch1_cont: DC_LATCH1_CONT,
    _reserved81: [u8; 0x04],
    dc_latch0_stat: DC_LATCH0_STAT,
    dc_latch1_stat: DC_LATCH1_STAT,
    dc_latch0_time_pos: [DC_LATCH0_TIME_POS; 2],
    dc_latch0_time_neg: [DC_LATCH0_TIME_NEG; 2],
    dc_latch1_time_pos: [DC_LATCH1_TIME_POS; 2],
    dc_latch1_time_neg: [DC_LATCH1_TIME_NEG; 2],
    _reserved87: [u8; 0x20],
    dc_ecat_cng_ev_time: DC_ECAT_CNG_EV_TIME,
    _reserved88: [u8; 0x04],
    dc_pdi_start_ev_time: DC_PDI_START_EV_TIME,
    dc_pdi_cng_ev_time: DC_PDI_CNG_EV_TIME,
    _reserved90: [u8; 0x0400],
    id: ID,
    _reserved91: [u8; 0x04],
    status: STATUS,
}
impl RegisterBlock {
    #[doc = "0x00 - Type of EtherCAT Controller"]
    #[inline(always)]
    pub const fn type_(&self) -> &TYPE {
        &self.type_
    }
    #[doc = "0x01 - Revision of EtherCAT Controller"]
    #[inline(always)]
    pub const fn revision(&self) -> &REVISION {
        &self.revision
    }
    #[doc = "0x02 - Build Version"]
    #[inline(always)]
    pub const fn build(&self) -> &BUILD {
        &self.build
    }
    #[doc = "0x04 - FMMUs Supported"]
    #[inline(always)]
    pub const fn fmmu_num(&self) -> &FMMU_NUM {
        &self.fmmu_num
    }
    #[doc = "0x05 - SyncManagers Supported"]
    #[inline(always)]
    pub const fn sync_manager(&self) -> &SYNC_MANAGER {
        &self.sync_manager
    }
    #[doc = "0x06 - RAM Size"]
    #[inline(always)]
    pub const fn ram_size(&self) -> &RAM_SIZE {
        &self.ram_size
    }
    #[doc = "0x07 - Port Descriptor"]
    #[inline(always)]
    pub const fn port_desc(&self) -> &PORT_DESC {
        &self.port_desc
    }
    #[doc = "0x08 - ESC Features Supported"]
    #[inline(always)]
    pub const fn feature(&self) -> &FEATURE {
        &self.feature
    }
    #[doc = "0x10 - Configured Station Address"]
    #[inline(always)]
    pub const fn station_adr(&self) -> &STATION_ADR {
        &self.station_adr
    }
    #[doc = "0x12 - Configured Station Alias"]
    #[inline(always)]
    pub const fn station_alias(&self) -> &STATION_ALIAS {
        &self.station_alias
    }
    #[doc = "0x20 - Write Register Enable"]
    #[inline(always)]
    pub const fn wr_reg_enable(&self) -> &WR_REG_ENABLE {
        &self.wr_reg_enable
    }
    #[doc = "0x21 - Write Register Protection"]
    #[inline(always)]
    pub const fn wr_reg_protect(&self) -> &WR_REG_PROTECT {
        &self.wr_reg_protect
    }
    #[doc = "0x30 - ESC Write Enable"]
    #[inline(always)]
    pub const fn esc_wr_enable(&self) -> &ESC_WR_ENABLE {
        &self.esc_wr_enable
    }
    #[doc = "0x31 - ESC Write Protection"]
    #[inline(always)]
    pub const fn esc_wr_protect(&self) -> &ESC_WR_PROTECT {
        &self.esc_wr_protect
    }
    #[doc = "0x40 - ESC Reset ECAT \\[READ Mode\\]"]
    #[inline(always)]
    pub const fn readmode_esc_reset_ecat(&self) -> &READMODE_ESC_RESET_ECAT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - ESC Reset ECAT \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_esc_reset_ecat(&self) -> &WRITEMODE_ESC_RESET_ECAT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x41 - ESC Reset PDI \\[READ Mode\\]"]
    #[inline(always)]
    pub const fn readmode_esc_reset_pdi(&self) -> &READMODE_ESC_RESET_PDI {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x41 - ESC Reset PDI \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_esc_reset_pdi(&self) -> &WRITEMODE_ESC_RESET_PDI {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x100 - ESC DL Control"]
    #[inline(always)]
    pub const fn esc_dl_control(&self) -> &ESC_DL_CONTROL {
        &self.esc_dl_control
    }
    #[doc = "0x108 - Physical Read/Write Offset"]
    #[inline(always)]
    pub const fn physical_rw_offset(&self) -> &PHYSICAL_RW_OFFSET {
        &self.physical_rw_offset
    }
    #[doc = "0x110 - ESC DL Status"]
    #[inline(always)]
    pub const fn esc_dl_status(&self) -> &ESC_DL_STATUS {
        &self.esc_dl_status
    }
    #[doc = "0x120 - AL Control"]
    #[inline(always)]
    pub const fn al_control(&self) -> &AL_CONTROL {
        &self.al_control
    }
    #[doc = "0x130 - AL Status"]
    #[inline(always)]
    pub const fn al_status(&self) -> &AL_STATUS {
        &self.al_status
    }
    #[doc = "0x134 - AL Status Code"]
    #[inline(always)]
    pub const fn al_status_code(&self) -> &AL_STATUS_CODE {
        &self.al_status_code
    }
    #[doc = "0x138 - RUN LED Override"]
    #[inline(always)]
    pub const fn run_led(&self) -> &RUN_LED {
        &self.run_led
    }
    #[doc = "0x139 - RUN ERR Override"]
    #[inline(always)]
    pub const fn err_led(&self) -> &ERR_LED {
        &self.err_led
    }
    #[doc = "0x140 - PDI Control"]
    #[inline(always)]
    pub const fn pdi_control(&self) -> &PDI_CONTROL {
        &self.pdi_control
    }
    #[doc = "0x141 - ESC Configuration"]
    #[inline(always)]
    pub const fn esc_config(&self) -> &ESC_CONFIG {
        &self.esc_config
    }
    #[doc = "0x150 - PDI Control"]
    #[inline(always)]
    pub const fn pdi_config(&self) -> &PDI_CONFIG {
        &self.pdi_config
    }
    #[doc = "0x151 - Sync/Latch\\[1:0\\]
PDI Configuration"]
    #[inline(always)]
    pub const fn sync_latch_config(&self) -> &SYNC_LATCH_CONFIG {
        &self.sync_latch_config
    }
    #[doc = "0x152 - PDI Synchronous Microcontroller extended Configuration"]
    #[inline(always)]
    pub const fn pdi_ext_config(&self) -> &PDI_EXT_CONFIG {
        &self.pdi_ext_config
    }
    #[doc = "0x200 - ECAT Event Mask"]
    #[inline(always)]
    pub const fn event_mask(&self) -> &EVENT_MASK {
        &self.event_mask
    }
    #[doc = "0x204 - PDI AL Event Mask"]
    #[inline(always)]
    pub const fn al_event_mask(&self) -> &AL_EVENT_MASK {
        &self.al_event_mask
    }
    #[doc = "0x210 - ECAT Event Request"]
    #[inline(always)]
    pub const fn event_req(&self) -> &EVENT_REQ {
        &self.event_req
    }
    #[doc = "0x220 - AL Event Request"]
    #[inline(always)]
    pub const fn al_event_req(&self) -> &AL_EVENT_REQ {
        &self.al_event_req
    }
    #[doc = "0x300 - RX Error Counter Port 0"]
    #[inline(always)]
    pub const fn rx_err_count0(&self) -> &RX_ERR_COUNT0 {
        &self.rx_err_count0
    }
    #[doc = "0x302 - RX Error Counter Port 1"]
    #[inline(always)]
    pub const fn rx_err_count1(&self) -> &RX_ERR_COUNT1 {
        &self.rx_err_count1
    }
    #[doc = "0x308 - Forwarded RX Error Counter Port 0"]
    #[inline(always)]
    pub const fn fwd_rx_err_count0(&self) -> &FWD_RX_ERR_COUNT0 {
        &self.fwd_rx_err_count0
    }
    #[doc = "0x309 - Forwarded RX Error Counter Port 1"]
    #[inline(always)]
    pub const fn fwd_rx_err_count1(&self) -> &FWD_RX_ERR_COUNT1 {
        &self.fwd_rx_err_count1
    }
    #[doc = "0x30c - ECAT Processing Unit Error Counter"]
    #[inline(always)]
    pub const fn proc_err_count(&self) -> &PROC_ERR_COUNT {
        &self.proc_err_count
    }
    #[doc = "0x30d - PDI Error Counter"]
    #[inline(always)]
    pub const fn pdi_err_count(&self) -> &PDI_ERR_COUNT {
        &self.pdi_err_count
    }
    #[doc = "0x310 - Lost Link Counter Port 0"]
    #[inline(always)]
    pub const fn lost_link_count0(&self) -> &LOST_LINK_COUNT0 {
        &self.lost_link_count0
    }
    #[doc = "0x311 - Lost Link Counter Port 1"]
    #[inline(always)]
    pub const fn lost_link_count1(&self) -> &LOST_LINK_COUNT1 {
        &self.lost_link_count1
    }
    #[doc = "0x400 - Watchdog Divider"]
    #[inline(always)]
    pub const fn wd_divide(&self) -> &WD_DIVIDE {
        &self.wd_divide
    }
    #[doc = "0x410 - Watchdog Time PDI"]
    #[inline(always)]
    pub const fn wd_time_pdi(&self) -> &WD_TIME_PDI {
        &self.wd_time_pdi
    }
    #[doc = "0x420 - Watchdog Time Process Data"]
    #[inline(always)]
    pub const fn wd_time_pdata(&self) -> &WD_TIME_PDATA {
        &self.wd_time_pdata
    }
    #[doc = "0x440 - Watchdog Status Process Data"]
    #[inline(always)]
    pub const fn wd_stat_pdata(&self) -> &WD_STAT_PDATA {
        &self.wd_stat_pdata
    }
    #[doc = "0x442 - Watchdog Counter Process Data"]
    #[inline(always)]
    pub const fn wd_count_pdata(&self) -> &WD_COUNT_PDATA {
        &self.wd_count_pdata
    }
    #[doc = "0x443 - Watchdog Counter PDI"]
    #[inline(always)]
    pub const fn wd_count_pdi(&self) -> &WD_COUNT_PDI {
        &self.wd_count_pdi
    }
    #[doc = "0x500 - EEPROM Configuration"]
    #[inline(always)]
    pub const fn eep_conf(&self) -> &EEP_CONF {
        &self.eep_conf
    }
    #[doc = "0x501 - EEPROM PDI Access State"]
    #[inline(always)]
    pub const fn eep_state(&self) -> &EEP_STATE {
        &self.eep_state
    }
    #[doc = "0x502 - EEPROM Control/Status"]
    #[inline(always)]
    pub const fn eep_cont_stat(&self) -> &EEP_CONT_STAT {
        &self.eep_cont_stat
    }
    #[doc = "0x504 - EEPROM Address"]
    #[inline(always)]
    pub const fn eep_adr(&self) -> &EEP_ADR {
        &self.eep_adr
    }
    #[doc = "0x508..0x510 - EEPROM Read/Write data"]
    #[inline(always)]
    pub const fn eep_data(&self, n: usize) -> &EEP_DATA {
        &self.eep_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x508..0x510 - EEPROM Read/Write data"]
    #[inline(always)]
    pub fn eep_data_iter(&self) -> impl Iterator<Item = &EEP_DATA> {
        self.eep_data.iter()
    }
    #[doc = "0x510 - MII Management Control/Status"]
    #[inline(always)]
    pub const fn mii_cont_stat(&self) -> &MII_CONT_STAT {
        &self.mii_cont_stat
    }
    #[doc = "0x512 - PHY Address"]
    #[inline(always)]
    pub const fn mii_phy_adr(&self) -> &MII_PHY_ADR {
        &self.mii_phy_adr
    }
    #[doc = "0x513 - PHY Register Address"]
    #[inline(always)]
    pub const fn mii_phy_reg_adr(&self) -> &MII_PHY_REG_ADR {
        &self.mii_phy_reg_adr
    }
    #[doc = "0x514 - PHY Data"]
    #[inline(always)]
    pub const fn mii_phy_data(&self) -> &MII_PHY_DATA {
        &self.mii_phy_data
    }
    #[doc = "0x516 - MII ECAT ACS STATE"]
    #[inline(always)]
    pub const fn mii_ecat_acs_state(&self) -> &MII_ECAT_ACS_STATE {
        &self.mii_ecat_acs_state
    }
    #[doc = "0x517 - MII PDI ACS STATE"]
    #[inline(always)]
    pub const fn mii_pdi_acs_state(&self) -> &MII_PDI_ACS_STATE {
        &self.mii_pdi_acs_state
    }
    #[doc = "0x900 - Receive Time Port 0"]
    #[inline(always)]
    pub const fn dc_rcv_time_port0(&self) -> &DC_RCV_TIME_PORT0 {
        &self.dc_rcv_time_port0
    }
    #[doc = "0x904 - Receive Time Port 1"]
    #[inline(always)]
    pub const fn dc_rcv_time_port1(&self) -> &DC_RCV_TIME_PORT1 {
        &self.dc_rcv_time_port1
    }
    #[doc = "0x910 - System Time \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_dc_sys_time(&self) -> &WRITEMODE_DC_SYS_TIME {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2320).cast() }
    }
    #[doc = "0x910..0x918 - System Time read access"]
    #[inline(always)]
    pub const fn readmode_dc_sys_time(&self, n: usize) -> &READMODE_DC_SYS_TIME {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2320).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x910..0x918 - System Time read access"]
    #[inline(always)]
    pub fn readmode_dc_sys_time_iter(&self) -> impl Iterator<Item = &READMODE_DC_SYS_TIME> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2320).add(4 * n).cast() })
    }
    #[doc = "0x918..0x920 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub const fn receive_time_pu(&self, n: usize) -> &RECEIVE_TIME_PU {
        &self.receive_time_pu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x918..0x920 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn receive_time_pu_iter(&self) -> impl Iterator<Item = &RECEIVE_TIME_PU> {
        self.receive_time_pu.iter()
    }
    #[doc = "0x920..0x928 - Difference between local time and System Time"]
    #[inline(always)]
    pub const fn dc_sys_time_offset(&self, n: usize) -> &DC_SYS_TIME_OFFSET {
        &self.dc_sys_time_offset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x920..0x928 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset_iter(&self) -> impl Iterator<Item = &DC_SYS_TIME_OFFSET> {
        self.dc_sys_time_offset.iter()
    }
    #[doc = "0x928 - System Time Delay"]
    #[inline(always)]
    pub const fn dc_sys_time_delay(&self) -> &DC_SYS_TIME_DELAY {
        &self.dc_sys_time_delay
    }
    #[doc = "0x92c - System Time Difference"]
    #[inline(always)]
    pub const fn dc_sys_time_diff(&self) -> &DC_SYS_TIME_DIFF {
        &self.dc_sys_time_diff
    }
    #[doc = "0x930 - Speed Counter Start"]
    #[inline(always)]
    pub const fn dc_speed_count_start(&self) -> &DC_SPEED_COUNT_START {
        &self.dc_speed_count_start
    }
    #[doc = "0x932 - Speed Counter Diff"]
    #[inline(always)]
    pub const fn dc_speed_count_diff(&self) -> &DC_SPEED_COUNT_DIFF {
        &self.dc_speed_count_diff
    }
    #[doc = "0x934 - System Time Difference Filter Depth"]
    #[inline(always)]
    pub const fn dc_sys_time_fil_depth(&self) -> &DC_SYS_TIME_FIL_DEPTH {
        &self.dc_sys_time_fil_depth
    }
    #[doc = "0x935 - Speed Counter Filter Depth"]
    #[inline(always)]
    pub const fn dc_speed_count_fil_depth(&self) -> &DC_SPEED_COUNT_FIL_DEPTH {
        &self.dc_speed_count_fil_depth
    }
    #[doc = "0x980 - Cyclic Unit Control"]
    #[inline(always)]
    pub const fn dc_cyc_cont(&self) -> &DC_CYC_CONT {
        &self.dc_cyc_cont
    }
    #[doc = "0x981 - Activation register"]
    #[inline(always)]
    pub const fn dc_act(&self) -> &DC_ACT {
        &self.dc_act
    }
    #[doc = "0x982 - Pulse Length of SyncSignals"]
    #[inline(always)]
    pub const fn dc_pulse_len(&self) -> &DC_PULSE_LEN {
        &self.dc_pulse_len
    }
    #[doc = "0x984 - Activation Status"]
    #[inline(always)]
    pub const fn dc_act_stat(&self) -> &DC_ACT_STAT {
        &self.dc_act_stat
    }
    #[doc = "0x98e - SYNC0 Status"]
    #[inline(always)]
    pub const fn dc_sync0_stat(&self) -> &DC_SYNC0_STAT {
        &self.dc_sync0_stat
    }
    #[doc = "0x98f - SYNC1 Status"]
    #[inline(always)]
    pub const fn dc_sync1_stat(&self) -> &DC_SYNC1_STAT {
        &self.dc_sync1_stat
    }
    #[doc = "0x990..0x998 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub const fn dc_cyc_start_time(&self, n: usize) -> &DC_CYC_START_TIME {
        &self.dc_cyc_start_time[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x990..0x998 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time_iter(&self) -> impl Iterator<Item = &DC_CYC_START_TIME> {
        self.dc_cyc_start_time.iter()
    }
    #[doc = "0x998..0x9a0 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub const fn dc_next_sync1_pulse(&self, n: usize) -> &DC_NEXT_SYNC1_PULSE {
        &self.dc_next_sync1_pulse[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x998..0x9a0 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub fn dc_next_sync1_pulse_iter(&self) -> impl Iterator<Item = &DC_NEXT_SYNC1_PULSE> {
        self.dc_next_sync1_pulse.iter()
    }
    #[doc = "0x9a0 - SYNC0 Cycle Time"]
    #[inline(always)]
    pub const fn dc_sync0_cyc_time(&self) -> &DC_SYNC0_CYC_TIME {
        &self.dc_sync0_cyc_time
    }
    #[doc = "0x9a4 - SYNC1 Cycle Time"]
    #[inline(always)]
    pub const fn dc_sync1_cyc_time(&self) -> &DC_SYNC1_CYC_TIME {
        &self.dc_sync1_cyc_time
    }
    #[doc = "0x9a8 - Latch0 Control"]
    #[inline(always)]
    pub const fn dc_latch0_cont(&self) -> &DC_LATCH0_CONT {
        &self.dc_latch0_cont
    }
    #[doc = "0x9a9 - Latch1 Control"]
    #[inline(always)]
    pub const fn dc_latch1_cont(&self) -> &DC_LATCH1_CONT {
        &self.dc_latch1_cont
    }
    #[doc = "0x9ae - Latch0 Status"]
    #[inline(always)]
    pub const fn dc_latch0_stat(&self) -> &DC_LATCH0_STAT {
        &self.dc_latch0_stat
    }
    #[doc = "0x9af - Latch1 Status"]
    #[inline(always)]
    pub const fn dc_latch1_stat(&self) -> &DC_LATCH1_STAT {
        &self.dc_latch1_stat
    }
    #[doc = "0x9b0..0x9b8 - Register captures System time at the positive edge of the Latch0 signal"]
    #[inline(always)]
    pub const fn dc_latch0_time_pos(&self, n: usize) -> &DC_LATCH0_TIME_POS {
        &self.dc_latch0_time_pos[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9b0..0x9b8 - Register captures System time at the positive edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_pos_iter(&self) -> impl Iterator<Item = &DC_LATCH0_TIME_POS> {
        self.dc_latch0_time_pos.iter()
    }
    #[doc = "0x9b8..0x9c0 - Register captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub const fn dc_latch0_time_neg(&self, n: usize) -> &DC_LATCH0_TIME_NEG {
        &self.dc_latch0_time_neg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9b8..0x9c0 - Register captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_neg_iter(&self) -> impl Iterator<Item = &DC_LATCH0_TIME_NEG> {
        self.dc_latch0_time_neg.iter()
    }
    #[doc = "0x9c0..0x9c8 - Register captures System time at the positive edge of the Latch1 signal"]
    #[inline(always)]
    pub const fn dc_latch1_time_pos(&self, n: usize) -> &DC_LATCH1_TIME_POS {
        &self.dc_latch1_time_pos[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c0..0x9c8 - Register captures System time at the positive edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_pos_iter(&self) -> impl Iterator<Item = &DC_LATCH1_TIME_POS> {
        self.dc_latch1_time_pos.iter()
    }
    #[doc = "0x9c8..0x9d0 - Register captures System time at the negative edge of the Latch1 signal"]
    #[inline(always)]
    pub const fn dc_latch1_time_neg(&self, n: usize) -> &DC_LATCH1_TIME_NEG {
        &self.dc_latch1_time_neg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c8..0x9d0 - Register captures System time at the negative edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_neg_iter(&self) -> impl Iterator<Item = &DC_LATCH1_TIME_NEG> {
        self.dc_latch1_time_neg.iter()
    }
    #[doc = "0x9f0 - EtherCAT Buffer Change Event Time"]
    #[inline(always)]
    pub const fn dc_ecat_cng_ev_time(&self) -> &DC_ECAT_CNG_EV_TIME {
        &self.dc_ecat_cng_ev_time
    }
    #[doc = "0x9f8 - PDI Buffer Start Event Time"]
    #[inline(always)]
    pub const fn dc_pdi_start_ev_time(&self) -> &DC_PDI_START_EV_TIME {
        &self.dc_pdi_start_ev_time
    }
    #[doc = "0x9fc - PDI Buffer Change Event Time"]
    #[inline(always)]
    pub const fn dc_pdi_cng_ev_time(&self) -> &DC_PDI_CNG_EV_TIME {
        &self.dc_pdi_cng_ev_time
    }
    #[doc = "0xe00 - ECAT0 Module ID"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0xe08 - ECAT0 Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
#[doc = "TYPE (r) register accessor: Type of EtherCAT Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Type of EtherCAT Controller"]
pub mod type_;
#[doc = "REVISION (r) register accessor: Revision of EtherCAT Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "Revision of EtherCAT Controller"]
pub mod revision;
#[doc = "BUILD (r) register accessor: Build Version\n\nYou can [`read`](crate::Reg::read) this register and get [`build::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@build`]
module"]
pub type BUILD = crate::Reg<build::BUILD_SPEC>;
#[doc = "Build Version"]
pub mod build;
#[doc = "FMMU_NUM (r) register accessor: FMMUs Supported\n\nYou can [`read`](crate::Reg::read) this register and get [`fmmu_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_num`]
module"]
pub type FMMU_NUM = crate::Reg<fmmu_num::FMMU_NUM_SPEC>;
#[doc = "FMMUs Supported"]
pub mod fmmu_num;
#[doc = "SYNC_MANAGER (r) register accessor: SyncManagers Supported\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_manager::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_manager`]
module"]
pub type SYNC_MANAGER = crate::Reg<sync_manager::SYNC_MANAGER_SPEC>;
#[doc = "SyncManagers Supported"]
pub mod sync_manager;
#[doc = "RAM_SIZE (r) register accessor: RAM Size\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_size`]
module"]
pub type RAM_SIZE = crate::Reg<ram_size::RAM_SIZE_SPEC>;
#[doc = "RAM Size"]
pub mod ram_size;
#[doc = "PORT_DESC (r) register accessor: Port Descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`port_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_desc`]
module"]
pub type PORT_DESC = crate::Reg<port_desc::PORT_DESC_SPEC>;
#[doc = "Port Descriptor"]
pub mod port_desc;
#[doc = "FEATURE (r) register accessor: ESC Features Supported\n\nYou can [`read`](crate::Reg::read) this register and get [`feature::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature`]
module"]
pub type FEATURE = crate::Reg<feature::FEATURE_SPEC>;
#[doc = "ESC Features Supported"]
pub mod feature;
#[doc = "STATION_ADR (r) register accessor: Configured Station Address\n\nYou can [`read`](crate::Reg::read) this register and get [`station_adr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@station_adr`]
module"]
pub type STATION_ADR = crate::Reg<station_adr::STATION_ADR_SPEC>;
#[doc = "Configured Station Address"]
pub mod station_adr;
#[doc = "STATION_ALIAS (rw) register accessor: Configured Station Alias\n\nYou can [`read`](crate::Reg::read) this register and get [`station_alias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`station_alias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@station_alias`]
module"]
pub type STATION_ALIAS = crate::Reg<station_alias::STATION_ALIAS_SPEC>;
#[doc = "Configured Station Alias"]
pub mod station_alias;
#[doc = "WR_REG_ENABLE (r) register accessor: Write Register Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_reg_enable::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_reg_enable`]
module"]
pub type WR_REG_ENABLE = crate::Reg<wr_reg_enable::WR_REG_ENABLE_SPEC>;
#[doc = "Write Register Enable"]
pub mod wr_reg_enable;
#[doc = "WR_REG_PROTECT (r) register accessor: Write Register Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_reg_protect::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_reg_protect`]
module"]
pub type WR_REG_PROTECT = crate::Reg<wr_reg_protect::WR_REG_PROTECT_SPEC>;
#[doc = "Write Register Protection"]
pub mod wr_reg_protect;
#[doc = "ESC_WR_ENABLE (r) register accessor: ESC Write Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_wr_enable::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_wr_enable`]
module"]
pub type ESC_WR_ENABLE = crate::Reg<esc_wr_enable::ESC_WR_ENABLE_SPEC>;
#[doc = "ESC Write Enable"]
pub mod esc_wr_enable;
#[doc = "ESC_WR_PROTECT (r) register accessor: ESC Write Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_wr_protect::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_wr_protect`]
module"]
pub type ESC_WR_PROTECT = crate::Reg<esc_wr_protect::ESC_WR_PROTECT_SPEC>;
#[doc = "ESC Write Protection"]
pub mod esc_wr_protect;
#[doc = "WRITEMode_ESC_RESET_ECAT (r) register accessor: ESC Reset ECAT \\[WRITE Mode\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`writemode_esc_reset_ecat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writemode_esc_reset_ecat`]
module"]
#[doc(alias = "WRITEMode_ESC_RESET_ECAT")]
pub type WRITEMODE_ESC_RESET_ECAT = crate::Reg<writemode_esc_reset_ecat::WRITEMODE_ESC_RESET_ECAT_SPEC>;
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]"]
pub mod writemode_esc_reset_ecat;
#[doc = "READMode_ESC_RESET_ECAT (r) register accessor: ESC Reset ECAT \\[READ Mode\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`readmode_esc_reset_ecat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readmode_esc_reset_ecat`]
module"]
#[doc(alias = "READMode_ESC_RESET_ECAT")]
pub type READMODE_ESC_RESET_ECAT = crate::Reg<readmode_esc_reset_ecat::READMODE_ESC_RESET_ECAT_SPEC>;
#[doc = "ESC Reset ECAT \\[READ Mode\\]"]
pub mod readmode_esc_reset_ecat;
#[doc = "WRITEMode_ESC_RESET_PDI (r) register accessor: ESC Reset PDI \\[WRITE Mode\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`writemode_esc_reset_pdi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writemode_esc_reset_pdi`]
module"]
#[doc(alias = "WRITEMode_ESC_RESET_PDI")]
pub type WRITEMODE_ESC_RESET_PDI = crate::Reg<writemode_esc_reset_pdi::WRITEMODE_ESC_RESET_PDI_SPEC>;
#[doc = "ESC Reset PDI \\[WRITE Mode\\]"]
pub mod writemode_esc_reset_pdi;
#[doc = "READMode_ESC_RESET_PDI (r) register accessor: ESC Reset PDI \\[READ Mode\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`readmode_esc_reset_pdi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readmode_esc_reset_pdi`]
module"]
#[doc(alias = "READMode_ESC_RESET_PDI")]
pub type READMODE_ESC_RESET_PDI = crate::Reg<readmode_esc_reset_pdi::READMODE_ESC_RESET_PDI_SPEC>;
#[doc = "ESC Reset PDI \\[READ Mode\\]"]
pub mod readmode_esc_reset_pdi;
#[doc = "ESC_DL_CONTROL (r) register accessor: ESC DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_dl_control::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_dl_control`]
module"]
pub type ESC_DL_CONTROL = crate::Reg<esc_dl_control::ESC_DL_CONTROL_SPEC>;
#[doc = "ESC DL Control"]
pub mod esc_dl_control;
#[doc = "PHYSICAL_RW_OFFSET (r) register accessor: Physical Read/Write Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`physical_rw_offset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_rw_offset`]
module"]
pub type PHYSICAL_RW_OFFSET = crate::Reg<physical_rw_offset::PHYSICAL_RW_OFFSET_SPEC>;
#[doc = "Physical Read/Write Offset"]
pub mod physical_rw_offset;
#[doc = "ESC_DL_STATUS (r) register accessor: ESC DL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_dl_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_dl_status`]
module"]
pub type ESC_DL_STATUS = crate::Reg<esc_dl_status::ESC_DL_STATUS_SPEC>;
#[doc = "ESC DL Status"]
pub mod esc_dl_status;
#[doc = "AL_CONTROL (r) register accessor: AL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`al_control::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_control`]
module"]
pub type AL_CONTROL = crate::Reg<al_control::AL_CONTROL_SPEC>;
#[doc = "AL Control"]
pub mod al_control;
#[doc = "AL_STATUS (rw) register accessor: AL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`al_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`al_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_status`]
module"]
pub type AL_STATUS = crate::Reg<al_status::AL_STATUS_SPEC>;
#[doc = "AL Status"]
pub mod al_status;
#[doc = "AL_STATUS_CODE (rw) register accessor: AL Status Code\n\nYou can [`read`](crate::Reg::read) this register and get [`al_status_code::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`al_status_code::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_status_code`]
module"]
pub type AL_STATUS_CODE = crate::Reg<al_status_code::AL_STATUS_CODE_SPEC>;
#[doc = "AL Status Code"]
pub mod al_status_code;
#[doc = "RUN_LED (rw) register accessor: RUN LED Override\n\nYou can [`read`](crate::Reg::read) this register and get [`run_led::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`run_led::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@run_led`]
module"]
pub type RUN_LED = crate::Reg<run_led::RUN_LED_SPEC>;
#[doc = "RUN LED Override"]
pub mod run_led;
#[doc = "ERR_LED (rw) register accessor: RUN ERR Override\n\nYou can [`read`](crate::Reg::read) this register and get [`err_led::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_led::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_led`]
module"]
pub type ERR_LED = crate::Reg<err_led::ERR_LED_SPEC>;
#[doc = "RUN ERR Override"]
pub mod err_led;
#[doc = "PDI_CONTROL (r) register accessor: PDI Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_control::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_control`]
module"]
pub type PDI_CONTROL = crate::Reg<pdi_control::PDI_CONTROL_SPEC>;
#[doc = "PDI Control"]
pub mod pdi_control;
#[doc = "ESC_CONFIG (r) register accessor: ESC Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_config`]
module"]
pub type ESC_CONFIG = crate::Reg<esc_config::ESC_CONFIG_SPEC>;
#[doc = "ESC Configuration"]
pub mod esc_config;
#[doc = "PDI_CONFIG (r) register accessor: PDI Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_config`]
module"]
pub type PDI_CONFIG = crate::Reg<pdi_config::PDI_CONFIG_SPEC>;
#[doc = "PDI Control"]
pub mod pdi_config;
#[doc = "SYNC_LATCH_CONFIG (r) register accessor: Sync/Latch\\[1:0\\]
PDI Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_latch_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_latch_config`]
module"]
pub type SYNC_LATCH_CONFIG = crate::Reg<sync_latch_config::SYNC_LATCH_CONFIG_SPEC>;
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration"]
pub mod sync_latch_config;
#[doc = "PDI_EXT_CONFIG (r) register accessor: PDI Synchronous Microcontroller extended Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_ext_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_ext_config`]
module"]
pub type PDI_EXT_CONFIG = crate::Reg<pdi_ext_config::PDI_EXT_CONFIG_SPEC>;
#[doc = "PDI Synchronous Microcontroller extended Configuration"]
pub mod pdi_ext_config;
#[doc = "EVENT_MASK (r) register accessor: ECAT Event Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`event_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_mask`]
module"]
pub type EVENT_MASK = crate::Reg<event_mask::EVENT_MASK_SPEC>;
#[doc = "ECAT Event Mask"]
pub mod event_mask;
#[doc = "AL_EVENT_MASK (rw) register accessor: PDI AL Event Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`al_event_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`al_event_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_event_mask`]
module"]
pub type AL_EVENT_MASK = crate::Reg<al_event_mask::AL_EVENT_MASK_SPEC>;
#[doc = "PDI AL Event Mask"]
pub mod al_event_mask;
#[doc = "EVENT_REQ (r) register accessor: ECAT Event Request\n\nYou can [`read`](crate::Reg::read) this register and get [`event_req::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_req`]
module"]
pub type EVENT_REQ = crate::Reg<event_req::EVENT_REQ_SPEC>;
#[doc = "ECAT Event Request"]
pub mod event_req;
#[doc = "AL_EVENT_REQ (rw) register accessor: AL Event Request\n\nYou can [`read`](crate::Reg::read) this register and get [`al_event_req::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`al_event_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_event_req`]
module"]
pub type AL_EVENT_REQ = crate::Reg<al_event_req::AL_EVENT_REQ_SPEC>;
#[doc = "AL Event Request"]
pub mod al_event_req;
#[doc = "RX_ERR_COUNT0 (r) register accessor: RX Error Counter Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err_count0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_count0`]
module"]
pub type RX_ERR_COUNT0 = crate::Reg<rx_err_count0::RX_ERR_COUNT0_SPEC>;
#[doc = "RX Error Counter Port 0"]
pub mod rx_err_count0;
#[doc = "RX_ERR_COUNT1 (r) register accessor: RX Error Counter Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err_count1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_count1`]
module"]
pub type RX_ERR_COUNT1 = crate::Reg<rx_err_count1::RX_ERR_COUNT1_SPEC>;
#[doc = "RX Error Counter Port 1"]
pub mod rx_err_count1;
#[doc = "FWD_RX_ERR_COUNT0 (r) register accessor: Forwarded RX Error Counter Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fwd_rx_err_count0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwd_rx_err_count0`]
module"]
pub type FWD_RX_ERR_COUNT0 = crate::Reg<fwd_rx_err_count0::FWD_RX_ERR_COUNT0_SPEC>;
#[doc = "Forwarded RX Error Counter Port 0"]
pub mod fwd_rx_err_count0;
#[doc = "FWD_RX_ERR_COUNT1 (r) register accessor: Forwarded RX Error Counter Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fwd_rx_err_count1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwd_rx_err_count1`]
module"]
pub type FWD_RX_ERR_COUNT1 = crate::Reg<fwd_rx_err_count1::FWD_RX_ERR_COUNT1_SPEC>;
#[doc = "Forwarded RX Error Counter Port 1"]
pub mod fwd_rx_err_count1;
#[doc = "PROC_ERR_COUNT (r) register accessor: ECAT Processing Unit Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`proc_err_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@proc_err_count`]
module"]
pub type PROC_ERR_COUNT = crate::Reg<proc_err_count::PROC_ERR_COUNT_SPEC>;
#[doc = "ECAT Processing Unit Error Counter"]
pub mod proc_err_count;
#[doc = "PDI_ERR_COUNT (r) register accessor: PDI Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`pdi_err_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_err_count`]
module"]
pub type PDI_ERR_COUNT = crate::Reg<pdi_err_count::PDI_ERR_COUNT_SPEC>;
#[doc = "PDI Error Counter"]
pub mod pdi_err_count;
#[doc = "LOST_LINK_COUNT0 (r) register accessor: Lost Link Counter Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lost_link_count0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lost_link_count0`]
module"]
pub type LOST_LINK_COUNT0 = crate::Reg<lost_link_count0::LOST_LINK_COUNT0_SPEC>;
#[doc = "Lost Link Counter Port 0"]
pub mod lost_link_count0;
#[doc = "LOST_LINK_COUNT1 (r) register accessor: Lost Link Counter Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lost_link_count1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lost_link_count1`]
module"]
pub type LOST_LINK_COUNT1 = crate::Reg<lost_link_count1::LOST_LINK_COUNT1_SPEC>;
#[doc = "Lost Link Counter Port 1"]
pub mod lost_link_count1;
#[doc = "WD_DIVIDE (rw) register accessor: Watchdog Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_divide::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_divide::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_divide`]
module"]
pub type WD_DIVIDE = crate::Reg<wd_divide::WD_DIVIDE_SPEC>;
#[doc = "Watchdog Divider"]
pub mod wd_divide;
#[doc = "WD_TIME_PDI (rw) register accessor: Watchdog Time PDI\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_time_pdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_time_pdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_time_pdi`]
module"]
pub type WD_TIME_PDI = crate::Reg<wd_time_pdi::WD_TIME_PDI_SPEC>;
#[doc = "Watchdog Time PDI"]
pub mod wd_time_pdi;
#[doc = "WD_TIME_PDATA (rw) register accessor: Watchdog Time Process Data\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_time_pdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_time_pdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_time_pdata`]
module"]
pub type WD_TIME_PDATA = crate::Reg<wd_time_pdata::WD_TIME_PDATA_SPEC>;
#[doc = "Watchdog Time Process Data"]
pub mod wd_time_pdata;
#[doc = "WD_STAT_PDATA (r) register accessor: Watchdog Status Process Data\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_stat_pdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_stat_pdata`]
module"]
pub type WD_STAT_PDATA = crate::Reg<wd_stat_pdata::WD_STAT_PDATA_SPEC>;
#[doc = "Watchdog Status Process Data"]
pub mod wd_stat_pdata;
#[doc = "WD_COUNT_PDATA (r) register accessor: Watchdog Counter Process Data\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_count_pdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_count_pdata`]
module"]
pub type WD_COUNT_PDATA = crate::Reg<wd_count_pdata::WD_COUNT_PDATA_SPEC>;
#[doc = "Watchdog Counter Process Data"]
pub mod wd_count_pdata;
#[doc = "WD_COUNT_PDI (r) register accessor: Watchdog Counter PDI\n\nYou can [`read`](crate::Reg::read) this register and get [`wd_count_pdi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_count_pdi`]
module"]
pub type WD_COUNT_PDI = crate::Reg<wd_count_pdi::WD_COUNT_PDI_SPEC>;
#[doc = "Watchdog Counter PDI"]
pub mod wd_count_pdi;
#[doc = "EEP_CONF (r) register accessor: EEPROM Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_conf`]
module"]
pub type EEP_CONF = crate::Reg<eep_conf::EEP_CONF_SPEC>;
#[doc = "EEPROM Configuration"]
pub mod eep_conf;
#[doc = "EEP_STATE (rw) register accessor: EEPROM PDI Access State\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_state`]
module"]
pub type EEP_STATE = crate::Reg<eep_state::EEP_STATE_SPEC>;
#[doc = "EEPROM PDI Access State"]
pub mod eep_state;
#[doc = "EEP_CONT_STAT (rw) register accessor: EEPROM Control/Status\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_cont_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_cont_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_cont_stat`]
module"]
pub type EEP_CONT_STAT = crate::Reg<eep_cont_stat::EEP_CONT_STAT_SPEC>;
#[doc = "EEPROM Control/Status"]
pub mod eep_cont_stat;
#[doc = "EEP_ADR (rw) register accessor: EEPROM Address\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_adr`]
module"]
pub type EEP_ADR = crate::Reg<eep_adr::EEP_ADR_SPEC>;
#[doc = "EEPROM Address"]
pub mod eep_adr;
#[doc = "EEP_DATA (rw) register accessor: EEPROM Read/Write data\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_data`]
module"]
pub type EEP_DATA = crate::Reg<eep_data::EEP_DATA_SPEC>;
#[doc = "EEPROM Read/Write data"]
pub mod eep_data;
#[doc = "MII_CONT_STAT (rw) register accessor: MII Management Control/Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_cont_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mii_cont_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_cont_stat`]
module"]
pub type MII_CONT_STAT = crate::Reg<mii_cont_stat::MII_CONT_STAT_SPEC>;
#[doc = "MII Management Control/Status"]
pub mod mii_cont_stat;
#[doc = "MII_PHY_ADR (rw) register accessor: PHY Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_phy_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mii_phy_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_phy_adr`]
module"]
pub type MII_PHY_ADR = crate::Reg<mii_phy_adr::MII_PHY_ADR_SPEC>;
#[doc = "PHY Address"]
pub mod mii_phy_adr;
#[doc = "MII_PHY_REG_ADR (rw) register accessor: PHY Register Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_phy_reg_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mii_phy_reg_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_phy_reg_adr`]
module"]
pub type MII_PHY_REG_ADR = crate::Reg<mii_phy_reg_adr::MII_PHY_REG_ADR_SPEC>;
#[doc = "PHY Register Address"]
pub mod mii_phy_reg_adr;
#[doc = "MII_PHY_DATA (rw) register accessor: PHY Data\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_phy_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mii_phy_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_phy_data`]
module"]
pub type MII_PHY_DATA = crate::Reg<mii_phy_data::MII_PHY_DATA_SPEC>;
#[doc = "PHY Data"]
pub mod mii_phy_data;
#[doc = "MII_ECAT_ACS_STATE (r) register accessor: MII ECAT ACS STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_ecat_acs_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_ecat_acs_state`]
module"]
pub type MII_ECAT_ACS_STATE = crate::Reg<mii_ecat_acs_state::MII_ECAT_ACS_STATE_SPEC>;
#[doc = "MII ECAT ACS STATE"]
pub mod mii_ecat_acs_state;
#[doc = "MII_PDI_ACS_STATE (rw) register accessor: MII PDI ACS STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mii_pdi_acs_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mii_pdi_acs_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_pdi_acs_state`]
module"]
pub type MII_PDI_ACS_STATE = crate::Reg<mii_pdi_acs_state::MII_PDI_ACS_STATE_SPEC>;
#[doc = "MII PDI ACS STATE"]
pub mod mii_pdi_acs_state;
#[doc = "DC_RCV_TIME_PORT0 (r) register accessor: Receive Time Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_rcv_time_port0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_rcv_time_port0`]
module"]
pub type DC_RCV_TIME_PORT0 = crate::Reg<dc_rcv_time_port0::DC_RCV_TIME_PORT0_SPEC>;
#[doc = "Receive Time Port 0"]
pub mod dc_rcv_time_port0;
#[doc = "DC_RCV_TIME_PORT1 (r) register accessor: Receive Time Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_rcv_time_port1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_rcv_time_port1`]
module"]
pub type DC_RCV_TIME_PORT1 = crate::Reg<dc_rcv_time_port1::DC_RCV_TIME_PORT1_SPEC>;
#[doc = "Receive Time Port 1"]
pub mod dc_rcv_time_port1;
#[doc = "RECEIVE_TIME_PU (r) register accessor: Local time of the beginning of a frame\n\nYou can [`read`](crate::Reg::read) this register and get [`receive_time_pu::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_time_pu`]
module"]
pub type RECEIVE_TIME_PU = crate::Reg<receive_time_pu::RECEIVE_TIME_PU_SPEC>;
#[doc = "Local time of the beginning of a frame"]
pub mod receive_time_pu;
#[doc = "READMode_DC_SYS_TIME (r) register accessor: System Time read access\n\nYou can [`read`](crate::Reg::read) this register and get [`readmode_dc_sys_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readmode_dc_sys_time`]
module"]
#[doc(alias = "READMode_DC_SYS_TIME")]
pub type READMODE_DC_SYS_TIME = crate::Reg<readmode_dc_sys_time::READMODE_DC_SYS_TIME_SPEC>;
#[doc = "System Time read access"]
pub mod readmode_dc_sys_time;
#[doc = "WRITEMode_DC_SYS_TIME (w) register accessor: System Time \\[WRITE Mode\\]\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writemode_dc_sys_time::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writemode_dc_sys_time`]
module"]
#[doc(alias = "WRITEMode_DC_SYS_TIME")]
pub type WRITEMODE_DC_SYS_TIME = crate::Reg<writemode_dc_sys_time::WRITEMODE_DC_SYS_TIME_SPEC>;
#[doc = "System Time \\[WRITE Mode\\]"]
pub mod writemode_dc_sys_time;
#[doc = "DC_SYS_TIME_OFFSET (rw) register accessor: Difference between local time and System Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sys_time_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_offset`]
module"]
pub type DC_SYS_TIME_OFFSET = crate::Reg<dc_sys_time_offset::DC_SYS_TIME_OFFSET_SPEC>;
#[doc = "Difference between local time and System Time"]
pub mod dc_sys_time_offset;
#[doc = "DC_SYS_TIME_DELAY (rw) register accessor: System Time Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sys_time_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_delay`]
module"]
pub type DC_SYS_TIME_DELAY = crate::Reg<dc_sys_time_delay::DC_SYS_TIME_DELAY_SPEC>;
#[doc = "System Time Delay"]
pub mod dc_sys_time_delay;
#[doc = "DC_SYS_TIME_DIFF (r) register accessor: System Time Difference\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_diff::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_diff`]
module"]
pub type DC_SYS_TIME_DIFF = crate::Reg<dc_sys_time_diff::DC_SYS_TIME_DIFF_SPEC>;
#[doc = "System Time Difference"]
pub mod dc_sys_time_diff;
#[doc = "DC_SPEED_COUNT_START (rw) register accessor: Speed Counter Start\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_speed_count_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_speed_count_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_speed_count_start`]
module"]
pub type DC_SPEED_COUNT_START = crate::Reg<dc_speed_count_start::DC_SPEED_COUNT_START_SPEC>;
#[doc = "Speed Counter Start"]
pub mod dc_speed_count_start;
#[doc = "DC_SPEED_COUNT_DIFF (r) register accessor: Speed Counter Diff\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_speed_count_diff::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_speed_count_diff`]
module"]
pub type DC_SPEED_COUNT_DIFF = crate::Reg<dc_speed_count_diff::DC_SPEED_COUNT_DIFF_SPEC>;
#[doc = "Speed Counter Diff"]
pub mod dc_speed_count_diff;
#[doc = "DC_SYS_TIME_FIL_DEPTH (rw) register accessor: System Time Difference Filter Depth\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sys_time_fil_depth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sys_time_fil_depth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_fil_depth`]
module"]
pub type DC_SYS_TIME_FIL_DEPTH = crate::Reg<dc_sys_time_fil_depth::DC_SYS_TIME_FIL_DEPTH_SPEC>;
#[doc = "System Time Difference Filter Depth"]
pub mod dc_sys_time_fil_depth;
#[doc = "DC_SPEED_COUNT_FIL_DEPTH (rw) register accessor: Speed Counter Filter Depth\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_speed_count_fil_depth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_speed_count_fil_depth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_speed_count_fil_depth`]
module"]
pub type DC_SPEED_COUNT_FIL_DEPTH = crate::Reg<dc_speed_count_fil_depth::DC_SPEED_COUNT_FIL_DEPTH_SPEC>;
#[doc = "Speed Counter Filter Depth"]
pub mod dc_speed_count_fil_depth;
#[doc = "DC_CYC_CONT (r) register accessor: Cyclic Unit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_cyc_cont::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_cyc_cont`]
module"]
pub type DC_CYC_CONT = crate::Reg<dc_cyc_cont::DC_CYC_CONT_SPEC>;
#[doc = "Cyclic Unit Control"]
pub mod dc_cyc_cont;
#[doc = "DC_ACT (rw) register accessor: Activation register\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_act::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_act::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_act`]
module"]
pub type DC_ACT = crate::Reg<dc_act::DC_ACT_SPEC>;
#[doc = "Activation register"]
pub mod dc_act;
#[doc = "DC_PULSE_LEN (r) register accessor: Pulse Length of SyncSignals\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_pulse_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_pulse_len`]
module"]
pub type DC_PULSE_LEN = crate::Reg<dc_pulse_len::DC_PULSE_LEN_SPEC>;
#[doc = "Pulse Length of SyncSignals"]
pub mod dc_pulse_len;
#[doc = "DC_ACT_STAT (r) register accessor: Activation Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_act_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_act_stat`]
module"]
pub type DC_ACT_STAT = crate::Reg<dc_act_stat::DC_ACT_STAT_SPEC>;
#[doc = "Activation Status"]
pub mod dc_act_stat;
#[doc = "DC_SYNC0_STAT (r) register accessor: SYNC0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sync0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync0_stat`]
module"]
pub type DC_SYNC0_STAT = crate::Reg<dc_sync0_stat::DC_SYNC0_STAT_SPEC>;
#[doc = "SYNC0 Status"]
pub mod dc_sync0_stat;
#[doc = "DC_SYNC1_STAT (r) register accessor: SYNC1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sync1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync1_stat`]
module"]
pub type DC_SYNC1_STAT = crate::Reg<dc_sync1_stat::DC_SYNC1_STAT_SPEC>;
#[doc = "SYNC1 Status"]
pub mod dc_sync1_stat;
#[doc = "DC_CYC_START_TIME (rw) register accessor: Start Time Cyclic Operation\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_cyc_start_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_cyc_start_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_cyc_start_time`]
module"]
pub type DC_CYC_START_TIME = crate::Reg<dc_cyc_start_time::DC_CYC_START_TIME_SPEC>;
#[doc = "Start Time Cyclic Operation"]
pub mod dc_cyc_start_time;
#[doc = "DC_NEXT_SYNC1_PULSE (r) register accessor: System time of next SYNC1 pulse in ns\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_next_sync1_pulse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_next_sync1_pulse`]
module"]
pub type DC_NEXT_SYNC1_PULSE = crate::Reg<dc_next_sync1_pulse::DC_NEXT_SYNC1_PULSE_SPEC>;
#[doc = "System time of next SYNC1 pulse in ns"]
pub mod dc_next_sync1_pulse;
#[doc = "DC_SYNC0_CYC_TIME (rw) register accessor: SYNC0 Cycle Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sync0_cyc_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sync0_cyc_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync0_cyc_time`]
module"]
pub type DC_SYNC0_CYC_TIME = crate::Reg<dc_sync0_cyc_time::DC_SYNC0_CYC_TIME_SPEC>;
#[doc = "SYNC0 Cycle Time"]
pub mod dc_sync0_cyc_time;
#[doc = "DC_SYNC1_CYC_TIME (rw) register accessor: SYNC1 Cycle Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_sync1_cyc_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_sync1_cyc_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync1_cyc_time`]
module"]
pub type DC_SYNC1_CYC_TIME = crate::Reg<dc_sync1_cyc_time::DC_SYNC1_CYC_TIME_SPEC>;
#[doc = "SYNC1 Cycle Time"]
pub mod dc_sync1_cyc_time;
#[doc = "DC_LATCH0_CONT (rw) register accessor: Latch0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch0_cont::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_latch0_cont::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_cont`]
module"]
pub type DC_LATCH0_CONT = crate::Reg<dc_latch0_cont::DC_LATCH0_CONT_SPEC>;
#[doc = "Latch0 Control"]
pub mod dc_latch0_cont;
#[doc = "DC_LATCH1_CONT (rw) register accessor: Latch1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch1_cont::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_latch1_cont::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_cont`]
module"]
pub type DC_LATCH1_CONT = crate::Reg<dc_latch1_cont::DC_LATCH1_CONT_SPEC>;
#[doc = "Latch1 Control"]
pub mod dc_latch1_cont;
#[doc = "DC_LATCH0_STAT (r) register accessor: Latch0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_stat`]
module"]
pub type DC_LATCH0_STAT = crate::Reg<dc_latch0_stat::DC_LATCH0_STAT_SPEC>;
#[doc = "Latch0 Status"]
pub mod dc_latch0_stat;
#[doc = "DC_LATCH1_STAT (r) register accessor: Latch1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_stat`]
module"]
pub type DC_LATCH1_STAT = crate::Reg<dc_latch1_stat::DC_LATCH1_STAT_SPEC>;
#[doc = "Latch1 Status"]
pub mod dc_latch1_stat;
#[doc = "DC_LATCH0_TIME_POS (r) register accessor: Register captures System time at the positive edge of the Latch0 signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch0_time_pos::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_time_pos`]
module"]
pub type DC_LATCH0_TIME_POS = crate::Reg<dc_latch0_time_pos::DC_LATCH0_TIME_POS_SPEC>;
#[doc = "Register captures System time at the positive edge of the Latch0 signal"]
pub mod dc_latch0_time_pos;
#[doc = "DC_LATCH0_TIME_NEG (r) register accessor: Register captures System time at the negative edge of the Latch0 signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch0_time_neg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_time_neg`]
module"]
pub type DC_LATCH0_TIME_NEG = crate::Reg<dc_latch0_time_neg::DC_LATCH0_TIME_NEG_SPEC>;
#[doc = "Register captures System time at the negative edge of the Latch0 signal"]
pub mod dc_latch0_time_neg;
#[doc = "DC_LATCH1_TIME_POS (r) register accessor: Register captures System time at the positive edge of the Latch1 signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch1_time_pos::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_time_pos`]
module"]
pub type DC_LATCH1_TIME_POS = crate::Reg<dc_latch1_time_pos::DC_LATCH1_TIME_POS_SPEC>;
#[doc = "Register captures System time at the positive edge of the Latch1 signal"]
pub mod dc_latch1_time_pos;
#[doc = "DC_LATCH1_TIME_NEG (r) register accessor: Register captures System time at the negative edge of the Latch1 signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_latch1_time_neg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_time_neg`]
module"]
pub type DC_LATCH1_TIME_NEG = crate::Reg<dc_latch1_time_neg::DC_LATCH1_TIME_NEG_SPEC>;
#[doc = "Register captures System time at the negative edge of the Latch1 signal"]
pub mod dc_latch1_time_neg;
#[doc = "DC_ECAT_CNG_EV_TIME (r) register accessor: EtherCAT Buffer Change Event Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_ecat_cng_ev_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_ecat_cng_ev_time`]
module"]
pub type DC_ECAT_CNG_EV_TIME = crate::Reg<dc_ecat_cng_ev_time::DC_ECAT_CNG_EV_TIME_SPEC>;
#[doc = "EtherCAT Buffer Change Event Time"]
pub mod dc_ecat_cng_ev_time;
#[doc = "DC_PDI_START_EV_TIME (r) register accessor: PDI Buffer Start Event Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_pdi_start_ev_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_pdi_start_ev_time`]
module"]
pub type DC_PDI_START_EV_TIME = crate::Reg<dc_pdi_start_ev_time::DC_PDI_START_EV_TIME_SPEC>;
#[doc = "PDI Buffer Start Event Time"]
pub mod dc_pdi_start_ev_time;
#[doc = "DC_PDI_CNG_EV_TIME (r) register accessor: PDI Buffer Change Event Time\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_pdi_cng_ev_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_pdi_cng_ev_time`]
module"]
pub type DC_PDI_CNG_EV_TIME = crate::Reg<dc_pdi_cng_ev_time::DC_PDI_CNG_EV_TIME_SPEC>;
#[doc = "PDI Buffer Change Event Time"]
pub mod dc_pdi_cng_ev_time;
#[doc = "ID (r) register accessor: ECAT0 Module ID\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ECAT0 Module ID"]
pub mod id;
#[doc = "STATUS (r) register accessor: ECAT0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "ECAT0 Status"]
pub mod status;
