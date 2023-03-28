#[doc = "Register `IWDG_SR` reader"]
pub struct R(crate::R<IWDG_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PVU` reader - PVU"]
pub type PVU_R = crate::BitReader<bool>;
#[doc = "Field `RVU` reader - RVU"]
pub type RVU_R = crate::BitReader<bool>;
#[doc = "Field `WVU` reader - WVU"]
pub type WVU_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PVU"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RVU"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WVU"]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_sr](index.html) module"]
pub struct IWDG_SR_SPEC;
impl crate::RegisterSpec for IWDG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_sr::R](R) reader structure"]
impl crate::Readable for IWDG_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IWDG_SR to value 0"]
impl crate::Resettable for IWDG_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
