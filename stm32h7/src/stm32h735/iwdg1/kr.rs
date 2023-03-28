#[doc = "Register `KR` writer"]
pub struct W(crate::W<KR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KR_SPEC>;
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
impl From<crate::W<KR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section23.3.6: Register access protection) Writing the key value CCCCh starts the watchdog (except if the hardware watchdog option is selected)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum KEY_AW {
    #[doc = "21845: Enable access to PR, RLR and WINR registers (0x5555)"]
    Enable = 21845,
    #[doc = "43690: Reset the watchdog value (0xAAAA)"]
    Reset = 43690,
    #[doc = "52428: Start the watchdog (0xCCCC)"]
    Start = 52428,
}
impl From<KEY_AW> for u16 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section23.3.6: Register access protection) Writing the key value CCCCh starts the watchdog (except if the hardware watchdog option is selected)"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KR_SPEC, u16, KEY_AW, 16, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(KEY_AW::Enable)
    }
    #[doc = "Reset the watchdog value (0xAAAA)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEY_AW::Reset)
    }
    #[doc = "Start the watchdog (0xCCCC)"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(KEY_AW::Start)
    }
}
impl W {
    #[doc = "Bits 0:15 - Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section23.3.6: Register access protection) Writing the key value CCCCh starts the watchdog (except if the hardware watchdog option is selected)"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kr](index.html) module"]
pub struct KR_SPEC;
impl crate::RegisterSpec for KR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [kr::W](W) writer structure"]
impl crate::Writable for KR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
