#[doc = "Register `AHBENR` reader"]
pub struct R(crate::R<AHBENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBENR` writer"]
pub struct W(crate::W<AHBENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBENR_SPEC>;
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
impl From<crate::W<AHBENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMA clock enable bit"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA clock enable bit"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBENR_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
#[doc = "Field `MIFEN` reader - NVM interface clock enable bit"]
pub use DMAEN_R as MIFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable bit"]
pub use DMAEN_R as CRCEN_R;
#[doc = "Field `CRYPEN` reader - Crypto clock enable bit"]
pub use DMAEN_R as CRYPEN_R;
#[doc = "Field `MIFEN` writer - NVM interface clock enable bit"]
pub use DMAEN_W as MIFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable bit"]
pub use DMAEN_W as CRCEN_W;
#[doc = "Field `CRYPEN` writer - Crypto clock enable bit"]
pub use DMAEN_W as CRYPEN_W;
impl R {
    #[doc = "Bit 0 - DMA clock enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable bit"]
    #[inline(always)]
    pub fn mifen(&self) -> MIFEN_R {
        MIFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable bit"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - Crypto clock enable bit"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 8 - NVM interface clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mifen(&mut self) -> MIFEN_W<8> {
        MIFEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 24 - Crypto clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<24> {
        CRYPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbenr](index.html) module"]
pub struct AHBENR_SPEC;
impl crate::RegisterSpec for AHBENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbenr::R](R) reader structure"]
impl crate::Readable for AHBENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbenr::W](W) writer structure"]
impl crate::Writable for AHBENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBENR to value 0x0100"]
impl crate::Resettable for AHBENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
