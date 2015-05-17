use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Analysis_t {
    pub AnalysisType: AnalysisType_t,
    pub StepTime: Time_t,
    pub SlotTime: Time_t,
    pub SlotLength: Quantity_t,
    pub CurrentTime: Quantity_t,
    pub InitialTemperature: Temperature_t,
}
