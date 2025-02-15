#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLT1` reader - Fault 1 Interrupt Flag"]
pub type FLT1_R = crate::BitReader<bool>;
#[doc = "Field `FLT2` reader - Fault 2 Interrupt Flag"]
pub type FLT2_R = crate::BitReader<bool>;
#[doc = "Field `FLT3` reader - Fault 3 Interrupt Flag"]
pub type FLT3_R = crate::BitReader<bool>;
#[doc = "Field `FLT4` reader - Fault 4 Interrupt Flag"]
pub type FLT4_R = crate::BitReader<bool>;
#[doc = "Field `FLT5` reader - Fault 5 Interrupt Flag"]
pub type FLT5_R = crate::BitReader<bool>;
#[doc = "Field `SYSFLT` reader - System Fault Interrupt Flag"]
pub type SYSFLT_R = crate::BitReader<bool>;
#[doc = "Field `FLT6` reader - Fault 6 Interrupt Flag"]
pub type FLT6_R = crate::BitReader<bool>;
#[doc = "Field `DLLRDY` reader - DLL Ready Interrupt Flag"]
pub type DLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `BMPER` reader - Burst mode Period Interrupt Flag"]
pub type BMPER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Flag"]
    #[inline(always)]
    pub fn flt6(&self) -> FLT6_R {
        FLT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
