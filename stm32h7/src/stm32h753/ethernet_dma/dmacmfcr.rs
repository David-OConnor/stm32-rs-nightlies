#[doc = "Register `DMACMFCR` reader"]
pub struct R(crate::R<DMACMFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACMFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACMFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACMFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFC` reader - Dropped Packet Counters"]
pub type MFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter"]
pub type MFCO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacmfcr](index.html) module"]
pub struct DMACMFCR_SPEC;
impl crate::RegisterSpec for DMACMFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacmfcr::R](R) reader structure"]
impl crate::Readable for DMACMFCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACMFCR to value 0"]
impl crate::Resettable for DMACMFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
