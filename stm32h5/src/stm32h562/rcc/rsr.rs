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
pub type RMVF_R = crate::BitReader<bool>;
#[doc = "Field `RMVF` writer - remove reset flag Set and reset by software to reset the value of the reset flags."]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `PINRSTF` reader - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub type PINRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PINRSTF` writer - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub type PINRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `BORRSTF` reader - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub type BORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `BORRSTF` writer - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub type BORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `SFTRSTF` reader - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
pub type SFTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTF` writer - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
pub type SFTRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `IWDGRSTF` reader - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub type IWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `IWDGRSTF` writer - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub type IWDGRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `WWDGRSTF` reader - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub type WWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRSTF` writer - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub type WWDGRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
pub type LPWRRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
pub type LPWRRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 23 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    #[doc = "Bit 26 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<26> {
        PINRSTF_W::new(self)
    }
    #[doc = "Bit 27 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<27> {
        BORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<28> {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 29 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<29> {
        IWDGRSTF_W::new(self)
    }
    #[doc = "Bit 30 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<30> {
        WWDGRSTF_W::new(self)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<31> {
        LPWRRSTF_W::new(self)
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
#[doc = "`reset()` method sets RSR to value 0x0c00_0000"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
