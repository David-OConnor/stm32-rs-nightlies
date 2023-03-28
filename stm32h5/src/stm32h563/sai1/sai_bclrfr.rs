#[doc = "Register `SAI_BCLRFR` writer"]
pub struct W(crate::W<SAI_BCLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_BCLRFR_SPEC>;
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
impl From<crate::W<SAI_BCLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_BCLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCLRFR_SPEC, bool, O>;
#[doc = "Field `CMUTEDET` writer - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCLRFR_SPEC, bool, O>;
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCLRFR_SPEC, bool, O>;
#[doc = "Field `CCNRDY` writer - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCLRFR_SPEC, bool, O>;
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC’97or SPDIF mode. Reading this bit always returns the value 0."]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCLRFR_SPEC, bool, O>;
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC’97or SPDIF mode Reading this bit always returns the value 0."]
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_BCLRFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    #[doc = "Bit 4 - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC’97or SPDIF mode. Reading this bit always returns the value 0."]
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC’97or SPDIF mode Reading this bit always returns the value 0."]
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bclrfr](index.html) module"]
pub struct SAI_BCLRFR_SPEC;
impl crate::RegisterSpec for SAI_BCLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sai_bclrfr::W](W) writer structure"]
impl crate::Writable for SAI_BCLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_BCLRFR to value 0"]
impl crate::Resettable for SAI_BCLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
