
pub enum ResultAddress {
    ResultRangeStatus = 0x04D,
    ResultInterruptStatusGpio = 0x04F,
    ResultHistoryBuffer = 0x052, //0x052-0x060
    ResultRangeVal = 0x062,
    ResultRangeRaw = 0x064,
    ResultRangeReturnRate = 0x066,
    ResultRangeReferenceRate = 0x068,
    ResultRangeReturnSignalCount = 0x06C,
    ResultRangeReferenceSignalCount = 0x070,
    ResultRangeReturnAmbCount = 0x074,
    ResultRangeReferenceAmbCount = 0x078,
    ResultRangeReturnConvTime = 0x07C,
    ResultRangeReferenceConvTime = 0x080,
}