#[doc = "Register `AHB3ENR` reader"]
pub struct R(crate::R<AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3ENR` writer"]
pub struct W(crate::W<AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3ENR_SPEC>;
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
impl From<crate::W<AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCEN` reader - Flexible memory controller clock enable"]
pub type FMCEN_R = crate::BitReader<FMCEN_A>;
#[doc = "Flexible memory controller clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCEN_A {
    #[doc = "0: FMC clock disabled"]
    Disabled = 0,
    #[doc = "1: FMC clock enabled"]
    Enabled = 1,
}
impl From<FMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCEN_A {
        match self.bits {
            false => FMCEN_A::Disabled,
            true => FMCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCEN_A::Enabled
    }
}
#[doc = "Field `FMCEN` writer - Flexible memory controller clock enable"]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, FMCEN_A, O>;
impl<'a, const O: u8> FMCEN_W<'a, O> {
    #[doc = "FMC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCEN_A::Disabled)
    }
    #[doc = "FMC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCEN_A::Enabled)
    }
}
#[doc = "Field `OSPI1EN` reader - OctoSPI1 memory interface clock enable"]
pub type OSPI1EN_R = crate::BitReader<OSPI1EN_A>;
#[doc = "OctoSPI1 memory interface clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1EN_A {
    #[doc = "0: OctoSPI x clock disabled"]
    Disabled = 0,
    #[doc = "1: OctoSPI x clock enabled"]
    Enabled = 1,
}
impl From<OSPI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: OSPI1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPI1EN_A {
        match self.bits {
            false => OSPI1EN_A::Disabled,
            true => OSPI1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPI1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPI1EN_A::Enabled
    }
}
#[doc = "Field `OSPI1EN` writer - OctoSPI1 memory interface clock enable"]
pub type OSPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, OSPI1EN_A, O>;
impl<'a, const O: u8> OSPI1EN_W<'a, O> {
    #[doc = "OctoSPI x clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSPI1EN_A::Disabled)
    }
    #[doc = "OctoSPI x clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSPI1EN_A::Enabled)
    }
}
#[doc = "Field `OSPI2EN` reader - OSPI2EN memory interface clock enable"]
pub use OSPI1EN_R as OSPI2EN_R;
#[doc = "Field `OSPI2EN` writer - OSPI2EN memory interface clock enable"]
pub use OSPI1EN_W as OSPI2EN_W;
impl R {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clock enable"]
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OSPI2EN memory interface clock enable"]
    #[inline(always)]
    pub fn ospi2en(&self) -> OSPI2EN_R {
        OSPI2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<0> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 8 - OctoSPI1 memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ospi1en(&mut self) -> OSPI1EN_W<8> {
        OSPI1EN_W::new(self)
    }
    #[doc = "Bit 9 - OSPI2EN memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ospi2en(&mut self) -> OSPI2EN_W<9> {
        OSPI2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](index.html) module"]
pub struct AHB3ENR_SPEC;
impl crate::RegisterSpec for AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3enr::R](R) reader structure"]
impl crate::Readable for AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](W) writer structure"]
impl crate::Writable for AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
