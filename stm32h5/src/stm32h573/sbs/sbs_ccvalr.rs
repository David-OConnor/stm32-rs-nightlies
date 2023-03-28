#[doc = "Register `SBS_CCVALR` reader"]
pub struct R(crate::R<SBS_CCVALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_CCVALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_CCVALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_CCVALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ANSRC1` reader - compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type ANSRC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APSRC1` reader - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type APSRC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANSRC2` reader - Compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type ANSRC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APSRC2` reader - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type APSRC2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn ansrc1(&self) -> ANSRC1_R {
        ANSRC1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn apsrc1(&self) -> APSRC1_R {
        APSRC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn ansrc2(&self) -> ANSRC2_R {
        ANSRC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn apsrc2(&self) -> APSRC2_R {
        APSRC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "SBS compensation cell for I/Os value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_ccvalr](index.html) module"]
pub struct SBS_CCVALR_SPEC;
impl crate::RegisterSpec for SBS_CCVALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_ccvalr::R](R) reader structure"]
impl crate::Readable for SBS_CCVALR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SBS_CCVALR to value 0x88"]
impl crate::Resettable for SBS_CCVALR_SPEC {
    const RESET_VALUE: Self::Ux = 0x88;
}
