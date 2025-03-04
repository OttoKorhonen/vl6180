
pub enum SysRangeAddress {
    SysRangeStart = 0x018,
    SysRangeThreshHigh = 0x019,
    SysRangeThreshLow = 0x01A,
    SysRangeIntermeasurementPeriod = 0x01B,
    SysRangeMaxConvergenceTime = 0x01C,
    SysRangeCrossTalkCompensationRate = 0x01E,
    SysRangeCrossTalkValidHeight = 0x021,
    SysRangeConvergenceEstimate = 0x022,
    SysRangePartToPartRangeOffset = 0x024,
    SysRangeRangeIgnoreValidHeight = 0x025,
    SysRangeRangeIgnoreThreshold = 0x026,
    SysRangeMaxAmbientLevelMult = 0x02C,
    SysRangeRangeCheckEnables = 0x002D,
    SysRangeVHVRecalibrate = 0x02E,
    SysRangeVHVRepeatRate = 0x031
}