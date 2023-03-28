#[doc = "Register `ITLINE23` reader"]
pub struct R(crate::R<ITLINE23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C1` reader - I2C1"]
pub type I2C1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 23 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline23](index.html) module"]
pub struct ITLINE23_SPEC;
impl crate::RegisterSpec for ITLINE23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline23::R](R) reader structure"]
impl crate::Readable for ITLINE23_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE23 to value 0"]
impl crate::Resettable for ITLINE23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
