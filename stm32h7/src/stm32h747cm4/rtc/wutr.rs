#[doc = "Register `WUTR` reader"]
pub struct R(crate::R<WUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUTR` writer"]
pub struct W(crate::W<WUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUTR_SPEC>;
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
impl From<crate::W<WUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUT` reader - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
pub type WUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WUT` writer - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
pub type WUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn wut(&mut self) -> WUT_W<0> {
        WUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wutr](index.html) module"]
pub struct WUTR_SPEC;
impl crate::RegisterSpec for WUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wutr::R](R) reader structure"]
impl crate::Readable for WUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wutr::W](W) writer structure"]
impl crate::Writable for WUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUTR to value 0xffff"]
impl crate::Resettable for WUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
