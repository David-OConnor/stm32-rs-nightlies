#[doc = "Register `AHBSMENR` reader"]
pub struct R(crate::R<AHBSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBSMENR` writer"]
pub struct W(crate::W<AHBSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBSMENR_SPEC>;
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
impl From<crate::W<AHBSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBSMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASMEN` reader - DMA clock enable during sleep mode bit"]
pub type DMASMEN_R = crate::BitReader<DMASMEN_A>;
#[doc = "DMA clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMASMEN_A {
    #[doc = "0: DMA clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: DMA clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<DMASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMASMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASMEN_A {
        match self.bits {
            false => DMASMEN_A::Disabled,
            true => DMASMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMASMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMASMEN_A::Enabled
    }
}
#[doc = "Field `DMASMEN` writer - DMA clock enable during sleep mode bit"]
pub type DMASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, DMASMEN_A, O>;
impl<'a, const O: u8> DMASMEN_W<'a, O> {
    #[doc = "DMA clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMASMEN_A::Disabled)
    }
    #[doc = "DMA clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMASMEN_A::Enabled)
    }
}
#[doc = "Field `MIFSMEN` reader - NVM interface clock enable during sleep mode bit"]
pub type MIFSMEN_R = crate::BitReader<MIFSMEN_A>;
#[doc = "NVM interface clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIFSMEN_A {
    #[doc = "0: NVM interface clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: NVM interface clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<MIFSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: MIFSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MIFSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIFSMEN_A {
        match self.bits {
            false => MIFSMEN_A::Disabled,
            true => MIFSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MIFSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MIFSMEN_A::Enabled
    }
}
#[doc = "Field `MIFSMEN` writer - NVM interface clock enable during sleep mode bit"]
pub type MIFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, MIFSMEN_A, O>;
impl<'a, const O: u8> MIFSMEN_W<'a, O> {
    #[doc = "NVM interface clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MIFSMEN_A::Disabled)
    }
    #[doc = "NVM interface clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MIFSMEN_A::Enabled)
    }
}
#[doc = "Field `SRAMSMEN` reader - SRAM interface clock enable during sleep mode bit"]
pub type SRAMSMEN_R = crate::BitReader<SRAMSMEN_A>;
#[doc = "SRAM interface clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMSMEN_A {
    #[doc = "0: NVM interface clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: NVM interface clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<SRAMSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAMSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSMEN_A {
        match self.bits {
            false => SRAMSMEN_A::Disabled,
            true => SRAMSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAMSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAMSMEN_A::Enabled
    }
}
#[doc = "Field `SRAMSMEN` writer - SRAM interface clock enable during sleep mode bit"]
pub type SRAMSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, SRAMSMEN_A, O>;
impl<'a, const O: u8> SRAMSMEN_W<'a, O> {
    #[doc = "NVM interface clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAMSMEN_A::Disabled)
    }
    #[doc = "NVM interface clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAMSMEN_A::Enabled)
    }
}
#[doc = "Field `CRCSMEN` reader - CRC clock enable during sleep mode bit"]
pub type CRCSMEN_R = crate::BitReader<CRCSMEN_A>;
#[doc = "CRC clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN_A {
    #[doc = "0: Test integration module clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: Test integration module clock enabled in Sleep mode (if enabled by CRCEN)"]
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
#[doc = "Field `CRCSMEN` writer - CRC clock enable during sleep mode bit"]
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, CRCSMEN_A, O>;
impl<'a, const O: u8> CRCSMEN_W<'a, O> {
    #[doc = "Test integration module clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCSMEN_A::Disabled)
    }
    #[doc = "Test integration module clock enabled in Sleep mode (if enabled by CRCEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCSMEN_A::Enabled)
    }
}
#[doc = "Field `CRYPSMEN` reader - Crypto clock enable during sleep mode bit"]
pub type CRYPSMEN_R = crate::BitReader<CRYPSMEN_A>;
#[doc = "Crypto clock enable during sleep mode bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRYPSMEN_A {
    #[doc = "0: Crypto clock disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: Crypto clock enabled in Sleep mode"]
    Enabled = 1,
}
impl From<CRYPSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRYPSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPSMEN_A {
        match self.bits {
            false => CRYPSMEN_A::Disabled,
            true => CRYPSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRYPSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRYPSMEN_A::Enabled
    }
}
#[doc = "Field `CRYPSMEN` writer - Crypto clock enable during sleep mode bit"]
pub type CRYPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, CRYPSMEN_A, O>;
impl<'a, const O: u8> CRYPSMEN_W<'a, O> {
    #[doc = "Crypto clock disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRYPSMEN_A::Disabled)
    }
    #[doc = "Crypto clock enabled in Sleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRYPSMEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn mifsmen(&self) -> MIFSMEN_R {
        MIFSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 24 - Crypto clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crypsmen(&self) -> CRYPSMEN_R {
        CRYPSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmasmen(&mut self) -> DMASMEN_W<0> {
        DMASMEN_W::new(self)
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn mifsmen(&mut self) -> MIFSMEN_W<8> {
        MIFSMEN_W::new(self)
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<9> {
        SRAMSMEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    #[doc = "Bit 24 - Crypto clock enable during sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn crypsmen(&mut self) -> CRYPSMEN_W<24> {
        CRYPSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbsmenr](index.html) module"]
pub struct AHBSMENR_SPEC;
impl crate::RegisterSpec for AHBSMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbsmenr::R](R) reader structure"]
impl crate::Readable for AHBSMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbsmenr::W](W) writer structure"]
impl crate::Writable for AHBSMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBSMENR to value 0x0111_1301"]
impl crate::Resettable for AHBSMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0111_1301;
}
