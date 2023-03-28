#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub type CWUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub type CWUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub type CWUF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4"]
pub type CWUF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5"]
pub type CWUF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CSMPSFBF` writer - Clear SMPS Step Down converter forced in Bypass interrupt flag"]
pub type CSMPSFBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CBORHF` writer - Clear BORH interrupt flag"]
pub type CBORHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CBLEWUF` writer - Clear BLE wakeup interrupt flag"]
pub type CBLEWUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `C802WUF` writer - Clear 802.15.4 wakeup interrupt flag"]
pub type C802WUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CCRPEF` writer - Clear critical radio phase end of activity interrupt flag"]
pub type CCRPEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CBLEAF` writer - Clear BLE end of activity interrupt flag"]
pub type CBLEAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `C802AF` writer - Clear 802.15.4 end of activity interrupt flag"]
pub type C802AF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `CC2HF` writer - Clear CPU2 Hold interrupt flag"]
pub type CC2HF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<0> {
        CWUF1_W::new(self)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<1> {
        CWUF2_W::new(self)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<2> {
        CWUF3_W::new(self)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<3> {
        CWUF4_W::new(self)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<4> {
        CWUF5_W::new(self)
    }
    #[doc = "Bit 7 - Clear SMPS Step Down converter forced in Bypass interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn csmpsfbf(&mut self) -> CSMPSFBF_W<7> {
        CSMPSFBF_W::new(self)
    }
    #[doc = "Bit 8 - Clear BORH interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cborhf(&mut self) -> CBORHF_W<8> {
        CBORHF_W::new(self)
    }
    #[doc = "Bit 9 - Clear BLE wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cblewuf(&mut self) -> CBLEWUF_W<9> {
        CBLEWUF_W::new(self)
    }
    #[doc = "Bit 10 - Clear 802.15.4 wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c802wuf(&mut self) -> C802WUF_W<10> {
        C802WUF_W::new(self)
    }
    #[doc = "Bit 11 - Clear critical radio phase end of activity interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccrpef(&mut self) -> CCRPEF_W<11> {
        CCRPEF_W::new(self)
    }
    #[doc = "Bit 12 - Clear BLE end of activity interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbleaf(&mut self) -> CBLEAF_W<12> {
        CBLEAF_W::new(self)
    }
    #[doc = "Bit 13 - Clear 802.15.4 end of activity interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c802af(&mut self) -> C802AF_W<13> {
        C802AF_W::new(self)
    }
    #[doc = "Bit 14 - Clear CPU2 Hold interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2hf(&mut self) -> CC2HF_W<14> {
        CC2HF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
