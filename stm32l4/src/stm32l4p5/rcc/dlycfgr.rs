#[doc = "Register `DLYCFGR` reader"]
pub struct R(crate::R<DLYCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYCFGR` writer"]
pub struct W(crate::W<DLYCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYCFGR_SPEC>;
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
impl From<crate::W<DLYCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCTOSPI1_DLY` reader - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
pub type OCTOSPI1_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCTOSPI1_DLY` writer - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
pub type OCTOSPI1_DLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DLYCFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `OCTOSPI2_DLY` reader - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
pub type OCTOSPI2_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCTOSPI2_DLY` writer - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
pub type OCTOSPI2_DLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DLYCFGR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
    #[inline(always)]
    pub fn octospi1_dly(&self) -> OCTOSPI1_DLY_R {
        OCTOSPI1_DLY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
    #[inline(always)]
    pub fn octospi2_dly(&self) -> OCTOSPI2_DLY_R {
        OCTOSPI2_DLY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Delay sampling configuration on OCTOSPI1 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_dly(&mut self) -> OCTOSPI1_DLY_W<0> {
        OCTOSPI1_DLY_W::new(self)
    }
    #[doc = "Bits 4:7 - Delay sampling configuration on OCTOSPI2 to be used for internal sampling clock (called feedback clock) or for DQS data strobe"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2_dly(&mut self) -> OCTOSPI2_DLY_W<4> {
        OCTOSPI2_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "delay configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlycfgr](index.html) module"]
pub struct DLYCFGR_SPEC;
impl crate::RegisterSpec for DLYCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlycfgr::R](R) reader structure"]
impl crate::Readable for DLYCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlycfgr::W](W) writer structure"]
impl crate::Writable for DLYCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYCFGR to value 0"]
impl crate::Resettable for DLYCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
