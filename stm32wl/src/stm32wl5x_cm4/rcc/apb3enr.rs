#[doc = "Register `APB3ENR` reader"]
pub struct R(crate::R<APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3ENR` writer"]
pub struct W(crate::W<APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3ENR_SPEC>;
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
impl From<crate::W<APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBGHZSPIEN` reader - sub-GHz radio SPI clock enable"]
pub type SUBGHZSPIEN_R = crate::BitReader<SUBGHZSPIEN_A>;
#[doc = "sub-GHz radio SPI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHZSPIEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<SUBGHZSPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SUBGHZSPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUBGHZSPIEN_A {
        match self.bits {
            false => SUBGHZSPIEN_A::Disabled,
            true => SUBGHZSPIEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUBGHZSPIEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUBGHZSPIEN_A::Enabled
    }
}
#[doc = "Field `SUBGHZSPIEN` writer - sub-GHz radio SPI clock enable"]
pub type SUBGHZSPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, SUBGHZSPIEN_A, O>;
impl<'a, const O: u8> SUBGHZSPIEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUBGHZSPIEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUBGHZSPIEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable"]
    #[inline(always)]
    pub fn subghzspien(&self) -> SUBGHZSPIEN_R {
        SUBGHZSPIEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspien(&mut self) -> SUBGHZSPIEN_W<0> {
        SUBGHZSPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3enr](index.html) module"]
pub struct APB3ENR_SPEC;
impl crate::RegisterSpec for APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3enr::R](R) reader structure"]
impl crate::Readable for APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3enr::W](W) writer structure"]
impl crate::Writable for APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
