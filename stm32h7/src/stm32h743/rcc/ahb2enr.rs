#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMIEN` reader - DCMI peripheral clock"]
pub type DCMIEN_R = crate::BitReader<DCMIEN_A>;
#[doc = "DCMI peripheral clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DCMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMIEN_A {
        match self.bits {
            false => DCMIEN_A::Disabled,
            true => DCMIEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN_A::Enabled
    }
}
#[doc = "Field `DCMIEN` writer - DCMI peripheral clock"]
pub type DCMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, DCMIEN_A, O>;
impl<'a, const O: u8> DCMIEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::Enabled)
    }
}
#[doc = "Field `CRYPTEN` reader - CRYPT peripheral clock enable"]
pub use DCMIEN_R as CRYPTEN_R;
#[doc = "Field `HASHEN` reader - HASH peripheral clock enable"]
pub use DCMIEN_R as HASHEN_R;
#[doc = "Field `RNGEN` reader - RNG peripheral clocks enable"]
pub use DCMIEN_R as RNGEN_R;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable"]
pub use DCMIEN_R as SDMMC2EN_R;
#[doc = "Field `SRAM1EN` reader - SRAM1 block enable"]
pub use DCMIEN_R as SRAM1EN_R;
#[doc = "Field `SRAM2EN` reader - SRAM2 block enable"]
pub use DCMIEN_R as SRAM2EN_R;
#[doc = "Field `SRAM3EN` reader - SRAM3 block enable"]
pub use DCMIEN_R as SRAM3EN_R;
#[doc = "Field `CRYPTEN` writer - CRYPT peripheral clock enable"]
pub use DCMIEN_W as CRYPTEN_W;
#[doc = "Field `HASHEN` writer - HASH peripheral clock enable"]
pub use DCMIEN_W as HASHEN_W;
#[doc = "Field `RNGEN` writer - RNG peripheral clocks enable"]
pub use DCMIEN_W as RNGEN_W;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable"]
pub use DCMIEN_W as SDMMC2EN_W;
#[doc = "Field `SRAM1EN` writer - SRAM1 block enable"]
pub use DCMIEN_W as SRAM1EN_W;
#[doc = "Field `SRAM2EN` writer - SRAM2 block enable"]
pub use DCMIEN_W as SRAM2EN_W;
#[doc = "Field `SRAM3EN` writer - SRAM3 block enable"]
pub use DCMIEN_W as SRAM3EN_W;
impl R {
    #[doc = "Bit 0 - DCMI peripheral clock"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMI peripheral clock"]
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<0> {
        DCMIEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypten(&mut self) -> CRYPTEN_W<4> {
        CRYPTEN_W::new(self)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<5> {
        HASHEN_W::new(self)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<6> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<9> {
        SDMMC2EN_W::new(self)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<29> {
        SRAM1EN_W::new(self)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<30> {
        SRAM2EN_W::new(self)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<31> {
        SRAM3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
