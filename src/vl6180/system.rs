
///enum for system addresses
pub enum SystemAddress {
    SystemModeGpio0 = 0x010,
    SystemModeGpio1 = 0x011,
    SystemHistoryControl = 0x012,
    SystemInterruptConfigGpio = 0x014,
    SystemInterruptClear = 0x015,
    SystemFreshOutOfReset = 0x016,
    SystemGroupedParameterHold = 0x017,
}

///enum to reset system gpios
pub enum SystemReset {
    ResetGpio0 = 0x60,
    ResetGpio1 = 0x20
}