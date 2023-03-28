#[doc = "Register `WPABR` reader"]
pub struct R(crate::R<WPABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPABR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPABR` writer"]
pub struct W(crate::W<WPABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPABR_SPEC>;
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
impl From<crate::W<WPABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPABR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LM` reader - Latency mode"]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Latency mode"]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPABR_SPEC, bool, O>;
#[doc = "Field `WZL` reader - Write zero latency"]
pub type WZL_R = crate::BitReader<bool>;
#[doc = "Field `WZL` writer - Write zero latency"]
pub type WZL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPABR_SPEC, bool, O>;
#[doc = "Field `TACC` reader - Access time"]
pub type TACC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TACC` writer - Access time"]
pub type TACC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPABR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRWR` reader - Read write recovery time"]
pub type TRWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRWR` writer - Read write recovery time"]
pub type TRWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPABR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<0> {
        LM_W::new(self)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    #[must_use]
    pub fn wzl(&mut self) -> WZL_W<1> {
        WZL_W::new(self)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    #[must_use]
    pub fn tacc(&mut self) -> TACC_W<8> {
        TACC_W::new(self)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    #[must_use]
    pub fn trwr(&mut self) -> TRWR_W<16> {
        TRWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "write alternate bytes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpabr](index.html) module"]
pub struct WPABR_SPEC;
impl crate::RegisterSpec for WPABR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpabr::R](R) reader structure"]
impl crate::Readable for WPABR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpabr::W](W) writer structure"]
impl crate::Writable for WPABR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPABR to value 0"]
impl crate::Resettable for WPABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
