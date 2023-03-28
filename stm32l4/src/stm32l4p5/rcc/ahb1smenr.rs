#[doc = "Register `AHB1SMENR` reader"]
pub struct R(crate::R<AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1SMENR` writer"]
pub struct W(crate::W<AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1SMENR_SPEC>;
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
impl From<crate::W<AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_R = crate::BitReader<DMA1SMEN_A>;
#[doc = "DMA1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN_A {
    #[doc = "0: DMAx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DMA1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1SMEN_A {
        match self.bits {
            false => DMA1SMEN_A::Disabled,
            true => DMA1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1SMEN_A::Enabled
    }
}
#[doc = "Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, DMA1SMEN_A, O>;
impl<'a, const O: u8> DMA1SMEN_W<'a, O> {
    #[doc = "DMAx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1SMEN_A::Disabled)
    }
    #[doc = "DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1SMEN_A::Enabled)
    }
}
#[doc = "Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes"]
pub use DMA1SMEN_R as DMA2SMEN_R;
#[doc = "Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes"]
pub use DMA1SMEN_W as DMA2SMEN_W;
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX clock enable during Sleep and Stop modes"]
pub type DMAMUX1SMEN_R = crate::BitReader<DMAMUX1SMEN_A>;
#[doc = "DMAMUX clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1SMEN_A {
    #[doc = "0: DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DMAMUX1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1SMEN_A {
        match self.bits {
            false => DMAMUX1SMEN_A::Disabled,
            true => DMAMUX1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAMUX1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAMUX1SMEN_A::Enabled
    }
}
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX clock enable during Sleep and Stop modes"]
pub type DMAMUX1SMEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB1SMENR_SPEC, DMAMUX1SMEN_A, O>;
impl<'a, const O: u8> DMAMUX1SMEN_W<'a, O> {
    #[doc = "DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAMUX1SMEN_A::Disabled)
    }
    #[doc = "DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAMUX1SMEN_A::Enabled)
    }
}
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FLASHSMEN_R = crate::BitReader<FLASHSMEN_A>;
#[doc = "Flash memory interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHSMEN_A {
    #[doc = "0: Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<FLASHSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHSMEN_A {
        match self.bits {
            false => FLASHSMEN_A::Disabled,
            true => FLASHSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHSMEN_A::Enabled
    }
}
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, FLASHSMEN_A, O>;
impl<'a, const O: u8> FLASHSMEN_W<'a, O> {
    #[doc = "Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHSMEN_A::Disabled)
    }
    #[doc = "Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHSMEN_A::Enabled)
    }
}
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_R = crate::BitReader<SRAM1SMEN_A>;
#[doc = "SRAM1 interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SMEN_A {
    #[doc = "0: SRAM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SRAM1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1SMEN_A {
        match self.bits {
            false => SRAM1SMEN_A::Disabled,
            true => SRAM1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM1SMEN_A::Enabled
    }
}
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, SRAM1SMEN_A, O>;
impl<'a, const O: u8> SRAM1SMEN_W<'a, O> {
    #[doc = "SRAM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM1SMEN_A::Disabled)
    }
    #[doc = "SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM1SMEN_A::Enabled)
    }
}
#[doc = "Field `CRCSMEN` reader - CRCSMEN"]
pub type CRCSMEN_R = crate::BitReader<CRCSMEN_A>;
#[doc = "CRCSMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN_A {
    #[doc = "0: CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: CRC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<CRCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSMEN_A {
        match self.bits {
            false => CRCSMEN_A::Disabled,
            true => CRCSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCSMEN_A::Enabled
    }
}
#[doc = "Field `CRCSMEN` writer - CRCSMEN"]
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, CRCSMEN_A, O>;
impl<'a, const O: u8> CRCSMEN_W<'a, O> {
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCSMEN_A::Disabled)
    }
    #[doc = "CRC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCSMEN_A::Enabled)
    }
}
#[doc = "Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_R = crate::BitReader<TSCSMEN_A>;
#[doc = "Touch Sensing Controller clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCSMEN_A {
    #[doc = "0: TSC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: TSC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<TSCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCSMEN_A {
        match self.bits {
            false => TSCSMEN_A::Disabled,
            true => TSCSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCSMEN_A::Enabled
    }
}
#[doc = "Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, TSCSMEN_A, O>;
impl<'a, const O: u8> TSCSMEN_W<'a, O> {
    #[doc = "TSC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSCSMEN_A::Disabled)
    }
    #[doc = "TSC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSCSMEN_A::Enabled)
    }
}
#[doc = "Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes"]
pub type DMA2DSMEN_R = crate::BitReader<DMA2DSMEN_A>;
#[doc = "DMA2D clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DSMEN_A {
    #[doc = "0: DMA2D clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DMA2DSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2DSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2DSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2DSMEN_A {
        match self.bits {
            false => DMA2DSMEN_A::Disabled,
            true => DMA2DSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA2DSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA2DSMEN_A::Enabled
    }
}
#[doc = "Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes"]
pub type DMA2DSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, DMA2DSMEN_A, O>;
impl<'a, const O: u8> DMA2DSMEN_W<'a, O> {
    #[doc = "DMA2D clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2DSMEN_A::Disabled)
    }
    #[doc = "DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2DSMEN_A::Enabled)
    }
}
#[doc = "Field `GFXMMUSMEN` reader - GFXMMU clock enable during Sleep and Stop modes"]
pub type GFXMMUSMEN_R = crate::BitReader<GFXMMUSMEN_A>;
#[doc = "GFXMMU clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMUSMEN_A {
    #[doc = "0: GFXMMU clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<GFXMMUSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GFXMMUSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFXMMUSMEN_A {
        match self.bits {
            false => GFXMMUSMEN_A::Disabled,
            true => GFXMMUSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GFXMMUSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GFXMMUSMEN_A::Enabled
    }
}
#[doc = "Field `GFXMMUSMEN` writer - GFXMMU clock enable during Sleep and Stop modes"]
pub type GFXMMUSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1SMENR_SPEC, GFXMMUSMEN_A, O>;
impl<'a, const O: u8> GFXMMUSMEN_W<'a, O> {
    #[doc = "GFXMMU clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GFXMMUSMEN_A::Disabled)
    }
    #[doc = "GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GFXMMUSMEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GFXMMU clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gfxmmusmen(&self) -> GFXMMUSMEN_R {
        GFXMMUSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<0> {
        DMA1SMEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<1> {
        DMA2SMEN_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<2> {
        DMAMUX1SMEN_W::new(self)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<8> {
        FLASHSMEN_W::new(self)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<9> {
        SRAM1SMEN_W::new(self)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<16> {
        TSCSMEN_W::new(self)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<17> {
        DMA2DSMEN_W::new(self)
    }
    #[doc = "Bit 18 - GFXMMU clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmusmen(&mut self) -> GFXMMUSMEN_W<18> {
        GFXMMUSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1smenr](index.html) module"]
pub struct AHB1SMENR_SPEC;
impl crate::RegisterSpec for AHB1SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1smenr::R](R) reader structure"]
impl crate::Readable for AHB1SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1smenr::W](W) writer structure"]
impl crate::Writable for AHB1SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1SMENR to value 0x0007_1307"]
impl crate::Resettable for AHB1SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_1307;
}
