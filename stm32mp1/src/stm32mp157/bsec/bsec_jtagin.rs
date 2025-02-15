#[doc = "Register `BSEC_JTAGIN` reader"]
pub struct R(crate::R<BSEC_JTAGIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_JTAGIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_JTAGIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_JTAGIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BSEC JTAG input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_jtagin](index.html) module"]
pub struct BSEC_JTAGIN_SPEC;
impl crate::RegisterSpec for BSEC_JTAGIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_jtagin::R](R) reader structure"]
impl crate::Readable for BSEC_JTAGIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSEC_JTAGIN to value 0"]
impl crate::Resettable for BSEC_JTAGIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
