#[doc = "Register `FDCAN_TTLGT` reader"]
pub struct R(crate::R<FDCAN_TTLGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTLGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTLGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTLGT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LT` reader - LT"]
pub type LT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GT` reader - GT"]
pub type GT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - LT"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - GT"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT local and global time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttlgt](index.html) module"]
pub struct FDCAN_TTLGT_SPEC;
impl crate::RegisterSpec for FDCAN_TTLGT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttlgt::R](R) reader structure"]
impl crate::Readable for FDCAN_TTLGT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TTLGT to value 0"]
impl crate::Resettable for FDCAN_TTLGT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
