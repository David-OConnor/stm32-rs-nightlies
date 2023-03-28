#[doc = "Register `WUSR` reader"]
pub struct R(crate::R<WUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUF1` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF1_R = crate::BitReader<bool>;
#[doc = "Field `WUF2` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF2_R = crate::BitReader<bool>;
#[doc = "Field `WUF3` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF3_R = crate::BitReader<bool>;
#[doc = "Field `WUF4` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF4_R = crate::BitReader<bool>;
#[doc = "Field `WUF5` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF5_R = crate::BitReader<bool>;
#[doc = "Field `WUF6` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF6_R = crate::BitReader<bool>;
#[doc = "Field `WUF7` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF7_R = crate::BitReader<bool>;
#[doc = "Field `WUF8` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF8_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PWR wakeup status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wusr](index.html) module"]
pub struct WUSR_SPEC;
impl crate::RegisterSpec for WUSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wusr::R](R) reader structure"]
impl crate::Readable for WUSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WUSR to value 0"]
impl crate::Resettable for WUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
