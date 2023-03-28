#[doc = "Register `AHB3SMENR` reader"]
pub struct R(crate::R<AHB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3SMENR` writer"]
pub struct W(crate::W<AHB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3SMENR_SPEC>;
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
impl From<crate::W<AHB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCSMEN` reader - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FMCSMEN_R = crate::BitReader<FMCSMEN_A>;
#[doc = "Flexible memory controller clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCSMEN_A {
    #[doc = "0: FMC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: FMC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<FMCSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCSMEN_A {
        match self.bits {
            false => FMCSMEN_A::Disabled,
            true => FMCSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCSMEN_A::Enabled
    }
}
#[doc = "Field `FMCSMEN` writer - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FMCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, FMCSMEN_A, O>;
impl<'a, const O: u8> FMCSMEN_W<'a, O> {
    #[doc = "FMC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCSMEN_A::Disabled)
    }
    #[doc = "FMC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCSMEN_A::Enabled)
    }
}
#[doc = "Field `OSPI1SMEN` reader - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
pub type OSPI1SMEN_R = crate::BitReader<OSPI1SMEN_A>;
#[doc = "OctoSPI1 memory interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1SMEN_A {
    #[doc = "0: OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OSPI1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSPI1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPI1SMEN_A {
        match self.bits {
            false => OSPI1SMEN_A::Disabled,
            true => OSPI1SMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPI1SMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPI1SMEN_A::Enabled
    }
}
#[doc = "Field `OSPI1SMEN` writer - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
pub type OSPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, OSPI1SMEN_A, O>;
impl<'a, const O: u8> OSPI1SMEN_W<'a, O> {
    #[doc = "OctoSPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSPI1SMEN_A::Disabled)
    }
    #[doc = "OctoSPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSPI1SMEN_A::Enabled)
    }
}
#[doc = "Field `OCTOSPI2` reader - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
pub type OCTOSPI2_R = crate::BitReader<OCTOSPI2_A>;
#[doc = "OctoSPI2 memory interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCTOSPI2_A {
    #[doc = "0: OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OCTOSPI2_A> for bool {
    #[inline(always)]
    fn from(variant: OCTOSPI2_A) -> Self {
        variant as u8 != 0
    }
}
impl OCTOSPI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTOSPI2_A {
        match self.bits {
            false => OCTOSPI2_A::Disabled,
            true => OCTOSPI2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OCTOSPI2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OCTOSPI2_A::Enabled
    }
}
#[doc = "Field `OCTOSPI2` writer - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
pub type OCTOSPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, OCTOSPI2_A, O>;
impl<'a, const O: u8> OCTOSPI2_W<'a, O> {
    #[doc = "OctoSPI2 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OCTOSPI2_A::Disabled)
    }
    #[doc = "OctoSPI2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OCTOSPI2_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FMCSMEN_R {
        FMCSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ospi1smen(&self) -> OSPI1SMEN_R {
        OSPI1SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn octospi2(&self) -> OCTOSPI2_R {
        OCTOSPI2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsmen(&mut self) -> FMCSMEN_W<0> {
        FMCSMEN_W::new(self)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ospi1smen(&mut self) -> OSPI1SMEN_W<8> {
        OSPI1SMEN_W::new(self)
    }
    #[doc = "Bit 9 - OctoSPI2 memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2(&mut self) -> OCTOSPI2_W<9> {
        OCTOSPI2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3smenr](index.html) module"]
pub struct AHB3SMENR_SPEC;
impl crate::RegisterSpec for AHB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3smenr::R](R) reader structure"]
impl crate::Readable for AHB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3smenr::W](W) writer structure"]
impl crate::Writable for AHB3SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3SMENR to value 0x0301"]
impl crate::Resettable for AHB3SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0301;
}
