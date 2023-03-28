#[doc = "Register `SUBGHZSPICR` reader"]
pub struct R(crate::R<SUBGHZSPICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBGHZSPICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBGHZSPICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBGHZSPICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBGHZSPICR` writer"]
pub struct W(crate::W<SUBGHZSPICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBGHZSPICR_SPEC>;
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
impl From<crate::W<SUBGHZSPICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBGHZSPICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSS` reader - sub-GHz SPI NSS control"]
pub type NSS_R = crate::BitReader<NSS_A>;
#[doc = "sub-GHz SPI NSS control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSS_A {
    #[doc = "0: Sub-GHz SPI NSS signal at level low"]
    Low = 0,
    #[doc = "1: Sub-GHz SPI NSS signal is at level high"]
    High = 1,
}
impl From<NSS_A> for bool {
    #[inline(always)]
    fn from(variant: NSS_A) -> Self {
        variant as u8 != 0
    }
}
impl NSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSS_A {
        match self.bits {
            false => NSS_A::Low,
            true => NSS_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NSS_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NSS_A::High
    }
}
#[doc = "Field `NSS` writer - sub-GHz SPI NSS control"]
pub type NSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUBGHZSPICR_SPEC, NSS_A, O>;
impl<'a, const O: u8> NSS_W<'a, O> {
    #[doc = "Sub-GHz SPI NSS signal at level low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NSS_A::Low)
    }
    #[doc = "Sub-GHz SPI NSS signal is at level high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NSS_A::High)
    }
}
impl R {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<15> {
        NSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power SPI3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subghzspicr](index.html) module"]
pub struct SUBGHZSPICR_SPEC;
impl crate::RegisterSpec for SUBGHZSPICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subghzspicr::R](R) reader structure"]
impl crate::Readable for SUBGHZSPICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subghzspicr::W](W) writer structure"]
impl crate::Writable for SUBGHZSPICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUBGHZSPICR to value 0x8000"]
impl crate::Resettable for SUBGHZSPICR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
