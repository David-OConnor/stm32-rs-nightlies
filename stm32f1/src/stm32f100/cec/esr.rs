#[doc = "Register `ESR` reader"]
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BTE` reader - Bit timing error"]
pub type BTE_R = crate::BitReader<bool>;
#[doc = "Field `BPE` reader - Bit period error"]
pub type BPE_R = crate::BitReader<bool>;
#[doc = "Field `RBTFE` reader - Rx block transfer finished error"]
pub type RBTFE_R = crate::BitReader<bool>;
#[doc = "Field `SBE` reader - Start bit error"]
pub type SBE_R = crate::BitReader<bool>;
#[doc = "Field `ACKE` reader - Block acknowledge error"]
pub type ACKE_R = crate::BitReader<bool>;
#[doc = "Field `LINE` reader - Line error"]
pub type LINE_R = crate::BitReader<bool>;
#[doc = "Field `TBTFE` reader - Tx block transfer finished error"]
pub type TBTFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Bit timing error"]
    #[inline(always)]
    pub fn bte(&self) -> BTE_R {
        BTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit period error"]
    #[inline(always)]
    pub fn bpe(&self) -> BPE_R {
        BPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx block transfer finished error"]
    #[inline(always)]
    pub fn rbtfe(&self) -> RBTFE_R {
        RBTFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start bit error"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Block acknowledge error"]
    #[inline(always)]
    pub fn acke(&self) -> ACKE_R {
        ACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line error"]
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tx block transfer finished error"]
    #[inline(always)]
    pub fn tbtfe(&self) -> TBTFE_R {
        TBTFE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "CEC error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](index.html) module"]
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr::R](R) reader structure"]
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
