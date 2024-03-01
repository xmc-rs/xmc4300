#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mac_configuration: MacConfiguration,
    mac_frame_filter: MacFrameFilter,
    hash_table_high: HashTableHigh,
    hash_table_low: HashTableLow,
    gmii_address: GmiiAddress,
    gmii_data: GmiiData,
    flow_control: FlowControl,
    vlan_tag: VlanTag,
    version: Version,
    debug: Debug,
    remote_wake_up_frame_filter: RemoteWakeUpFrameFilter,
    pmt_control_status: PmtControlStatus,
    _reserved12: [u8; 0x08],
    interrupt_status: InterruptStatus,
    interrupt_mask: InterruptMask,
    mac_address0_high: MacAddress0High,
    mac_address0_low: MacAddress0Low,
    mac_address1_high: MacAddress1High,
    mac_address1_low: MacAddress1Low,
    mac_address2_high: MacAddress2High,
    mac_address2_low: MacAddress2Low,
    mac_address3_high: MacAddress3High,
    mac_address3_low: MacAddress3Low,
    _reserved22: [u8; 0xa0],
    mmc_control: MmcControl,
    mmc_receive_interrupt: MmcReceiveInterrupt,
    mmc_transmit_interrupt: MmcTransmitInterrupt,
    mmc_receive_interrupt_mask: MmcReceiveInterruptMask,
    mmc_transmit_interrupt_mask: MmcTransmitInterruptMask,
    tx_octet_count_good_bad: TxOctetCountGoodBad,
    tx_frame_count_good_bad: TxFrameCountGoodBad,
    tx_broadcast_frames_good: TxBroadcastFramesGood,
    tx_multicast_frames_good: TxMulticastFramesGood,
    tx_64octets_frames_good_bad: Tx64octetsFramesGoodBad,
    tx_65to127octets_frames_good_bad: Tx65to127octetsFramesGoodBad,
    tx_128to255octets_frames_good_bad: Tx128to255octetsFramesGoodBad,
    tx_256to511octets_frames_good_bad: Tx256to511octetsFramesGoodBad,
    tx_512to1023octets_frames_good_bad: Tx512to1023octetsFramesGoodBad,
    tx_1024tomaxoctets_frames_good_bad: Tx1024tomaxoctetsFramesGoodBad,
    tx_unicast_frames_good_bad: TxUnicastFramesGoodBad,
    tx_multicast_frames_good_bad: TxMulticastFramesGoodBad,
    tx_broadcast_frames_good_bad: TxBroadcastFramesGoodBad,
    tx_underflow_error_frames: TxUnderflowErrorFrames,
    tx_single_collision_good_frames: TxSingleCollisionGoodFrames,
    tx_multiple_collision_good_frames: TxMultipleCollisionGoodFrames,
    tx_deferred_frames: TxDeferredFrames,
    tx_late_collision_frames: TxLateCollisionFrames,
    tx_excessive_collision_frames: TxExcessiveCollisionFrames,
    tx_carrier_error_frames: TxCarrierErrorFrames,
    tx_octet_count_good: TxOctetCountGood,
    tx_frame_count_good: TxFrameCountGood,
    tx_excessive_deferral_error: TxExcessiveDeferralError,
    tx_pause_frames: TxPauseFrames,
    tx_vlan_frames_good: TxVlanFramesGood,
    tx_osize_frames_good: TxOsizeFramesGood,
    _reserved53: [u8; 0x04],
    rx_frames_count_good_bad: RxFramesCountGoodBad,
    rx_octet_count_good_bad: RxOctetCountGoodBad,
    rx_octet_count_good: RxOctetCountGood,
    rx_broadcast_frames_good: RxBroadcastFramesGood,
    rx_multicast_frames_good: RxMulticastFramesGood,
    rx_crc_error_frames: RxCrcErrorFrames,
    rx_alignment_error_frames: RxAlignmentErrorFrames,
    rx_runt_error_frames: RxRuntErrorFrames,
    rx_jabber_error_frames: RxJabberErrorFrames,
    rx_undersize_frames_good: RxUndersizeFramesGood,
    rx_oversize_frames_good: RxOversizeFramesGood,
    rx_64octets_frames_good_bad: Rx64octetsFramesGoodBad,
    rx_65to127octets_frames_good_bad: Rx65to127octetsFramesGoodBad,
    rx_128to255octets_frames_good_bad: Rx128to255octetsFramesGoodBad,
    rx_256to511octets_frames_good_bad: Rx256to511octetsFramesGoodBad,
    rx_512to1023octets_frames_good_bad: Rx512to1023octetsFramesGoodBad,
    rx_1024tomaxoctets_frames_good_bad: Rx1024tomaxoctetsFramesGoodBad,
    rx_unicast_frames_good: RxUnicastFramesGood,
    rx_length_error_frames: RxLengthErrorFrames,
    rx_out_of_range_type_frames: RxOutOfRangeTypeFrames,
    rx_pause_frames: RxPauseFrames,
    rx_fifo_overflow_frames: RxFifoOverflowFrames,
    rx_vlan_frames_good_bad: RxVlanFramesGoodBad,
    rx_watchdog_error_frames: RxWatchdogErrorFrames,
    rx_receive_error_frames: RxReceiveErrorFrames,
    rx_control_frames_good: RxControlFramesGood,
    _reserved79: [u8; 0x18],
    mmc_ipc_receive_interrupt_mask: MmcIpcReceiveInterruptMask,
    _reserved80: [u8; 0x04],
    mmc_ipc_receive_interrupt: MmcIpcReceiveInterrupt,
    _reserved81: [u8; 0x04],
    rxipv4_good_frames: Rxipv4GoodFrames,
    rxipv4_header_error_frames: Rxipv4HeaderErrorFrames,
    rxipv4_no_payload_frames: Rxipv4NoPayloadFrames,
    rxipv4_fragmented_frames: Rxipv4FragmentedFrames,
    rxipv4_udp_checksum_disabled_frames: Rxipv4UdpChecksumDisabledFrames,
    rxipv6_good_frames: Rxipv6GoodFrames,
    rxipv6_header_error_frames: Rxipv6HeaderErrorFrames,
    rxipv6_no_payload_frames: Rxipv6NoPayloadFrames,
    rxudp_good_frames: RxudpGoodFrames,
    rxudp_error_frames: RxudpErrorFrames,
    rxtcp_good_frames: RxtcpGoodFrames,
    rxtcp_error_frames: RxtcpErrorFrames,
    rxicmp_good_frames: RxicmpGoodFrames,
    rxicmp_error_frames: RxicmpErrorFrames,
    _reserved95: [u8; 0x08],
    rxipv4_good_octets: Rxipv4GoodOctets,
    rxipv4_header_error_octets: Rxipv4HeaderErrorOctets,
    rxipv4_no_payload_octets: Rxipv4NoPayloadOctets,
    rxipv4_fragmented_octets: Rxipv4FragmentedOctets,
    rxipv4_udp_checksum_disable_octets: Rxipv4UdpChecksumDisableOctets,
    rxipv6_good_octets: Rxipv6GoodOctets,
    rxipv6_header_error_octets: Rxipv6HeaderErrorOctets,
    rxipv6_no_payload_octets: Rxipv6NoPayloadOctets,
    rxudp_good_octets: RxudpGoodOctets,
    rxudp_error_octets: RxudpErrorOctets,
    rxtcp_good_octets: RxtcpGoodOctets,
    rxtcp_error_octets: RxtcpErrorOctets,
    rxicmp_good_octets: RxicmpGoodOctets,
    rxicmp_error_octets: RxicmpErrorOctets,
    _reserved109: [u8; 0x0478],
    timestamp_control: TimestampControl,
    sub_second_increment: SubSecondIncrement,
    system_time_seconds: SystemTimeSeconds,
    system_time_nanoseconds: SystemTimeNanoseconds,
    system_time_seconds_update: SystemTimeSecondsUpdate,
    system_time_nanoseconds_update: SystemTimeNanosecondsUpdate,
    timestamp_addend: TimestampAddend,
    target_time_seconds: TargetTimeSeconds,
    target_time_nanoseconds: TargetTimeNanoseconds,
    system_time_higher_word_seconds: SystemTimeHigherWordSeconds,
    timestamp_status: TimestampStatus,
    _reserved120: [u8; 0x08d4],
    bus_mode: BusMode,
    transmit_poll_demand: TransmitPollDemand,
    receive_poll_demand: ReceivePollDemand,
    receive_descriptor_list_address: ReceiveDescriptorListAddress,
    transmit_descriptor_list_address: TransmitDescriptorListAddress,
    status: Status,
    operation_mode: OperationMode,
    interrupt_enable: InterruptEnable,
    missed_frame_and_buffer_overflow_counter: MissedFrameAndBufferOverflowCounter,
    receive_interrupt_watchdog_timer: ReceiveInterruptWatchdogTimer,
    _reserved130: [u8; 0x04],
    ahb_status: AhbStatus,
    _reserved131: [u8; 0x18],
    current_host_transmit_descriptor: CurrentHostTransmitDescriptor,
    current_host_receive_descriptor: CurrentHostReceiveDescriptor,
    current_host_transmit_buffer_address: CurrentHostTransmitBufferAddress,
    current_host_receive_buffer_address: CurrentHostReceiveBufferAddress,
    hw_feature: HwFeature,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    #[inline(always)]
    pub const fn mac_configuration(&self) -> &MacConfiguration {
        &self.mac_configuration
    }
    #[doc = "0x04 - MAC Frame Filter"]
    #[inline(always)]
    pub const fn mac_frame_filter(&self) -> &MacFrameFilter {
        &self.mac_frame_filter
    }
    #[doc = "0x08 - Hash Table High Register"]
    #[inline(always)]
    pub const fn hash_table_high(&self) -> &HashTableHigh {
        &self.hash_table_high
    }
    #[doc = "0x0c - Hash Table Low Register"]
    #[inline(always)]
    pub const fn hash_table_low(&self) -> &HashTableLow {
        &self.hash_table_low
    }
    #[doc = "0x10 - MII Address Register"]
    #[inline(always)]
    pub const fn gmii_address(&self) -> &GmiiAddress {
        &self.gmii_address
    }
    #[doc = "0x14 - MII Data Register"]
    #[inline(always)]
    pub const fn gmii_data(&self) -> &GmiiData {
        &self.gmii_data
    }
    #[doc = "0x18 - Flow Control Register"]
    #[inline(always)]
    pub const fn flow_control(&self) -> &FlowControl {
        &self.flow_control
    }
    #[doc = "0x1c - VLAN Tag Register"]
    #[inline(always)]
    pub const fn vlan_tag(&self) -> &VlanTag {
        &self.vlan_tag
    }
    #[doc = "0x20 - Version Register"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x24 - Debug Register"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x28 - Remote Wake Up Frame Filter Register"]
    #[inline(always)]
    pub const fn remote_wake_up_frame_filter(&self) -> &RemoteWakeUpFrameFilter {
        &self.remote_wake_up_frame_filter
    }
    #[doc = "0x2c - PMT Control and Status Register"]
    #[inline(always)]
    pub const fn pmt_control_status(&self) -> &PmtControlStatus {
        &self.pmt_control_status
    }
    #[doc = "0x38 - Interrupt Register"]
    #[inline(always)]
    pub const fn interrupt_status(&self) -> &InterruptStatus {
        &self.interrupt_status
    }
    #[doc = "0x3c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &InterruptMask {
        &self.interrupt_mask
    }
    #[doc = "0x40 - MAC Address0 High Register"]
    #[inline(always)]
    pub const fn mac_address0_high(&self) -> &MacAddress0High {
        &self.mac_address0_high
    }
    #[doc = "0x44 - MAC Address0 Low Register"]
    #[inline(always)]
    pub const fn mac_address0_low(&self) -> &MacAddress0Low {
        &self.mac_address0_low
    }
    #[doc = "0x48 - MAC Address1 High Register"]
    #[inline(always)]
    pub const fn mac_address1_high(&self) -> &MacAddress1High {
        &self.mac_address1_high
    }
    #[doc = "0x4c - MAC Address1 Low Register"]
    #[inline(always)]
    pub const fn mac_address1_low(&self) -> &MacAddress1Low {
        &self.mac_address1_low
    }
    #[doc = "0x50 - MAC Address2 High Register"]
    #[inline(always)]
    pub const fn mac_address2_high(&self) -> &MacAddress2High {
        &self.mac_address2_high
    }
    #[doc = "0x54 - MAC Address2 Low Register"]
    #[inline(always)]
    pub const fn mac_address2_low(&self) -> &MacAddress2Low {
        &self.mac_address2_low
    }
    #[doc = "0x58 - MAC Address3 High Register"]
    #[inline(always)]
    pub const fn mac_address3_high(&self) -> &MacAddress3High {
        &self.mac_address3_high
    }
    #[doc = "0x5c - MAC Address3 Low Register"]
    #[inline(always)]
    pub const fn mac_address3_low(&self) -> &MacAddress3Low {
        &self.mac_address3_low
    }
    #[doc = "0x100 - MMC Control Register"]
    #[inline(always)]
    pub const fn mmc_control(&self) -> &MmcControl {
        &self.mmc_control
    }
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_receive_interrupt(&self) -> &MmcReceiveInterrupt {
        &self.mmc_receive_interrupt
    }
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_transmit_interrupt(&self) -> &MmcTransmitInterrupt {
        &self.mmc_transmit_interrupt
    }
    #[doc = "0x10c - MMC Reveive Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_receive_interrupt_mask(&self) -> &MmcReceiveInterruptMask {
        &self.mmc_receive_interrupt_mask
    }
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_transmit_interrupt_mask(&self) -> &MmcTransmitInterruptMask {
        &self.mmc_transmit_interrupt_mask
    }
    #[doc = "0x114 - Transmit Octet Count for Good and Bad Frames Register"]
    #[inline(always)]
    pub const fn tx_octet_count_good_bad(&self) -> &TxOctetCountGoodBad {
        &self.tx_octet_count_good_bad
    }
    #[doc = "0x118 - Transmit Frame Count for Goodand Bad Frames Register"]
    #[inline(always)]
    pub const fn tx_frame_count_good_bad(&self) -> &TxFrameCountGoodBad {
        &self.tx_frame_count_good_bad
    }
    #[doc = "0x11c - Transmit Frame Count for Good Broadcast Frames"]
    #[inline(always)]
    pub const fn tx_broadcast_frames_good(&self) -> &TxBroadcastFramesGood {
        &self.tx_broadcast_frames_good
    }
    #[doc = "0x120 - Transmit Frame Count for Good Multicast Frames"]
    #[inline(always)]
    pub const fn tx_multicast_frames_good(&self) -> &TxMulticastFramesGood {
        &self.tx_multicast_frames_good
    }
    #[doc = "0x124 - Transmit Octet Count for Good and Bad 64 Byte Frames"]
    #[inline(always)]
    pub const fn tx_64octets_frames_good_bad(&self) -> &Tx64octetsFramesGoodBad {
        &self.tx_64octets_frames_good_bad
    }
    #[doc = "0x128 - Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_65to127octets_frames_good_bad(&self) -> &Tx65to127octetsFramesGoodBad {
        &self.tx_65to127octets_frames_good_bad
    }
    #[doc = "0x12c - Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_128to255octets_frames_good_bad(&self) -> &Tx128to255octetsFramesGoodBad {
        &self.tx_128to255octets_frames_good_bad
    }
    #[doc = "0x130 - Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_256to511octets_frames_good_bad(&self) -> &Tx256to511octetsFramesGoodBad {
        &self.tx_256to511octets_frames_good_bad
    }
    #[doc = "0x134 - Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_512to1023octets_frames_good_bad(&self) -> &Tx512to1023octetsFramesGoodBad {
        &self.tx_512to1023octets_frames_good_bad
    }
    #[doc = "0x138 - Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
    #[inline(always)]
    pub const fn tx_1024tomaxoctets_frames_good_bad(&self) -> &Tx1024tomaxoctetsFramesGoodBad {
        &self.tx_1024tomaxoctets_frames_good_bad
    }
    #[doc = "0x13c - Transmit Frame Count for Good and Bad Unicast Frames"]
    #[inline(always)]
    pub const fn tx_unicast_frames_good_bad(&self) -> &TxUnicastFramesGoodBad {
        &self.tx_unicast_frames_good_bad
    }
    #[doc = "0x140 - Transmit Frame Count for Good and Bad Multicast Frames"]
    #[inline(always)]
    pub const fn tx_multicast_frames_good_bad(&self) -> &TxMulticastFramesGoodBad {
        &self.tx_multicast_frames_good_bad
    }
    #[doc = "0x144 - Transmit Frame Count for Good and Bad Broadcast Frames"]
    #[inline(always)]
    pub const fn tx_broadcast_frames_good_bad(&self) -> &TxBroadcastFramesGoodBad {
        &self.tx_broadcast_frames_good_bad
    }
    #[doc = "0x148 - Transmit Frame Count for Underflow Error Frames"]
    #[inline(always)]
    pub const fn tx_underflow_error_frames(&self) -> &TxUnderflowErrorFrames {
        &self.tx_underflow_error_frames
    }
    #[doc = "0x14c - Transmit Frame Count for Frames Transmitted after Single Collision"]
    #[inline(always)]
    pub const fn tx_single_collision_good_frames(&self) -> &TxSingleCollisionGoodFrames {
        &self.tx_single_collision_good_frames
    }
    #[doc = "0x150 - Transmit Frame Count for Frames Transmitted after Multiple Collision"]
    #[inline(always)]
    pub const fn tx_multiple_collision_good_frames(&self) -> &TxMultipleCollisionGoodFrames {
        &self.tx_multiple_collision_good_frames
    }
    #[doc = "0x154 - Tx Deferred Frames Register"]
    #[inline(always)]
    pub const fn tx_deferred_frames(&self) -> &TxDeferredFrames {
        &self.tx_deferred_frames
    }
    #[doc = "0x158 - Transmit Frame Count for Late Collision Error Frames"]
    #[inline(always)]
    pub const fn tx_late_collision_frames(&self) -> &TxLateCollisionFrames {
        &self.tx_late_collision_frames
    }
    #[doc = "0x15c - Transmit Frame Count for Excessive Collision Error Frames"]
    #[inline(always)]
    pub const fn tx_excessive_collision_frames(&self) -> &TxExcessiveCollisionFrames {
        &self.tx_excessive_collision_frames
    }
    #[doc = "0x160 - Transmit Frame Count for Carrier Sense Error Frames"]
    #[inline(always)]
    pub const fn tx_carrier_error_frames(&self) -> &TxCarrierErrorFrames {
        &self.tx_carrier_error_frames
    }
    #[doc = "0x164 - Tx Octet Count Good Register"]
    #[inline(always)]
    pub const fn tx_octet_count_good(&self) -> &TxOctetCountGood {
        &self.tx_octet_count_good
    }
    #[doc = "0x168 - Tx Frame Count Good Register"]
    #[inline(always)]
    pub const fn tx_frame_count_good(&self) -> &TxFrameCountGood {
        &self.tx_frame_count_good
    }
    #[doc = "0x16c - Transmit Frame Count for Excessive Deferral Error Frames"]
    #[inline(always)]
    pub const fn tx_excessive_deferral_error(&self) -> &TxExcessiveDeferralError {
        &self.tx_excessive_deferral_error
    }
    #[doc = "0x170 - Transmit Frame Count for Good PAUSE Frames"]
    #[inline(always)]
    pub const fn tx_pause_frames(&self) -> &TxPauseFrames {
        &self.tx_pause_frames
    }
    #[doc = "0x174 - Transmit Frame Count for Good VLAN Frames"]
    #[inline(always)]
    pub const fn tx_vlan_frames_good(&self) -> &TxVlanFramesGood {
        &self.tx_vlan_frames_good
    }
    #[doc = "0x178 - Transmit Frame Count for Good Oversize Frames"]
    #[inline(always)]
    pub const fn tx_osize_frames_good(&self) -> &TxOsizeFramesGood {
        &self.tx_osize_frames_good
    }
    #[doc = "0x180 - Receive Frame Count for Good and Bad Frames"]
    #[inline(always)]
    pub const fn rx_frames_count_good_bad(&self) -> &RxFramesCountGoodBad {
        &self.rx_frames_count_good_bad
    }
    #[doc = "0x184 - Receive Octet Count for Good and Bad Frames"]
    #[inline(always)]
    pub const fn rx_octet_count_good_bad(&self) -> &RxOctetCountGoodBad {
        &self.rx_octet_count_good_bad
    }
    #[doc = "0x188 - Rx Octet Count Good Register"]
    #[inline(always)]
    pub const fn rx_octet_count_good(&self) -> &RxOctetCountGood {
        &self.rx_octet_count_good
    }
    #[doc = "0x18c - Receive Frame Count for Good Broadcast Frames"]
    #[inline(always)]
    pub const fn rx_broadcast_frames_good(&self) -> &RxBroadcastFramesGood {
        &self.rx_broadcast_frames_good
    }
    #[doc = "0x190 - Receive Frame Count for Good Multicast Frames"]
    #[inline(always)]
    pub const fn rx_multicast_frames_good(&self) -> &RxMulticastFramesGood {
        &self.rx_multicast_frames_good
    }
    #[doc = "0x194 - Receive Frame Count for CRC Error Frames"]
    #[inline(always)]
    pub const fn rx_crc_error_frames(&self) -> &RxCrcErrorFrames {
        &self.rx_crc_error_frames
    }
    #[doc = "0x198 - Receive Frame Count for Alignment Error Frames"]
    #[inline(always)]
    pub const fn rx_alignment_error_frames(&self) -> &RxAlignmentErrorFrames {
        &self.rx_alignment_error_frames
    }
    #[doc = "0x19c - Receive Frame Count for Runt Error Frames"]
    #[inline(always)]
    pub const fn rx_runt_error_frames(&self) -> &RxRuntErrorFrames {
        &self.rx_runt_error_frames
    }
    #[doc = "0x1a0 - Receive Frame Count for Jabber Error Frames"]
    #[inline(always)]
    pub const fn rx_jabber_error_frames(&self) -> &RxJabberErrorFrames {
        &self.rx_jabber_error_frames
    }
    #[doc = "0x1a4 - Receive Frame Count for Undersize Frames"]
    #[inline(always)]
    pub const fn rx_undersize_frames_good(&self) -> &RxUndersizeFramesGood {
        &self.rx_undersize_frames_good
    }
    #[doc = "0x1a8 - Rx Oversize Frames Good Register"]
    #[inline(always)]
    pub const fn rx_oversize_frames_good(&self) -> &RxOversizeFramesGood {
        &self.rx_oversize_frames_good
    }
    #[doc = "0x1ac - Receive Frame Count for Good and Bad 64 Byte Frames"]
    #[inline(always)]
    pub const fn rx_64octets_frames_good_bad(&self) -> &Rx64octetsFramesGoodBad {
        &self.rx_64octets_frames_good_bad
    }
    #[doc = "0x1b0 - Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_65to127octets_frames_good_bad(&self) -> &Rx65to127octetsFramesGoodBad {
        &self.rx_65to127octets_frames_good_bad
    }
    #[doc = "0x1b4 - Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_128to255octets_frames_good_bad(&self) -> &Rx128to255octetsFramesGoodBad {
        &self.rx_128to255octets_frames_good_bad
    }
    #[doc = "0x1b8 - Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_256to511octets_frames_good_bad(&self) -> &Rx256to511octetsFramesGoodBad {
        &self.rx_256to511octets_frames_good_bad
    }
    #[doc = "0x1bc - Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_512to1023octets_frames_good_bad(&self) -> &Rx512to1023octetsFramesGoodBad {
        &self.rx_512to1023octets_frames_good_bad
    }
    #[doc = "0x1c0 - Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
    #[inline(always)]
    pub const fn rx_1024tomaxoctets_frames_good_bad(&self) -> &Rx1024tomaxoctetsFramesGoodBad {
        &self.rx_1024tomaxoctets_frames_good_bad
    }
    #[doc = "0x1c4 - Receive Frame Count for Good Unicast Frames"]
    #[inline(always)]
    pub const fn rx_unicast_frames_good(&self) -> &RxUnicastFramesGood {
        &self.rx_unicast_frames_good
    }
    #[doc = "0x1c8 - Receive Frame Count for Length Error Frames"]
    #[inline(always)]
    pub const fn rx_length_error_frames(&self) -> &RxLengthErrorFrames {
        &self.rx_length_error_frames
    }
    #[doc = "0x1cc - Receive Frame Count for Out of Range Frames"]
    #[inline(always)]
    pub const fn rx_out_of_range_type_frames(&self) -> &RxOutOfRangeTypeFrames {
        &self.rx_out_of_range_type_frames
    }
    #[doc = "0x1d0 - Receive Frame Count for PAUSE Frames"]
    #[inline(always)]
    pub const fn rx_pause_frames(&self) -> &RxPauseFrames {
        &self.rx_pause_frames
    }
    #[doc = "0x1d4 - Receive Frame Count for FIFO Overflow Frames"]
    #[inline(always)]
    pub const fn rx_fifo_overflow_frames(&self) -> &RxFifoOverflowFrames {
        &self.rx_fifo_overflow_frames
    }
    #[doc = "0x1d8 - Receive Frame Count for Good and Bad VLAN Frames"]
    #[inline(always)]
    pub const fn rx_vlan_frames_good_bad(&self) -> &RxVlanFramesGoodBad {
        &self.rx_vlan_frames_good_bad
    }
    #[doc = "0x1dc - Receive Frame Count for Watchdog Error Frames"]
    #[inline(always)]
    pub const fn rx_watchdog_error_frames(&self) -> &RxWatchdogErrorFrames {
        &self.rx_watchdog_error_frames
    }
    #[doc = "0x1e0 - Receive Frame Count for Receive Error Frames"]
    #[inline(always)]
    pub const fn rx_receive_error_frames(&self) -> &RxReceiveErrorFrames {
        &self.rx_receive_error_frames
    }
    #[doc = "0x1e4 - Receive Frame Count for Good Control Frames Frames"]
    #[inline(always)]
    pub const fn rx_control_frames_good(&self) -> &RxControlFramesGood {
        &self.rx_control_frames_good
    }
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_ipc_receive_interrupt_mask(&self) -> &MmcIpcReceiveInterruptMask {
        &self.mmc_ipc_receive_interrupt_mask
    }
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_ipc_receive_interrupt(&self) -> &MmcIpcReceiveInterrupt {
        &self.mmc_ipc_receive_interrupt
    }
    #[doc = "0x210 - RxIPv4 Good Frames Register"]
    #[inline(always)]
    pub const fn rxipv4_good_frames(&self) -> &Rxipv4GoodFrames {
        &self.rxipv4_good_frames
    }
    #[doc = "0x214 - Receive IPV4 Header Error Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_header_error_frames(&self) -> &Rxipv4HeaderErrorFrames {
        &self.rxipv4_header_error_frames
    }
    #[doc = "0x218 - Receive IPV4 No Payload Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_no_payload_frames(&self) -> &Rxipv4NoPayloadFrames {
        &self.rxipv4_no_payload_frames
    }
    #[doc = "0x21c - Receive IPV4 Fragmented Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_fragmented_frames(&self) -> &Rxipv4FragmentedFrames {
        &self.rxipv4_fragmented_frames
    }
    #[doc = "0x220 - Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_udp_checksum_disabled_frames(&self) -> &Rxipv4UdpChecksumDisabledFrames {
        &self.rxipv4_udp_checksum_disabled_frames
    }
    #[doc = "0x224 - RxIPv6 Good Frames Register"]
    #[inline(always)]
    pub const fn rxipv6_good_frames(&self) -> &Rxipv6GoodFrames {
        &self.rxipv6_good_frames
    }
    #[doc = "0x228 - Receive IPV6 Header Error Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_header_error_frames(&self) -> &Rxipv6HeaderErrorFrames {
        &self.rxipv6_header_error_frames
    }
    #[doc = "0x22c - Receive IPV6 No Payload Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_no_payload_frames(&self) -> &Rxipv6NoPayloadFrames {
        &self.rxipv6_no_payload_frames
    }
    #[doc = "0x230 - RxUDP Good Frames Register"]
    #[inline(always)]
    pub const fn rxudp_good_frames(&self) -> &RxudpGoodFrames {
        &self.rxudp_good_frames
    }
    #[doc = "0x234 - RxUDP Error Frames Register"]
    #[inline(always)]
    pub const fn rxudp_error_frames(&self) -> &RxudpErrorFrames {
        &self.rxudp_error_frames
    }
    #[doc = "0x238 - RxTCP Good Frames Register"]
    #[inline(always)]
    pub const fn rxtcp_good_frames(&self) -> &RxtcpGoodFrames {
        &self.rxtcp_good_frames
    }
    #[doc = "0x23c - RxTCP Error Frames Register"]
    #[inline(always)]
    pub const fn rxtcp_error_frames(&self) -> &RxtcpErrorFrames {
        &self.rxtcp_error_frames
    }
    #[doc = "0x240 - RxICMP Good Frames Register"]
    #[inline(always)]
    pub const fn rxicmp_good_frames(&self) -> &RxicmpGoodFrames {
        &self.rxicmp_good_frames
    }
    #[doc = "0x244 - RxICMP Error Frames Register"]
    #[inline(always)]
    pub const fn rxicmp_error_frames(&self) -> &RxicmpErrorFrames {
        &self.rxicmp_error_frames
    }
    #[doc = "0x250 - RxIPv4 Good Octets Register"]
    #[inline(always)]
    pub const fn rxipv4_good_octets(&self) -> &Rxipv4GoodOctets {
        &self.rxipv4_good_octets
    }
    #[doc = "0x254 - Receive IPV4 Header Error Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_header_error_octets(&self) -> &Rxipv4HeaderErrorOctets {
        &self.rxipv4_header_error_octets
    }
    #[doc = "0x258 - Receive IPV4 No Payload Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_no_payload_octets(&self) -> &Rxipv4NoPayloadOctets {
        &self.rxipv4_no_payload_octets
    }
    #[doc = "0x25c - Receive IPV4 Fragmented Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_fragmented_octets(&self) -> &Rxipv4FragmentedOctets {
        &self.rxipv4_fragmented_octets
    }
    #[doc = "0x260 - Receive IPV4 Fragmented Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_udp_checksum_disable_octets(&self) -> &Rxipv4UdpChecksumDisableOctets {
        &self.rxipv4_udp_checksum_disable_octets
    }
    #[doc = "0x264 - RxIPv6 Good Octets Register"]
    #[inline(always)]
    pub const fn rxipv6_good_octets(&self) -> &Rxipv6GoodOctets {
        &self.rxipv6_good_octets
    }
    #[doc = "0x268 - Receive IPV6 Header Error Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_header_error_octets(&self) -> &Rxipv6HeaderErrorOctets {
        &self.rxipv6_header_error_octets
    }
    #[doc = "0x26c - Receive IPV6 No Payload Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_no_payload_octets(&self) -> &Rxipv6NoPayloadOctets {
        &self.rxipv6_no_payload_octets
    }
    #[doc = "0x270 - Receive UDP Good Octets Register"]
    #[inline(always)]
    pub const fn rxudp_good_octets(&self) -> &RxudpGoodOctets {
        &self.rxudp_good_octets
    }
    #[doc = "0x274 - Receive UDP Error Octets Register"]
    #[inline(always)]
    pub const fn rxudp_error_octets(&self) -> &RxudpErrorOctets {
        &self.rxudp_error_octets
    }
    #[doc = "0x278 - Receive TCP Good Octets Register"]
    #[inline(always)]
    pub const fn rxtcp_good_octets(&self) -> &RxtcpGoodOctets {
        &self.rxtcp_good_octets
    }
    #[doc = "0x27c - Receive TCP Error Octets Register"]
    #[inline(always)]
    pub const fn rxtcp_error_octets(&self) -> &RxtcpErrorOctets {
        &self.rxtcp_error_octets
    }
    #[doc = "0x280 - Receive ICMP Good Octets Register"]
    #[inline(always)]
    pub const fn rxicmp_good_octets(&self) -> &RxicmpGoodOctets {
        &self.rxicmp_good_octets
    }
    #[doc = "0x284 - Receive ICMP Error Octets Register"]
    #[inline(always)]
    pub const fn rxicmp_error_octets(&self) -> &RxicmpErrorOctets {
        &self.rxicmp_error_octets
    }
    #[doc = "0x700 - Timestamp Control Register"]
    #[inline(always)]
    pub const fn timestamp_control(&self) -> &TimestampControl {
        &self.timestamp_control
    }
    #[doc = "0x704 - Sub-Second Increment Register"]
    #[inline(always)]
    pub const fn sub_second_increment(&self) -> &SubSecondIncrement {
        &self.sub_second_increment
    }
    #[doc = "0x708 - System Time - Seconds Register"]
    #[inline(always)]
    pub const fn system_time_seconds(&self) -> &SystemTimeSeconds {
        &self.system_time_seconds
    }
    #[doc = "0x70c - System Time Nanoseconds Register"]
    #[inline(always)]
    pub const fn system_time_nanoseconds(&self) -> &SystemTimeNanoseconds {
        &self.system_time_nanoseconds
    }
    #[doc = "0x710 - System Time - Seconds Update Register"]
    #[inline(always)]
    pub const fn system_time_seconds_update(&self) -> &SystemTimeSecondsUpdate {
        &self.system_time_seconds_update
    }
    #[doc = "0x714 - System Time Nanoseconds Update Register"]
    #[inline(always)]
    pub const fn system_time_nanoseconds_update(&self) -> &SystemTimeNanosecondsUpdate {
        &self.system_time_nanoseconds_update
    }
    #[doc = "0x718 - Timestamp Addend Register"]
    #[inline(always)]
    pub const fn timestamp_addend(&self) -> &TimestampAddend {
        &self.timestamp_addend
    }
    #[doc = "0x71c - Target Time Seconds Register"]
    #[inline(always)]
    pub const fn target_time_seconds(&self) -> &TargetTimeSeconds {
        &self.target_time_seconds
    }
    #[doc = "0x720 - Target Time Nanoseconds Register"]
    #[inline(always)]
    pub const fn target_time_nanoseconds(&self) -> &TargetTimeNanoseconds {
        &self.target_time_nanoseconds
    }
    #[doc = "0x724 - System Time - Higher Word Seconds Register"]
    #[inline(always)]
    pub const fn system_time_higher_word_seconds(&self) -> &SystemTimeHigherWordSeconds {
        &self.system_time_higher_word_seconds
    }
    #[doc = "0x728 - Timestamp Status Register"]
    #[inline(always)]
    pub const fn timestamp_status(&self) -> &TimestampStatus {
        &self.timestamp_status
    }
    #[doc = "0x1000 - Bus Mode Register"]
    #[inline(always)]
    pub const fn bus_mode(&self) -> &BusMode {
        &self.bus_mode
    }
    #[doc = "0x1004 - Transmit Poll Demand Register"]
    #[inline(always)]
    pub const fn transmit_poll_demand(&self) -> &TransmitPollDemand {
        &self.transmit_poll_demand
    }
    #[doc = "0x1008 - Receive Poll Demand Register"]
    #[inline(always)]
    pub const fn receive_poll_demand(&self) -> &ReceivePollDemand {
        &self.receive_poll_demand
    }
    #[doc = "0x100c - Receive Descriptor Address Register"]
    #[inline(always)]
    pub const fn receive_descriptor_list_address(&self) -> &ReceiveDescriptorListAddress {
        &self.receive_descriptor_list_address
    }
    #[doc = "0x1010 - Transmit descripter Address Register"]
    #[inline(always)]
    pub const fn transmit_descriptor_list_address(&self) -> &TransmitDescriptorListAddress {
        &self.transmit_descriptor_list_address
    }
    #[doc = "0x1014 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1018 - Operation Mode Register"]
    #[inline(always)]
    pub const fn operation_mode(&self) -> &OperationMode {
        &self.operation_mode
    }
    #[doc = "0x101c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &InterruptEnable {
        &self.interrupt_enable
    }
    #[doc = "0x1020 - Missed Frame and Buffer Overflow Counter Register"]
    #[inline(always)]
    pub const fn missed_frame_and_buffer_overflow_counter(&self) -> &MissedFrameAndBufferOverflowCounter {
        &self.missed_frame_and_buffer_overflow_counter
    }
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    #[inline(always)]
    pub const fn receive_interrupt_watchdog_timer(&self) -> &ReceiveInterruptWatchdogTimer {
        &self.receive_interrupt_watchdog_timer
    }
    #[doc = "0x102c - AHB Status Register"]
    #[inline(always)]
    pub const fn ahb_status(&self) -> &AhbStatus {
        &self.ahb_status
    }
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    #[inline(always)]
    pub const fn current_host_transmit_descriptor(&self) -> &CurrentHostTransmitDescriptor {
        &self.current_host_transmit_descriptor
    }
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    #[inline(always)]
    pub const fn current_host_receive_descriptor(&self) -> &CurrentHostReceiveDescriptor {
        &self.current_host_receive_descriptor
    }
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    #[inline(always)]
    pub const fn current_host_transmit_buffer_address(&self) -> &CurrentHostTransmitBufferAddress {
        &self.current_host_transmit_buffer_address
    }
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    #[inline(always)]
    pub const fn current_host_receive_buffer_address(&self) -> &CurrentHostReceiveBufferAddress {
        &self.current_host_receive_buffer_address
    }
    #[doc = "0x1058 - HW Feature Register"]
    #[inline(always)]
    pub const fn hw_feature(&self) -> &HwFeature {
        &self.hw_feature
    }
}
#[doc = "MAC_CONFIGURATION (rw) register accessor: MAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_configuration`]
module"]
#[doc(alias = "MAC_CONFIGURATION")]
pub type MacConfiguration = crate::Reg<mac_configuration::MacConfigurationSpec>;
#[doc = "MAC Configuration Register"]
pub mod mac_configuration;
#[doc = "MAC_FRAME_FILTER (rw) register accessor: MAC Frame Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_frame_filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_frame_filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_frame_filter`]
module"]
#[doc(alias = "MAC_FRAME_FILTER")]
pub type MacFrameFilter = crate::Reg<mac_frame_filter::MacFrameFilterSpec>;
#[doc = "MAC Frame Filter"]
pub mod mac_frame_filter;
#[doc = "HASH_TABLE_HIGH (rw) register accessor: Hash Table High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_table_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_table_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_table_high`]
module"]
#[doc(alias = "HASH_TABLE_HIGH")]
pub type HashTableHigh = crate::Reg<hash_table_high::HashTableHighSpec>;
#[doc = "Hash Table High Register"]
pub mod hash_table_high;
#[doc = "HASH_TABLE_LOW (rw) register accessor: Hash Table Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_table_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_table_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_table_low`]
module"]
#[doc(alias = "HASH_TABLE_LOW")]
pub type HashTableLow = crate::Reg<hash_table_low::HashTableLowSpec>;
#[doc = "Hash Table Low Register"]
pub mod hash_table_low;
#[doc = "GMII_ADDRESS (rw) register accessor: MII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_address`]
module"]
#[doc(alias = "GMII_ADDRESS")]
pub type GmiiAddress = crate::Reg<gmii_address::GmiiAddressSpec>;
#[doc = "MII Address Register"]
pub mod gmii_address;
#[doc = "GMII_DATA (rw) register accessor: MII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_data`]
module"]
#[doc(alias = "GMII_DATA")]
pub type GmiiData = crate::Reg<gmii_data::GmiiDataSpec>;
#[doc = "MII Data Register"]
pub mod gmii_data;
#[doc = "FLOW_CONTROL (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow_control`]
module"]
#[doc(alias = "FLOW_CONTROL")]
pub type FlowControl = crate::Reg<flow_control::FlowControlSpec>;
#[doc = "Flow Control Register"]
pub mod flow_control;
#[doc = "VLAN_TAG (rw) register accessor: VLAN Tag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_tag`]
module"]
#[doc(alias = "VLAN_TAG")]
pub type VlanTag = crate::Reg<vlan_tag::VlanTagSpec>;
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "VERSION (r) register accessor: Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "Version Register"]
pub mod version;
#[doc = "DEBUG (r) register accessor: Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
#[doc(alias = "DEBUG")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "Debug Register"]
pub mod debug;
#[doc = "REMOTE_WAKE_UP_FRAME_FILTER (rw) register accessor: Remote Wake Up Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remote_wake_up_frame_filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remote_wake_up_frame_filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remote_wake_up_frame_filter`]
module"]
#[doc(alias = "REMOTE_WAKE_UP_FRAME_FILTER")]
pub type RemoteWakeUpFrameFilter = crate::Reg<remote_wake_up_frame_filter::RemoteWakeUpFrameFilterSpec>;
#[doc = "Remote Wake Up Frame Filter Register"]
pub mod remote_wake_up_frame_filter;
#[doc = "PMT_CONTROL_STATUS (rw) register accessor: PMT Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_control_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt_control_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_control_status`]
module"]
#[doc(alias = "PMT_CONTROL_STATUS")]
pub type PmtControlStatus = crate::Reg<pmt_control_status::PmtControlStatusSpec>;
#[doc = "PMT Control and Status Register"]
pub mod pmt_control_status;
#[doc = "INTERRUPT_STATUS (r) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status`]
module"]
#[doc(alias = "INTERRUPT_STATUS")]
pub type InterruptStatus = crate::Reg<interrupt_status::InterruptStatusSpec>;
#[doc = "Interrupt Register"]
pub mod interrupt_status;
#[doc = "INTERRUPT_MASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
#[doc(alias = "INTERRUPT_MASK")]
pub type InterruptMask = crate::Reg<interrupt_mask::InterruptMaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod interrupt_mask;
#[doc = "MAC_ADDRESS0_HIGH (rw) register accessor: MAC Address0 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address0_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address0_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address0_high`]
module"]
#[doc(alias = "MAC_ADDRESS0_HIGH")]
pub type MacAddress0High = crate::Reg<mac_address0_high::MacAddress0HighSpec>;
#[doc = "MAC Address0 High Register"]
pub mod mac_address0_high;
#[doc = "MAC_ADDRESS0_LOW (rw) register accessor: MAC Address0 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address0_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address0_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address0_low`]
module"]
#[doc(alias = "MAC_ADDRESS0_LOW")]
pub type MacAddress0Low = crate::Reg<mac_address0_low::MacAddress0LowSpec>;
#[doc = "MAC Address0 Low Register"]
pub mod mac_address0_low;
#[doc = "MAC_ADDRESS1_HIGH (rw) register accessor: MAC Address1 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address1_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address1_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address1_high`]
module"]
#[doc(alias = "MAC_ADDRESS1_HIGH")]
pub type MacAddress1High = crate::Reg<mac_address1_high::MacAddress1HighSpec>;
#[doc = "MAC Address1 High Register"]
pub mod mac_address1_high;
#[doc = "MAC_ADDRESS1_LOW (rw) register accessor: MAC Address1 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address1_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address1_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address1_low`]
module"]
#[doc(alias = "MAC_ADDRESS1_LOW")]
pub type MacAddress1Low = crate::Reg<mac_address1_low::MacAddress1LowSpec>;
#[doc = "MAC Address1 Low Register"]
pub mod mac_address1_low;
#[doc = "MAC_ADDRESS2_HIGH (rw) register accessor: MAC Address2 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address2_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address2_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address2_high`]
module"]
#[doc(alias = "MAC_ADDRESS2_HIGH")]
pub type MacAddress2High = crate::Reg<mac_address2_high::MacAddress2HighSpec>;
#[doc = "MAC Address2 High Register"]
pub mod mac_address2_high;
#[doc = "MAC_ADDRESS2_LOW (rw) register accessor: MAC Address2 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address2_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address2_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address2_low`]
module"]
#[doc(alias = "MAC_ADDRESS2_LOW")]
pub type MacAddress2Low = crate::Reg<mac_address2_low::MacAddress2LowSpec>;
#[doc = "MAC Address2 Low Register"]
pub mod mac_address2_low;
#[doc = "MAC_ADDRESS3_HIGH (rw) register accessor: MAC Address3 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address3_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address3_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address3_high`]
module"]
#[doc(alias = "MAC_ADDRESS3_HIGH")]
pub type MacAddress3High = crate::Reg<mac_address3_high::MacAddress3HighSpec>;
#[doc = "MAC Address3 High Register"]
pub mod mac_address3_high;
#[doc = "MAC_ADDRESS3_LOW (rw) register accessor: MAC Address3 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address3_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address3_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address3_low`]
module"]
#[doc(alias = "MAC_ADDRESS3_LOW")]
pub type MacAddress3Low = crate::Reg<mac_address3_low::MacAddress3LowSpec>;
#[doc = "MAC Address3 Low Register"]
pub mod mac_address3_low;
#[doc = "MMC_CONTROL (rw) register accessor: MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_control`]
module"]
#[doc(alias = "MMC_CONTROL")]
pub type MmcControl = crate::Reg<mmc_control::MmcControlSpec>;
#[doc = "MMC Control Register"]
pub mod mmc_control;
#[doc = "MMC_RECEIVE_INTERRUPT (r) register accessor: MMC Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_receive_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_receive_interrupt`]
module"]
#[doc(alias = "MMC_RECEIVE_INTERRUPT")]
pub type MmcReceiveInterrupt = crate::Reg<mmc_receive_interrupt::MmcReceiveInterruptSpec>;
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_receive_interrupt;
#[doc = "MMC_TRANSMIT_INTERRUPT (r) register accessor: MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_transmit_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_transmit_interrupt`]
module"]
#[doc(alias = "MMC_TRANSMIT_INTERRUPT")]
pub type MmcTransmitInterrupt = crate::Reg<mmc_transmit_interrupt::MmcTransmitInterruptSpec>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_transmit_interrupt;
#[doc = "MMC_RECEIVE_INTERRUPT_MASK (rw) register accessor: MMC Reveive Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_receive_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_receive_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_receive_interrupt_mask`]
module"]
#[doc(alias = "MMC_RECEIVE_INTERRUPT_MASK")]
pub type MmcReceiveInterruptMask = crate::Reg<mmc_receive_interrupt_mask::MmcReceiveInterruptMaskSpec>;
#[doc = "MMC Reveive Interrupt Mask Register"]
pub mod mmc_receive_interrupt_mask;
#[doc = "MMC_TRANSMIT_INTERRUPT_MASK (rw) register accessor: MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_transmit_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_transmit_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_transmit_interrupt_mask`]
module"]
#[doc(alias = "MMC_TRANSMIT_INTERRUPT_MASK")]
pub type MmcTransmitInterruptMask = crate::Reg<mmc_transmit_interrupt_mask::MmcTransmitInterruptMaskSpec>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_transmit_interrupt_mask;
#[doc = "TX_OCTET_COUNT_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_octet_count_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_octet_count_good_bad`]
module"]
#[doc(alias = "TX_OCTET_COUNT_GOOD_BAD")]
pub type TxOctetCountGoodBad = crate::Reg<tx_octet_count_good_bad::TxOctetCountGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad Frames Register"]
pub mod tx_octet_count_good_bad;
#[doc = "TX_FRAME_COUNT_GOOD_BAD (r) register accessor: Transmit Frame Count for Goodand Bad Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_frame_count_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_frame_count_good_bad`]
module"]
#[doc(alias = "TX_FRAME_COUNT_GOOD_BAD")]
pub type TxFrameCountGoodBad = crate::Reg<tx_frame_count_good_bad::TxFrameCountGoodBadSpec>;
#[doc = "Transmit Frame Count for Goodand Bad Frames Register"]
pub mod tx_frame_count_good_bad;
#[doc = "TX_BROADCAST_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good Broadcast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_broadcast_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_broadcast_frames_good`]
module"]
#[doc(alias = "TX_BROADCAST_FRAMES_GOOD")]
pub type TxBroadcastFramesGood = crate::Reg<tx_broadcast_frames_good::TxBroadcastFramesGoodSpec>;
#[doc = "Transmit Frame Count for Good Broadcast Frames"]
pub mod tx_broadcast_frames_good;
#[doc = "TX_MULTICAST_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multicast_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multicast_frames_good`]
module"]
#[doc(alias = "TX_MULTICAST_FRAMES_GOOD")]
pub type TxMulticastFramesGood = crate::Reg<tx_multicast_frames_good::TxMulticastFramesGoodSpec>;
#[doc = "Transmit Frame Count for Good Multicast Frames"]
pub mod tx_multicast_frames_good;
#[doc = "TX_64OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_64octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_64octets_frames_good_bad`]
module"]
#[doc(alias = "TX_64OCTETS_FRAMES_GOOD_BAD")]
pub type Tx64octetsFramesGoodBad = crate::Reg<tx_64octets_frames_good_bad::Tx64octetsFramesGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames"]
pub mod tx_64octets_frames_good_bad;
#[doc = "TX_65TO127OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_65to127octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_65to127octets_frames_good_bad`]
module"]
#[doc(alias = "TX_65TO127OCTETS_FRAMES_GOOD_BAD")]
pub type Tx65to127octetsFramesGoodBad = crate::Reg<tx_65to127octets_frames_good_bad::Tx65to127octetsFramesGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod tx_65to127octets_frames_good_bad;
#[doc = "TX_128TO255OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_128to255octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_128to255octets_frames_good_bad`]
module"]
#[doc(alias = "TX_128TO255OCTETS_FRAMES_GOOD_BAD")]
pub type Tx128to255octetsFramesGoodBad = crate::Reg<tx_128to255octets_frames_good_bad::Tx128to255octetsFramesGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod tx_128to255octets_frames_good_bad;
#[doc = "TX_256TO511OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_256to511octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_256to511octets_frames_good_bad`]
module"]
#[doc(alias = "TX_256TO511OCTETS_FRAMES_GOOD_BAD")]
pub type Tx256to511octetsFramesGoodBad = crate::Reg<tx_256to511octets_frames_good_bad::Tx256to511octetsFramesGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod tx_256to511octets_frames_good_bad;
#[doc = "TX_512TO1023OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_512to1023octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_512to1023octets_frames_good_bad`]
module"]
#[doc(alias = "TX_512TO1023OCTETS_FRAMES_GOOD_BAD")]
pub type Tx512to1023octetsFramesGoodBad = crate::Reg<tx_512to1023octets_frames_good_bad::Tx512to1023octetsFramesGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
pub mod tx_512to1023octets_frames_good_bad;
#[doc = "TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_1024tomaxoctets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_1024tomaxoctets_frames_good_bad`]
module"]
#[doc(alias = "TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD")]
pub type Tx1024tomaxoctetsFramesGoodBad = crate::Reg<tx_1024tomaxoctets_frames_good_bad::Tx1024tomaxoctetsFramesGoodBadSpec>;
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
pub mod tx_1024tomaxoctets_frames_good_bad;
#[doc = "TX_UNICAST_FRAMES_GOOD_BAD (r) register accessor: Transmit Frame Count for Good and Bad Unicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_unicast_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_unicast_frames_good_bad`]
module"]
#[doc(alias = "TX_UNICAST_FRAMES_GOOD_BAD")]
pub type TxUnicastFramesGoodBad = crate::Reg<tx_unicast_frames_good_bad::TxUnicastFramesGoodBadSpec>;
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames"]
pub mod tx_unicast_frames_good_bad;
#[doc = "TX_MULTICAST_FRAMES_GOOD_BAD (r) register accessor: Transmit Frame Count for Good and Bad Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multicast_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multicast_frames_good_bad`]
module"]
#[doc(alias = "TX_MULTICAST_FRAMES_GOOD_BAD")]
pub type TxMulticastFramesGoodBad = crate::Reg<tx_multicast_frames_good_bad::TxMulticastFramesGoodBadSpec>;
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames"]
pub mod tx_multicast_frames_good_bad;
#[doc = "TX_BROADCAST_FRAMES_GOOD_BAD (r) register accessor: Transmit Frame Count for Good and Bad Broadcast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_broadcast_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_broadcast_frames_good_bad`]
module"]
#[doc(alias = "TX_BROADCAST_FRAMES_GOOD_BAD")]
pub type TxBroadcastFramesGoodBad = crate::Reg<tx_broadcast_frames_good_bad::TxBroadcastFramesGoodBadSpec>;
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames"]
pub mod tx_broadcast_frames_good_bad;
#[doc = "TX_UNDERFLOW_ERROR_FRAMES (r) register accessor: Transmit Frame Count for Underflow Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_underflow_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_underflow_error_frames`]
module"]
#[doc(alias = "TX_UNDERFLOW_ERROR_FRAMES")]
pub type TxUnderflowErrorFrames = crate::Reg<tx_underflow_error_frames::TxUnderflowErrorFramesSpec>;
#[doc = "Transmit Frame Count for Underflow Error Frames"]
pub mod tx_underflow_error_frames;
#[doc = "TX_SINGLE_COLLISION_GOOD_FRAMES (r) register accessor: Transmit Frame Count for Frames Transmitted after Single Collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_single_collision_good_frames`]
module"]
#[doc(alias = "TX_SINGLE_COLLISION_GOOD_FRAMES")]
pub type TxSingleCollisionGoodFrames = crate::Reg<tx_single_collision_good_frames::TxSingleCollisionGoodFramesSpec>;
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod tx_single_collision_good_frames;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_FRAMES (r) register accessor: Transmit Frame Count for Frames Transmitted after Multiple Collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_multiple_collision_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multiple_collision_good_frames`]
module"]
#[doc(alias = "TX_MULTIPLE_COLLISION_GOOD_FRAMES")]
pub type TxMultipleCollisionGoodFrames = crate::Reg<tx_multiple_collision_good_frames::TxMultipleCollisionGoodFramesSpec>;
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision"]
pub mod tx_multiple_collision_good_frames;
#[doc = "TX_DEFERRED_FRAMES (r) register accessor: Tx Deferred Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_deferred_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_deferred_frames`]
module"]
#[doc(alias = "TX_DEFERRED_FRAMES")]
pub type TxDeferredFrames = crate::Reg<tx_deferred_frames::TxDeferredFramesSpec>;
#[doc = "Tx Deferred Frames Register"]
pub mod tx_deferred_frames;
#[doc = "TX_LATE_COLLISION_FRAMES (r) register accessor: Transmit Frame Count for Late Collision Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_late_collision_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_late_collision_frames`]
module"]
#[doc(alias = "TX_LATE_COLLISION_FRAMES")]
pub type TxLateCollisionFrames = crate::Reg<tx_late_collision_frames::TxLateCollisionFramesSpec>;
#[doc = "Transmit Frame Count for Late Collision Error Frames"]
pub mod tx_late_collision_frames;
#[doc = "TX_EXCESSIVE_COLLISION_FRAMES (r) register accessor: Transmit Frame Count for Excessive Collision Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_excessive_collision_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_excessive_collision_frames`]
module"]
#[doc(alias = "TX_EXCESSIVE_COLLISION_FRAMES")]
pub type TxExcessiveCollisionFrames = crate::Reg<tx_excessive_collision_frames::TxExcessiveCollisionFramesSpec>;
#[doc = "Transmit Frame Count for Excessive Collision Error Frames"]
pub mod tx_excessive_collision_frames;
#[doc = "TX_CARRIER_ERROR_FRAMES (r) register accessor: Transmit Frame Count for Carrier Sense Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_carrier_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_carrier_error_frames`]
module"]
#[doc(alias = "TX_CARRIER_ERROR_FRAMES")]
pub type TxCarrierErrorFrames = crate::Reg<tx_carrier_error_frames::TxCarrierErrorFramesSpec>;
#[doc = "Transmit Frame Count for Carrier Sense Error Frames"]
pub mod tx_carrier_error_frames;
#[doc = "TX_OCTET_COUNT_GOOD (r) register accessor: Tx Octet Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_octet_count_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_octet_count_good`]
module"]
#[doc(alias = "TX_OCTET_COUNT_GOOD")]
pub type TxOctetCountGood = crate::Reg<tx_octet_count_good::TxOctetCountGoodSpec>;
#[doc = "Tx Octet Count Good Register"]
pub mod tx_octet_count_good;
#[doc = "TX_FRAME_COUNT_GOOD (r) register accessor: Tx Frame Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_frame_count_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_frame_count_good`]
module"]
#[doc(alias = "TX_FRAME_COUNT_GOOD")]
pub type TxFrameCountGood = crate::Reg<tx_frame_count_good::TxFrameCountGoodSpec>;
#[doc = "Tx Frame Count Good Register"]
pub mod tx_frame_count_good;
#[doc = "TX_EXCESSIVE_DEFERRAL_ERROR (r) register accessor: Transmit Frame Count for Excessive Deferral Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_excessive_deferral_error::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_excessive_deferral_error`]
module"]
#[doc(alias = "TX_EXCESSIVE_DEFERRAL_ERROR")]
pub type TxExcessiveDeferralError = crate::Reg<tx_excessive_deferral_error::TxExcessiveDeferralErrorSpec>;
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames"]
pub mod tx_excessive_deferral_error;
#[doc = "TX_PAUSE_FRAMES (r) register accessor: Transmit Frame Count for Good PAUSE Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_pause_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pause_frames`]
module"]
#[doc(alias = "TX_PAUSE_FRAMES")]
pub type TxPauseFrames = crate::Reg<tx_pause_frames::TxPauseFramesSpec>;
#[doc = "Transmit Frame Count for Good PAUSE Frames"]
pub mod tx_pause_frames;
#[doc = "TX_VLAN_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good VLAN Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_vlan_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_vlan_frames_good`]
module"]
#[doc(alias = "TX_VLAN_FRAMES_GOOD")]
pub type TxVlanFramesGood = crate::Reg<tx_vlan_frames_good::TxVlanFramesGoodSpec>;
#[doc = "Transmit Frame Count for Good VLAN Frames"]
pub mod tx_vlan_frames_good;
#[doc = "TX_OSIZE_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good Oversize Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_osize_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_osize_frames_good`]
module"]
#[doc(alias = "TX_OSIZE_FRAMES_GOOD")]
pub type TxOsizeFramesGood = crate::Reg<tx_osize_frames_good::TxOsizeFramesGoodSpec>;
#[doc = "Transmit Frame Count for Good Oversize Frames"]
pub mod tx_osize_frames_good;
#[doc = "RX_FRAMES_COUNT_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_frames_count_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_frames_count_good_bad`]
module"]
#[doc(alias = "RX_FRAMES_COUNT_GOOD_BAD")]
pub type RxFramesCountGoodBad = crate::Reg<rx_frames_count_good_bad::RxFramesCountGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad Frames"]
pub mod rx_frames_count_good_bad;
#[doc = "RX_OCTET_COUNT_GOOD_BAD (r) register accessor: Receive Octet Count for Good and Bad Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_octet_count_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_octet_count_good_bad`]
module"]
#[doc(alias = "RX_OCTET_COUNT_GOOD_BAD")]
pub type RxOctetCountGoodBad = crate::Reg<rx_octet_count_good_bad::RxOctetCountGoodBadSpec>;
#[doc = "Receive Octet Count for Good and Bad Frames"]
pub mod rx_octet_count_good_bad;
#[doc = "RX_OCTET_COUNT_GOOD (r) register accessor: Rx Octet Count Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_octet_count_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_octet_count_good`]
module"]
#[doc(alias = "RX_OCTET_COUNT_GOOD")]
pub type RxOctetCountGood = crate::Reg<rx_octet_count_good::RxOctetCountGoodSpec>;
#[doc = "Rx Octet Count Good Register"]
pub mod rx_octet_count_good;
#[doc = "RX_BROADCAST_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Broadcast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_broadcast_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_broadcast_frames_good`]
module"]
#[doc(alias = "RX_BROADCAST_FRAMES_GOOD")]
pub type RxBroadcastFramesGood = crate::Reg<rx_broadcast_frames_good::RxBroadcastFramesGoodSpec>;
#[doc = "Receive Frame Count for Good Broadcast Frames"]
pub mod rx_broadcast_frames_good;
#[doc = "RX_MULTICAST_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Multicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_multicast_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_multicast_frames_good`]
module"]
#[doc(alias = "RX_MULTICAST_FRAMES_GOOD")]
pub type RxMulticastFramesGood = crate::Reg<rx_multicast_frames_good::RxMulticastFramesGoodSpec>;
#[doc = "Receive Frame Count for Good Multicast Frames"]
pub mod rx_multicast_frames_good;
#[doc = "RX_CRC_ERROR_FRAMES (r) register accessor: Receive Frame Count for CRC Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_error_frames`]
module"]
#[doc(alias = "RX_CRC_ERROR_FRAMES")]
pub type RxCrcErrorFrames = crate::Reg<rx_crc_error_frames::RxCrcErrorFramesSpec>;
#[doc = "Receive Frame Count for CRC Error Frames"]
pub mod rx_crc_error_frames;
#[doc = "RX_ALIGNMENT_ERROR_FRAMES (r) register accessor: Receive Frame Count for Alignment Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_alignment_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_alignment_error_frames`]
module"]
#[doc(alias = "RX_ALIGNMENT_ERROR_FRAMES")]
pub type RxAlignmentErrorFrames = crate::Reg<rx_alignment_error_frames::RxAlignmentErrorFramesSpec>;
#[doc = "Receive Frame Count for Alignment Error Frames"]
pub mod rx_alignment_error_frames;
#[doc = "RX_RUNT_ERROR_FRAMES (r) register accessor: Receive Frame Count for Runt Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_runt_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_runt_error_frames`]
module"]
#[doc(alias = "RX_RUNT_ERROR_FRAMES")]
pub type RxRuntErrorFrames = crate::Reg<rx_runt_error_frames::RxRuntErrorFramesSpec>;
#[doc = "Receive Frame Count for Runt Error Frames"]
pub mod rx_runt_error_frames;
#[doc = "RX_JABBER_ERROR_FRAMES (r) register accessor: Receive Frame Count for Jabber Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_jabber_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_jabber_error_frames`]
module"]
#[doc(alias = "RX_JABBER_ERROR_FRAMES")]
pub type RxJabberErrorFrames = crate::Reg<rx_jabber_error_frames::RxJabberErrorFramesSpec>;
#[doc = "Receive Frame Count for Jabber Error Frames"]
pub mod rx_jabber_error_frames;
#[doc = "RX_UNDERSIZE_FRAMES_GOOD (r) register accessor: Receive Frame Count for Undersize Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_undersize_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_undersize_frames_good`]
module"]
#[doc(alias = "RX_UNDERSIZE_FRAMES_GOOD")]
pub type RxUndersizeFramesGood = crate::Reg<rx_undersize_frames_good::RxUndersizeFramesGoodSpec>;
#[doc = "Receive Frame Count for Undersize Frames"]
pub mod rx_undersize_frames_good;
#[doc = "RX_OVERSIZE_FRAMES_GOOD (r) register accessor: Rx Oversize Frames Good Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_oversize_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_oversize_frames_good`]
module"]
#[doc(alias = "RX_OVERSIZE_FRAMES_GOOD")]
pub type RxOversizeFramesGood = crate::Reg<rx_oversize_frames_good::RxOversizeFramesGoodSpec>;
#[doc = "Rx Oversize Frames Good Register"]
pub mod rx_oversize_frames_good;
#[doc = "RX_64OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_64octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_64octets_frames_good_bad`]
module"]
#[doc(alias = "RX_64OCTETS_FRAMES_GOOD_BAD")]
pub type Rx64octetsFramesGoodBad = crate::Reg<rx_64octets_frames_good_bad::Rx64octetsFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames"]
pub mod rx_64octets_frames_good_bad;
#[doc = "RX_65TO127OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_65to127octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_65to127octets_frames_good_bad`]
module"]
#[doc(alias = "RX_65TO127OCTETS_FRAMES_GOOD_BAD")]
pub type Rx65to127octetsFramesGoodBad = crate::Reg<rx_65to127octets_frames_good_bad::Rx65to127octetsFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod rx_65to127octets_frames_good_bad;
#[doc = "RX_128TO255OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_128to255octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_128to255octets_frames_good_bad`]
module"]
#[doc(alias = "RX_128TO255OCTETS_FRAMES_GOOD_BAD")]
pub type Rx128to255octetsFramesGoodBad = crate::Reg<rx_128to255octets_frames_good_bad::Rx128to255octetsFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod rx_128to255octets_frames_good_bad;
#[doc = "RX_256TO511OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 256 to 511 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_256to511octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_256to511octets_frames_good_bad`]
module"]
#[doc(alias = "RX_256TO511OCTETS_FRAMES_GOOD_BAD")]
pub type Rx256to511octetsFramesGoodBad = crate::Reg<rx_256to511octets_frames_good_bad::Rx256to511octetsFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod rx_256to511octets_frames_good_bad;
#[doc = "RX_512TO1023OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_512to1023octets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_512to1023octets_frames_good_bad`]
module"]
#[doc(alias = "RX_512TO1023OCTETS_FRAMES_GOOD_BAD")]
pub type Rx512to1023octetsFramesGoodBad = crate::Reg<rx_512to1023octets_frames_good_bad::Rx512to1023octetsFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
pub mod rx_512to1023octets_frames_good_bad;
#[doc = "RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_1024tomaxoctets_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_1024tomaxoctets_frames_good_bad`]
module"]
#[doc(alias = "RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD")]
pub type Rx1024tomaxoctetsFramesGoodBad = crate::Reg<rx_1024tomaxoctets_frames_good_bad::Rx1024tomaxoctetsFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
pub mod rx_1024tomaxoctets_frames_good_bad;
#[doc = "RX_UNICAST_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Unicast Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_unicast_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_unicast_frames_good`]
module"]
#[doc(alias = "RX_UNICAST_FRAMES_GOOD")]
pub type RxUnicastFramesGood = crate::Reg<rx_unicast_frames_good::RxUnicastFramesGoodSpec>;
#[doc = "Receive Frame Count for Good Unicast Frames"]
pub mod rx_unicast_frames_good;
#[doc = "RX_LENGTH_ERROR_FRAMES (r) register accessor: Receive Frame Count for Length Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_length_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_length_error_frames`]
module"]
#[doc(alias = "RX_LENGTH_ERROR_FRAMES")]
pub type RxLengthErrorFrames = crate::Reg<rx_length_error_frames::RxLengthErrorFramesSpec>;
#[doc = "Receive Frame Count for Length Error Frames"]
pub mod rx_length_error_frames;
#[doc = "RX_OUT_OF_RANGE_TYPE_FRAMES (r) register accessor: Receive Frame Count for Out of Range Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_out_of_range_type_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_out_of_range_type_frames`]
module"]
#[doc(alias = "RX_OUT_OF_RANGE_TYPE_FRAMES")]
pub type RxOutOfRangeTypeFrames = crate::Reg<rx_out_of_range_type_frames::RxOutOfRangeTypeFramesSpec>;
#[doc = "Receive Frame Count for Out of Range Frames"]
pub mod rx_out_of_range_type_frames;
#[doc = "RX_PAUSE_FRAMES (r) register accessor: Receive Frame Count for PAUSE Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_pause_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_pause_frames`]
module"]
#[doc(alias = "RX_PAUSE_FRAMES")]
pub type RxPauseFrames = crate::Reg<rx_pause_frames::RxPauseFramesSpec>;
#[doc = "Receive Frame Count for PAUSE Frames"]
pub mod rx_pause_frames;
#[doc = "RX_FIFO_OVERFLOW_FRAMES (r) register accessor: Receive Frame Count for FIFO Overflow Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_fifo_overflow_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_overflow_frames`]
module"]
#[doc(alias = "RX_FIFO_OVERFLOW_FRAMES")]
pub type RxFifoOverflowFrames = crate::Reg<rx_fifo_overflow_frames::RxFifoOverflowFramesSpec>;
#[doc = "Receive Frame Count for FIFO Overflow Frames"]
pub mod rx_fifo_overflow_frames;
#[doc = "RX_VLAN_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad VLAN Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_vlan_frames_good_bad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_vlan_frames_good_bad`]
module"]
#[doc(alias = "RX_VLAN_FRAMES_GOOD_BAD")]
pub type RxVlanFramesGoodBad = crate::Reg<rx_vlan_frames_good_bad::RxVlanFramesGoodBadSpec>;
#[doc = "Receive Frame Count for Good and Bad VLAN Frames"]
pub mod rx_vlan_frames_good_bad;
#[doc = "RX_WATCHDOG_ERROR_FRAMES (r) register accessor: Receive Frame Count for Watchdog Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_watchdog_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_watchdog_error_frames`]
module"]
#[doc(alias = "RX_WATCHDOG_ERROR_FRAMES")]
pub type RxWatchdogErrorFrames = crate::Reg<rx_watchdog_error_frames::RxWatchdogErrorFramesSpec>;
#[doc = "Receive Frame Count for Watchdog Error Frames"]
pub mod rx_watchdog_error_frames;
#[doc = "RX_RECEIVE_ERROR_FRAMES (r) register accessor: Receive Frame Count for Receive Error Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_receive_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_receive_error_frames`]
module"]
#[doc(alias = "RX_RECEIVE_ERROR_FRAMES")]
pub type RxReceiveErrorFrames = crate::Reg<rx_receive_error_frames::RxReceiveErrorFramesSpec>;
#[doc = "Receive Frame Count for Receive Error Frames"]
pub mod rx_receive_error_frames;
#[doc = "RX_CONTROL_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Control Frames Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_control_frames_good::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_control_frames_good`]
module"]
#[doc(alias = "RX_CONTROL_FRAMES_GOOD")]
pub type RxControlFramesGood = crate::Reg<rx_control_frames_good::RxControlFramesGoodSpec>;
#[doc = "Receive Frame Count for Good Control Frames Frames"]
pub mod rx_control_frames_good;
#[doc = "MMC_IPC_RECEIVE_INTERRUPT_MASK (rw) register accessor: MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_receive_interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_ipc_receive_interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_receive_interrupt_mask`]
module"]
#[doc(alias = "MMC_IPC_RECEIVE_INTERRUPT_MASK")]
pub type MmcIpcReceiveInterruptMask = crate::Reg<mmc_ipc_receive_interrupt_mask::MmcIpcReceiveInterruptMaskSpec>;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_receive_interrupt_mask;
#[doc = "MMC_IPC_RECEIVE_INTERRUPT (r) register accessor: MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_receive_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_receive_interrupt`]
module"]
#[doc(alias = "MMC_IPC_RECEIVE_INTERRUPT")]
pub type MmcIpcReceiveInterrupt = crate::Reg<mmc_ipc_receive_interrupt::MmcIpcReceiveInterruptSpec>;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_receive_interrupt;
#[doc = "RXIPV4_GOOD_FRAMES (r) register accessor: RxIPv4 Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_good_frames`]
module"]
#[doc(alias = "RXIPV4_GOOD_FRAMES")]
pub type Rxipv4GoodFrames = crate::Reg<rxipv4_good_frames::Rxipv4GoodFramesSpec>;
#[doc = "RxIPv4 Good Frames Register"]
pub mod rxipv4_good_frames;
#[doc = "RXIPV4_HEADER_ERROR_FRAMES (r) register accessor: Receive IPV4 Header Error Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_header_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_header_error_frames`]
module"]
#[doc(alias = "RXIPV4_HEADER_ERROR_FRAMES")]
pub type Rxipv4HeaderErrorFrames = crate::Reg<rxipv4_header_error_frames::Rxipv4HeaderErrorFramesSpec>;
#[doc = "Receive IPV4 Header Error Frame Counter Register"]
pub mod rxipv4_header_error_frames;
#[doc = "RXIPV4_NO_PAYLOAD_FRAMES (r) register accessor: Receive IPV4 No Payload Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_no_payload_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_no_payload_frames`]
module"]
#[doc(alias = "RXIPV4_NO_PAYLOAD_FRAMES")]
pub type Rxipv4NoPayloadFrames = crate::Reg<rxipv4_no_payload_frames::Rxipv4NoPayloadFramesSpec>;
#[doc = "Receive IPV4 No Payload Frame Counter Register"]
pub mod rxipv4_no_payload_frames;
#[doc = "RXIPV4_FRAGMENTED_FRAMES (r) register accessor: Receive IPV4 Fragmented Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_fragmented_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_fragmented_frames`]
module"]
#[doc(alias = "RXIPV4_FRAGMENTED_FRAMES")]
pub type Rxipv4FragmentedFrames = crate::Reg<rxipv4_fragmented_frames::Rxipv4FragmentedFramesSpec>;
#[doc = "Receive IPV4 Fragmented Frame Counter Register"]
pub mod rxipv4_fragmented_frames;
#[doc = "RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES (r) register accessor: Receive IPV4 UDP Checksum Disabled Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udp_checksum_disabled_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_udp_checksum_disabled_frames`]
module"]
#[doc(alias = "RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES")]
pub type Rxipv4UdpChecksumDisabledFrames = crate::Reg<rxipv4_udp_checksum_disabled_frames::Rxipv4UdpChecksumDisabledFramesSpec>;
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
pub mod rxipv4_udp_checksum_disabled_frames;
#[doc = "RXIPV6_GOOD_FRAMES (r) register accessor: RxIPv6 Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_good_frames`]
module"]
#[doc(alias = "RXIPV6_GOOD_FRAMES")]
pub type Rxipv6GoodFrames = crate::Reg<rxipv6_good_frames::Rxipv6GoodFramesSpec>;
#[doc = "RxIPv6 Good Frames Register"]
pub mod rxipv6_good_frames;
#[doc = "RXIPV6_HEADER_ERROR_FRAMES (r) register accessor: Receive IPV6 Header Error Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_header_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_header_error_frames`]
module"]
#[doc(alias = "RXIPV6_HEADER_ERROR_FRAMES")]
pub type Rxipv6HeaderErrorFrames = crate::Reg<rxipv6_header_error_frames::Rxipv6HeaderErrorFramesSpec>;
#[doc = "Receive IPV6 Header Error Frame Counter Register"]
pub mod rxipv6_header_error_frames;
#[doc = "RXIPV6_NO_PAYLOAD_FRAMES (r) register accessor: Receive IPV6 No Payload Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_no_payload_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_no_payload_frames`]
module"]
#[doc(alias = "RXIPV6_NO_PAYLOAD_FRAMES")]
pub type Rxipv6NoPayloadFrames = crate::Reg<rxipv6_no_payload_frames::Rxipv6NoPayloadFramesSpec>;
#[doc = "Receive IPV6 No Payload Frame Counter Register"]
pub mod rxipv6_no_payload_frames;
#[doc = "RXUDP_GOOD_FRAMES (r) register accessor: RxUDP Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_good_frames`]
module"]
#[doc(alias = "RXUDP_GOOD_FRAMES")]
pub type RxudpGoodFrames = crate::Reg<rxudp_good_frames::RxudpGoodFramesSpec>;
#[doc = "RxUDP Good Frames Register"]
pub mod rxudp_good_frames;
#[doc = "RXUDP_ERROR_FRAMES (r) register accessor: RxUDP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_error_frames`]
module"]
#[doc(alias = "RXUDP_ERROR_FRAMES")]
pub type RxudpErrorFrames = crate::Reg<rxudp_error_frames::RxudpErrorFramesSpec>;
#[doc = "RxUDP Error Frames Register"]
pub mod rxudp_error_frames;
#[doc = "RXTCP_GOOD_FRAMES (r) register accessor: RxTCP Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_good_frames`]
module"]
#[doc(alias = "RXTCP_GOOD_FRAMES")]
pub type RxtcpGoodFrames = crate::Reg<rxtcp_good_frames::RxtcpGoodFramesSpec>;
#[doc = "RxTCP Good Frames Register"]
pub mod rxtcp_good_frames;
#[doc = "RXTCP_ERROR_FRAMES (r) register accessor: RxTCP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_error_frames`]
module"]
#[doc(alias = "RXTCP_ERROR_FRAMES")]
pub type RxtcpErrorFrames = crate::Reg<rxtcp_error_frames::RxtcpErrorFramesSpec>;
#[doc = "RxTCP Error Frames Register"]
pub mod rxtcp_error_frames;
#[doc = "RXICMP_GOOD_FRAMES (r) register accessor: RxICMP Good Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_good_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_good_frames`]
module"]
#[doc(alias = "RXICMP_GOOD_FRAMES")]
pub type RxicmpGoodFrames = crate::Reg<rxicmp_good_frames::RxicmpGoodFramesSpec>;
#[doc = "RxICMP Good Frames Register"]
pub mod rxicmp_good_frames;
#[doc = "RXICMP_ERROR_FRAMES (r) register accessor: RxICMP Error Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_error_frames::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_error_frames`]
module"]
#[doc(alias = "RXICMP_ERROR_FRAMES")]
pub type RxicmpErrorFrames = crate::Reg<rxicmp_error_frames::RxicmpErrorFramesSpec>;
#[doc = "RxICMP Error Frames Register"]
pub mod rxicmp_error_frames;
#[doc = "RXIPV4_GOOD_OCTETS (r) register accessor: RxIPv4 Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_good_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_good_octets`]
module"]
#[doc(alias = "RXIPV4_GOOD_OCTETS")]
pub type Rxipv4GoodOctets = crate::Reg<rxipv4_good_octets::Rxipv4GoodOctetsSpec>;
#[doc = "RxIPv4 Good Octets Register"]
pub mod rxipv4_good_octets;
#[doc = "RXIPV4_HEADER_ERROR_OCTETS (r) register accessor: Receive IPV4 Header Error Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_header_error_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_header_error_octets`]
module"]
#[doc(alias = "RXIPV4_HEADER_ERROR_OCTETS")]
pub type Rxipv4HeaderErrorOctets = crate::Reg<rxipv4_header_error_octets::Rxipv4HeaderErrorOctetsSpec>;
#[doc = "Receive IPV4 Header Error Octet Counter Register"]
pub mod rxipv4_header_error_octets;
#[doc = "RXIPV4_NO_PAYLOAD_OCTETS (r) register accessor: Receive IPV4 No Payload Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_no_payload_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_no_payload_octets`]
module"]
#[doc(alias = "RXIPV4_NO_PAYLOAD_OCTETS")]
pub type Rxipv4NoPayloadOctets = crate::Reg<rxipv4_no_payload_octets::Rxipv4NoPayloadOctetsSpec>;
#[doc = "Receive IPV4 No Payload Octet Counter Register"]
pub mod rxipv4_no_payload_octets;
#[doc = "RXIPV4_FRAGMENTED_OCTETS (r) register accessor: Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_fragmented_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_fragmented_octets`]
module"]
#[doc(alias = "RXIPV4_FRAGMENTED_OCTETS")]
pub type Rxipv4FragmentedOctets = crate::Reg<rxipv4_fragmented_octets::Rxipv4FragmentedOctetsSpec>;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_fragmented_octets;
#[doc = "RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS (r) register accessor: Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udp_checksum_disable_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_udp_checksum_disable_octets`]
module"]
#[doc(alias = "RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS")]
pub type Rxipv4UdpChecksumDisableOctets = crate::Reg<rxipv4_udp_checksum_disable_octets::Rxipv4UdpChecksumDisableOctetsSpec>;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_udp_checksum_disable_octets;
#[doc = "RXIPV6_GOOD_OCTETS (r) register accessor: RxIPv6 Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_good_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_good_octets`]
module"]
#[doc(alias = "RXIPV6_GOOD_OCTETS")]
pub type Rxipv6GoodOctets = crate::Reg<rxipv6_good_octets::Rxipv6GoodOctetsSpec>;
#[doc = "RxIPv6 Good Octets Register"]
pub mod rxipv6_good_octets;
#[doc = "RXIPV6_HEADER_ERROR_OCTETS (r) register accessor: Receive IPV6 Header Error Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_header_error_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_header_error_octets`]
module"]
#[doc(alias = "RXIPV6_HEADER_ERROR_OCTETS")]
pub type Rxipv6HeaderErrorOctets = crate::Reg<rxipv6_header_error_octets::Rxipv6HeaderErrorOctetsSpec>;
#[doc = "Receive IPV6 Header Error Octet Counter Register"]
pub mod rxipv6_header_error_octets;
#[doc = "RXIPV6_NO_PAYLOAD_OCTETS (r) register accessor: Receive IPV6 No Payload Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_no_payload_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_no_payload_octets`]
module"]
#[doc(alias = "RXIPV6_NO_PAYLOAD_OCTETS")]
pub type Rxipv6NoPayloadOctets = crate::Reg<rxipv6_no_payload_octets::Rxipv6NoPayloadOctetsSpec>;
#[doc = "Receive IPV6 No Payload Octet Counter Register"]
pub mod rxipv6_no_payload_octets;
#[doc = "RXUDP_GOOD_OCTETS (r) register accessor: Receive UDP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_good_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_good_octets`]
module"]
#[doc(alias = "RXUDP_GOOD_OCTETS")]
pub type RxudpGoodOctets = crate::Reg<rxudp_good_octets::RxudpGoodOctetsSpec>;
#[doc = "Receive UDP Good Octets Register"]
pub mod rxudp_good_octets;
#[doc = "RXUDP_ERROR_OCTETS (r) register accessor: Receive UDP Error Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_error_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_error_octets`]
module"]
#[doc(alias = "RXUDP_ERROR_OCTETS")]
pub type RxudpErrorOctets = crate::Reg<rxudp_error_octets::RxudpErrorOctetsSpec>;
#[doc = "Receive UDP Error Octets Register"]
pub mod rxudp_error_octets;
#[doc = "RXTCP_GOOD_OCTETS (r) register accessor: Receive TCP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_good_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_good_octets`]
module"]
#[doc(alias = "RXTCP_GOOD_OCTETS")]
pub type RxtcpGoodOctets = crate::Reg<rxtcp_good_octets::RxtcpGoodOctetsSpec>;
#[doc = "Receive TCP Good Octets Register"]
pub mod rxtcp_good_octets;
#[doc = "RXTCP_ERROR_OCTETS (r) register accessor: Receive TCP Error Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_error_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_error_octets`]
module"]
#[doc(alias = "RXTCP_ERROR_OCTETS")]
pub type RxtcpErrorOctets = crate::Reg<rxtcp_error_octets::RxtcpErrorOctetsSpec>;
#[doc = "Receive TCP Error Octets Register"]
pub mod rxtcp_error_octets;
#[doc = "RXICMP_GOOD_OCTETS (r) register accessor: Receive ICMP Good Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_good_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_good_octets`]
module"]
#[doc(alias = "RXICMP_GOOD_OCTETS")]
pub type RxicmpGoodOctets = crate::Reg<rxicmp_good_octets::RxicmpGoodOctetsSpec>;
#[doc = "Receive ICMP Good Octets Register"]
pub mod rxicmp_good_octets;
#[doc = "RXICMP_ERROR_OCTETS (r) register accessor: Receive ICMP Error Octets Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_error_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_error_octets`]
module"]
#[doc(alias = "RXICMP_ERROR_OCTETS")]
pub type RxicmpErrorOctets = crate::Reg<rxicmp_error_octets::RxicmpErrorOctetsSpec>;
#[doc = "Receive ICMP Error Octets Register"]
pub mod rxicmp_error_octets;
#[doc = "TIMESTAMP_CONTROL (rw) register accessor: Timestamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_control`]
module"]
#[doc(alias = "TIMESTAMP_CONTROL")]
pub type TimestampControl = crate::Reg<timestamp_control::TimestampControlSpec>;
#[doc = "Timestamp Control Register"]
pub mod timestamp_control;
#[doc = "SUB_SECOND_INCREMENT (rw) register accessor: Sub-Second Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sub_second_increment::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sub_second_increment::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub_second_increment`]
module"]
#[doc(alias = "SUB_SECOND_INCREMENT")]
pub type SubSecondIncrement = crate::Reg<sub_second_increment::SubSecondIncrementSpec>;
#[doc = "Sub-Second Increment Register"]
pub mod sub_second_increment;
#[doc = "SYSTEM_TIME_SECONDS (r) register accessor: System Time - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_seconds::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_seconds`]
module"]
#[doc(alias = "SYSTEM_TIME_SECONDS")]
pub type SystemTimeSeconds = crate::Reg<system_time_seconds::SystemTimeSecondsSpec>;
#[doc = "System Time - Seconds Register"]
pub mod system_time_seconds;
#[doc = "SYSTEM_TIME_NANOSECONDS (r) register accessor: System Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_nanoseconds::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_nanoseconds`]
module"]
#[doc(alias = "SYSTEM_TIME_NANOSECONDS")]
pub type SystemTimeNanoseconds = crate::Reg<system_time_nanoseconds::SystemTimeNanosecondsSpec>;
#[doc = "System Time Nanoseconds Register"]
pub mod system_time_nanoseconds;
#[doc = "SYSTEM_TIME_SECONDS_UPDATE (rw) register accessor: System Time - Seconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_seconds_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_seconds_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_seconds_update`]
module"]
#[doc(alias = "SYSTEM_TIME_SECONDS_UPDATE")]
pub type SystemTimeSecondsUpdate = crate::Reg<system_time_seconds_update::SystemTimeSecondsUpdateSpec>;
#[doc = "System Time - Seconds Update Register"]
pub mod system_time_seconds_update;
#[doc = "SYSTEM_TIME_NANOSECONDS_UPDATE (rw) register accessor: System Time Nanoseconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_nanoseconds_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_nanoseconds_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_nanoseconds_update`]
module"]
#[doc(alias = "SYSTEM_TIME_NANOSECONDS_UPDATE")]
pub type SystemTimeNanosecondsUpdate = crate::Reg<system_time_nanoseconds_update::SystemTimeNanosecondsUpdateSpec>;
#[doc = "System Time Nanoseconds Update Register"]
pub mod system_time_nanoseconds_update;
#[doc = "TIMESTAMP_ADDEND (rw) register accessor: Timestamp Addend Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_addend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_addend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_addend`]
module"]
#[doc(alias = "TIMESTAMP_ADDEND")]
pub type TimestampAddend = crate::Reg<timestamp_addend::TimestampAddendSpec>;
#[doc = "Timestamp Addend Register"]
pub mod timestamp_addend;
#[doc = "TARGET_TIME_SECONDS (rw) register accessor: Target Time Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_seconds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_seconds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_seconds`]
module"]
#[doc(alias = "TARGET_TIME_SECONDS")]
pub type TargetTimeSeconds = crate::Reg<target_time_seconds::TargetTimeSecondsSpec>;
#[doc = "Target Time Seconds Register"]
pub mod target_time_seconds;
#[doc = "TARGET_TIME_NANOSECONDS (rw) register accessor: Target Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_nanoseconds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_nanoseconds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_nanoseconds`]
module"]
#[doc(alias = "TARGET_TIME_NANOSECONDS")]
pub type TargetTimeNanoseconds = crate::Reg<target_time_nanoseconds::TargetTimeNanosecondsSpec>;
#[doc = "Target Time Nanoseconds Register"]
pub mod target_time_nanoseconds;
#[doc = "SYSTEM_TIME_HIGHER_WORD_SECONDS (rw) register accessor: System Time - Higher Word Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_higher_word_seconds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_higher_word_seconds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_higher_word_seconds`]
module"]
#[doc(alias = "SYSTEM_TIME_HIGHER_WORD_SECONDS")]
pub type SystemTimeHigherWordSeconds = crate::Reg<system_time_higher_word_seconds::SystemTimeHigherWordSecondsSpec>;
#[doc = "System Time - Higher Word Seconds Register"]
pub mod system_time_higher_word_seconds;
#[doc = "TIMESTAMP_STATUS (r) register accessor: Timestamp Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_status`]
module"]
#[doc(alias = "TIMESTAMP_STATUS")]
pub type TimestampStatus = crate::Reg<timestamp_status::TimestampStatusSpec>;
#[doc = "Timestamp Status Register"]
pub mod timestamp_status;
#[doc = "BUS_MODE (rw) register accessor: Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_mode`]
module"]
#[doc(alias = "BUS_MODE")]
pub type BusMode = crate::Reg<bus_mode::BusModeSpec>;
#[doc = "Bus Mode Register"]
pub mod bus_mode;
#[doc = "TRANSMIT_POLL_DEMAND (rw) register accessor: Transmit Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_poll_demand::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_poll_demand::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_poll_demand`]
module"]
#[doc(alias = "TRANSMIT_POLL_DEMAND")]
pub type TransmitPollDemand = crate::Reg<transmit_poll_demand::TransmitPollDemandSpec>;
#[doc = "Transmit Poll Demand Register"]
pub mod transmit_poll_demand;
#[doc = "RECEIVE_POLL_DEMAND (rw) register accessor: Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_poll_demand::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_poll_demand::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_poll_demand`]
module"]
#[doc(alias = "RECEIVE_POLL_DEMAND")]
pub type ReceivePollDemand = crate::Reg<receive_poll_demand::ReceivePollDemandSpec>;
#[doc = "Receive Poll Demand Register"]
pub mod receive_poll_demand;
#[doc = "RECEIVE_DESCRIPTOR_LIST_ADDRESS (rw) register accessor: Receive Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_descriptor_list_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_descriptor_list_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_descriptor_list_address`]
module"]
#[doc(alias = "RECEIVE_DESCRIPTOR_LIST_ADDRESS")]
pub type ReceiveDescriptorListAddress = crate::Reg<receive_descriptor_list_address::ReceiveDescriptorListAddressSpec>;
#[doc = "Receive Descriptor Address Register"]
pub mod receive_descriptor_list_address;
#[doc = "TRANSMIT_DESCRIPTOR_LIST_ADDRESS (rw) register accessor: Transmit descripter Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_descriptor_list_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_descriptor_list_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_descriptor_list_address`]
module"]
#[doc(alias = "TRANSMIT_DESCRIPTOR_LIST_ADDRESS")]
pub type TransmitDescriptorListAddress = crate::Reg<transmit_descriptor_list_address::TransmitDescriptorListAddressSpec>;
#[doc = "Transmit descripter Address Register"]
pub mod transmit_descriptor_list_address;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "OPERATION_MODE (rw) register accessor: Operation Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operation_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operation_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_mode`]
module"]
#[doc(alias = "OPERATION_MODE")]
pub type OperationMode = crate::Reg<operation_mode::OperationModeSpec>;
#[doc = "Operation Mode Register"]
pub mod operation_mode;
#[doc = "INTERRUPT_ENABLE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`]
module"]
#[doc(alias = "INTERRUPT_ENABLE")]
pub type InterruptEnable = crate::Reg<interrupt_enable::InterruptEnableSpec>;
#[doc = "Interrupt Enable Register"]
pub mod interrupt_enable;
#[doc = "MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER (r) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`missed_frame_and_buffer_overflow_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@missed_frame_and_buffer_overflow_counter`]
module"]
#[doc(alias = "MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER")]
pub type MissedFrameAndBufferOverflowCounter = crate::Reg<missed_frame_and_buffer_overflow_counter::MissedFrameAndBufferOverflowCounterSpec>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod missed_frame_and_buffer_overflow_counter;
#[doc = "RECEIVE_INTERRUPT_WATCHDOG_TIMER (rw) register accessor: Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_interrupt_watchdog_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_interrupt_watchdog_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_interrupt_watchdog_timer`]
module"]
#[doc(alias = "RECEIVE_INTERRUPT_WATCHDOG_TIMER")]
pub type ReceiveInterruptWatchdogTimer = crate::Reg<receive_interrupt_watchdog_timer::ReceiveInterruptWatchdogTimerSpec>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod receive_interrupt_watchdog_timer;
#[doc = "AHB_STATUS (r) register accessor: AHB Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_status`]
module"]
#[doc(alias = "AHB_STATUS")]
pub type AhbStatus = crate::Reg<ahb_status::AhbStatusSpec>;
#[doc = "AHB Status Register"]
pub mod ahb_status;
#[doc = "CURRENT_HOST_TRANSMIT_DESCRIPTOR (r) register accessor: Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_transmit_descriptor::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_transmit_descriptor`]
module"]
#[doc(alias = "CURRENT_HOST_TRANSMIT_DESCRIPTOR")]
pub type CurrentHostTransmitDescriptor = crate::Reg<current_host_transmit_descriptor::CurrentHostTransmitDescriptorSpec>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod current_host_transmit_descriptor;
#[doc = "CURRENT_HOST_RECEIVE_DESCRIPTOR (r) register accessor: Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_receive_descriptor::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_receive_descriptor`]
module"]
#[doc(alias = "CURRENT_HOST_RECEIVE_DESCRIPTOR")]
pub type CurrentHostReceiveDescriptor = crate::Reg<current_host_receive_descriptor::CurrentHostReceiveDescriptorSpec>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod current_host_receive_descriptor;
#[doc = "CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS (r) register accessor: Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_transmit_buffer_address::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_transmit_buffer_address`]
module"]
#[doc(alias = "CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS")]
pub type CurrentHostTransmitBufferAddress = crate::Reg<current_host_transmit_buffer_address::CurrentHostTransmitBufferAddressSpec>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod current_host_transmit_buffer_address;
#[doc = "CURRENT_HOST_RECEIVE_BUFFER_ADDRESS (r) register accessor: Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_host_receive_buffer_address::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_receive_buffer_address`]
module"]
#[doc(alias = "CURRENT_HOST_RECEIVE_BUFFER_ADDRESS")]
pub type CurrentHostReceiveBufferAddress = crate::Reg<current_host_receive_buffer_address::CurrentHostReceiveBufferAddressSpec>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod current_host_receive_buffer_address;
#[doc = "HW_FEATURE (rw) register accessor: HW Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_feature::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_feature::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_feature`]
module"]
#[doc(alias = "HW_FEATURE")]
pub type HwFeature = crate::Reg<hw_feature::HwFeatureSpec>;
#[doc = "HW Feature Register"]
pub mod hw_feature;
