#[doc = "Register `DDRPHYC_ZQ0SR0` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0SR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0SR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0SR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0SR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZCTRL` reader - ZCTRL"]
pub type ZCTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ZERR` reader - ZERR"]
pub type ZERR_R = crate::BitReader<bool>;
#[doc = "Field `ZDONE` reader - ZDONE"]
pub type ZDONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - ZCTRL"]
    #[inline(always)]
    pub fn zctrl(&self) -> ZCTRL_R {
        ZCTRL_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 30 - ZERR"]
    #[inline(always)]
    pub fn zerr(&self) -> ZERR_R {
        ZERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ZDONE"]
    #[inline(always)]
    pub fn zdone(&self) -> ZDONE_R {
        ZDONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DDRPHYC ZQ0S register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr0](index.html) module"]
pub struct DDRPHYC_ZQ0SR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0SR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_zq0sr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0SR0 to value 0x014a"]
impl crate::Resettable for DDRPHYC_ZQ0SR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x014a;
}
