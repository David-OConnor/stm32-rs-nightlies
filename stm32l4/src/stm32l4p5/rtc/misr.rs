#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISR` writer"]
pub struct W(crate::W<MISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISR_SPEC>;
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
impl From<crate::W<MISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRAMF` reader - Alarm A masked flag"]
pub type ALRAMF_R = crate::BitReader<bool>;
#[doc = "Field `ALRAMF` writer - Alarm A masked flag"]
pub type ALRAMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
#[doc = "Field `ALRBMF` reader - Alarm B masked flag"]
pub type ALRBMF_R = crate::BitReader<bool>;
#[doc = "Field `ALRBMF` writer - Alarm B masked flag"]
pub type ALRBMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag"]
pub type WUTMF_R = crate::BitReader<bool>;
#[doc = "Field `WUTMF` writer - Wakeup timer masked flag"]
pub type WUTMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
#[doc = "Field `TSMF` reader - Timestamp masked flag"]
pub type TSMF_R = crate::BitReader<bool>;
#[doc = "Field `TSMF` writer - Timestamp masked flag"]
pub type TSMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag"]
pub type TSOVMF_R = crate::BitReader<bool>;
#[doc = "Field `TSOVMF` writer - Timestamp overflow masked flag"]
pub type TSOVMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag"]
pub type ITSMF_R = crate::BitReader<bool>;
#[doc = "Field `ITSMF` writer - Internal timestamp masked flag"]
pub type ITSMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
#[doc = "Field `SSRUMF` reader - SSR underflow masked flag"]
pub type SSRUMF_R = crate::BitReader<bool>;
#[doc = "Field `SSRUMF` writer - SSR underflow masked flag"]
pub type SSRUMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn alramf(&mut self) -> ALRAMF_W<0> {
        ALRAMF_W::new(self)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrbmf(&mut self) -> ALRBMF_W<1> {
        ALRBMF_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn wutmf(&mut self) -> WUTMF_W<2> {
        WUTMF_W::new(self)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsmf(&mut self) -> TSMF_W<3> {
        TSMF_W::new(self)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsovmf(&mut self) -> TSOVMF_W<4> {
        TSOVMF_W::new(self)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn itsmf(&mut self) -> ITSMF_W<5> {
        ITSMF_W::new(self)
    }
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssrumf(&mut self) -> SSRUMF_W<6> {
        SSRUMF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misr::W](W) writer structure"]
impl crate::Writable for MISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
