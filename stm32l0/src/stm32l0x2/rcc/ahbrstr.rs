#[doc = "Register `AHBRSTR` reader"]
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRSTR` writer"]
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARST` reader - DMA reset"]
pub type DMARST_R = crate::BitReader<DMARSTW_A>;
#[doc = "DMA reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARSTW_A {
    #[doc = "1: Reset the module"]
    Reset = 1,
}
impl From<DMARSTW_A> for bool {
    #[inline(always)]
    fn from(variant: DMARSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl DMARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMARSTW_A> {
        match self.bits {
            true => Some(DMARSTW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMARSTW_A::Reset
    }
}
#[doc = "Field `DMARST` writer - DMA reset"]
pub type DMARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, DMARSTW_A, O>;
impl<'a, const O: u8> DMARST_W<'a, O> {
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMARSTW_A::Reset)
    }
}
#[doc = "Field `MIFRST` reader - Memory interface reset"]
pub use DMARST_R as MIFRST_R;
#[doc = "Field `CRCRST` reader - Test integration module reset"]
pub use DMARST_R as CRCRST_R;
#[doc = "Field `TOUCHRST` reader - Touch Sensing reset"]
pub use DMARST_R as TOUCHRST_R;
#[doc = "Field `RNGRST` reader - Random Number Generator module reset"]
pub use DMARST_R as RNGRST_R;
#[doc = "Field `CRYPRST` reader - Crypto module reset"]
pub use DMARST_R as CRYPRST_R;
#[doc = "Field `MIFRST` writer - Memory interface reset"]
pub use DMARST_W as MIFRST_W;
#[doc = "Field `CRCRST` writer - Test integration module reset"]
pub use DMARST_W as CRCRST_W;
#[doc = "Field `TOUCHRST` writer - Touch Sensing reset"]
pub use DMARST_W as TOUCHRST_W;
#[doc = "Field `RNGRST` writer - Random Number Generator module reset"]
pub use DMARST_W as RNGRST_W;
#[doc = "Field `CRYPRST` writer - Crypto module reset"]
pub use DMARST_W as CRYPRST_W;
impl R {
    #[doc = "Bit 0 - DMA reset"]
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Memory interface reset"]
    #[inline(always)]
    pub fn mifrst(&self) -> MIFRST_R {
        MIFRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Test integration module reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing reset"]
    #[inline(always)]
    pub fn touchrst(&self) -> TOUCHRST_R {
        TOUCHRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Random Number Generator module reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Crypto module reset"]
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA reset"]
    #[inline(always)]
    #[must_use]
    pub fn dmarst(&mut self) -> DMARST_W<0> {
        DMARST_W::new(self)
    }
    #[doc = "Bit 8 - Memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn mifrst(&mut self) -> MIFRST_W<8> {
        MIFRST_W::new(self)
    }
    #[doc = "Bit 12 - Test integration module reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 16 - Touch Sensing reset"]
    #[inline(always)]
    #[must_use]
    pub fn touchrst(&mut self) -> TOUCHRST_W<16> {
        TOUCHRST_W::new(self)
    }
    #[doc = "Bit 20 - Random Number Generator module reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<20> {
        RNGRST_W::new(self)
    }
    #[doc = "Bit 24 - Crypto module reset"]
    #[inline(always)]
    #[must_use]
    pub fn cryprst(&mut self) -> CRYPRST_W<24> {
        CRYPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrstr](index.html) module"]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrstr::R](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrstr::W](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
