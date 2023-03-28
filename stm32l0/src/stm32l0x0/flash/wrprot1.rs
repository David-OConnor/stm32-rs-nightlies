#[doc = "Register `WRPROT1` reader"]
pub struct R(crate::R<WRPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPROT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRPROT1` reader - Write protection"]
pub type WRPROT1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrprot1(&self) -> WRPROT1_R {
        WRPROT1_R::new(self.bits)
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprot1](index.html) module"]
pub struct WRPROT1_SPEC;
impl crate::RegisterSpec for WRPROT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrprot1::R](R) reader structure"]
impl crate::Readable for WRPROT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRPROT1 to value 0"]
impl crate::Resettable for WRPROT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
