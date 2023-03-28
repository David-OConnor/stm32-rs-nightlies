#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP_PRESENT` writer - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
pub type AP_PRESENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u16, u16, 16, O>;
#[doc = "Field `AP_ENABLED` writer - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
pub type AP_ENABLED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
    #[inline(always)]
    #[must_use]
    pub fn ap_present(&mut self) -> AP_PRESENT_W<0> {
        AP_PRESENT_W::new(self)
    }
    #[doc = "Bits 16:31 - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ap_enabled(&mut self) -> AP_ENABLED_W<16> {
        AP_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU status register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0x0001_0003"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0003;
}
