#[doc = "Register `HPMS` reader"]
pub struct R(crate::R<HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIDX` reader - BIDX"]
pub type BIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSI` reader - MSI"]
pub type MSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIDX` reader - FIDX"]
pub type FIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLST` reader - FLST"]
pub type FLST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - BIDX"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - MSI"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - FIDX"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - FLST"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpms](index.html) module"]
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpms::R](R) reader structure"]
impl crate::Readable for HPMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HPMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
