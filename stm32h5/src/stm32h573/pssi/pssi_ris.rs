#[doc = "Register `PSSI_RIS` reader"]
pub struct R(crate::R<PSSI_RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSI_RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSI_RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSI_RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVR_RIS` reader - Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR."]
pub type OVR_RIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR."]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi_ris](index.html) module"]
pub struct PSSI_RIS_SPEC;
impl crate::RegisterSpec for PSSI_RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssi_ris::R](R) reader structure"]
impl crate::Readable for PSSI_RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSSI_RIS to value 0"]
impl crate::Resettable for PSSI_RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
