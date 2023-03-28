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
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
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
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBENR_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DMAEN_R as SRAMEN_R;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub use DMAEN_R as FLITFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMAEN_R as CRCEN_R;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub use DMAEN_R as IOPAEN_R;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub use DMAEN_R as IOPBEN_R;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub use DMAEN_R as IOPCEN_R;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub use DMAEN_R as IOPDEN_R;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub use DMAEN_R as IOPFEN_R;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DMAEN_W as SRAMEN_W;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub use DMAEN_W as FLITFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMAEN_W as CRCEN_W;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub use DMAEN_W as IOPAEN_W;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub use DMAEN_W as IOPBEN_W;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub use DMAEN_W as IOPCEN_W;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub use DMAEN_W as IOPDEN_W;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub use DMAEN_W as IOPFEN_W;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<4> {
        FLITFEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<17> {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<18> {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<19> {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<20> {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IOPFEN_W<22> {
        IOPFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbenr](index.html) module"]
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
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AHBENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
