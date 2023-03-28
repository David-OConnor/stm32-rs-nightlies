#[doc = "Register `CRYP_SR` reader"]
pub struct R(crate::R<CRYP_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IFEM` reader - IFEM"]
pub type IFEM_R = crate::BitReader<bool>;
#[doc = "Field `IFNF` reader - IFNF"]
pub type IFNF_R = crate::BitReader<bool>;
#[doc = "Field `OFNE` reader - OFNE"]
pub type OFNE_R = crate::BitReader<bool>;
#[doc = "Field `OFFU` reader - OFFU"]
pub type OFFU_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - IFEM"]
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IFNF"]
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OFNE"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OFFU"]
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "CRYP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_sr](index.html) module"]
pub struct CRYP_SR_SPEC;
impl crate::RegisterSpec for CRYP_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_sr::R](R) reader structure"]
impl crate::Readable for CRYP_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRYP_SR to value 0x03"]
impl crate::Resettable for CRYP_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
