#[doc = "Register `GPSR` reader"]
pub struct R(crate::R<GPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDFE` reader - CMDFE"]
pub type CMDFE_R = crate::BitReader<bool>;
#[doc = "Field `CMDFF` reader - CMDFF"]
pub type CMDFF_R = crate::BitReader<bool>;
#[doc = "Field `PWRFE` reader - PWRFE"]
pub type PWRFE_R = crate::BitReader<bool>;
#[doc = "Field `PWRFF` reader - PWRFF"]
pub type PWRFF_R = crate::BitReader<bool>;
#[doc = "Field `PRDFE` reader - PRDFE"]
pub type PRDFE_R = crate::BitReader<bool>;
#[doc = "Field `PRDFF` reader - PRDFF"]
pub type PRDFF_R = crate::BitReader<bool>;
#[doc = "Field `RCB` reader - RCB"]
pub type RCB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CMDFE"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMDFF"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "DSI Host generic packet status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpsr](index.html) module"]
pub struct GPSR_SPEC;
impl crate::RegisterSpec for GPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpsr::R](R) reader structure"]
impl crate::Readable for GPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPSR to value 0x15"]
impl crate::Resettable for GPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x15;
}
