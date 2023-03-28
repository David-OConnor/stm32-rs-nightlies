#[doc = "Register `PMSR` reader"]
pub struct R(crate::R<PMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOPF` reader - Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `SBF` reader - System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
pub type SBF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 5 - Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "PWR status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmsr](index.html) module"]
pub struct PMSR_SPEC;
impl crate::RegisterSpec for PMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmsr::R](R) reader structure"]
impl crate::Readable for PMSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMSR to value 0"]
impl crate::Resettable for PMSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
