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
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
#[doc = "DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::Disabled,
            true => DMA1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBENR_SPEC, DMA1EN_A, O>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DMA1EN_R as SRAMEN_R;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub use DMA1EN_R as FLITFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `FSMCEN` reader - FSMC clock enable"]
pub use DMA1EN_R as FSMCEN_R;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub use DMA1EN_R as SDIOEN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DMA1EN_W as SRAMEN_W;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub use DMA1EN_W as FLITFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
#[doc = "Field `FSMCEN` writer - FSMC clock enable"]
pub use DMA1EN_W as FSMCEN_W;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub use DMA1EN_W as SDIOEN_W;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
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
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<8> {
        FSMCEN_W::new(self)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<10> {
        SDIOEN_W::new(self)
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
