#[doc = "Register `SMCR` reader"]
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCR` writer"]
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRADPROT` reader - ALRADPROT"]
pub type ALRADPROT_R = crate::BitReader<bool>;
#[doc = "Field `ALRADPROT` writer - ALRADPROT"]
pub type ALRADPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `ALRBDPROT` reader - ALRBDPROT"]
pub type ALRBDPROT_R = crate::BitReader<bool>;
#[doc = "Field `ALRBDPROT` writer - ALRBDPROT"]
pub type ALRBDPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `WUTDPROT` reader - WUTDPROT"]
pub type WUTDPROT_R = crate::BitReader<bool>;
#[doc = "Field `WUTDPROT` writer - WUTDPROT"]
pub type WUTDPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `TSDPROT` reader - TSDPROT"]
pub type TSDPROT_R = crate::BitReader<bool>;
#[doc = "Field `TSDPROT` writer - TSDPROT"]
pub type TSDPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `CALDPROT` reader - CALDPROT"]
pub type CALDPROT_R = crate::BitReader<bool>;
#[doc = "Field `CALDPROT` writer - CALDPROT"]
pub type CALDPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `INITDPROT` reader - INITDPROT"]
pub type INITDPROT_R = crate::BitReader<bool>;
#[doc = "Field `INITDPROT` writer - INITDPROT"]
pub type INITDPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `DECPROT` reader - DECPROT"]
pub type DECPROT_R = crate::BitReader<bool>;
#[doc = "Field `DECPROT` writer - DECPROT"]
pub type DECPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    pub fn alradprot(&self) -> ALRADPROT_R {
        ALRADPROT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    pub fn alrbdprot(&self) -> ALRBDPROT_R {
        ALRBDPROT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    pub fn wutdprot(&self) -> WUTDPROT_R {
        WUTDPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    pub fn tsdprot(&self) -> TSDPROT_R {
        TSDPROT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    pub fn caldprot(&self) -> CALDPROT_R {
        CALDPROT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    pub fn initdprot(&self) -> INITDPROT_R {
        INITDPROT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    pub fn decprot(&self) -> DECPROT_R {
        DECPROT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    #[must_use]
    pub fn alradprot(&mut self) -> ALRADPROT_W<0> {
        ALRADPROT_W::new(self)
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn alrbdprot(&mut self) -> ALRBDPROT_W<1> {
        ALRBDPROT_W::new(self)
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn wutdprot(&mut self) -> WUTDPROT_W<2> {
        WUTDPROT_W::new(self)
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn tsdprot(&mut self) -> TSDPROT_W<3> {
        TSDPROT_W::new(self)
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn caldprot(&mut self) -> CALDPROT_W<13> {
        CALDPROT_W::new(self)
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn initdprot(&mut self) -> INITDPROT_W<14> {
        INITDPROT_W::new(self)
    }
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    #[must_use]
    pub fn decprot(&mut self) -> DECPROT_W<15> {
        DECPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be written only when the APB access is secure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](index.html) module"]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcr::R](R) reader structure"]
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcr::W](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCR to value 0xe00f"]
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xe00f;
}
