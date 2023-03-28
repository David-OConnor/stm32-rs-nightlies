#[doc = "Register `C1_AHB2LPENR` reader"]
pub struct R(crate::R<C1_AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB2LPENR` writer"]
pub struct W(crate::W<C1_AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB2LPENR_SPEC>;
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
impl From<crate::W<C1_AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMILPEN` reader - DCMI peripheral clock enable during csleep mode"]
pub type DCMILPEN_R = crate::BitReader<DCMILPEN_A>;
#[doc = "DCMI peripheral clock enable during csleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMILPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<DCMILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMILPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMILPEN_A {
        match self.bits {
            false => DCMILPEN_A::Disabled,
            true => DCMILPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMILPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMILPEN_A::Enabled
    }
}
#[doc = "Field `DCMILPEN` writer - DCMI peripheral clock enable during csleep mode"]
pub type DCMILPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB2LPENR_SPEC, DCMILPEN_A, O>;
impl<'a, const O: u8> DCMILPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMILPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMILPEN_A::Enabled)
    }
}
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode"]
pub use DCMILPEN_R as CRYPTLPEN_R;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode"]
pub use DCMILPEN_R as HASHLPEN_R;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode"]
pub use DCMILPEN_R as RNGLPEN_R;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SDMMC2LPEN_R;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SRAM1LPEN_R;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SRAM2LPEN_R;
#[doc = "Field `SRAM3LPEN` reader - SRAM3 Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SRAM3LPEN_R;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode"]
pub use DCMILPEN_W as CRYPTLPEN_W;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode"]
pub use DCMILPEN_W as HASHLPEN_W;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode"]
pub use DCMILPEN_W as RNGLPEN_W;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SDMMC2LPEN_W;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SRAM1LPEN_W;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SRAM2LPEN_W;
#[doc = "Field `SRAM3LPEN` writer - SRAM3 Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SRAM3LPEN_W;
impl R {
    #[doc = "Bit 0 - DCMI peripheral clock enable during csleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMI peripheral clock enable during csleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<0> {
        DCMILPEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W<4> {
        CRYPTLPEN_W::new(self)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<5> {
        HASHLPEN_W::new(self)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<6> {
        RNGLPEN_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<9> {
        SDMMC2LPEN_W::new(self)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<29> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<30> {
        SRAM2LPEN_W::new(self)
    }
    #[doc = "Bit 31 - SRAM3 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<31> {
        SRAM3LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb2lpenr](index.html) module"]
pub struct C1_AHB2LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb2lpenr::R](R) reader structure"]
impl crate::Readable for C1_AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb2lpenr::W](W) writer structure"]
impl crate::Writable for C1_AHB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB2LPENR to value 0"]
impl crate::Resettable for C1_AHB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
