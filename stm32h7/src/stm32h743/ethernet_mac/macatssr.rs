#[doc = "Register `MACATSSR` reader"]
pub struct R(crate::R<MACATSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACATSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACATSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACATSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AUXTSHI` reader - Auxiliary Timestamp"]
pub type AUXTSHI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Auxiliary Timestamp"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
#[doc = "Auxiliary timestamp seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macatssr](index.html) module"]
pub struct MACATSSR_SPEC;
impl crate::RegisterSpec for MACATSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macatssr::R](R) reader structure"]
impl crate::Readable for MACATSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACATSSR to value 0"]
impl crate::Resettable for MACATSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
