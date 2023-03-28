#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODR3` reader - Port output data (y = 0..15)"]
pub type ODR3_R = crate::BitReader<ODR3_A>;
#[doc = "Port output data (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODR3_A {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<ODR3_A> for bool {
    #[inline(always)]
    fn from(variant: ODR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ODR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODR3_A {
        match self.bits {
            false => ODR3_A::Low,
            true => ODR3_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ODR3_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ODR3_A::High
    }
}
#[doc = "Field `ODR3` writer - Port output data (y = 0..15)"]
pub type ODR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, ODR3_A, O>;
impl<'a, const O: u8> ODR3_W<'a, O> {
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR3_A::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR3_A::High)
    }
}
impl R {
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<3> {
        ODR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
