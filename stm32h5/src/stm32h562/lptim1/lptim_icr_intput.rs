#[doc = "Register `LPTIM_ICR_intput` writer"]
pub struct W(crate::W<LPTIM_ICR_INTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_ICR_INTPUT_SPEC>;
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
impl From<crate::W<LPTIM_ICR_INTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_ICR_INTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
pub type CC1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
pub type ARRMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
pub type EXTTRIGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
pub type ARROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `UECF` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
pub type UECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `REPOKCF` writer - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
pub type REPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC2CF` writer - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC1OCF` writer - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `CC2OCF` writer - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
#[doc = "Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
pub type DIEROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_INTPUT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cc1cf(&mut self) -> CC1CF_W<0> {
        CC1CF_W::new(self)
    }
    #[doc = "Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<1> {
        ARRMCF_W::new(self)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<2> {
        EXTTRIGCF_W::new(self)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<4> {
        ARROKCF_W::new(self)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<5> {
        UPCF_W::new(self)
    }
    #[doc = "Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<6> {
        DOWNCF_W::new(self)
    }
    #[doc = "Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn uecf(&mut self) -> UECF_W<7> {
        UECF_W::new(self)
    }
    #[doc = "Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn repokcf(&mut self) -> REPOKCF_W<8> {
        REPOKCF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2cf(&mut self) -> CC2CF_W<9> {
        CC2CF_W::new(self)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc1ocf(&mut self) -> CC1OCF_W<12> {
        CC1OCF_W::new(self)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2ocf(&mut self) -> CC2OCF_W<13> {
        CC2OCF_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn dierokcf(&mut self) -> DIEROKCF_W<24> {
        DIEROKCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_icr_intput](index.html) module"]
pub struct LPTIM_ICR_INTPUT_SPEC;
impl crate::RegisterSpec for LPTIM_ICR_INTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lptim_icr_intput::W](W) writer structure"]
impl crate::Writable for LPTIM_ICR_INTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_ICR_intput to value 0"]
impl crate::Resettable for LPTIM_ICR_INTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
