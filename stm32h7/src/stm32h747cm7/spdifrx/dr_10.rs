#[doc = "Register `DR_10` reader"]
pub struct R(crate::R<DR_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRNL1` reader - Data value"]
pub type DRNL1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DRNL2` reader - Data value"]
pub type DRNL2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data value"]
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data value"]
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_10](index.html) module"]
pub struct DR_10_SPEC;
impl crate::RegisterSpec for DR_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr_10::R](R) reader structure"]
impl crate::Readable for DR_10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DR_10 to value 0"]
impl crate::Resettable for DR_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
