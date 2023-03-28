#[doc = "Register `C1_RSR` reader"]
pub struct R(crate::R<C1_RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_RSR` writer"]
pub struct W(crate::W<C1_RSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<C1_RSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_RSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVF_A>;
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF_A {
    #[doc = "0: Not clearing the the reset flags"]
    NotActive = 0,
    #[doc = "1: Clear the reset flags"]
    Clear = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMVF_A {
        match self.bits {
            false => RMVF_A::NotActive,
            true => RMVF_A::Clear,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == RMVF_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF_A::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_RSR_SPEC, RMVF_A, O>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    #[doc = "Not clearing the the reset flags"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(RMVF_A::NotActive)
    }
    #[doc = "Clear the reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::Clear)
    }
}
#[doc = "Field `CPURSTF` reader - CPU reset flag"]
pub type CPURSTF_R = crate::BitReader<CPURSTFR_A>;
#[doc = "CPU reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPURSTFR_A {
    #[doc = "0: No reset occoured for block"]
    NoResetOccoured = 0,
    #[doc = "1: Reset occoured for block"]
    ResetOccourred = 1,
}
impl From<CPURSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: CPURSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CPURSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPURSTFR_A {
        match self.bits {
            false => CPURSTFR_A::NoResetOccoured,
            true => CPURSTFR_A::ResetOccourred,
        }
    }
    #[doc = "Checks if the value of the field is `NoResetOccoured`"]
    #[inline(always)]
    pub fn is_no_reset_occoured(&self) -> bool {
        *self == CPURSTFR_A::NoResetOccoured
    }
    #[doc = "Checks if the value of the field is `ResetOccourred`"]
    #[inline(always)]
    pub fn is_reset_occourred(&self) -> bool {
        *self == CPURSTFR_A::ResetOccourred
    }
}
#[doc = "Field `CPURSTF` writer - CPU reset flag"]
pub type CPURSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_RSR_SPEC, CPURSTFR_A, O>;
impl<'a, const O: u8> CPURSTF_W<'a, O> {
    #[doc = "No reset occoured for block"]
    #[inline(always)]
    pub fn no_reset_occoured(self) -> &'a mut W {
        self.variant(CPURSTFR_A::NoResetOccoured)
    }
    #[doc = "Reset occoured for block"]
    #[inline(always)]
    pub fn reset_occourred(self) -> &'a mut W {
        self.variant(CPURSTFR_A::ResetOccourred)
    }
}
#[doc = "Field `D1RSTF` reader - D1 domain power switch reset flag"]
pub use CPURSTF_R as D1RSTF_R;
#[doc = "Field `D2RSTF` reader - D2 domain power switch reset flag"]
pub use CPURSTF_R as D2RSTF_R;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub use CPURSTF_R as BORRSTF_R;
#[doc = "Field `PINRSTF` reader - Pin reset flag (NRST)"]
pub use CPURSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub use CPURSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - System reset from CPU reset flag"]
pub use CPURSTF_R as SFTRSTF_R;
#[doc = "Field `IWDG1RSTF` reader - Independent Watchdog reset flag"]
pub use CPURSTF_R as IWDG1RSTF_R;
#[doc = "Field `WWDG1RSTF` reader - Window Watchdog reset flag"]
pub use CPURSTF_R as WWDG1RSTF_R;
#[doc = "Field `LPWRRSTF` reader - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub use CPURSTF_R as LPWRRSTF_R;
#[doc = "Field `D1RSTF` writer - D1 domain power switch reset flag"]
pub use CPURSTF_W as D1RSTF_W;
#[doc = "Field `D2RSTF` writer - D2 domain power switch reset flag"]
pub use CPURSTF_W as D2RSTF_W;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub use CPURSTF_W as BORRSTF_W;
#[doc = "Field `PINRSTF` writer - Pin reset flag (NRST)"]
pub use CPURSTF_W as PINRSTF_W;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub use CPURSTF_W as PORRSTF_W;
#[doc = "Field `SFTRSTF` writer - System reset from CPU reset flag"]
pub use CPURSTF_W as SFTRSTF_W;
#[doc = "Field `IWDG1RSTF` writer - Independent Watchdog reset flag"]
pub use CPURSTF_W as IWDG1RSTF_W;
#[doc = "Field `WWDG1RSTF` writer - Window Watchdog reset flag"]
pub use CPURSTF_W as WWDG1RSTF_W;
#[doc = "Field `LPWRRSTF` writer - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub use CPURSTF_W as LPWRRSTF_W;
impl R {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&self) -> CPURSTF_R {
        CPURSTF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&self) -> D1RSTF_R {
        D1RSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&self) -> D2RSTF_R {
        D2RSTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<16> {
        RMVF_W::new(self)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn cpurstf(&mut self) -> CPURSTF_W<17> {
        CPURSTF_W::new(self)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn d1rstf(&mut self) -> D1RSTF_W<19> {
        D1RSTF_W::new(self)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn d2rstf(&mut self) -> D2RSTF_W<20> {
        D2RSTF_W::new(self)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<21> {
        BORRSTF_W::new(self)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<22> {
        PINRSTF_W::new(self)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<23> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<24> {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<26> {
        IWDG1RSTF_W::new(self)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W<28> {
        WWDG1RSTF_W::new(self)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<30> {
        LPWRRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_rsr](index.html) module"]
pub struct C1_RSR_SPEC;
impl crate::RegisterSpec for C1_RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_rsr::R](R) reader structure"]
impl crate::Readable for C1_RSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_rsr::W](W) writer structure"]
impl crate::Writable for C1_RSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_RSR to value 0"]
impl crate::Resettable for C1_RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
