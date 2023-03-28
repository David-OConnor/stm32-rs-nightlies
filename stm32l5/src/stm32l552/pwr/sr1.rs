#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUF1` reader - Wakeup flag 1"]
pub type WUF1_R = crate::BitReader<bool>;
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub type WUF2_R = crate::BitReader<bool>;
#[doc = "Field `WUF3` reader - Wakeup flag 3"]
pub type WUF3_R = crate::BitReader<bool>;
#[doc = "Field `WUF4` reader - Wakeup flag 4"]
pub type WUF4_R = crate::BitReader<bool>;
#[doc = "Field `WUF5` reader - Wakeup flag 5"]
pub type WUF5_R = crate::BitReader<bool>;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader<bool>;
#[doc = "Field `SMPSBYPRDY` reader - SMPSBYPRDY"]
pub type SMPSBYPRDY_R = crate::BitReader<bool>;
#[doc = "Field `EXTSMPSRDY` reader - EXTSMPSRDY"]
pub type EXTSMPSRDY_R = crate::BitReader<bool>;
#[doc = "Field `SMPSHPRDY` reader - SMPSHPRDY"]
pub type SMPSHPRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SMPSBYPRDY"]
    #[inline(always)]
    pub fn smpsbyprdy(&self) -> SMPSBYPRDY_R {
        SMPSBYPRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTSMPSRDY"]
    #[inline(always)]
    pub fn extsmpsrdy(&self) -> EXTSMPSRDY_R {
        EXTSMPSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SMPSHPRDY"]
    #[inline(always)]
    pub fn smpshprdy(&self) -> SMPSHPRDY_R {
        SMPSHPRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
