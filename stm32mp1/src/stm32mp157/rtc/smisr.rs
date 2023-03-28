#[doc = "Register `SMISR` reader"]
pub struct R(crate::R<SMISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALRAMF` reader - ALRAMF"]
pub type ALRAMF_R = crate::BitReader<bool>;
#[doc = "Field `ALRBMF` reader - ALRBMF"]
pub type ALRBMF_R = crate::BitReader<bool>;
#[doc = "Field `WUTMF` reader - WUTMF"]
pub type WUTMF_R = crate::BitReader<bool>;
#[doc = "Field `TSMF` reader - TSMF"]
pub type TSMF_R = crate::BitReader<bool>;
#[doc = "Field `TSOVMF` reader - TSOVMF"]
pub type TSOVMF_R = crate::BitReader<bool>;
#[doc = "Field `ITSMF` reader - ITSMF"]
pub type ITSMF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ALRAMF"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBMF"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTMF"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSMF"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TSOVMF"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITSMF"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smisr](index.html) module"]
pub struct SMISR_SPEC;
impl crate::RegisterSpec for SMISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smisr::R](R) reader structure"]
impl crate::Readable for SMISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMISR to value 0"]
impl crate::Resettable for SMISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
