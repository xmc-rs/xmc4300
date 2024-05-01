#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    type_: Type,
    revision: Revision,
    build: Build,
    fmmu_num: FmmuNum,
    sync_manager: SyncManager,
    ram_size: RamSize,
    port_desc: PortDesc,
    feature: Feature,
    _reserved8: [u8; 0x06],
    station_adr: StationAdr,
    station_alias: StationAlias,
    _reserved10: [u8; 0x0c],
    wr_reg_enable: WrRegEnable,
    wr_reg_protect: WrRegProtect,
    _reserved12: [u8; 0x0e],
    esc_wr_enable: EscWrEnable,
    esc_wr_protect: EscWrProtect,
    _reserved14: [u8; 0x0e],
    _reserved_14_readmode_esc_reset_ecat: [u8; 0x01],
    _reserved_15_readmode_esc_reset_pdi: [u8; 0x01],
    _reserved16: [u8; 0xbe],
    esc_dl_control: EscDlControl,
    _reserved17: [u8; 0x04],
    physical_rw_offset: PhysicalRwOffset,
    _reserved18: [u8; 0x06],
    esc_dl_status: EscDlStatus,
    _reserved19: [u8; 0x0e],
    al_control: AlControl,
    _reserved20: [u8; 0x0e],
    al_status: AlStatus,
    _reserved21: [u8; 0x02],
    al_status_code: AlStatusCode,
    _reserved22: [u8; 0x02],
    run_led: RunLed,
    err_led: ErrLed,
    _reserved24: [u8; 0x06],
    pdi_control: PdiControl,
    esc_config: EscConfig,
    _reserved26: [u8; 0x0e],
    pdi_config: PdiConfig,
    sync_latch_config: SyncLatchConfig,
    pdi_ext_config: PdiExtConfig,
    _reserved29: [u8; 0xac],
    event_mask: EventMask,
    _reserved30: [u8; 0x02],
    al_event_mask: AlEventMask,
    _reserved31: [u8; 0x08],
    event_req: EventReq,
    _reserved32: [u8; 0x0e],
    al_event_req: AlEventReq,
    _reserved33: [u8; 0xdc],
    rx_err_count0: RxErrCount0,
    rx_err_count1: RxErrCount1,
    _reserved35: [u8; 0x04],
    fwd_rx_err_count0: FwdRxErrCount0,
    fwd_rx_err_count1: FwdRxErrCount1,
    _reserved37: [u8; 0x02],
    proc_err_count: ProcErrCount,
    pdi_err_count: PdiErrCount,
    _reserved39: [u8; 0x02],
    lost_link_count0: LostLinkCount0,
    lost_link_count1: LostLinkCount1,
    _reserved41: [u8; 0xee],
    wd_divide: WdDivide,
    _reserved42: [u8; 0x0e],
    wd_time_pdi: WdTimePdi,
    _reserved43: [u8; 0x0e],
    wd_time_pdata: WdTimePdata,
    _reserved44: [u8; 0x1e],
    wd_stat_pdata: WdStatPdata,
    wd_count_pdata: WdCountPdata,
    wd_count_pdi: WdCountPdi,
    _reserved47: [u8; 0xbc],
    eep_conf: EepConf,
    eep_state: EepState,
    eep_cont_stat: EepContStat,
    eep_adr: EepAdr,
    eep_data: [EepData; 2],
    mii_cont_stat: MiiContStat,
    mii_phy_adr: MiiPhyAdr,
    mii_phy_reg_adr: MiiPhyRegAdr,
    mii_phy_data: MiiPhyData,
    mii_ecat_acs_state: MiiEcatAcsState,
    mii_pdi_acs_state: MiiPdiAcsState,
    _reserved58: [u8; 0x03e8],
    dc_rcv_time_port0: DcRcvTimePort0,
    dc_rcv_time_port1: DcRcvTimePort1,
    _reserved60: [u8; 0x08],
    _reserved_60_readmode_dc_sys_time: [u8; 0x08],
    receive_time_pu: [ReceiveTimePu; 2],
    dc_sys_time_offset: [DcSysTimeOffset; 2],
    dc_sys_time_delay: DcSysTimeDelay,
    dc_sys_time_diff: DcSysTimeDiff,
    dc_speed_count_start: DcSpeedCountStart,
    dc_speed_count_diff: DcSpeedCountDiff,
    dc_sys_time_fil_depth: DcSysTimeFilDepth,
    dc_speed_count_fil_depth: DcSpeedCountFilDepth,
    _reserved69: [u8; 0x4a],
    dc_cyc_cont: DcCycCont,
    dc_act: DcAct,
    dc_pulse_len: DcPulseLen,
    dc_act_stat: DcActStat,
    _reserved73: [u8; 0x09],
    dc_sync0_stat: DcSync0Stat,
    dc_sync1_stat: DcSync1Stat,
    dc_cyc_start_time: [DcCycStartTime; 2],
    dc_next_sync1_pulse: [DcNextSync1Pulse; 2],
    dc_sync0_cyc_time: DcSync0CycTime,
    dc_sync1_cyc_time: DcSync1CycTime,
    dc_latch0_cont: DcLatch0Cont,
    dc_latch1_cont: DcLatch1Cont,
    _reserved81: [u8; 0x04],
    dc_latch0_stat: DcLatch0Stat,
    dc_latch1_stat: DcLatch1Stat,
    dc_latch0_time_pos: [DcLatch0TimePos; 2],
    dc_latch0_time_neg: [DcLatch0TimeNeg; 2],
    dc_latch1_time_pos: [DcLatch1TimePos; 2],
    dc_latch1_time_neg: [DcLatch1TimeNeg; 2],
    _reserved87: [u8; 0x20],
    dc_ecat_cng_ev_time: DcEcatCngEvTime,
    _reserved88: [u8; 0x04],
    dc_pdi_start_ev_time: DcPdiStartEvTime,
    dc_pdi_cng_ev_time: DcPdiCngEvTime,
    _reserved90: [u8; 0x0400],
    id: Id,
    _reserved91: [u8; 0x04],
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x00 - Type of EtherCAT Controller"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x01 - Revision of EtherCAT Controller"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x02 - Build Version"]
    #[inline(always)]
    pub const fn build(&self) -> &Build {
        &self.build
    }
    #[doc = "0x04 - FMMUs Supported"]
    #[inline(always)]
    pub const fn fmmu_num(&self) -> &FmmuNum {
        &self.fmmu_num
    }
    #[doc = "0x05 - SyncManagers Supported"]
    #[inline(always)]
    pub const fn sync_manager(&self) -> &SyncManager {
        &self.sync_manager
    }
    #[doc = "0x06 - RAM Size"]
    #[inline(always)]
    pub const fn ram_size(&self) -> &RamSize {
        &self.ram_size
    }
    #[doc = "0x07 - Port Descriptor"]
    #[inline(always)]
    pub const fn port_desc(&self) -> &PortDesc {
        &self.port_desc
    }
    #[doc = "0x08 - ESC Features Supported"]
    #[inline(always)]
    pub const fn feature(&self) -> &Feature {
        &self.feature
    }
    #[doc = "0x10 - Configured Station Address"]
    #[inline(always)]
    pub const fn station_adr(&self) -> &StationAdr {
        &self.station_adr
    }
    #[doc = "0x12 - Configured Station Alias"]
    #[inline(always)]
    pub const fn station_alias(&self) -> &StationAlias {
        &self.station_alias
    }
    #[doc = "0x20 - Write Register Enable"]
    #[inline(always)]
    pub const fn wr_reg_enable(&self) -> &WrRegEnable {
        &self.wr_reg_enable
    }
    #[doc = "0x21 - Write Register Protection"]
    #[inline(always)]
    pub const fn wr_reg_protect(&self) -> &WrRegProtect {
        &self.wr_reg_protect
    }
    #[doc = "0x30 - ESC Write Enable"]
    #[inline(always)]
    pub const fn esc_wr_enable(&self) -> &EscWrEnable {
        &self.esc_wr_enable
    }
    #[doc = "0x31 - ESC Write Protection"]
    #[inline(always)]
    pub const fn esc_wr_protect(&self) -> &EscWrProtect {
        &self.esc_wr_protect
    }
    #[doc = "0x40 - ESC Reset ECAT \\[READ Mode\\]"]
    #[inline(always)]
    pub const fn readmode_esc_reset_ecat(&self) -> &ReadmodeEscResetEcat {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - ESC Reset ECAT \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_esc_reset_ecat(&self) -> &WritemodeEscResetEcat {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x41 - ESC Reset PDI \\[READ Mode\\]"]
    #[inline(always)]
    pub const fn readmode_esc_reset_pdi(&self) -> &ReadmodeEscResetPdi {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x41 - ESC Reset PDI \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_esc_reset_pdi(&self) -> &WritemodeEscResetPdi {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x100 - ESC DL Control"]
    #[inline(always)]
    pub const fn esc_dl_control(&self) -> &EscDlControl {
        &self.esc_dl_control
    }
    #[doc = "0x108 - Physical Read/Write Offset"]
    #[inline(always)]
    pub const fn physical_rw_offset(&self) -> &PhysicalRwOffset {
        &self.physical_rw_offset
    }
    #[doc = "0x110 - ESC DL Status"]
    #[inline(always)]
    pub const fn esc_dl_status(&self) -> &EscDlStatus {
        &self.esc_dl_status
    }
    #[doc = "0x120 - AL Control"]
    #[inline(always)]
    pub const fn al_control(&self) -> &AlControl {
        &self.al_control
    }
    #[doc = "0x130 - AL Status"]
    #[inline(always)]
    pub const fn al_status(&self) -> &AlStatus {
        &self.al_status
    }
    #[doc = "0x134 - AL Status Code"]
    #[inline(always)]
    pub const fn al_status_code(&self) -> &AlStatusCode {
        &self.al_status_code
    }
    #[doc = "0x138 - RUN LED Override"]
    #[inline(always)]
    pub const fn run_led(&self) -> &RunLed {
        &self.run_led
    }
    #[doc = "0x139 - RUN ERR Override"]
    #[inline(always)]
    pub const fn err_led(&self) -> &ErrLed {
        &self.err_led
    }
    #[doc = "0x140 - PDI Control"]
    #[inline(always)]
    pub const fn pdi_control(&self) -> &PdiControl {
        &self.pdi_control
    }
    #[doc = "0x141 - ESC Configuration"]
    #[inline(always)]
    pub const fn esc_config(&self) -> &EscConfig {
        &self.esc_config
    }
    #[doc = "0x150 - PDI Control"]
    #[inline(always)]
    pub const fn pdi_config(&self) -> &PdiConfig {
        &self.pdi_config
    }
    #[doc = "0x151 - Sync/Latch\\[1:0\\]
PDI Configuration"]
    #[inline(always)]
    pub const fn sync_latch_config(&self) -> &SyncLatchConfig {
        &self.sync_latch_config
    }
    #[doc = "0x152 - PDI Synchronous Microcontroller extended Configuration"]
    #[inline(always)]
    pub const fn pdi_ext_config(&self) -> &PdiExtConfig {
        &self.pdi_ext_config
    }
    #[doc = "0x200 - ECAT Event Mask"]
    #[inline(always)]
    pub const fn event_mask(&self) -> &EventMask {
        &self.event_mask
    }
    #[doc = "0x204 - PDI AL Event Mask"]
    #[inline(always)]
    pub const fn al_event_mask(&self) -> &AlEventMask {
        &self.al_event_mask
    }
    #[doc = "0x210 - ECAT Event Request"]
    #[inline(always)]
    pub const fn event_req(&self) -> &EventReq {
        &self.event_req
    }
    #[doc = "0x220 - AL Event Request"]
    #[inline(always)]
    pub const fn al_event_req(&self) -> &AlEventReq {
        &self.al_event_req
    }
    #[doc = "0x300 - RX Error Counter Port 0"]
    #[inline(always)]
    pub const fn rx_err_count0(&self) -> &RxErrCount0 {
        &self.rx_err_count0
    }
    #[doc = "0x302 - RX Error Counter Port 1"]
    #[inline(always)]
    pub const fn rx_err_count1(&self) -> &RxErrCount1 {
        &self.rx_err_count1
    }
    #[doc = "0x308 - Forwarded RX Error Counter Port 0"]
    #[inline(always)]
    pub const fn fwd_rx_err_count0(&self) -> &FwdRxErrCount0 {
        &self.fwd_rx_err_count0
    }
    #[doc = "0x309 - Forwarded RX Error Counter Port 1"]
    #[inline(always)]
    pub const fn fwd_rx_err_count1(&self) -> &FwdRxErrCount1 {
        &self.fwd_rx_err_count1
    }
    #[doc = "0x30c - ECAT Processing Unit Error Counter"]
    #[inline(always)]
    pub const fn proc_err_count(&self) -> &ProcErrCount {
        &self.proc_err_count
    }
    #[doc = "0x30d - PDI Error Counter"]
    #[inline(always)]
    pub const fn pdi_err_count(&self) -> &PdiErrCount {
        &self.pdi_err_count
    }
    #[doc = "0x310 - Lost Link Counter Port 0"]
    #[inline(always)]
    pub const fn lost_link_count0(&self) -> &LostLinkCount0 {
        &self.lost_link_count0
    }
    #[doc = "0x311 - Lost Link Counter Port 1"]
    #[inline(always)]
    pub const fn lost_link_count1(&self) -> &LostLinkCount1 {
        &self.lost_link_count1
    }
    #[doc = "0x400 - Watchdog Divider"]
    #[inline(always)]
    pub const fn wd_divide(&self) -> &WdDivide {
        &self.wd_divide
    }
    #[doc = "0x410 - Watchdog Time PDI"]
    #[inline(always)]
    pub const fn wd_time_pdi(&self) -> &WdTimePdi {
        &self.wd_time_pdi
    }
    #[doc = "0x420 - Watchdog Time Process Data"]
    #[inline(always)]
    pub const fn wd_time_pdata(&self) -> &WdTimePdata {
        &self.wd_time_pdata
    }
    #[doc = "0x440 - Watchdog Status Process Data"]
    #[inline(always)]
    pub const fn wd_stat_pdata(&self) -> &WdStatPdata {
        &self.wd_stat_pdata
    }
    #[doc = "0x442 - Watchdog Counter Process Data"]
    #[inline(always)]
    pub const fn wd_count_pdata(&self) -> &WdCountPdata {
        &self.wd_count_pdata
    }
    #[doc = "0x443 - Watchdog Counter PDI"]
    #[inline(always)]
    pub const fn wd_count_pdi(&self) -> &WdCountPdi {
        &self.wd_count_pdi
    }
    #[doc = "0x500 - EEPROM Configuration"]
    #[inline(always)]
    pub const fn eep_conf(&self) -> &EepConf {
        &self.eep_conf
    }
    #[doc = "0x501 - EEPROM PDI Access State"]
    #[inline(always)]
    pub const fn eep_state(&self) -> &EepState {
        &self.eep_state
    }
    #[doc = "0x502 - EEPROM Control/Status"]
    #[inline(always)]
    pub const fn eep_cont_stat(&self) -> &EepContStat {
        &self.eep_cont_stat
    }
    #[doc = "0x504 - EEPROM Address"]
    #[inline(always)]
    pub const fn eep_adr(&self) -> &EepAdr {
        &self.eep_adr
    }
    #[doc = "0x508..0x510 - EEPROM Read/Write data"]
    #[inline(always)]
    pub const fn eep_data(&self, n: usize) -> &EepData {
        &self.eep_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x508..0x510 - EEPROM Read/Write data"]
    #[inline(always)]
    pub fn eep_data_iter(&self) -> impl Iterator<Item = &EepData> {
        self.eep_data.iter()
    }
    #[doc = "0x510 - MII Management Control/Status"]
    #[inline(always)]
    pub const fn mii_cont_stat(&self) -> &MiiContStat {
        &self.mii_cont_stat
    }
    #[doc = "0x512 - PHY Address"]
    #[inline(always)]
    pub const fn mii_phy_adr(&self) -> &MiiPhyAdr {
        &self.mii_phy_adr
    }
    #[doc = "0x513 - PHY Register Address"]
    #[inline(always)]
    pub const fn mii_phy_reg_adr(&self) -> &MiiPhyRegAdr {
        &self.mii_phy_reg_adr
    }
    #[doc = "0x514 - PHY Data"]
    #[inline(always)]
    pub const fn mii_phy_data(&self) -> &MiiPhyData {
        &self.mii_phy_data
    }
    #[doc = "0x516 - MII ECAT ACS STATE"]
    #[inline(always)]
    pub const fn mii_ecat_acs_state(&self) -> &MiiEcatAcsState {
        &self.mii_ecat_acs_state
    }
    #[doc = "0x517 - MII PDI ACS STATE"]
    #[inline(always)]
    pub const fn mii_pdi_acs_state(&self) -> &MiiPdiAcsState {
        &self.mii_pdi_acs_state
    }
    #[doc = "0x900 - Receive Time Port 0"]
    #[inline(always)]
    pub const fn dc_rcv_time_port0(&self) -> &DcRcvTimePort0 {
        &self.dc_rcv_time_port0
    }
    #[doc = "0x904 - Receive Time Port 1"]
    #[inline(always)]
    pub const fn dc_rcv_time_port1(&self) -> &DcRcvTimePort1 {
        &self.dc_rcv_time_port1
    }
    #[doc = "0x910 - System Time \\[WRITE Mode\\]"]
    #[inline(always)]
    pub const fn writemode_dc_sys_time(&self) -> &WritemodeDcSysTime {
        unsafe { &*(self as *const Self).cast::<u8>().add(2320).cast() }
    }
    #[doc = "0x910..0x918 - System Time read access"]
    #[inline(always)]
    pub const fn readmode_dc_sys_time(&self, n: usize) -> &ReadmodeDcSysTime {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(2320).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x910..0x918 - System Time read access"]
    #[inline(always)]
    pub fn readmode_dc_sys_time_iter(&self) -> impl Iterator<Item = &ReadmodeDcSysTime> {
        (0..2).map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(2320).add(4 * n).cast() })
    }
    #[doc = "0x918..0x920 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub const fn receive_time_pu(&self, n: usize) -> &ReceiveTimePu {
        &self.receive_time_pu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x918..0x920 - Local time of the beginning of a frame"]
    #[inline(always)]
    pub fn receive_time_pu_iter(&self) -> impl Iterator<Item = &ReceiveTimePu> {
        self.receive_time_pu.iter()
    }
    #[doc = "0x920..0x928 - Difference between local time and System Time"]
    #[inline(always)]
    pub const fn dc_sys_time_offset(&self, n: usize) -> &DcSysTimeOffset {
        &self.dc_sys_time_offset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x920..0x928 - Difference between local time and System Time"]
    #[inline(always)]
    pub fn dc_sys_time_offset_iter(&self) -> impl Iterator<Item = &DcSysTimeOffset> {
        self.dc_sys_time_offset.iter()
    }
    #[doc = "0x928 - System Time Delay"]
    #[inline(always)]
    pub const fn dc_sys_time_delay(&self) -> &DcSysTimeDelay {
        &self.dc_sys_time_delay
    }
    #[doc = "0x92c - System Time Difference"]
    #[inline(always)]
    pub const fn dc_sys_time_diff(&self) -> &DcSysTimeDiff {
        &self.dc_sys_time_diff
    }
    #[doc = "0x930 - Speed Counter Start"]
    #[inline(always)]
    pub const fn dc_speed_count_start(&self) -> &DcSpeedCountStart {
        &self.dc_speed_count_start
    }
    #[doc = "0x932 - Speed Counter Diff"]
    #[inline(always)]
    pub const fn dc_speed_count_diff(&self) -> &DcSpeedCountDiff {
        &self.dc_speed_count_diff
    }
    #[doc = "0x934 - System Time Difference Filter Depth"]
    #[inline(always)]
    pub const fn dc_sys_time_fil_depth(&self) -> &DcSysTimeFilDepth {
        &self.dc_sys_time_fil_depth
    }
    #[doc = "0x935 - Speed Counter Filter Depth"]
    #[inline(always)]
    pub const fn dc_speed_count_fil_depth(&self) -> &DcSpeedCountFilDepth {
        &self.dc_speed_count_fil_depth
    }
    #[doc = "0x980 - Cyclic Unit Control"]
    #[inline(always)]
    pub const fn dc_cyc_cont(&self) -> &DcCycCont {
        &self.dc_cyc_cont
    }
    #[doc = "0x981 - Activation register"]
    #[inline(always)]
    pub const fn dc_act(&self) -> &DcAct {
        &self.dc_act
    }
    #[doc = "0x982 - Pulse Length of SyncSignals"]
    #[inline(always)]
    pub const fn dc_pulse_len(&self) -> &DcPulseLen {
        &self.dc_pulse_len
    }
    #[doc = "0x984 - Activation Status"]
    #[inline(always)]
    pub const fn dc_act_stat(&self) -> &DcActStat {
        &self.dc_act_stat
    }
    #[doc = "0x98e - SYNC0 Status"]
    #[inline(always)]
    pub const fn dc_sync0_stat(&self) -> &DcSync0Stat {
        &self.dc_sync0_stat
    }
    #[doc = "0x98f - SYNC1 Status"]
    #[inline(always)]
    pub const fn dc_sync1_stat(&self) -> &DcSync1Stat {
        &self.dc_sync1_stat
    }
    #[doc = "0x990..0x998 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub const fn dc_cyc_start_time(&self, n: usize) -> &DcCycStartTime {
        &self.dc_cyc_start_time[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x990..0x998 - Start Time Cyclic Operation"]
    #[inline(always)]
    pub fn dc_cyc_start_time_iter(&self) -> impl Iterator<Item = &DcCycStartTime> {
        self.dc_cyc_start_time.iter()
    }
    #[doc = "0x998..0x9a0 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub const fn dc_next_sync1_pulse(&self, n: usize) -> &DcNextSync1Pulse {
        &self.dc_next_sync1_pulse[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x998..0x9a0 - System time of next SYNC1 pulse in ns"]
    #[inline(always)]
    pub fn dc_next_sync1_pulse_iter(&self) -> impl Iterator<Item = &DcNextSync1Pulse> {
        self.dc_next_sync1_pulse.iter()
    }
    #[doc = "0x9a0 - SYNC0 Cycle Time"]
    #[inline(always)]
    pub const fn dc_sync0_cyc_time(&self) -> &DcSync0CycTime {
        &self.dc_sync0_cyc_time
    }
    #[doc = "0x9a4 - SYNC1 Cycle Time"]
    #[inline(always)]
    pub const fn dc_sync1_cyc_time(&self) -> &DcSync1CycTime {
        &self.dc_sync1_cyc_time
    }
    #[doc = "0x9a8 - Latch0 Control"]
    #[inline(always)]
    pub const fn dc_latch0_cont(&self) -> &DcLatch0Cont {
        &self.dc_latch0_cont
    }
    #[doc = "0x9a9 - Latch1 Control"]
    #[inline(always)]
    pub const fn dc_latch1_cont(&self) -> &DcLatch1Cont {
        &self.dc_latch1_cont
    }
    #[doc = "0x9ae - Latch0 Status"]
    #[inline(always)]
    pub const fn dc_latch0_stat(&self) -> &DcLatch0Stat {
        &self.dc_latch0_stat
    }
    #[doc = "0x9af - Latch1 Status"]
    #[inline(always)]
    pub const fn dc_latch1_stat(&self) -> &DcLatch1Stat {
        &self.dc_latch1_stat
    }
    #[doc = "0x9b0..0x9b8 - Register captures System time at the positive edge of the Latch0 signal"]
    #[inline(always)]
    pub const fn dc_latch0_time_pos(&self, n: usize) -> &DcLatch0TimePos {
        &self.dc_latch0_time_pos[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9b0..0x9b8 - Register captures System time at the positive edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_pos_iter(&self) -> impl Iterator<Item = &DcLatch0TimePos> {
        self.dc_latch0_time_pos.iter()
    }
    #[doc = "0x9b8..0x9c0 - Register captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub const fn dc_latch0_time_neg(&self, n: usize) -> &DcLatch0TimeNeg {
        &self.dc_latch0_time_neg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9b8..0x9c0 - Register captures System time at the negative edge of the Latch0 signal"]
    #[inline(always)]
    pub fn dc_latch0_time_neg_iter(&self) -> impl Iterator<Item = &DcLatch0TimeNeg> {
        self.dc_latch0_time_neg.iter()
    }
    #[doc = "0x9c0..0x9c8 - Register captures System time at the positive edge of the Latch1 signal"]
    #[inline(always)]
    pub const fn dc_latch1_time_pos(&self, n: usize) -> &DcLatch1TimePos {
        &self.dc_latch1_time_pos[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c0..0x9c8 - Register captures System time at the positive edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_pos_iter(&self) -> impl Iterator<Item = &DcLatch1TimePos> {
        self.dc_latch1_time_pos.iter()
    }
    #[doc = "0x9c8..0x9d0 - Register captures System time at the negative edge of the Latch1 signal"]
    #[inline(always)]
    pub const fn dc_latch1_time_neg(&self, n: usize) -> &DcLatch1TimeNeg {
        &self.dc_latch1_time_neg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c8..0x9d0 - Register captures System time at the negative edge of the Latch1 signal"]
    #[inline(always)]
    pub fn dc_latch1_time_neg_iter(&self) -> impl Iterator<Item = &DcLatch1TimeNeg> {
        self.dc_latch1_time_neg.iter()
    }
    #[doc = "0x9f0 - EtherCAT Buffer Change Event Time"]
    #[inline(always)]
    pub const fn dc_ecat_cng_ev_time(&self) -> &DcEcatCngEvTime {
        &self.dc_ecat_cng_ev_time
    }
    #[doc = "0x9f8 - PDI Buffer Start Event Time"]
    #[inline(always)]
    pub const fn dc_pdi_start_ev_time(&self) -> &DcPdiStartEvTime {
        &self.dc_pdi_start_ev_time
    }
    #[doc = "0x9fc - PDI Buffer Change Event Time"]
    #[inline(always)]
    pub const fn dc_pdi_cng_ev_time(&self) -> &DcPdiCngEvTime {
        &self.dc_pdi_cng_ev_time
    }
    #[doc = "0xe00 - ECAT0 Module ID"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0xe08 - ECAT0 Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "TYPE (r) register accessor: Type of EtherCAT Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "Type of EtherCAT Controller"]
pub mod type_;
#[doc = "REVISION (r) register accessor: Revision of EtherCAT Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revision::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "Revision of EtherCAT Controller"]
pub mod revision;
#[doc = "BUILD (r) register accessor: Build Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`build::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@build`]
module"]
#[doc(alias = "BUILD")]
pub type Build = crate::Reg<build::BuildSpec>;
#[doc = "Build Version"]
pub mod build;
#[doc = "FMMU_NUM (r) register accessor: FMMUs Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmmu_num`]
module"]
#[doc(alias = "FMMU_NUM")]
pub type FmmuNum = crate::Reg<fmmu_num::FmmuNumSpec>;
#[doc = "FMMUs Supported"]
pub mod fmmu_num;
#[doc = "SYNC_MANAGER (r) register accessor: SyncManagers Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_manager::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_manager`]
module"]
#[doc(alias = "SYNC_MANAGER")]
pub type SyncManager = crate::Reg<sync_manager::SyncManagerSpec>;
#[doc = "SyncManagers Supported"]
pub mod sync_manager;
#[doc = "RAM_SIZE (r) register accessor: RAM Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_size`]
module"]
#[doc(alias = "RAM_SIZE")]
pub type RamSize = crate::Reg<ram_size::RamSizeSpec>;
#[doc = "RAM Size"]
pub mod ram_size;
#[doc = "PORT_DESC (r) register accessor: Port Descriptor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_desc`]
module"]
#[doc(alias = "PORT_DESC")]
pub type PortDesc = crate::Reg<port_desc::PortDescSpec>;
#[doc = "Port Descriptor"]
pub mod port_desc;
#[doc = "FEATURE (r) register accessor: ESC Features Supported\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature`]
module"]
#[doc(alias = "FEATURE")]
pub type Feature = crate::Reg<feature::FeatureSpec>;
#[doc = "ESC Features Supported"]
pub mod feature;
#[doc = "STATION_ADR (r) register accessor: Configured Station Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`station_adr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@station_adr`]
module"]
#[doc(alias = "STATION_ADR")]
pub type StationAdr = crate::Reg<station_adr::StationAdrSpec>;
#[doc = "Configured Station Address"]
pub mod station_adr;
#[doc = "STATION_ALIAS (rw) register accessor: Configured Station Alias\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`station_alias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`station_alias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@station_alias`]
module"]
#[doc(alias = "STATION_ALIAS")]
pub type StationAlias = crate::Reg<station_alias::StationAliasSpec>;
#[doc = "Configured Station Alias"]
pub mod station_alias;
#[doc = "WR_REG_ENABLE (r) register accessor: Write Register Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_reg_enable::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_reg_enable`]
module"]
#[doc(alias = "WR_REG_ENABLE")]
pub type WrRegEnable = crate::Reg<wr_reg_enable::WrRegEnableSpec>;
#[doc = "Write Register Enable"]
pub mod wr_reg_enable;
#[doc = "WR_REG_PROTECT (r) register accessor: Write Register Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_reg_protect::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_reg_protect`]
module"]
#[doc(alias = "WR_REG_PROTECT")]
pub type WrRegProtect = crate::Reg<wr_reg_protect::WrRegProtectSpec>;
#[doc = "Write Register Protection"]
pub mod wr_reg_protect;
#[doc = "ESC_WR_ENABLE (r) register accessor: ESC Write Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_wr_enable::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_wr_enable`]
module"]
#[doc(alias = "ESC_WR_ENABLE")]
pub type EscWrEnable = crate::Reg<esc_wr_enable::EscWrEnableSpec>;
#[doc = "ESC Write Enable"]
pub mod esc_wr_enable;
#[doc = "ESC_WR_PROTECT (r) register accessor: ESC Write Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_wr_protect::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_wr_protect`]
module"]
#[doc(alias = "ESC_WR_PROTECT")]
pub type EscWrProtect = crate::Reg<esc_wr_protect::EscWrProtectSpec>;
#[doc = "ESC Write Protection"]
pub mod esc_wr_protect;
#[doc = "WRITEMode_ESC_RESET_ECAT (r) register accessor: ESC Reset ECAT \\[WRITE Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writemode_esc_reset_ecat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writemode_esc_reset_ecat`]
module"]
#[doc(alias = "WRITEMode_ESC_RESET_ECAT")]
pub type WritemodeEscResetEcat = crate::Reg<writemode_esc_reset_ecat::WritemodeEscResetEcatSpec>;
#[doc = "ESC Reset ECAT \\[WRITE Mode\\]"]
pub mod writemode_esc_reset_ecat;
#[doc = "READMode_ESC_RESET_ECAT (r) register accessor: ESC Reset ECAT \\[READ Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_esc_reset_ecat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readmode_esc_reset_ecat`]
module"]
#[doc(alias = "READMode_ESC_RESET_ECAT")]
pub type ReadmodeEscResetEcat = crate::Reg<readmode_esc_reset_ecat::ReadmodeEscResetEcatSpec>;
#[doc = "ESC Reset ECAT \\[READ Mode\\]"]
pub mod readmode_esc_reset_ecat;
#[doc = "WRITEMode_ESC_RESET_PDI (r) register accessor: ESC Reset PDI \\[WRITE Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writemode_esc_reset_pdi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writemode_esc_reset_pdi`]
module"]
#[doc(alias = "WRITEMode_ESC_RESET_PDI")]
pub type WritemodeEscResetPdi = crate::Reg<writemode_esc_reset_pdi::WritemodeEscResetPdiSpec>;
#[doc = "ESC Reset PDI \\[WRITE Mode\\]"]
pub mod writemode_esc_reset_pdi;
#[doc = "READMode_ESC_RESET_PDI (r) register accessor: ESC Reset PDI \\[READ Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_esc_reset_pdi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readmode_esc_reset_pdi`]
module"]
#[doc(alias = "READMode_ESC_RESET_PDI")]
pub type ReadmodeEscResetPdi = crate::Reg<readmode_esc_reset_pdi::ReadmodeEscResetPdiSpec>;
#[doc = "ESC Reset PDI \\[READ Mode\\]"]
pub mod readmode_esc_reset_pdi;
#[doc = "ESC_DL_CONTROL (r) register accessor: ESC DL Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_dl_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_dl_control`]
module"]
#[doc(alias = "ESC_DL_CONTROL")]
pub type EscDlControl = crate::Reg<esc_dl_control::EscDlControlSpec>;
#[doc = "ESC DL Control"]
pub mod esc_dl_control;
#[doc = "PHYSICAL_RW_OFFSET (r) register accessor: Physical Read/Write Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_rw_offset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@physical_rw_offset`]
module"]
#[doc(alias = "PHYSICAL_RW_OFFSET")]
pub type PhysicalRwOffset = crate::Reg<physical_rw_offset::PhysicalRwOffsetSpec>;
#[doc = "Physical Read/Write Offset"]
pub mod physical_rw_offset;
#[doc = "ESC_DL_STATUS (r) register accessor: ESC DL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_dl_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_dl_status`]
module"]
#[doc(alias = "ESC_DL_STATUS")]
pub type EscDlStatus = crate::Reg<esc_dl_status::EscDlStatusSpec>;
#[doc = "ESC DL Status"]
pub mod esc_dl_status;
#[doc = "AL_CONTROL (r) register accessor: AL Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_control`]
module"]
#[doc(alias = "AL_CONTROL")]
pub type AlControl = crate::Reg<al_control::AlControlSpec>;
#[doc = "AL Control"]
pub mod al_control;
#[doc = "AL_STATUS (rw) register accessor: AL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_status`]
module"]
#[doc(alias = "AL_STATUS")]
pub type AlStatus = crate::Reg<al_status::AlStatusSpec>;
#[doc = "AL Status"]
pub mod al_status;
#[doc = "AL_STATUS_CODE (rw) register accessor: AL Status Code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_status_code::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_status_code::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_status_code`]
module"]
#[doc(alias = "AL_STATUS_CODE")]
pub type AlStatusCode = crate::Reg<al_status_code::AlStatusCodeSpec>;
#[doc = "AL Status Code"]
pub mod al_status_code;
#[doc = "RUN_LED (rw) register accessor: RUN LED Override\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`run_led::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`run_led::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@run_led`]
module"]
#[doc(alias = "RUN_LED")]
pub type RunLed = crate::Reg<run_led::RunLedSpec>;
#[doc = "RUN LED Override"]
pub mod run_led;
#[doc = "ERR_LED (rw) register accessor: RUN ERR Override\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_led::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_led::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_led`]
module"]
#[doc(alias = "ERR_LED")]
pub type ErrLed = crate::Reg<err_led::ErrLedSpec>;
#[doc = "RUN ERR Override"]
pub mod err_led;
#[doc = "PDI_CONTROL (r) register accessor: PDI Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_control::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_control`]
module"]
#[doc(alias = "PDI_CONTROL")]
pub type PdiControl = crate::Reg<pdi_control::PdiControlSpec>;
#[doc = "PDI Control"]
pub mod pdi_control;
#[doc = "ESC_CONFIG (r) register accessor: ESC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_config::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_config`]
module"]
#[doc(alias = "ESC_CONFIG")]
pub type EscConfig = crate::Reg<esc_config::EscConfigSpec>;
#[doc = "ESC Configuration"]
pub mod esc_config;
#[doc = "PDI_CONFIG (r) register accessor: PDI Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_config::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_config`]
module"]
#[doc(alias = "PDI_CONFIG")]
pub type PdiConfig = crate::Reg<pdi_config::PdiConfigSpec>;
#[doc = "PDI Control"]
pub mod pdi_config;
#[doc = "SYNC_LATCH_CONFIG (r) register accessor: Sync/Latch\\[1:0\\]
PDI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_latch_config::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_latch_config`]
module"]
#[doc(alias = "SYNC_LATCH_CONFIG")]
pub type SyncLatchConfig = crate::Reg<sync_latch_config::SyncLatchConfigSpec>;
#[doc = "Sync/Latch\\[1:0\\]
PDI Configuration"]
pub mod sync_latch_config;
#[doc = "PDI_EXT_CONFIG (r) register accessor: PDI Synchronous Microcontroller extended Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_ext_config::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_ext_config`]
module"]
#[doc(alias = "PDI_EXT_CONFIG")]
pub type PdiExtConfig = crate::Reg<pdi_ext_config::PdiExtConfigSpec>;
#[doc = "PDI Synchronous Microcontroller extended Configuration"]
pub mod pdi_ext_config;
#[doc = "EVENT_MASK (r) register accessor: ECAT Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_mask::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_mask`]
module"]
#[doc(alias = "EVENT_MASK")]
pub type EventMask = crate::Reg<event_mask::EventMaskSpec>;
#[doc = "ECAT Event Mask"]
pub mod event_mask;
#[doc = "AL_EVENT_MASK (rw) register accessor: PDI AL Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_event_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_event_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_event_mask`]
module"]
#[doc(alias = "AL_EVENT_MASK")]
pub type AlEventMask = crate::Reg<al_event_mask::AlEventMaskSpec>;
#[doc = "PDI AL Event Mask"]
pub mod al_event_mask;
#[doc = "EVENT_REQ (r) register accessor: ECAT Event Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_req::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_req`]
module"]
#[doc(alias = "EVENT_REQ")]
pub type EventReq = crate::Reg<event_req::EventReqSpec>;
#[doc = "ECAT Event Request"]
pub mod event_req;
#[doc = "AL_EVENT_REQ (rw) register accessor: AL Event Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`al_event_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`al_event_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@al_event_req`]
module"]
#[doc(alias = "AL_EVENT_REQ")]
pub type AlEventReq = crate::Reg<al_event_req::AlEventReqSpec>;
#[doc = "AL Event Request"]
pub mod al_event_req;
#[doc = "RX_ERR_COUNT0 (r) register accessor: RX Error Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_count0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_count0`]
module"]
#[doc(alias = "RX_ERR_COUNT0")]
pub type RxErrCount0 = crate::Reg<rx_err_count0::RxErrCount0Spec>;
#[doc = "RX Error Counter Port 0"]
pub mod rx_err_count0;
#[doc = "RX_ERR_COUNT1 (r) register accessor: RX Error Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_err_count1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_err_count1`]
module"]
#[doc(alias = "RX_ERR_COUNT1")]
pub type RxErrCount1 = crate::Reg<rx_err_count1::RxErrCount1Spec>;
#[doc = "RX Error Counter Port 1"]
pub mod rx_err_count1;
#[doc = "FWD_RX_ERR_COUNT0 (r) register accessor: Forwarded RX Error Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwd_rx_err_count0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwd_rx_err_count0`]
module"]
#[doc(alias = "FWD_RX_ERR_COUNT0")]
pub type FwdRxErrCount0 = crate::Reg<fwd_rx_err_count0::FwdRxErrCount0Spec>;
#[doc = "Forwarded RX Error Counter Port 0"]
pub mod fwd_rx_err_count0;
#[doc = "FWD_RX_ERR_COUNT1 (r) register accessor: Forwarded RX Error Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwd_rx_err_count1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwd_rx_err_count1`]
module"]
#[doc(alias = "FWD_RX_ERR_COUNT1")]
pub type FwdRxErrCount1 = crate::Reg<fwd_rx_err_count1::FwdRxErrCount1Spec>;
#[doc = "Forwarded RX Error Counter Port 1"]
pub mod fwd_rx_err_count1;
#[doc = "PROC_ERR_COUNT (r) register accessor: ECAT Processing Unit Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`proc_err_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@proc_err_count`]
module"]
#[doc(alias = "PROC_ERR_COUNT")]
pub type ProcErrCount = crate::Reg<proc_err_count::ProcErrCountSpec>;
#[doc = "ECAT Processing Unit Error Counter"]
pub mod proc_err_count;
#[doc = "PDI_ERR_COUNT (r) register accessor: PDI Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdi_err_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdi_err_count`]
module"]
#[doc(alias = "PDI_ERR_COUNT")]
pub type PdiErrCount = crate::Reg<pdi_err_count::PdiErrCountSpec>;
#[doc = "PDI Error Counter"]
pub mod pdi_err_count;
#[doc = "LOST_LINK_COUNT0 (r) register accessor: Lost Link Counter Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lost_link_count0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lost_link_count0`]
module"]
#[doc(alias = "LOST_LINK_COUNT0")]
pub type LostLinkCount0 = crate::Reg<lost_link_count0::LostLinkCount0Spec>;
#[doc = "Lost Link Counter Port 0"]
pub mod lost_link_count0;
#[doc = "LOST_LINK_COUNT1 (r) register accessor: Lost Link Counter Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lost_link_count1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lost_link_count1`]
module"]
#[doc(alias = "LOST_LINK_COUNT1")]
pub type LostLinkCount1 = crate::Reg<lost_link_count1::LostLinkCount1Spec>;
#[doc = "Lost Link Counter Port 1"]
pub mod lost_link_count1;
#[doc = "WD_DIVIDE (rw) register accessor: Watchdog Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_divide::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_divide::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_divide`]
module"]
#[doc(alias = "WD_DIVIDE")]
pub type WdDivide = crate::Reg<wd_divide::WdDivideSpec>;
#[doc = "Watchdog Divider"]
pub mod wd_divide;
#[doc = "WD_TIME_PDI (rw) register accessor: Watchdog Time PDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_time_pdi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_time_pdi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_time_pdi`]
module"]
#[doc(alias = "WD_TIME_PDI")]
pub type WdTimePdi = crate::Reg<wd_time_pdi::WdTimePdiSpec>;
#[doc = "Watchdog Time PDI"]
pub mod wd_time_pdi;
#[doc = "WD_TIME_PDATA (rw) register accessor: Watchdog Time Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_time_pdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd_time_pdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_time_pdata`]
module"]
#[doc(alias = "WD_TIME_PDATA")]
pub type WdTimePdata = crate::Reg<wd_time_pdata::WdTimePdataSpec>;
#[doc = "Watchdog Time Process Data"]
pub mod wd_time_pdata;
#[doc = "WD_STAT_PDATA (r) register accessor: Watchdog Status Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_stat_pdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_stat_pdata`]
module"]
#[doc(alias = "WD_STAT_PDATA")]
pub type WdStatPdata = crate::Reg<wd_stat_pdata::WdStatPdataSpec>;
#[doc = "Watchdog Status Process Data"]
pub mod wd_stat_pdata;
#[doc = "WD_COUNT_PDATA (r) register accessor: Watchdog Counter Process Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_count_pdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_count_pdata`]
module"]
#[doc(alias = "WD_COUNT_PDATA")]
pub type WdCountPdata = crate::Reg<wd_count_pdata::WdCountPdataSpec>;
#[doc = "Watchdog Counter Process Data"]
pub mod wd_count_pdata;
#[doc = "WD_COUNT_PDI (r) register accessor: Watchdog Counter PDI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd_count_pdi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd_count_pdi`]
module"]
#[doc(alias = "WD_COUNT_PDI")]
pub type WdCountPdi = crate::Reg<wd_count_pdi::WdCountPdiSpec>;
#[doc = "Watchdog Counter PDI"]
pub mod wd_count_pdi;
#[doc = "EEP_CONF (r) register accessor: EEPROM Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_conf`]
module"]
#[doc(alias = "EEP_CONF")]
pub type EepConf = crate::Reg<eep_conf::EepConfSpec>;
#[doc = "EEPROM Configuration"]
pub mod eep_conf;
#[doc = "EEP_STATE (rw) register accessor: EEPROM PDI Access State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_state`]
module"]
#[doc(alias = "EEP_STATE")]
pub type EepState = crate::Reg<eep_state::EepStateSpec>;
#[doc = "EEPROM PDI Access State"]
pub mod eep_state;
#[doc = "EEP_CONT_STAT (rw) register accessor: EEPROM Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_cont_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_cont_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_cont_stat`]
module"]
#[doc(alias = "EEP_CONT_STAT")]
pub type EepContStat = crate::Reg<eep_cont_stat::EepContStatSpec>;
#[doc = "EEPROM Control/Status"]
pub mod eep_cont_stat;
#[doc = "EEP_ADR (rw) register accessor: EEPROM Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_adr`]
module"]
#[doc(alias = "EEP_ADR")]
pub type EepAdr = crate::Reg<eep_adr::EepAdrSpec>;
#[doc = "EEPROM Address"]
pub mod eep_adr;
#[doc = "EEP_DATA (rw) register accessor: EEPROM Read/Write data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eep_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eep_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep_data`]
module"]
#[doc(alias = "EEP_DATA")]
pub type EepData = crate::Reg<eep_data::EepDataSpec>;
#[doc = "EEPROM Read/Write data"]
pub mod eep_data;
#[doc = "MII_CONT_STAT (rw) register accessor: MII Management Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_cont_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_cont_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_cont_stat`]
module"]
#[doc(alias = "MII_CONT_STAT")]
pub type MiiContStat = crate::Reg<mii_cont_stat::MiiContStatSpec>;
#[doc = "MII Management Control/Status"]
pub mod mii_cont_stat;
#[doc = "MII_PHY_ADR (rw) register accessor: PHY Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_phy_adr`]
module"]
#[doc(alias = "MII_PHY_ADR")]
pub type MiiPhyAdr = crate::Reg<mii_phy_adr::MiiPhyAdrSpec>;
#[doc = "PHY Address"]
pub mod mii_phy_adr;
#[doc = "MII_PHY_REG_ADR (rw) register accessor: PHY Register Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_reg_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_reg_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_phy_reg_adr`]
module"]
#[doc(alias = "MII_PHY_REG_ADR")]
pub type MiiPhyRegAdr = crate::Reg<mii_phy_reg_adr::MiiPhyRegAdrSpec>;
#[doc = "PHY Register Address"]
pub mod mii_phy_reg_adr;
#[doc = "MII_PHY_DATA (rw) register accessor: PHY Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_phy_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_phy_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_phy_data`]
module"]
#[doc(alias = "MII_PHY_DATA")]
pub type MiiPhyData = crate::Reg<mii_phy_data::MiiPhyDataSpec>;
#[doc = "PHY Data"]
pub mod mii_phy_data;
#[doc = "MII_ECAT_ACS_STATE (r) register accessor: MII ECAT ACS STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_ecat_acs_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_ecat_acs_state`]
module"]
#[doc(alias = "MII_ECAT_ACS_STATE")]
pub type MiiEcatAcsState = crate::Reg<mii_ecat_acs_state::MiiEcatAcsStateSpec>;
#[doc = "MII ECAT ACS STATE"]
pub mod mii_ecat_acs_state;
#[doc = "MII_PDI_ACS_STATE (rw) register accessor: MII PDI ACS STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_pdi_acs_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_pdi_acs_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mii_pdi_acs_state`]
module"]
#[doc(alias = "MII_PDI_ACS_STATE")]
pub type MiiPdiAcsState = crate::Reg<mii_pdi_acs_state::MiiPdiAcsStateSpec>;
#[doc = "MII PDI ACS STATE"]
pub mod mii_pdi_acs_state;
#[doc = "DC_RCV_TIME_PORT0 (r) register accessor: Receive Time Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_rcv_time_port0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_rcv_time_port0`]
module"]
#[doc(alias = "DC_RCV_TIME_PORT0")]
pub type DcRcvTimePort0 = crate::Reg<dc_rcv_time_port0::DcRcvTimePort0Spec>;
#[doc = "Receive Time Port 0"]
pub mod dc_rcv_time_port0;
#[doc = "DC_RCV_TIME_PORT1 (r) register accessor: Receive Time Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_rcv_time_port1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_rcv_time_port1`]
module"]
#[doc(alias = "DC_RCV_TIME_PORT1")]
pub type DcRcvTimePort1 = crate::Reg<dc_rcv_time_port1::DcRcvTimePort1Spec>;
#[doc = "Receive Time Port 1"]
pub mod dc_rcv_time_port1;
#[doc = "RECEIVE_TIME_PU (r) register accessor: Local time of the beginning of a frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_time_pu::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_time_pu`]
module"]
#[doc(alias = "RECEIVE_TIME_PU")]
pub type ReceiveTimePu = crate::Reg<receive_time_pu::ReceiveTimePuSpec>;
#[doc = "Local time of the beginning of a frame"]
pub mod receive_time_pu;
#[doc = "READMode_DC_SYS_TIME (r) register accessor: System Time read access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_dc_sys_time::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readmode_dc_sys_time`]
module"]
#[doc(alias = "READMode_DC_SYS_TIME")]
pub type ReadmodeDcSysTime = crate::Reg<readmode_dc_sys_time::ReadmodeDcSysTimeSpec>;
#[doc = "System Time read access"]
pub mod readmode_dc_sys_time;
#[doc = "WRITEMode_DC_SYS_TIME (w) register accessor: System Time \\[WRITE Mode\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writemode_dc_sys_time::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writemode_dc_sys_time`]
module"]
#[doc(alias = "WRITEMode_DC_SYS_TIME")]
pub type WritemodeDcSysTime = crate::Reg<writemode_dc_sys_time::WritemodeDcSysTimeSpec>;
#[doc = "System Time \\[WRITE Mode\\]"]
pub mod writemode_dc_sys_time;
#[doc = "DC_SYS_TIME_OFFSET (rw) register accessor: Difference between local time and System Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sys_time_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_offset`]
module"]
#[doc(alias = "DC_SYS_TIME_OFFSET")]
pub type DcSysTimeOffset = crate::Reg<dc_sys_time_offset::DcSysTimeOffsetSpec>;
#[doc = "Difference between local time and System Time"]
pub mod dc_sys_time_offset;
#[doc = "DC_SYS_TIME_DELAY (rw) register accessor: System Time Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sys_time_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_delay`]
module"]
#[doc(alias = "DC_SYS_TIME_DELAY")]
pub type DcSysTimeDelay = crate::Reg<dc_sys_time_delay::DcSysTimeDelaySpec>;
#[doc = "System Time Delay"]
pub mod dc_sys_time_delay;
#[doc = "DC_SYS_TIME_DIFF (r) register accessor: System Time Difference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_diff::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_diff`]
module"]
#[doc(alias = "DC_SYS_TIME_DIFF")]
pub type DcSysTimeDiff = crate::Reg<dc_sys_time_diff::DcSysTimeDiffSpec>;
#[doc = "System Time Difference"]
pub mod dc_sys_time_diff;
#[doc = "DC_SPEED_COUNT_START (rw) register accessor: Speed Counter Start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_speed_count_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_speed_count_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_speed_count_start`]
module"]
#[doc(alias = "DC_SPEED_COUNT_START")]
pub type DcSpeedCountStart = crate::Reg<dc_speed_count_start::DcSpeedCountStartSpec>;
#[doc = "Speed Counter Start"]
pub mod dc_speed_count_start;
#[doc = "DC_SPEED_COUNT_DIFF (r) register accessor: Speed Counter Diff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_speed_count_diff::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_speed_count_diff`]
module"]
#[doc(alias = "DC_SPEED_COUNT_DIFF")]
pub type DcSpeedCountDiff = crate::Reg<dc_speed_count_diff::DcSpeedCountDiffSpec>;
#[doc = "Speed Counter Diff"]
pub mod dc_speed_count_diff;
#[doc = "DC_SYS_TIME_FIL_DEPTH (rw) register accessor: System Time Difference Filter Depth\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sys_time_fil_depth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sys_time_fil_depth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sys_time_fil_depth`]
module"]
#[doc(alias = "DC_SYS_TIME_FIL_DEPTH")]
pub type DcSysTimeFilDepth = crate::Reg<dc_sys_time_fil_depth::DcSysTimeFilDepthSpec>;
#[doc = "System Time Difference Filter Depth"]
pub mod dc_sys_time_fil_depth;
#[doc = "DC_SPEED_COUNT_FIL_DEPTH (rw) register accessor: Speed Counter Filter Depth\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_speed_count_fil_depth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_speed_count_fil_depth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_speed_count_fil_depth`]
module"]
#[doc(alias = "DC_SPEED_COUNT_FIL_DEPTH")]
pub type DcSpeedCountFilDepth = crate::Reg<dc_speed_count_fil_depth::DcSpeedCountFilDepthSpec>;
#[doc = "Speed Counter Filter Depth"]
pub mod dc_speed_count_fil_depth;
#[doc = "DC_CYC_CONT (r) register accessor: Cyclic Unit Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_cyc_cont::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_cyc_cont`]
module"]
#[doc(alias = "DC_CYC_CONT")]
pub type DcCycCont = crate::Reg<dc_cyc_cont::DcCycContSpec>;
#[doc = "Cyclic Unit Control"]
pub mod dc_cyc_cont;
#[doc = "DC_ACT (rw) register accessor: Activation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_act::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_act::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_act`]
module"]
#[doc(alias = "DC_ACT")]
pub type DcAct = crate::Reg<dc_act::DcActSpec>;
#[doc = "Activation register"]
pub mod dc_act;
#[doc = "DC_PULSE_LEN (r) register accessor: Pulse Length of SyncSignals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pulse_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_pulse_len`]
module"]
#[doc(alias = "DC_PULSE_LEN")]
pub type DcPulseLen = crate::Reg<dc_pulse_len::DcPulseLenSpec>;
#[doc = "Pulse Length of SyncSignals"]
pub mod dc_pulse_len;
#[doc = "DC_ACT_STAT (r) register accessor: Activation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_act_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_act_stat`]
module"]
#[doc(alias = "DC_ACT_STAT")]
pub type DcActStat = crate::Reg<dc_act_stat::DcActStatSpec>;
#[doc = "Activation Status"]
pub mod dc_act_stat;
#[doc = "DC_SYNC0_STAT (r) register accessor: SYNC0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync0_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync0_stat`]
module"]
#[doc(alias = "DC_SYNC0_STAT")]
pub type DcSync0Stat = crate::Reg<dc_sync0_stat::DcSync0StatSpec>;
#[doc = "SYNC0 Status"]
pub mod dc_sync0_stat;
#[doc = "DC_SYNC1_STAT (r) register accessor: SYNC1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync1_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync1_stat`]
module"]
#[doc(alias = "DC_SYNC1_STAT")]
pub type DcSync1Stat = crate::Reg<dc_sync1_stat::DcSync1StatSpec>;
#[doc = "SYNC1 Status"]
pub mod dc_sync1_stat;
#[doc = "DC_CYC_START_TIME (rw) register accessor: Start Time Cyclic Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_cyc_start_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_cyc_start_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_cyc_start_time`]
module"]
#[doc(alias = "DC_CYC_START_TIME")]
pub type DcCycStartTime = crate::Reg<dc_cyc_start_time::DcCycStartTimeSpec>;
#[doc = "Start Time Cyclic Operation"]
pub mod dc_cyc_start_time;
#[doc = "DC_NEXT_SYNC1_PULSE (r) register accessor: System time of next SYNC1 pulse in ns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_next_sync1_pulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_next_sync1_pulse`]
module"]
#[doc(alias = "DC_NEXT_SYNC1_PULSE")]
pub type DcNextSync1Pulse = crate::Reg<dc_next_sync1_pulse::DcNextSync1PulseSpec>;
#[doc = "System time of next SYNC1 pulse in ns"]
pub mod dc_next_sync1_pulse;
#[doc = "DC_SYNC0_CYC_TIME (rw) register accessor: SYNC0 Cycle Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync0_cyc_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sync0_cyc_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync0_cyc_time`]
module"]
#[doc(alias = "DC_SYNC0_CYC_TIME")]
pub type DcSync0CycTime = crate::Reg<dc_sync0_cyc_time::DcSync0CycTimeSpec>;
#[doc = "SYNC0 Cycle Time"]
pub mod dc_sync0_cyc_time;
#[doc = "DC_SYNC1_CYC_TIME (rw) register accessor: SYNC1 Cycle Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_sync1_cyc_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_sync1_cyc_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_sync1_cyc_time`]
module"]
#[doc(alias = "DC_SYNC1_CYC_TIME")]
pub type DcSync1CycTime = crate::Reg<dc_sync1_cyc_time::DcSync1CycTimeSpec>;
#[doc = "SYNC1 Cycle Time"]
pub mod dc_sync1_cyc_time;
#[doc = "DC_LATCH0_CONT (rw) register accessor: Latch0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_cont::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_latch0_cont::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_cont`]
module"]
#[doc(alias = "DC_LATCH0_CONT")]
pub type DcLatch0Cont = crate::Reg<dc_latch0_cont::DcLatch0ContSpec>;
#[doc = "Latch0 Control"]
pub mod dc_latch0_cont;
#[doc = "DC_LATCH1_CONT (rw) register accessor: Latch1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_cont::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_latch1_cont::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_cont`]
module"]
#[doc(alias = "DC_LATCH1_CONT")]
pub type DcLatch1Cont = crate::Reg<dc_latch1_cont::DcLatch1ContSpec>;
#[doc = "Latch1 Control"]
pub mod dc_latch1_cont;
#[doc = "DC_LATCH0_STAT (r) register accessor: Latch0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_stat`]
module"]
#[doc(alias = "DC_LATCH0_STAT")]
pub type DcLatch0Stat = crate::Reg<dc_latch0_stat::DcLatch0StatSpec>;
#[doc = "Latch0 Status"]
pub mod dc_latch0_stat;
#[doc = "DC_LATCH1_STAT (r) register accessor: Latch1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_stat`]
module"]
#[doc(alias = "DC_LATCH1_STAT")]
pub type DcLatch1Stat = crate::Reg<dc_latch1_stat::DcLatch1StatSpec>;
#[doc = "Latch1 Status"]
pub mod dc_latch1_stat;
#[doc = "DC_LATCH0_TIME_POS (r) register accessor: Register captures System time at the positive edge of the Latch0 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_time_pos::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_time_pos`]
module"]
#[doc(alias = "DC_LATCH0_TIME_POS")]
pub type DcLatch0TimePos = crate::Reg<dc_latch0_time_pos::DcLatch0TimePosSpec>;
#[doc = "Register captures System time at the positive edge of the Latch0 signal"]
pub mod dc_latch0_time_pos;
#[doc = "DC_LATCH0_TIME_NEG (r) register accessor: Register captures System time at the negative edge of the Latch0 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch0_time_neg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch0_time_neg`]
module"]
#[doc(alias = "DC_LATCH0_TIME_NEG")]
pub type DcLatch0TimeNeg = crate::Reg<dc_latch0_time_neg::DcLatch0TimeNegSpec>;
#[doc = "Register captures System time at the negative edge of the Latch0 signal"]
pub mod dc_latch0_time_neg;
#[doc = "DC_LATCH1_TIME_POS (r) register accessor: Register captures System time at the positive edge of the Latch1 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_time_pos::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_time_pos`]
module"]
#[doc(alias = "DC_LATCH1_TIME_POS")]
pub type DcLatch1TimePos = crate::Reg<dc_latch1_time_pos::DcLatch1TimePosSpec>;
#[doc = "Register captures System time at the positive edge of the Latch1 signal"]
pub mod dc_latch1_time_pos;
#[doc = "DC_LATCH1_TIME_NEG (r) register accessor: Register captures System time at the negative edge of the Latch1 signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_latch1_time_neg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_latch1_time_neg`]
module"]
#[doc(alias = "DC_LATCH1_TIME_NEG")]
pub type DcLatch1TimeNeg = crate::Reg<dc_latch1_time_neg::DcLatch1TimeNegSpec>;
#[doc = "Register captures System time at the negative edge of the Latch1 signal"]
pub mod dc_latch1_time_neg;
#[doc = "DC_ECAT_CNG_EV_TIME (r) register accessor: EtherCAT Buffer Change Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_ecat_cng_ev_time::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_ecat_cng_ev_time`]
module"]
#[doc(alias = "DC_ECAT_CNG_EV_TIME")]
pub type DcEcatCngEvTime = crate::Reg<dc_ecat_cng_ev_time::DcEcatCngEvTimeSpec>;
#[doc = "EtherCAT Buffer Change Event Time"]
pub mod dc_ecat_cng_ev_time;
#[doc = "DC_PDI_START_EV_TIME (r) register accessor: PDI Buffer Start Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pdi_start_ev_time::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_pdi_start_ev_time`]
module"]
#[doc(alias = "DC_PDI_START_EV_TIME")]
pub type DcPdiStartEvTime = crate::Reg<dc_pdi_start_ev_time::DcPdiStartEvTimeSpec>;
#[doc = "PDI Buffer Start Event Time"]
pub mod dc_pdi_start_ev_time;
#[doc = "DC_PDI_CNG_EV_TIME (r) register accessor: PDI Buffer Change Event Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_pdi_cng_ev_time::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_pdi_cng_ev_time`]
module"]
#[doc(alias = "DC_PDI_CNG_EV_TIME")]
pub type DcPdiCngEvTime = crate::Reg<dc_pdi_cng_ev_time::DcPdiCngEvTimeSpec>;
#[doc = "PDI Buffer Change Event Time"]
pub mod dc_pdi_cng_ev_time;
#[doc = "ID (r) register accessor: ECAT0 Module ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "ECAT0 Module ID"]
pub mod id;
#[doc = "STATUS (r) register accessor: ECAT0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "ECAT0 Status"]
pub mod status;
