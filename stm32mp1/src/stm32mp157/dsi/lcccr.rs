#[doc = "Register `LCCCR` reader"]
pub struct R(crate::R<LCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COLC` reader - COLC"]
pub type COLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPE` reader - LPE"]
pub type LPE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host LTDC current color coding register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcccr](index.html) module"]
pub struct LCCCR_SPEC;
impl crate::RegisterSpec for LCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcccr::R](R) reader structure"]
impl crate::Readable for LCCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCCCR to value 0"]
impl crate::Resettable for LCCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
