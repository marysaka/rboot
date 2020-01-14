use register::mmio::*;

#[allow(non_snake_case)]
#[repr(C)]
pub struct TSEC {
    pub TSEC_THI_INCR_SYNCPT: ReadWrite<u32>,  // 0x0
    pub TSEC_THI_INCR_SYNCPT_CTRL: ReadWrite<u32>,  // 0x4
    pub TSEC_THI_INCR_SYNCPT_ERR: ReadWrite<u32>,  // 0x8
    pub TSEC_THI_CTXSW_INCR_SYNCPT: ReadWrite<u32>,  // 0xc
    reserved4: [u8; 0x10],
    pub TSEC_THI_CTXSW: ReadWrite<u32>,  // 0x20
    pub TSEC_THI_CTXSW_NEXT: ReadWrite<u32>,  // 0x24
    pub TSEC_THI_CONT_SYNCPT_EOF: ReadWrite<u32>,  // 0x28
    pub TSEC_THI_CONT_SYNCPT_L1: ReadWrite<u32>,  // 0x2c
    pub TSEC_THI_STREAMID0: ReadWrite<u32>,  // 0x30
    pub TSEC_THI_STREAMID1: ReadWrite<u32>,  // 0x34
    pub TSEC_THI_THI_SEC: ReadWrite<u32>,  // 0x38
    reserved11: [u8; 0x4],
    pub TSEC_THI_METHOD0: ReadWrite<u32>,  // 0x40
    pub TSEC_THI_METHOD1: ReadWrite<u32>,  // 0x44
    reserved13: [u8; 0x18],
    pub TSEC_THI_CONTEXT_SWITCH: ReadWrite<u32>,  // 0x60
    reserved14: [u8; 0x14],
    pub TSEC_THI_INT_STATUS: ReadWrite<u32>,  // 0x78
    pub TSEC_THI_INT_MASK: ReadWrite<u32>,  // 0x7c
    pub TSEC_THI_CONFIG0: ReadWrite<u32>,  // 0x80
    pub TSEC_THI_DBG_MISC: ReadWrite<u32>,  // 0x84
    pub TSEC_THI_SLCG_OVERRIDE_HIGH_A: ReadWrite<u32>,  // 0x88
    pub TSEC_THI_SLCG_OVERRIDE_LOW_A: ReadWrite<u32>,  // 0x8c
    reserved20: [u8; 0xd70],
    pub TSEC_THI_CLK_OVERRIDE: ReadWrite<u32>,  // 0xe00
    reserved21: [u8; 0x1fc],
    pub FALCON_IRQSSET: ReadWrite<u32>,  // 0x1000
    pub FALCON_IRQSCLR: ReadWrite<u32>,  // 0x1004
    pub FALCON_IRQSTAT: ReadWrite<u32>,  // 0x1008
    pub FALCON_IRQMODE: ReadWrite<u32>,  // 0x100c
    pub FALCON_IRQMSET: ReadWrite<u32>,  // 0x1010
    pub FALCON_IRQMCLR: ReadWrite<u32>,  // 0x1014
    pub FALCON_IRQMASK: ReadWrite<u32>,  // 0x1018
    pub FALCON_IRQDEST: ReadWrite<u32>,  // 0x101c
    pub FALCON_GPTMRINT: ReadWrite<u32>,  // 0x1020
    pub FALCON_GPTMRVAL: ReadWrite<u32>,  // 0x1024
    pub FALCON_GPTMRCTL: ReadWrite<u32>,  // 0x1028
    pub FALCON_PTIMER0: ReadWrite<u32>,  // 0x102c
    pub FALCON_PTIMER1: ReadWrite<u32>,  // 0x1030
    pub FALCON_WDTMRVAL: ReadWrite<u32>,  // 0x1034
    pub FALCON_WDTMRCTL: ReadWrite<u32>,  // 0x1038
    pub FALCON_IRQDEST2: ReadWrite<u32>,  // 0x103c
    pub FALCON_MAILBOX0: ReadWrite<u32>,  // 0x1040
    pub FALCON_MAILBOX1: ReadWrite<u32>,  // 0x1044
    pub FALCON_ITFEN: ReadWrite<u32>,  // 0x1048
    pub FALCON_IDLESTATE: ReadWrite<u32>,  // 0x104c
    pub FALCON_CURCTX: ReadWrite<u32>,  // 0x1050
    pub FALCON_NXTCTX: ReadWrite<u32>,  // 0x1054
    pub FALCON_CTXACK: ReadWrite<u32>,  // 0x1058
    pub FALCON_FHSTATE: ReadWrite<u32>,  // 0x105c
    pub FALCON_PRIVSTATE: ReadWrite<u32>,  // 0x1060
    pub FALCON_MTHDDATA: ReadWrite<u32>,  // 0x1064
    pub FALCON_MTHDID: ReadWrite<u32>,  // 0x1068
    pub FALCON_MTHDWDAT: ReadWrite<u32>,  // 0x106c
    pub FALCON_MTHDCOUNT: ReadWrite<u32>,  // 0x1070
    pub FALCON_MTHDPOP: ReadWrite<u32>,  // 0x1074
    pub FALCON_MTHDRAMSZ: ReadWrite<u32>,  // 0x1078
    pub FALCON_SFTRESET: ReadWrite<u32>,  // 0x107c
    pub FALCON_OS: ReadWrite<u32>,  // 0x1080
    pub FALCON_RM: ReadWrite<u32>,  // 0x1084
    pub FALCON_SOFT_PM: ReadWrite<u32>,  // 0x1088
    pub FALCON_SOFT_MODE: ReadWrite<u32>,  // 0x108c
    pub FALCON_DEBUG1: ReadWrite<u32>,  // 0x1090
    pub FALCON_DEBUGINFO: ReadWrite<u32>,  // 0x1094
    pub FALCON_IBRKPT1: ReadWrite<u32>,  // 0x1098
    pub FALCON_IBRKPT2: ReadWrite<u32>,  // 0x109c
    pub FALCON_CGCTL: ReadWrite<u32>,  // 0x10a0
    pub FALCON_ENGCTL: ReadWrite<u32>,  // 0x10a4
    pub FALCON_PMM: ReadWrite<u32>,  // 0x10a8
    pub FALCON_ADDR: ReadWrite<u32>,  // 0x10ac
    pub FALCON_IBRKPT3: ReadWrite<u32>,  // 0x10b0
    pub FALCON_IBRKPT4: ReadWrite<u32>,  // 0x10b4
    pub FALCON_IBRKPT5: ReadWrite<u32>,  // 0x10b8
    reserved68: [u8; 0x14],
    pub FALCON_EXCI: ReadWrite<u32>,  // 0x10d0
    pub FALCON_SVEC_SPR: ReadWrite<u32>,  // 0x10d4
    pub FALCON_RSTAT0: ReadWrite<u32>,  // 0x10d8
    pub FALCON_RSTAT3: ReadWrite<u32>,  // 0x10dc
    pub FALCON_UNK_E0: ReadWrite<u32>,  // 0x10e0
    reserved73: [u8; 0x1c],
    pub FALCON_CPUCTL: ReadWrite<u32>,  // 0x1100
    pub FALCON_BOOTVEC: ReadWrite<u32>,  // 0x1104
    pub FALCON_HWCFG: ReadWrite<u32>,  // 0x1108
    pub FALCON_DMACTL: ReadWrite<u32>,  // 0x110c
    pub FALCON_DMATRFBASE: ReadWrite<u32>,  // 0x1110
    pub FALCON_DMATRFMOFFS: ReadWrite<u32>,  // 0x1114
    pub FALCON_DMATRFCMD: ReadWrite<u32>,  // 0x1118
    pub FALCON_DMATRFFBOFFS: ReadWrite<u32>,  // 0x111c
    pub FALCON_DMAPOLL_FB: ReadWrite<u32>,  // 0x1120
    pub FALCON_DMAPOLL_CP: ReadWrite<u32>,  // 0x1124
    pub FALCON_DBG_STATE: ReadWrite<u32>,  // 0x1128
    pub FALCON_HWCFG1: ReadWrite<u32>,  // 0x112c
    pub FALCON_CPUCTL_ALIAS: ReadWrite<u32>,  // 0x1130
    reserved86: [u8; 0x4],
    pub FALCON_STACKCFG: ReadWrite<u32>,  // 0x1138
    reserved87: [u8; 0x4],
    pub FALCON_IMCTL: ReadWrite<u32>,  // 0x1140
    pub FALCON_IMSTAT: ReadWrite<u32>,  // 0x1144
    pub FALCON_TRACEIDX: ReadWrite<u32>,  // 0x1148
    pub FALCON_TRACEPC: ReadWrite<u32>,  // 0x114c
    pub FALCON_IMFILLRNG0: ReadWrite<u32>,  // 0x1150
    pub FALCON_IMFILLRNG1: ReadWrite<u32>,  // 0x1154
    pub FALCON_IMFILLCTL: ReadWrite<u32>,  // 0x1158
    pub FALCON_IMCTL_DEBUG: ReadWrite<u32>,  // 0x115c
    pub FALCON_CMEMBASE: ReadWrite<u32>,  // 0x1160
    pub FALCON_DMEMAPERT: ReadWrite<u32>,  // 0x1164
    pub FALCON_EXTERRADDR: ReadWrite<u32>,  // 0x1168
    pub FALCON_EXTERRSTAT: ReadWrite<u32>,  // 0x116c
    reserved99: [u8; 0xc],
    pub FALCON_CG2: ReadWrite<u32>,  // 0x117c
    pub FALCON_IMEMC0: ReadWrite<u32>,  // 0x1180
    pub FALCON_IMEMD0: ReadWrite<u32>,  // 0x1184
    pub FALCON_IMEMT0: ReadWrite<u32>,  // 0x1188
    reserved103: [u8; 0x4],
    pub FALCON_IMEMC1: ReadWrite<u32>,  // 0x1190
    pub FALCON_IMEMD1: ReadWrite<u32>,  // 0x1194
    pub FALCON_IMEMT1: ReadWrite<u32>,  // 0x1198
    reserved106: [u8; 0x4],
    pub FALCON_IMEMC2: ReadWrite<u32>,  // 0x11a0
    pub FALCON_IMEMD2: ReadWrite<u32>,  // 0x11a4
    pub FALCON_IMEMT2: ReadWrite<u32>,  // 0x11a8
    reserved109: [u8; 0x4],
    pub FALCON_IMEMC3: ReadWrite<u32>,  // 0x11b0
    pub FALCON_IMEMD3: ReadWrite<u32>,  // 0x11b4
    pub FALCON_IMEMT3: ReadWrite<u32>,  // 0x11b8
    reserved112: [u8; 0x4],
    pub FALCON_DMEMC0: ReadWrite<u32>,  // 0x11c0
    pub FALCON_DMEMD0: ReadWrite<u32>,  // 0x11c4
    pub FALCON_DMEMC1: ReadWrite<u32>,  // 0x11c8
    pub FALCON_DMEMD1: ReadWrite<u32>,  // 0x11cc
    pub FALCON_DMEMC2: ReadWrite<u32>,  // 0x11d0
    pub FALCON_DMEMD2: ReadWrite<u32>,  // 0x11d4
    pub FALCON_DMEMC3: ReadWrite<u32>,  // 0x11d8
    pub FALCON_DMEMD3: ReadWrite<u32>,  // 0x11dc
    pub FALCON_DMEMC4: ReadWrite<u32>,  // 0x11e0
    pub FALCON_DMEMD4: ReadWrite<u32>,  // 0x11e4
    pub FALCON_DMEMC5: ReadWrite<u32>,  // 0x11e8
    pub FALCON_DMEMD5: ReadWrite<u32>,  // 0x11ec
    pub FALCON_DMEMC6: ReadWrite<u32>,  // 0x11f0
    pub FALCON_DMEMD6: ReadWrite<u32>,  // 0x11f4
    pub FALCON_DMEMC7: ReadWrite<u32>,  // 0x11f8
    pub FALCON_DMEMD7: ReadWrite<u32>,  // 0x11fc
    pub FALCON_ICD_CMD: ReadWrite<u32>,  // 0x1200
    pub FALCON_ICD_ADDR: ReadWrite<u32>,  // 0x1204
    pub FALCON_ICD_WDATA: ReadWrite<u32>,  // 0x1208
    pub FALCON_ICD_RDATA: ReadWrite<u32>,  // 0x120c
    reserved132: [u8; 0x30],
    pub FALCON_SCTL: ReadWrite<u32>,  // 0x1240
    pub FALCON_SSTAT: ReadWrite<u32>,  // 0x1244
    pub FALCON_UNK_248: ReadWrite<u32>,  // 0x1248
    pub FALCON_UNK_24C: ReadWrite<u32>,  // 0x124c
    pub FALCON_UNK_250: ReadWrite<u32>,  // 0x1250
    reserved137: [u8; 0xc],
    pub FALCON_UNK_260: ReadWrite<u32>,  // 0x1260
    reserved138: [u8; 0x1c],
    pub FALCON_SPROT_IMEM: ReadWrite<u32>,  // 0x1280
    pub FALCON_SPROT_DMEM: ReadWrite<u32>,  // 0x1284
    pub FALCON_SPROT_CPUCTL: ReadWrite<u32>,  // 0x1288
    pub FALCON_SPROT_MISC: ReadWrite<u32>,  // 0x128c
    pub FALCON_SPROT_IRQ: ReadWrite<u32>,  // 0x1290
    pub FALCON_SPROT_MTHD: ReadWrite<u32>,  // 0x1294
    pub FALCON_SPROT_SCTL: ReadWrite<u32>,  // 0x1298
    pub FALCON_SPROT_WDTMR: ReadWrite<u32>,  // 0x129c
    reserved146: [u8; 0x20],
    pub FALCON_DMAINFO_FINISHED_FBRD_LOW: ReadWrite<u32>,  // 0x12c0
    pub FALCON_DMAINFO_FINISHED_FBRD_HIGH: ReadWrite<u32>,  // 0x12c4
    pub FALCON_DMAINFO_FINISHED_FBWR_LOW: ReadWrite<u32>,  // 0x12c8
    pub FALCON_DMAINFO_FINISHED_FBWR_HIGH: ReadWrite<u32>,  // 0x12cc
    pub FALCON_DMAINFO_CURRENT_FBRD_LOW: ReadWrite<u32>,  // 0x12d0
    pub FALCON_DMAINFO_CURRENT_FBRD_HIGH: ReadWrite<u32>,  // 0x12d4
    pub FALCON_DMAINFO_CURRENT_FBWR_LOW: ReadWrite<u32>,  // 0x12d8
    pub FALCON_DMAINFO_CURRENT_FBWR_HIGH: ReadWrite<u32>,  // 0x12dc
    pub FALCON_DMAINFO_CTL: ReadWrite<u32>,  // 0x12e0
    reserved155: [u8; 0x11c],
    pub TSEC_SCP_CTL0: ReadWrite<u32>,  // 0x1400
    pub TSEC_SCP_CTL1: ReadWrite<u32>,  // 0x1404
    pub TSEC_SCP_CTL_STAT: ReadWrite<u32>,  // 0x1408
    pub TSEC_SCP_CTL_LOCK: ReadWrite<u32>,  // 0x140c
    pub TSEC_SCP_UNK_10: ReadWrite<u32>,  // 0x1410
    pub TSEC_SCP_UNK_14: ReadWrite<u32>,  // 0x1414
    pub TSEC_SCP_CTL_PKEY: ReadWrite<u32>,  // 0x1418
    pub TSEC_SCP_UNK_1C: ReadWrite<u32>,  // 0x141c
    pub TSEC_SCP_SEQ_CTL: ReadWrite<u32>,  // 0x1420
    pub TSEC_SCP_SEQ_VAL: ReadWrite<u32>,  // 0x1424
    pub TSEC_SCP_SEQ_STAT: ReadWrite<u32>,  // 0x1428
    reserved166: [u8; 0x4],
    pub TSEC_SCP_INSN_STAT: ReadWrite<u32>,  // 0x1430
    reserved167: [u8; 0x1c],
    pub TSEC_SCP_UNK_50: ReadWrite<u32>,  // 0x1450
    pub TSEC_SCP_AUTH_STAT: ReadWrite<u32>,  // 0x1454
    pub TSEC_SCP_AES_STAT: ReadWrite<u32>,  // 0x1458
    reserved170: [u8; 0x14],
    pub TSEC_SCP_UNK_70: ReadWrite<u32>,  // 0x1470
    reserved171: [u8; 0xc],
    pub TSEC_SCP_IRQSTAT: ReadWrite<u32>,  // 0x1480
    pub TSEC_SCP_IRQMASK: ReadWrite<u32>,  // 0x1484
    reserved173: [u8; 0x8],
    pub TSEC_SCP_ACL_ERR: ReadWrite<u32>,  // 0x1490
    pub TSEC_SCP_UNK_94: ReadWrite<u32>,  // 0x1494
    pub TSEC_SCP_INSN_ERR: ReadWrite<u32>,  // 0x1498
    reserved176: [u8; 0x64],
    pub TSEC_TRNG_CLK_LIMIT_LOW: ReadWrite<u32>,  // 0x1500
    pub TSEC_TRNG_CLK_LIMIT_HIGH: ReadWrite<u32>,  // 0x1504
    pub TSEC_TRNG_UNK_08: ReadWrite<u32>,  // 0x1508
    pub TSEC_TRNG_TEST_CTL: ReadWrite<u32>,  // 0x150c
    pub TSEC_TRNG_TEST_CFG0: ReadWrite<u32>,  // 0x1510
    pub TSEC_TRNG_TEST_SEED0: ReadWrite<u32>,  // 0x1514
    pub TSEC_TRNG_TEST_CFG1: ReadWrite<u32>,  // 0x1518
    pub TSEC_TRNG_TEST_SEED1: ReadWrite<u32>,  // 0x151c
    pub TSEC_TRNG_UNK_20: ReadWrite<u32>,  // 0x1520
    pub TSEC_TRNG_UNK_24: ReadWrite<u32>,  // 0x1524
    pub TSEC_TRNG_UNK_28: ReadWrite<u32>,  // 0x1528
    pub TSEC_TRNG_CTL: ReadWrite<u32>,  // 0x152c
    reserved188: [u8; 0xd0],
    pub TSEC_TFBIF_CTL: ReadWrite<u32>,  // 0x1600
    pub TSEC_TFBIF_MCCIF_FIFOCTRL: ReadWrite<u32>,  // 0x1604
    pub TSEC_TFBIF_THROTTLE: ReadWrite<u32>,  // 0x1608
    pub TSEC_TFBIF_DBG_STAT0: ReadWrite<u32>,  // 0x160c
    pub TSEC_TFBIF_DBG_STAT1: ReadWrite<u32>,  // 0x1610
    pub TSEC_TFBIF_DBG_RDCOUNT_LO: ReadWrite<u32>,  // 0x1614
    pub TSEC_TFBIF_DBG_RDCOUNT_HI: ReadWrite<u32>,  // 0x1618
    pub TSEC_TFBIF_DBG_WRCOUNT_LO: ReadWrite<u32>,  // 0x161c
    pub TSEC_TFBIF_DBG_WRCOUNT_HI: ReadWrite<u32>,  // 0x1620
    pub TSEC_TFBIF_DBG_R32COUNT: ReadWrite<u32>,  // 0x1624
    pub TSEC_TFBIF_DBG_R64COUNT: ReadWrite<u32>,  // 0x1628
    pub TSEC_TFBIF_DBG_R128COUNT: ReadWrite<u32>,  // 0x162c
    pub TSEC_TFBIF_UNK_30: ReadWrite<u32>,  // 0x1630
    pub TSEC_TFBIF_MCCIF_FIFOCTRL1: ReadWrite<u32>,  // 0x1634
    pub TSEC_TFBIF_WRR_RDP: ReadWrite<u32>,  // 0x1638
    reserved203: [u8; 0x4],
    pub TSEC_TFBIF_SPROT_EMEM: ReadWrite<u32>,  // 0x1640
    pub TSEC_TFBIF_TRANSCFG: ReadWrite<u32>,  // 0x1644
    pub TSEC_TFBIF_REGIONCFG: ReadWrite<u32>,  // 0x1648
    pub TSEC_TFBIF_ACTMON_ACTIVE_MASK: ReadWrite<u32>,  // 0x164c
    pub TSEC_TFBIF_ACTMON_ACTIVE_BORPS: ReadWrite<u32>,  // 0x1650
    pub TSEC_TFBIF_ACTMON_ACTIVE_WEIGHT: ReadWrite<u32>,  // 0x1654
    reserved209: [u8; 0x8],
    pub TSEC_TFBIF_ACTMON_MCB_MASK: ReadWrite<u32>,  // 0x1660
    pub TSEC_TFBIF_ACTMON_MCB_BORPS: ReadWrite<u32>,  // 0x1664
    pub TSEC_TFBIF_ACTMON_MCB_WEIGHT: ReadWrite<u32>,  // 0x1668
    reserved212: [u8; 0x4],
    pub TSEC_TFBIF_THI_TRANSPROP: ReadWrite<u32>,  // 0x1670
    reserved213: [u8; 0x5c],
    pub TSEC_CG: ReadWrite<u32>,  // 0x16d0
    reserved214: [u8; 0x2c],
    pub TSEC_BAR0_CTL: ReadWrite<u32>,  // 0x1700
    pub TSEC_BAR0_ADDR: ReadWrite<u32>,  // 0x1704
    pub TSEC_BAR0_DATA: ReadWrite<u32>,  // 0x1708
    pub TSEC_BAR0_TIMEOUT: ReadWrite<u32>,  // 0x170c
    reserved218: [u8; 0xf0],
    pub TSEC_TEGRA_FALCON_IP_VER: ReadWrite<u32>,  // 0x1800
    pub TSEC_TEGRA_UNK_04: ReadWrite<u32>,  // 0x1804
    pub TSEC_TEGRA_UNK_08: ReadWrite<u32>,  // 0x1808
    pub TSEC_TEGRA_UNK_0C: ReadWrite<u32>,  // 0x180c
    pub TSEC_TEGRA_UNK_10: ReadWrite<u32>,  // 0x1810
    pub TSEC_TEGRA_UNK_14: ReadWrite<u32>,  // 0x1814
    pub TSEC_TEGRA_UNK_18: ReadWrite<u32>,  // 0x1818
    pub TSEC_TEGRA_UNK_1C: ReadWrite<u32>,  // 0x181c
    pub TSEC_TEGRA_UNK_20: ReadWrite<u32>,  // 0x1820
    pub TSEC_TEGRA_UNK_24: ReadWrite<u32>,  // 0x1824
    pub TSEC_TEGRA_UNK_28: ReadWrite<u32>,  // 0x1828
    pub TSEC_TEGRA_UNK_2C: ReadWrite<u32>,  // 0x182c
    pub TSEC_TEGRA_UNK_30: ReadWrite<u32>,  // 0x1830
    pub TSEC_TEGRA_UNK_34: ReadWrite<u32>,  // 0x1834
    pub TSEC_TEGRA_CTL: ReadWrite<u32>,  // 0x1838
}
