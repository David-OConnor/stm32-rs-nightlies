#[doc = "Register `RSR` reader"]
pub struct R(crate::R<RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSR` writer"]
pub struct W(crate::W<RSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSR_SPEC>;
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
impl From<crate::W<RSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMVF` reader - remove reset flag Set and reset by software to reset the value of the reset flags."]
pub type RMVF_R = crate::BitReader<RMVF_A>;
#[doc = "remove reset flag Set and reset by software to reset the value of the reset flags.\n\nValue on reset: 0"]
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
#[doc = "Field `RMVF` writer - remove reset flag Set and reset by software to reset the value of the reset flags."]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, RMVF_A, O>;
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
#[doc = "Field `CDRSTF` reader - CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)"]
pub type CDRSTF_R = crate::BitReader<CDRSTFR_A>;
#[doc = "CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRSTFR_A {
    #[doc = "0: No reset occoured for block"]
    NoResetOccoured = 0,
    #[doc = "1: Reset occoured for block"]
    ResetOccourred = 1,
}
impl From<CDRSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: CDRSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CDRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDRSTFR_A {
        match self.bits {
            false => CDRSTFR_A::NoResetOccoured,
            true => CDRSTFR_A::ResetOccourred,
        }
    }
    #[doc = "Checks if the value of the field is `NoResetOccoured`"]
    #[inline(always)]
    pub fn is_no_reset_occoured(&self) -> bool {
        *self == CDRSTFR_A::NoResetOccoured
    }
    #[doc = "Checks if the value of the field is `ResetOccourred`"]
    #[inline(always)]
    pub fn is_reset_occourred(&self) -> bool {
        *self == CDRSTFR_A::ResetOccourred
    }
}
#[doc = "Field `BORRSTF` reader - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub use CDRSTF_R as BORRSTF_R;
#[doc = "Field `PINRSTF` reader - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub use CDRSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs."]
pub use CDRSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7."]
pub use CDRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub use CDRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub use CDRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRRSTF` reader - reset due to illegal CD DStop or CD DStop2 or CPU CStop flag Reset by software by writing the RMVF bit. Set by hardware when the CPU domain goes erroneously in DStop or DStop2, or when the CPU goes erroneously in CStop."]
pub use CDRSTF_R as LPWRRSTF_R;
impl R {
    #[doc = "Bit 16 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)"]
    #[inline(always)]
    pub fn cdrstf(&self) -> CDRSTF_R {
        CDRSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs."]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - reset due to illegal CD DStop or CD DStop2 or CPU CStop flag Reset by software by writing the RMVF bit. Set by hardware when the CPU domain goes erroneously in DStop or DStop2, or when the CPU goes erroneously in CStop."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<16> {
        RMVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](index.html) module"]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr::R](R) reader structure"]
impl crate::Readable for RSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsr::W](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSR to value 0x00e8_0000"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00e8_0000;
}
