#[doc = "Register `PSSI_MIS` reader"]
pub struct R(crate::R<PSSI_MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSI_MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSI_MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSI_MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVR_MIS` reader - Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1."]
pub type OVR_MIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1."]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi_mis](index.html) module"]
pub struct PSSI_MIS_SPEC;
impl crate::RegisterSpec for PSSI_MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssi_mis::R](R) reader structure"]
impl crate::Readable for PSSI_MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSSI_MIS to value 0"]
impl crate::Resettable for PSSI_MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
